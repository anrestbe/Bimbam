use crate::{error::*, Span};

use std::iter::FromIterator;

mod engine;
mod integer_bits;
mod type_info;
pub use engine::*;
pub use integer_bits::*;
use serde::{
    de::{SeqAccess, Visitor},
    ser::SerializeSeq,
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::fmt;
use sway_types::Property;
pub use type_info::*;

/// A identifier to uniquely refer to our type terms
#[derive(Eq, Hash, PartialEq, Ord, PartialOrd, Clone, Copy, Debug, Default)]
pub struct TypeId(pub usize);

impl fmt::Display for TypeId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Serialize for TypeId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let ty = look_up_type_id(*self);
        let encoded = bincode::serialize(&ty).expect("failed to serialize type");
        let mut seq = serializer.serialize_seq(Some(encoded.len()))?;
        for val in encoded {
            seq.serialize_element(&val)?;
        }
        seq.end()
    }
}

struct TypeIdDeserializer;

impl<'de> Visitor<'de> for TypeIdDeserializer {
    type Value = TypeId;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("A bincoded representation of TypeInfo.")
    }
    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut bincoded_bytes = Vec::new();
        while let Some(byte) = seq.next_element()? {
            bincoded_bytes.push(byte);
        }

        let ty: TypeInfo =
            bincode::deserialize(&bincoded_bytes[..]).expect("Invalid typeinfo in cache");

        Ok(insert_type(ty))
    }
}

impl<'de> Deserialize<'de> for TypeId {
    fn deserialize<D>(deserializer: D) -> Result<TypeId, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(TypeIdDeserializer)
    }
}

pub(crate) trait JsonAbiString {
    fn json_abi_str(&self) -> String;
}

impl JsonAbiString for TypeId {
    fn json_abi_str(&self) -> String {
        look_up_type_id(*self).json_abi_str()
    }
}

pub(crate) trait FriendlyTypeString {
    fn friendly_type_str(&self) -> String;
}

impl FriendlyTypeString for TypeId {
    fn friendly_type_str(&self) -> String {
        look_up_type_id(*self).friendly_type_str()
    }
}

pub(crate) trait ToJsonAbi {
    fn generate_json_abi(&self) -> Option<Vec<Property>>;
}

impl ToJsonAbi for TypeId {
    fn generate_json_abi(&self) -> Option<Vec<Property>> {
        match look_up_type_id(*self) {
            TypeInfo::Struct { fields, .. } => {
                Some(fields.iter().map(|x| x.generate_json_abi()).collect())
            }
            TypeInfo::Enum { variant_types, .. } => Some(
                variant_types
                    .iter()
                    .map(|x| x.generate_json_abi())
                    .collect(),
            ),
            _ => None,
        }
    }
}

#[test]
fn basic_numeric_unknown() {
    let engine = Engine::default();

    let sp = Span {
        span: pest::Span::new(" ".into(), 0, 0).unwrap(),
        path: None,
    };
    // numerics
    let id = engine.insert_type(TypeInfo::Numeric);
    let id2 = engine.insert_type(TypeInfo::UnsignedInteger(IntegerBits::Eight));

    // Unify them together...
    engine.unify(id, id2, &sp).unwrap();

    assert_eq!(
        engine.resolve_type(id, &sp).unwrap(),
        TypeInfo::UnsignedInteger(IntegerBits::Eight)
    );
}
#[test]
fn chain_of_refs() {
    let engine = Engine::default();
    let sp = Span {
        span: pest::Span::new(" ".into(), 0, 0).unwrap(),
        path: None,
    };
    // numerics
    let id = engine.insert_type(TypeInfo::Numeric);
    let id2 = engine.insert_type(TypeInfo::Ref(id));
    let id3 = engine.insert_type(TypeInfo::Ref(id));
    let id4 = engine.insert_type(TypeInfo::UnsignedInteger(IntegerBits::Eight));

    // Unify them together...
    engine.unify(id4, id2, &sp).unwrap();

    assert_eq!(
        engine.resolve_type(id3, &sp).unwrap(),
        TypeInfo::UnsignedInteger(IntegerBits::Eight)
    );
}
#[test]
fn chain_of_refs_2() {
    let engine = Engine::default();
    let sp = Span {
        span: pest::Span::new(" ".into(), 0, 0).unwrap(),
        path: None,
    };
    // numerics
    let id = engine.insert_type(TypeInfo::Numeric);
    let id2 = engine.insert_type(TypeInfo::Ref(id));
    let id3 = engine.insert_type(TypeInfo::Ref(id));
    let id4 = engine.insert_type(TypeInfo::UnsignedInteger(IntegerBits::Eight));

    // Unify them together...
    engine.unify(id2, id4, &sp).unwrap();

    assert_eq!(
        engine.resolve_type(id3, &sp).unwrap(),
        TypeInfo::UnsignedInteger(IntegerBits::Eight)
    );
}

fn parse_str_type(raw: &str, span: Span) -> CompileResult<TypeInfo> {
    if raw.starts_with("str[") {
        let mut rest = raw.split_at("str[".len()).1.chars().collect::<Vec<_>>();
        if let Some(']') = rest.pop() {
            if let Ok(num) = String::from_iter(rest).parse() {
                return ok(TypeInfo::Str(num), vec![], vec![]);
            }
        }
        return err(
            vec![],
            vec![CompileError::InvalidStrType {
                raw: raw.to_string(),
                span,
            }],
        );
    }
    err(vec![], vec![CompileError::UnknownType { span }])
}

#[test]
fn test_str_parse() {
    match parse_str_type(
        "str[20]",
        Span {
            span: pest::Span::new("".into(), 0, 0).unwrap(),
            path: None,
        },
    )
    .value
    {
        Some(value) if value == TypeInfo::Str(20) => (),
        _ => panic!("failed test"),
    }
    match parse_str_type(
        "str[]",
        Span {
            span: pest::Span::new("".into(), 0, 0).unwrap(),
            path: None,
        },
    )
    .value
    {
        None => (),
        _ => panic!("failed test"),
    }
    match parse_str_type(
        "str[ab]",
        Span {
            span: pest::Span::new("".into(), 0, 0).unwrap(),
            path: None,
        },
    )
    .value
    {
        None => (),
        _ => panic!("failed test"),
    }
    match parse_str_type(
        "str [ab]",
        Span {
            span: pest::Span::new("".into(), 0, 0).unwrap(),
            path: None,
        },
    )
    .value
    {
        None => (),
        _ => panic!("failed test"),
    }

    match parse_str_type(
        "not even a str[ type",
        Span {
            span: pest::Span::new("".into(), 0, 0).unwrap(),
            path: None,
        },
    )
    .value
    {
        None => (),
        _ => panic!("failed test"),
    }
    match parse_str_type(
        "",
        Span {
            span: pest::Span::new("".into(), 0, 0).unwrap(),
            path: None,
        },
    )
    .value
    {
        None => (),
        _ => panic!("failed test"),
    }
    match parse_str_type(
        "20",
        Span {
            span: pest::Span::new("".into(), 0, 0).unwrap(),
            path: None,
        },
    )
    .value
    {
        None => (),
        _ => panic!("failed test"),
    }
    match parse_str_type(
        "[20]",
        Span {
            span: pest::Span::new("".into(), 0, 0).unwrap(),
            path: None,
        },
    )
    .value
    {
        None => (),
        _ => panic!("failed test"),
    }
}

use super::*;
use crate::parse_tree::AsmOp;
use crate::semantic_analysis::ast_node::*;
use crate::Ident;

#[derive(Clone, Debug)]
pub(crate) struct ContractCallMetadata {
    pub(crate) func_selector: [u8; 4],
    pub(crate) contract_address: Box<TypedExpression>,
}

#[derive(Clone, Debug)]
pub(crate) enum TypedExpressionVariant {
    Literal(Literal),
    FunctionApplication {
        name: CallPath,
        arguments: Vec<(Ident, TypedExpression)>,
        function_body: TypedCodeBlock,
        /// If this is `Some(val)` then `val` is the metadata. If this is `None`, then
        /// there is no selector.
        selector: Option<ContractCallMetadata>,
    },
    LazyOperator {
        op: LazyOp,
        lhs: Box<TypedExpression>,
        rhs: Box<TypedExpression>,
    },
    VariableExpression {
        name: Ident,
    },
    Unit,
    #[allow(dead_code)]
    Array {
        contents: Vec<TypedExpression>,
    },
    #[allow(dead_code)]
    MatchExpression {
        primary_expression: Box<TypedExpression>,
        branches: Vec<TypedMatchBranch>,
    },
    StructExpression {
        struct_name: Ident,
        fields: Vec<TypedStructExpressionField>,
    },
    CodeBlock(TypedCodeBlock),
    // a flag that this value will later be provided as a parameter, but is currently unknown
    FunctionParameter,
    IfExp {
        condition: Box<TypedExpression>,
        then: Box<TypedExpression>,
        r#else: Option<Box<TypedExpression>>,
    },
    AsmExpression {
        registers: Vec<TypedAsmRegisterDeclaration>,
        body: Vec<AsmOp>,
        returns: Option<(AsmRegister, Span)>,
        whole_block_span: Span,
    },
    // like a variable expression but it has multiple parts,
    // like looking up a field in a struct
    StructFieldAccess {
        prefix: Box<TypedExpression>,
        field_to_access: TypedStructField,
        resolved_type_of_parent: TypeId,
    },
    EnumInstantiation {
        /// for printing
        enum_decl: TypedEnumDeclaration,
        /// for printing
        variant_name: Ident,
        tag: usize,
        contents: Option<Box<TypedExpression>>,
    },
    AbiCast {
        abi_name: CallPath,
        address: Box<TypedExpression>,
        span: Span,
        abi: TypedAbiDeclaration,
    },
}

#[derive(Clone, Debug)]
pub(crate) struct TypedAsmRegisterDeclaration {
    pub(crate) initializer: Option<TypedExpression>,
    pub(crate) name: crate::Span,
}

impl TypedExpressionVariant {
    pub(crate) fn pretty_print(&self, type_engine: &crate::type_engine::Engine) -> String {
        match self {
            TypedExpressionVariant::Literal(lit) => format!(
                "literal {}",
                match lit {
                    Literal::U8(content) => content.to_string(),
                    Literal::U16(content) => content.to_string(),
                    Literal::U32(content) => content.to_string(),
                    Literal::U64(content) => content.to_string(),
                    Literal::String(content) => content.to_string(),
                    Literal::Boolean(content) => content.to_string(),
                    Literal::Byte(content) => content.to_string(),
                    Literal::B256(content) => content
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>()
                        .join(", "),
                }
            ),
            TypedExpressionVariant::FunctionApplication { name, .. } => {
                format!("\"{}\" fn entry", name.suffix.as_str())
            }
            TypedExpressionVariant::LazyOperator { op, .. } => match op {
                LazyOp::And => "&&".into(),
                LazyOp::Or => "||".into(),
            },
            TypedExpressionVariant::Unit => "unit".into(),
            TypedExpressionVariant::Array { .. } => "array".into(),
            TypedExpressionVariant::MatchExpression { .. } => "match exp".into(),
            TypedExpressionVariant::StructExpression { struct_name, .. } => {
                format!("\"{}\" struct init", struct_name.as_str())
            }
            TypedExpressionVariant::CodeBlock(_) => "code block entry".into(),
            TypedExpressionVariant::FunctionParameter => "fn param access".into(),
            TypedExpressionVariant::IfExp { .. } => "if exp".into(),
            TypedExpressionVariant::AsmExpression { .. } => "inline asm".into(),
            TypedExpressionVariant::AbiCast { abi_name, .. } => {
                format!("abi cast {}", abi_name.suffix.as_str())
            }
            TypedExpressionVariant::StructFieldAccess {
                resolved_type_of_parent,
                field_to_access,
                ..
            } => {
                format!(
                    "\"{}.{}\" struct field access",
                    todo!(
                        "engine
                        .look_up_type_id(resolved_type_of_parent)
                        .friendly_type_str(),"
                    ),
                    field_to_access.name.as_str()
                )
            }
            TypedExpressionVariant::VariableExpression { name, .. } => {
                format!("\"{}\" variable exp", name.as_str())
            }
            TypedExpressionVariant::EnumInstantiation {
                tag,
                enum_decl,
                variant_name,
                ..
            } => {
                format!(
                    "{}::{} enum instantiation (tag: {})",
                    enum_decl.name.as_str(), variant_name.as_str(), tag
                )
            }
        }
    }
}

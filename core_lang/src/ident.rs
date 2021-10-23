use crate::build_config::BuildConfig;
use crate::error::*;
use crate::parser::Rule;
use crate::span::Span;
use pest::iterators::Pair;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
pub struct Ident {
    // sub-names are the stuff after periods
    // like x.test.thing.method()
    // `test`, `thing`, and `method` are sub-names
    // the primary name is `x`
    pub span: Span,
}

// custom implementation of Hash so that namespacing isn't reliant on the span itself, which will
// often be different.
impl Hash for Ident {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_str().hash(state);
    }
}
impl PartialEq for Ident {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}

impl Eq for Ident {}

impl Ident {
    pub(crate) fn as_str(&self) -> &str {
        self.span.as_str()
    }
    pub(crate) fn parse_from_pair<'sc>(
        pair: Pair<'sc, Rule>,
        config: Option<&BuildConfig>,
    ) -> CompileResult< Ident> {
        let path = if let Some(config) = config {
            Some(config.path())
        } else {
            None
        };
        let span = {
            let pair = pair.clone();
            if pair.as_rule() != Rule::ident {
                let pair = pair.into_inner().next().unwrap();
                let sp = pair.as_span();
                Span::new_from_file(path, sp.input(), sp.start(), sp.end())
            } else {
                let sp = pair.as_span();
                Span::new_from_file(path, sp.input(), sp.start(), sp.end())
            }
        };
        let name = pair.as_str().trim();
        ok(
            Ident {

                span,
            },
            Vec::new(),
            Vec::new(),
        )
    }
}

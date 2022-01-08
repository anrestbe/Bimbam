use crate::build_config::BuildConfig;
use crate::parser::Rule;
use crate::span::Span;
use crate::{
    error::{ok, CompileResult},
    CodeBlock, Expression,
};
use pest::iterators::Pair;

/// A parsed while loop. Contains the `condition`, which is defined from an [Expression], and the `body` from a [CodeBlock].
#[derive(Debug, Clone)]
pub struct LoopControlFlow {
    control_type: LoopControlType,
}

#[derive(Debug, Clone)]
enum LoopControlType {
    Break,
    Continue,
}

impl LoopControlFlow {
    pub(crate) fn parse_from_pair(
        pair: Pair<Rule>,
        config: Option<&BuildConfig>,
    ) -> CompileResult<Self> {
        todo!();
    }
}

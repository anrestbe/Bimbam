use crate::Rule;
use pest::iterators::Pair;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Visibility {
    Private,
    Public,
}

impl Visibility {
    pub(crate) fn parse_from_pair(input: Pair<Rule>) -> Self {
        match input.as_str().trim() {
            "pub" => Visibility::Public,
            _ => Visibility::Private,
        }
    }
}

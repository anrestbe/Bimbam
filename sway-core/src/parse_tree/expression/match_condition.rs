use crate::Span;

use super::scrutinee::Scrutinee;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) enum MatchCondition {
    CatchAll(CatchAll),
    Scrutinee(Scrutinee),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatchAll {
    pub span: Span,
}

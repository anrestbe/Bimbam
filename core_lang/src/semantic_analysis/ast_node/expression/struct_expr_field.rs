use crate::semantic_analysis::TypedExpression;
use crate::Ident;

#[derive(Clone, Debug)]
pub(crate) struct TypedStructExpressionField {
    pub(crate) name: Ident,
    pub(crate) value: TypedExpression,
}

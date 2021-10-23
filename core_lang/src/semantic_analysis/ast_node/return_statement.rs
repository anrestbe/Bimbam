use super::TypedExpression;

#[derive(Clone, Debug)]
pub(crate) struct TypedReturnStatement {
    pub(crate) expr: TypedExpression,
}

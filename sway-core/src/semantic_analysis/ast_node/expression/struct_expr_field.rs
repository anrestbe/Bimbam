use crate::semantic_analysis::TypedExpression;
use crate::Ident;
use crate::{type_engine::TypeId, TypeParameter};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct TypedStructExpressionField {
    pub(crate) name: Ident,
    pub(crate) value: TypedExpression,
}

impl TypedStructExpressionField {
    pub(crate) fn copy_types(&mut self, type_mapping: &[(TypeParameter, TypeId)]) {
        self.value.copy_types(type_mapping);
    }
}

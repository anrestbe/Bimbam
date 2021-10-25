use core_lang::{Span, VariableDeclaration, Visibility};

use crate::core::token_type::VarBody;
use core_lang::Expression;

pub(crate) fn extract_visibility(visibility: &Visibility) -> String {
    match visibility {
        Visibility::Private => "".into(),
        Visibility::Public => "pub ".into(),
    }
}

pub(crate) fn extract_var_body(var_dec: &VariableDeclaration) -> VarBody {
    match &var_dec.body {
        Expression::FunctionApplication {
            name,
            arguments: _,
            span: _,
        } => VarBody::FunctionCall(name.suffix.primary_name.into()),
        Expression::StructExpression {
            struct_name,
            fields: _,
            span: _,
        } => VarBody::Type(struct_name.primary_name.into()),
        _ => VarBody::Other,
    }
}

pub(crate) fn extract_file_path(span: &Span) -> Option<String> {
    match &span.path {
        Some(path) => {
            if let Some(file_path) = path.as_os_str().to_str() {
                Some(file_path.into())
            } else {
                None
            }
        }
        _ => None,
    }
}

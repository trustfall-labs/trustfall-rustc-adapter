use trustfall::{FieldValue, provider::{AsVertex, ContextIterator, ContextOutcomeIterator, ResolveInfo}};

use super::vertex::Vertex;

pub(super) fn resolve_ty_property<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    property_name: &str,
    _resolve_info: &ResolveInfo,
) -> ContextOutcomeIterator<'a, V, FieldValue> {
    match property_name {
        "as_string" => {
            todo!("implement property 'as_string' in fn `resolve_ty_property()`")
        }
        _ => {
            unreachable!(
                "attempted to read unexpected property '{property_name}' on type 'Ty'"
            )
        }
    }
}

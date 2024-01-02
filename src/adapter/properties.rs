use rustc_interface::run_compiler;
use trustfall::{FieldValue, provider::{AsVertex, ContextIterator, ContextOutcomeIterator, ResolveInfo, resolve_property_with}};

use super::{vertex::Vertex, Adapter};

pub(super) fn resolve_fn_property<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    property_name: &str,
    _resolve_info: &ResolveInfo,
) -> ContextOutcomeIterator<'a, V, FieldValue> {
    match property_name {
        "ident" => todo!("implement property 'ident' in fn `resolve_fn_property()`"),
        _ => {
            unreachable!(
                "attempted to read unexpected property '{property_name}' on type 'Fn'"
            )
        }
    }
}

pub(super) fn resolve_item_property<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    property_name: &str,
    _resolve_info: &ResolveInfo,
    adapter: &'a Adapter
) -> ContextOutcomeIterator<'a, V, FieldValue> {
    match property_name {
        "ident" => resolve_property_with(
            contexts,
            move |vertex| {
                let Vertex::Item(item_id) = vertex else {
                    unimplemented!("")
                };
                let ident: String = run_compiler((&adapter.config).clone().into(), move |compiler| {
                    compiler.enter(move |queries| {
                        queries.global_ctxt().unwrap().enter(move |ctxt| {
                            let hir = ctxt.hir();
                            hir.item(*item_id).ident.as_str().into()
                        })
                    })
                });
                FieldValue::String(ident.into())
            },
        ),
        _ => {
            unreachable!(
                "attempted to read unexpected property '{property_name}' on type 'Item'"
            )
        }
    }
}

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

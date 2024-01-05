use rustc_interface::run_compiler;
use trustfall::{FieldValue, provider::{AsVertex, ContextIterator, ContextOutcomeIterator, ResolveInfo, resolve_property_with}};

use super::{vertex::Vertex, Adapter};

pub(super) fn resolve_fn_property<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    property_name: &str,
    _resolve_info: &ResolveInfo,
    adapter: &'a Adapter,
) -> ContextOutcomeIterator<'a, V, FieldValue> {
    match property_name {
        "ident" => resolve_property_with(
            contexts,
            move |vertex| {
                if let Some(item_id) = vertex.item_id() {
                    shared::resolve_ident(item_id, adapter)
                } else {
                    unimplemented!("{vertex:?} is not an Item")
                }
            },
        ),
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
    adapter: &'a Adapter,
) -> ContextOutcomeIterator<'a, V, FieldValue> {
    match property_name {
        "ident" => resolve_property_with(
            contexts,
            move |vertex| {
                if let Some(item_id) = vertex.item_id() {
                    shared::resolve_ident(item_id, adapter)
                } else {
                    unimplemented!("{vertex:?} is not an Item")
                }
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
    adapter: &'a Adapter,
) -> ContextOutcomeIterator<'a, V, FieldValue> {
    match property_name {
        "as_string" => resolve_property_with(
            contexts,
            move |vertex| {
                let Vertex::Ty(hir_id) = vertex else {
                    unimplemented!("vertex is not Ty: {vertex:#?}")
                };
                let string: String = run_compiler((&adapter.config).clone().into(), move |compiler| {
                    compiler.enter(move |queries| {
                        queries.global_ctxt().unwrap().enter(move |ctxt| {
                            let hir = ctxt.hir();
                            let def_id = hir.enclosing_body_owner(*hir_id);
                            ctxt.typeck(def_id).node_type(*hir_id).to_string()
                        })
                    })
                });
                FieldValue::String(string.into())
            },
        ),
        _ => {
            unreachable!(
                "attempted to read unexpected property '{property_name}' on type 'Ty'"
            )
        }
    }
}

pub(crate) mod shared {
    use rustc_hir::ItemId;
    use rustc_interface::run_compiler;
    use trustfall::FieldValue;

    use crate::adapter::Adapter;

    pub fn resolve_ident<'a>(
        item_id: ItemId,
        adapter: &'a Adapter,
    ) -> FieldValue {
        let ident: String = run_compiler((&adapter.config).clone().into(), move |compiler| {
            compiler.enter(move |queries| {
                queries.global_ctxt().unwrap().enter(move |ctxt| {
                    let hir = ctxt.hir();
                    hir.item(item_id).ident.as_str().into()
                })
            })
        });
        FieldValue::String(ident.into())
    }
}

use rustc_attr::StabilityLevel;
use trustfall::{FieldValue, provider::{AsVertex, ContextIterator, ContextOutcomeIterator, ResolveInfo, resolve_property_with}};

use super::{vertex::Vertex, Adapter};

pub(super) fn resolve_def_property<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    property_name: &str,
    _resolve_info: &ResolveInfo,
    adapter: &'a Adapter,
) -> ContextOutcomeIterator<'a, V, FieldValue> {
    match property_name {
        "ident" => resolve_property_with(
            contexts,
            move |vertex| {
                if let Some(def_id) = vertex.def_id() {
                    adapter.queries.global_ctxt().unwrap().enter(move |ctxt| {
                        if let Some(ident) = ctxt.opt_item_ident(def_id) {
                            FieldValue::String(ident.as_str().into())
                        } else {
                            FieldValue::NULL
                        }
                    })
                } else {
                    unimplemented!("{vertex:?} is not a Def")
                }
            },
        ),
        "path" => resolve_property_with(
            contexts,
            move |vertex| {
                if let Some(def_id) = vertex.def_id() {
                    adapter.queries.global_ctxt().unwrap().enter(move |ctxt| {
                        FieldValue::String(ctxt.def_path_str(def_id).into())
                    })
                } else {
                    unimplemented!("{vertex:?} is not a Def")
                }
            },
        ),
        _ => {
            unreachable!(
                "attempted to read unexpected property '{property_name}' on type 'Def'"
            )
        }
    }
}

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

pub(super) fn resolve_inside_const_context_property<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    _property_name: &str,
    _resolve_info: &ResolveInfo,
    adapter: &'a Adapter,
) -> ContextOutcomeIterator<'a, V, FieldValue> {
    resolve_property_with(
        contexts,
        move |vertex| {
            let hir_id = vertex
                .hir_id()
                .expect("vertex is not a 'Node'");

            let inside = adapter.queries.global_ctxt().unwrap().enter(move |ctxt| {
                let hir = ctxt.hir();
                hir.is_inside_const_context(hir_id)
            });

            FieldValue::Boolean(inside)
        },
    )
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



pub(super) fn resolve_node_property<'a, V: AsVertex<Vertex> + 'a>(
    _contexts: ContextIterator<'a, V>,
    property_name: &str,
    _resolve_info: &ResolveInfo,
    _adapter: &'a Adapter,
) -> ContextOutcomeIterator<'a, V, FieldValue> {
    match property_name {
        
        _ => {
            unreachable!(
                "attempted to read unexpected property '{property_name}' on type 'Node'"
            )
        }
    }
}

pub(super) fn resolve_stability_property<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    property_name: &str,
    _resolve_info: &ResolveInfo,
    adapter: &'a Adapter,
) -> ContextOutcomeIterator<'a, V, FieldValue> {
    match property_name {
        "feature" => resolve_property_with(
            contexts,
            move |vertex| {
                let def_id = vertex
                    .as_stability()
                    .expect("vertex is not variant 'Stability'");

                let string = adapter.queries.global_ctxt().unwrap().enter(move |ctxt| {
                    ctxt
                        .lookup_stability(def_id)
                        .expect("def_id should have a Stability struct")
                        .feature
                        .to_string()
                });

                FieldValue::String(string.into())
            },
        ),
        "stable" => resolve_property_with(
            contexts,
            move |vertex| {
                let def_id = vertex
                    .as_stability()
                    .expect("vertex is not variant 'Stability'");

                let stable = adapter.queries.global_ctxt().unwrap().enter(move |ctxt| {
                    ctxt
                        .lookup_stability(def_id)
                        .expect("def_id should have a Stability struct")
                        .is_stable()
                });

                FieldValue::Boolean(stable)
            },
        ),
        "reason" => resolve_property_with(
            contexts,
            move |vertex| {
                let def_id = vertex
                    .as_stability()
                    .expect("vertex is not variant 'Stability'");

                let reason = adapter.queries.global_ctxt().unwrap().enter(move |ctxt| {
                    let stability = ctxt
                        .lookup_stability(def_id)
                        .expect("def_id should have a Stability struct");
                    if let StabilityLevel::Unstable { reason, .. }  = stability.level {
                        reason.to_opt_reason()
                    } else {
                        None
                    }
                });

                if let Some(reason) = reason {
                    FieldValue::String(reason.to_string().into())
                } else {
                    FieldValue::NULL
                }
            },
        ),
        "issue" => resolve_property_with(
            contexts,
            move |vertex| {
                let def_id = vertex
                    .as_stability()
                    .expect("vertex is not variant 'Stability'");

                let issue = adapter.queries.global_ctxt().unwrap().enter(move |ctxt| {
                    let stability = ctxt
                        .lookup_stability(def_id)
                        .expect("def_id should have a Stability struct");
                    if let StabilityLevel::Unstable { issue, .. }  = stability.level {
                        issue
                    } else {
                        None
                    }
                });

                if let Some(issue_number) = issue {
                    let issue_number: u32 = issue_number.into();
                    FieldValue::Uint64(issue_number as u64)
                } else {
                    FieldValue::NULL
                }
            },
        ),
        "soft" => resolve_property_with(
            contexts,
            move |vertex| {
                let def_id = vertex
                    .as_stability()
                    .expect("vertex is not variant 'Stability'");

                let soft = adapter.queries.global_ctxt().unwrap().enter(move |ctxt| {
                    let stability = ctxt
                        .lookup_stability(def_id)
                        .expect("def_id should have a Stability struct");
                    if let StabilityLevel::Unstable { is_soft, .. }  = stability.level {
                        Some(is_soft)
                    } else {
                        None
                    }
                });

                if let Some(is_soft) = soft {
                    FieldValue::Boolean(is_soft.into())
                } else {
                    FieldValue::NULL
                }
            },
        ),
        "implied_by" => resolve_property_with(
            contexts,
            move |vertex| {
                let def_id = vertex
                    .as_stability()
                    .expect("vertex is not variant 'Stability'");

                let soft = adapter.queries.global_ctxt().unwrap().enter(move |ctxt| {
                    let stability = ctxt
                        .lookup_stability(def_id)
                        .expect("def_id should have a Stability struct");
                    if let StabilityLevel::Unstable { implied_by, .. }  = stability.level {
                        implied_by
                    } else {
                        None
                    }
                });

                if let Some(is_soft) = soft {
                    FieldValue::String(is_soft.to_string().into())
                } else {
                    FieldValue::NULL
                }
            },
        ),
        "since" => resolve_property_with(
            contexts,
            move |vertex| {
                let def_id = vertex
                    .as_stability()
                    .expect("vertex is not variant 'Stability'");

                let version = adapter.queries.global_ctxt().unwrap().enter(move |ctxt| {
                    let stability = ctxt
                        .lookup_stability(def_id)
                        .expect("def_id should have a Stability struct");
                    if let StabilityLevel::Stable { since, .. }  = stability.level {
                        match since {
                            rustc_attr::StableSince::Version(version) => Some(format!("{version}")),
                            _ => None,
                        }
                    } else {
                        None
                    }
                });

                if let Some(ver_number) = version {
                    FieldValue::String(ver_number.into())
                } else {
                    FieldValue::NULL
                }
            },
        ),
        "allowed_through_unstable_modules" => resolve_property_with(
            contexts,
            move |vertex| {
                let def_id = vertex
                    .as_stability()
                    .expect("vertex is not variant 'Stability'");

                let allowed = adapter.queries.global_ctxt().unwrap().enter(move |ctxt| {
                    let stability = ctxt
                        .lookup_stability(def_id)
                        .expect("def_id should have a Stability struct");
                    if let StabilityLevel::Stable { allowed_through_unstable_modules, .. }  = stability.level {
                        Some(allowed_through_unstable_modules)
                    } else {
                        None
                    }
                });

                if let Some(allowed) = allowed {
                    FieldValue::Boolean(allowed)
                } else {
                    FieldValue::NULL
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

pub(super) fn resolve_const_stability_property<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    property_name: &str,
    _resolve_info: &ResolveInfo,
    adapter: &'a Adapter,
) -> ContextOutcomeIterator<'a, V, FieldValue> {
    match property_name {
        "feature" => resolve_property_with(
            contexts,
            move |vertex| {
                let def_id = vertex
                    .as_const_stability()
                    .expect("vertex is not variant 'Stability'");

                let string = adapter.queries.global_ctxt().unwrap().enter(move |ctxt| {
                    ctxt
                        .lookup_const_stability(def_id)
                        .expect("def_id should have a ConstConstStability struct")
                        .feature
                        .to_string()
                });

                FieldValue::String(string.into())
            },
        ),
        "promotable" => resolve_property_with(
            contexts,
            move |vertex| {
                let def_id = vertex
                    .as_const_stability()
                    .expect("vertex is not variant 'Stability'");

                let stable = adapter.queries.global_ctxt().unwrap().enter(move |ctxt| {
                    ctxt
                        .lookup_const_stability(def_id)
                        .expect("def_id should have a ConstConstStability struct")
                        .is_const_stable()
                });

                FieldValue::Boolean(stable)
            },
        ),
        "stable" => resolve_property_with(
            contexts,
            move |vertex| {
                let def_id = vertex
                    .as_const_stability()
                    .expect("vertex is not variant 'Stability'");

                let promotable = adapter.queries.global_ctxt().unwrap().enter(move |ctxt| {
                    ctxt
                        .lookup_const_stability(def_id)
                        .expect("def_id should have a ConstConstStability struct")
                        .promotable
                });

                FieldValue::Boolean(promotable)
            },
        ),
        "reason" => resolve_property_with(
            contexts,
            move |vertex| {
                let def_id = vertex
                    .as_const_stability()
                    .expect("vertex is not variant 'Stability'");

                let reason = adapter.queries.global_ctxt().unwrap().enter(move |ctxt| {
                    let stability = ctxt
                        .lookup_const_stability(def_id)
                        .expect("def_id should have a ConstConstStability struct");
                    if let StabilityLevel::Unstable { reason, .. }  = stability.level {
                        reason.to_opt_reason()
                    } else {
                        None
                    }
                });

                if let Some(reason) = reason {
                    FieldValue::String(reason.to_string().into())
                } else {
                    FieldValue::NULL
                }
            },
        ),
        "issue" => resolve_property_with(
            contexts,
            move |vertex| {
                let def_id = vertex
                    .as_const_stability()
                    .expect("vertex is not variant 'Stability'");

                let issue = adapter.queries.global_ctxt().unwrap().enter(move |ctxt| {
                    let stability = ctxt
                        .lookup_const_stability(def_id)
                        .expect("def_id should have a ConstConstStability struct");
                    if let StabilityLevel::Unstable { issue, .. }  = stability.level {
                        issue
                    } else {
                        None
                    }
                });

                if let Some(issue_number) = issue {
                    let issue_number: u32 = issue_number.into();
                    FieldValue::Uint64(issue_number as u64)
                } else {
                    FieldValue::NULL
                }
            },
        ),
        "soft" => resolve_property_with(
            contexts,
            move |vertex| {
                let def_id = vertex
                    .as_const_stability()
                    .expect("vertex is not variant 'Stability'");

                let soft = adapter.queries.global_ctxt().unwrap().enter(move |ctxt| {
                    let stability = ctxt
                        .lookup_const_stability(def_id)
                        .expect("def_id should have a ConstConstStability struct");
                    if let StabilityLevel::Unstable { is_soft, .. }  = stability.level {
                        Some(is_soft)
                    } else {
                        None
                    }
                });

                if let Some(is_soft) = soft {
                    FieldValue::Boolean(is_soft.into())
                } else {
                    FieldValue::NULL
                }
            },
        ),
        "implied_by" => resolve_property_with(
            contexts,
            move |vertex| {
                let def_id = vertex
                    .as_const_stability()
                    .expect("vertex is not variant 'Stability'");

                let soft = adapter.queries.global_ctxt().unwrap().enter(move |ctxt| {
                    let stability = ctxt
                        .lookup_const_stability(def_id)
                        .expect("def_id should have a ConstConstStability struct");
                    if let StabilityLevel::Unstable { implied_by, .. }  = stability.level {
                        implied_by
                    } else {
                        None
                    }
                });

                if let Some(is_soft) = soft {
                    FieldValue::String(is_soft.to_string().into())
                } else {
                    FieldValue::NULL
                }
            },
        ),
        "since" => resolve_property_with(
            contexts,
            move |vertex| {
                let def_id = vertex
                    .as_const_stability()
                    .expect("vertex is not variant 'Stability'");

                let version = adapter.queries.global_ctxt().unwrap().enter(move |ctxt| {
                    let stability = ctxt
                        .lookup_const_stability(def_id)
                        .expect("def_id should have a ConstConstStability struct");
                    if let StabilityLevel::Stable { since, .. }  = stability.level {
                        match since {
                            rustc_attr::StableSince::Version(version) => Some(format!("{version}")),
                            _ => None,
                        }
                    } else {
                        None
                    }
                });

                if let Some(ver_number) = version {
                    FieldValue::String(ver_number.into())
                } else {
                    FieldValue::NULL
                }
            },
        ),
        "allowed_through_unstable_modules" => resolve_property_with(
            contexts,
            move |vertex| {
                let def_id = vertex
                    .as_const_stability()
                    .expect("vertex is not variant 'Stability'");

                let allowed = adapter.queries.global_ctxt().unwrap().enter(move |ctxt| {
                    let stability = ctxt
                        .lookup_const_stability(def_id)
                        .expect("def_id should have a ConstConstStability struct");
                    if let StabilityLevel::Stable { allowed_through_unstable_modules, .. }  = stability.level {
                        Some(allowed_through_unstable_modules)
                    } else {
                        None
                    }
                });

                if let Some(allowed) = allowed {
                    FieldValue::Boolean(allowed)
                } else {
                    FieldValue::NULL
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
                let string: String = adapter.queries.global_ctxt().unwrap().enter(move |ctxt| {
                    let hir = ctxt.hir();
                    let def_id = hir.enclosing_body_owner(*hir_id);
                    ctxt.typeck(def_id).node_type(*hir_id).to_string()
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
    use trustfall::FieldValue;

    use crate::adapter::Adapter;

    pub fn resolve_ident<'a>(
        item_id: ItemId,
        adapter: &'a Adapter,
    ) -> FieldValue {
        let ident: String = adapter.queries.global_ctxt().unwrap().enter(move |ctxt| {
            let hir = ctxt.hir();
            hir.item(item_id).ident.as_str().into()
        });
        FieldValue::String(ident.into())
    }
}

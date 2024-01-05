use trustfall::provider::{AsVertex, ContextIterator, ContextOutcomeIterator, EdgeParameters, ResolveEdgeInfo, VertexIterator};

use super::{vertex::Vertex, Adapter};

pub(super) fn resolve_block_edge<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    edge_name: &str,
    _parameters: &EdgeParameters,
    resolve_info: &ResolveEdgeInfo,
    adapter: &'a Adapter,
) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
    match edge_name {
        "parent" => block::parent(contexts, resolve_info, adapter),
        "statements" => block::statements(contexts, resolve_info, adapter),
        "type" => block::type_(contexts, resolve_info, adapter),
        _ => {
            unreachable!(
                "attempted to resolve unexpected edge '{edge_name}' on type 'Block'"
            )
        }
    }
}

mod block {
    use itertools::Itertools;
    use rustc_hir::StmtKind;
    use rustc_hir::ExprKind;
    use rustc_interface::run_compiler;
    use trustfall::provider::{
        resolve_neighbors_with, AsVertex, ContextIterator, ContextOutcomeIterator,
        ResolveEdgeInfo, VertexIterator,
    };

    use crate::adapter::Adapter;

    use super::super::vertex::Vertex;

    pub(super) fn parent<'a, V: AsVertex<Vertex> + 'a>(
        contexts: ContextIterator<'a, V>,
        _resolve_info: &ResolveEdgeInfo,
        adapter: &'a Adapter,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
        super::expr::parent(contexts, _resolve_info, adapter)
    }

    pub(super) fn statements<'a, V: AsVertex<Vertex> + 'a>(
        contexts: ContextIterator<'a, V>,
        _resolve_info: &ResolveEdgeInfo,
        adapter: &'a Adapter,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
        resolve_neighbors_with(
            contexts,
            move |vertex| {
                let hir_id = vertex
                    .hir_id()
                    .expect("vertex was not a Node");

                let stmt_ids = run_compiler((&adapter.config).clone().into(), move |compiler| {
                    compiler.enter(move |queries| {
                        queries.global_ctxt().unwrap().enter(move |ctxt| {
                            let hir = ctxt.hir();
                            let expr = hir.expect_expr(hir_id);
                            let ExprKind::Block(block, ..) = expr.kind else {
                                unimplemented!("expr was not of type Block: {expr:#?}")
                            };
                            block
                                .stmts
                                .iter()
                                .map(|stmt| {
                                    let id = stmt.hir_id;
                                    match stmt.kind {
                                        StmtKind::Local(..) => Vertex::LocalStatement(id),
                                        _ => Vertex::Statement(id),
                                    }
                                })
                                .collect_vec()
                        })
                    })
                });

                Box::new(stmt_ids.into_iter())
            },
        )
    }

    pub(super) fn type_<'a, V: AsVertex<Vertex> + 'a>(
        contexts: ContextIterator<'a, V>,
        _resolve_info: &ResolveEdgeInfo,
        adapter: &'a Adapter,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
        super::expr::type_(contexts, _resolve_info, adapter)
    }
}

pub(super) fn resolve_body_edge<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    edge_name: &str,
    _parameters: &EdgeParameters,
    resolve_info: &ResolveEdgeInfo,
    adapter: &'a Adapter,
) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
    match edge_name {
        "parent" => body::parent(contexts, resolve_info, adapter),
        "value" => body::value(contexts, resolve_info, adapter),
        _ => {
            unreachable!(
                "attempted to resolve unexpected edge '{edge_name}' on type 'Body'"
            )
        }
    }
}

mod body {
    use rustc_interface::run_compiler;
    use trustfall::provider::{
        resolve_neighbors_with, AsVertex, ContextIterator, ContextOutcomeIterator,
        ResolveEdgeInfo, VertexIterator,
    };

    use crate::adapter::Adapter;

    use super::super::vertex::Vertex;

    pub(super) fn parent<'a, V: AsVertex<Vertex> + 'a>(
        contexts: ContextIterator<'a, V>,
        _resolve_info: &ResolveEdgeInfo,
        adapter: &'a Adapter,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
        super::node::parent(contexts, _resolve_info, adapter)
    }

    pub(super) fn value<'a, V: AsVertex<Vertex> + 'a>(
        contexts: ContextIterator<'a, V>,
        _resolve_info: &ResolveEdgeInfo,
        adapter: &'a Adapter,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
        resolve_neighbors_with(
            contexts,
            move |vertex| {
                let body_id = vertex
                    .body_id()
                    .expect("vertex was not a Body");

                let hir_id = run_compiler((&adapter.config).clone().into(), move |compiler| {
                    compiler.enter(move |queries| {
                        queries.global_ctxt().unwrap().enter(move |ctxt| {
                            let hir = ctxt.hir();
                            hir.body(body_id).value.hir_id
                        })
                    })
                });

                Box::new(std::iter::once(Vertex::Expr(hir_id)))
            },
        )
    }
}

pub(super) fn resolve_crate_edge<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    edge_name: &str,
    _parameters: &EdgeParameters,
    resolve_info: &ResolveEdgeInfo,
    adapter: &'a Adapter,
) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
    match edge_name {
        "item" => crate_::item(contexts, resolve_info, adapter),
        _ => {
            unreachable!(
                "attempted to resolve unexpected edge '{edge_name}' on type 'Crate'"
            )
        }
    }
}

mod crate_ {
    use itertools::Itertools;
    use rustc_hir::ItemKind;
    use rustc_interface::run_compiler;
    use trustfall::provider::{
        resolve_neighbors_with, AsVertex, ContextIterator, ContextOutcomeIterator,
        ResolveEdgeInfo, VertexIterator,
    };

    use crate::adapter::Adapter;

    use super::super::vertex::Vertex;

    pub(super) fn item<'a, V: AsVertex<Vertex> + 'a>(
        contexts: ContextIterator<'a, V>,
        _resolve_info: &ResolveEdgeInfo,
        adapter: &'a Adapter,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
        resolve_neighbors_with(
            contexts,
            move |vertex| {
                let _ = vertex
                    .as_crate()
                    .expect("conversion failed, vertex was not a Crate");
                let items = run_compiler((&adapter.config).clone().into(), move |compiler| {
                    compiler.enter(move |queries| {
                        queries.global_ctxt().unwrap().enter(move |ctxt| {
                            let hir = ctxt.hir();
                            hir
                                .items()
                                .map(|id| {
                                    match hir.item(id).kind {
                                        ItemKind::Fn(..) =>  Vertex::Fn(id),
                                        _ => Vertex::Item(id),
                                    }    
                                })
                                .collect_vec()
                        })
                    })
                });
                Box::new(items.into_iter())
            },
        )
    }
}

pub(super) fn resolve_expr_edge<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    edge_name: &str,
    _parameters: &EdgeParameters,
    resolve_info: &ResolveEdgeInfo,
    adapter: &'a Adapter,
) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
    match edge_name {
        "parent" => expr::parent(contexts, resolve_info, adapter),
        "type" => expr::type_(contexts, resolve_info, adapter),
        _ => {
            unreachable!(
                "attempted to resolve unexpected edge '{edge_name}' on type 'Expr'"
            )
        }
    }
}

mod expr {
    use trustfall::provider::{
        resolve_neighbors_with, AsVertex, ContextIterator, ContextOutcomeIterator,
        ResolveEdgeInfo, VertexIterator,
    };

    use crate::adapter::Adapter;

    use super::super::vertex::Vertex;

    pub(super) fn parent<'a, V: AsVertex<Vertex> + 'a>(
        contexts: ContextIterator<'a, V>,
        _resolve_info: &ResolveEdgeInfo,
        adapter: &'a Adapter,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
        super::node::parent(contexts, _resolve_info, adapter)
    }

    pub(super) fn type_<'a, V: AsVertex<Vertex> + 'a>(
        contexts: ContextIterator<'a, V>,
        _resolve_info: &ResolveEdgeInfo,
        _adapter: &'a Adapter,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
        resolve_neighbors_with(
            contexts,
            move |vertex| {
                let hir_id = vertex
                    .hir_id()
                    .expect("conversion failed, vertex was not an Item");
                
                Box::new(std::iter::once(Vertex::Ty(hir_id)))
            },
        )
    }
}

pub(super) fn resolve_fn_edge<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    edge_name: &str,
    _parameters: &EdgeParameters,
    resolve_info: &ResolveEdgeInfo,
    adapter: &'a Adapter,
) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
    match edge_name {
        "body" => fn_::body(contexts, resolve_info, adapter),
        "parent" => fn_::parent(contexts, resolve_info, adapter),
        _ => {
            unreachable!(
                "attempted to resolve unexpected edge '{edge_name}' on type 'Fn'"
            )
        }
    }
}

mod fn_ {
    use rustc_interface::run_compiler;
    use trustfall::provider::{
        resolve_neighbors_with, AsVertex, ContextIterator, ContextOutcomeIterator,
        ResolveEdgeInfo, VertexIterator,
    };

    use crate::adapter::Adapter;

    use super::super::vertex::Vertex;

    pub(super) fn body<'a, V: AsVertex<Vertex> + 'a>(
        contexts: ContextIterator<'a, V>,
        _resolve_info: &ResolveEdgeInfo,
        adapter: &'a Adapter,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
        resolve_neighbors_with(
            contexts,
            move |vertex| {
                let item_id = vertex
                    .item_id()
                    .expect("expected vertex to be an Item");

                let body_id = run_compiler((&adapter.config).clone().into(), move |compiler| {
                    compiler.enter(move |queries| {
                        queries.global_ctxt().unwrap().enter(move |ctxt| {
                            let hir = ctxt.hir();
                            let (.., body_id) = hir.item(item_id).expect_fn();
                            body_id
                        })
                    })
                });

                Box::new(std::iter::once(Vertex::FnBody(body_id)))
            },
        )
    }

    pub(super) fn parent<'a, V: AsVertex<Vertex> + 'a>(
        contexts: ContextIterator<'a, V>,
        _resolve_info: &ResolveEdgeInfo,
        adapter: &'a Adapter,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
        super::item::parent(contexts, _resolve_info, adapter)
    }
}

pub(super) fn resolve_fn_body_edge<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    edge_name: &str,
    _parameters: &EdgeParameters,
    resolve_info: &ResolveEdgeInfo,
    adapter: &'a Adapter,
) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
    match edge_name {
        "parent" => fn_body::parent(contexts, resolve_info, adapter),
        "value" => fn_body::value(contexts, resolve_info, adapter),
        _ => {
            unreachable!(
                "attempted to resolve unexpected edge '{edge_name}' on type 'FnBody'"
            )
        }
    }
}

mod fn_body {
    use trustfall::provider::{
        AsVertex, ContextIterator, ContextOutcomeIterator,
        ResolveEdgeInfo, VertexIterator,
    };

    use crate::adapter::Adapter;

    use super::super::vertex::Vertex;

    pub(super) fn parent<'a, V: AsVertex<Vertex> + 'a>(
        contexts: ContextIterator<'a, V>,
        _resolve_info: &ResolveEdgeInfo,
        adapter: &'a Adapter,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
        super::body::parent(contexts, _resolve_info, adapter)
    }

    pub(super) fn value<'a, V: AsVertex<Vertex> + 'a>(
        contexts: ContextIterator<'a, V>,
        _resolve_info: &ResolveEdgeInfo,
        adapter: &'a Adapter,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
        super::body::value(contexts, _resolve_info, adapter)
    }
}

pub(super) fn resolve_item_edge<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    edge_name: &str,
    _parameters: &EdgeParameters,
    resolve_info: &ResolveEdgeInfo,
    adapter: &'a Adapter,
) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
    match edge_name {
        "parent" => item::parent(contexts, resolve_info, adapter),
        _ => {
            unreachable!(
                "attempted to resolve unexpected edge '{edge_name}' on type 'Item'"
            )
        }
    }
}

mod item {
    use trustfall::provider::{
        AsVertex, ContextIterator, ContextOutcomeIterator,
        ResolveEdgeInfo, VertexIterator,
    };

    use crate::adapter::Adapter;

    use super::super::vertex::Vertex;

    pub(super) fn parent<'a, V: AsVertex<Vertex> + 'a>(
        contexts: ContextIterator<'a, V>,
        _resolve_info: &ResolveEdgeInfo,
        adapter: &'a Adapter,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
        super::node::parent(contexts, _resolve_info, adapter)
    }
}

pub(super) fn resolve_local_statement_edge<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    edge_name: &str,
    _parameters: &EdgeParameters,
    resolve_info: &ResolveEdgeInfo,
    adapter: &'a Adapter
) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
    match edge_name {
        "init" => local_statement::init(contexts, resolve_info, adapter),
        "parent" => local_statement::parent(contexts, resolve_info, adapter),
        _ => {
            unreachable!(
                "attempted to resolve unexpected edge '{edge_name}' on type 'LocalStatement'"
            )
        }
    }
}

mod local_statement {
    use rustc_hir::{intravisit::Map, StmtKind, Local};
    use rustc_interface::run_compiler;
    use trustfall::provider::{
        resolve_neighbors_with, AsVertex, ContextIterator, ContextOutcomeIterator,
        ResolveEdgeInfo, VertexIterator,
    };

    use crate::adapter::Adapter;

    use super::super::vertex::Vertex;

    pub(super) fn init<'a, V: AsVertex<Vertex> + 'a>(
        contexts: ContextIterator<'a, V>,
        _resolve_info: &ResolveEdgeInfo,
        adapter: &'a Adapter,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
        resolve_neighbors_with(
            contexts,
            move |vertex: &Vertex| {
                let hir_id: rustc_hir::HirId = vertex
                    .hir_id()
                    .expect("conversion failed, vertex was not a Node");

                let opt_init_id = run_compiler((&adapter.config).clone().into(), move |compiler| {
                    compiler.enter(move |queries| {
                        queries.global_ctxt().unwrap().enter(move |ctxt| {
                            let hir = ctxt.hir();
                            let stmt = hir
                                .find(hir_id)
                                .expect("LocalStatement couldn't be found")
                                .expect_stmt();
                            let StmtKind::Local(Local { init: Some(init_expr), .. }) = stmt.kind else {
                                return None;
                            };
                            Some(init_expr.hir_id)
                        })
                    })
                });

                if let Some(init_id) = opt_init_id {
                    Box::new(std::iter::once(Vertex::Expr(init_id)))
                } else {
                    Box::new(std::iter::empty())
                }
            },
        )
    }

    pub(super) fn parent<'a, V: AsVertex<Vertex> + 'a>(
        contexts: ContextIterator<'a, V>,
        _resolve_info: &ResolveEdgeInfo,
        adapter: &'a Adapter,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
        super::statement::parent(contexts, _resolve_info, adapter)
    }
}

pub(super) fn resolve_node_edge<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    edge_name: &str,
    _parameters: &EdgeParameters,
    resolve_info: &ResolveEdgeInfo,
    adapter: &'a Adapter,
) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
    match edge_name {
        "parent" => node::parent(contexts, resolve_info, adapter),
        _ => {
            unreachable!(
                "attempted to resolve unexpected edge '{edge_name}' on type 'Node'"
            )
        }
    }
}

mod node {
    use rustc_interface::run_compiler;
    use trustfall::provider::{
        resolve_neighbors_with, AsVertex, ContextIterator, ContextOutcomeIterator,
        ResolveEdgeInfo, VertexIterator,
    };

    use crate::adapter::Adapter;

    use super::super::vertex::Vertex;

    pub(super) fn parent<'a, V: AsVertex<Vertex> + 'a>(
        contexts: ContextIterator<'a, V>,
        _resolve_info: &ResolveEdgeInfo,
        adapter: &'a Adapter,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
        resolve_neighbors_with(
            contexts,
            move |vertex| {
                let hir_id = vertex
                    .hir_id()
                    .expect("conversion failed, vertex was not a Node");
                let opt_parent_id = run_compiler((&adapter.config).clone().into(), move |compiler| {
                    compiler.enter(move |queries| {
                        queries.global_ctxt().unwrap().enter(move |ctxt| {
                            let hir = ctxt.hir();
                            hir.opt_parent_id(hir_id)
                        })
                    })
                });

                if let Some(parent_id) = opt_parent_id {
                    Box::new(std::iter::once(Vertex::Node(parent_id)))
                } else {
                    Box::new(std::iter::empty())
                }
            },
        )
    }
}

pub(super) fn resolve_statement_edge<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    edge_name: &str,
    _parameters: &EdgeParameters,
    resolve_info: &ResolveEdgeInfo,
    adapter: &'a Adapter,
) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
    match edge_name {
        "parent" => statement::parent(contexts, resolve_info, adapter),
        _ => {
            unreachable!(
                "attempted to resolve unexpected edge '{edge_name}' on type 'Statement'"
            )
        }
    }
}

mod statement {
    use trustfall::provider::{
        AsVertex, ContextIterator, ContextOutcomeIterator,
        ResolveEdgeInfo, VertexIterator,
    };

    use crate::adapter::Adapter;

    use super::super::vertex::Vertex;

    pub(super) fn parent<'a, V: AsVertex<Vertex> + 'a>(
        contexts: ContextIterator<'a, V>,
        _resolve_info: &ResolveEdgeInfo,
        adapter: &'a Adapter,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
        super::node::parent(contexts, _resolve_info, adapter)
    }
}

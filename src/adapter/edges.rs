use trustfall::provider::{AsVertex, ContextIterator, ContextOutcomeIterator, EdgeParameters, ResolveEdgeInfo, VertexIterator};

use super::vertex::Vertex;

pub(super) fn resolve_block_edge<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    edge_name: &str,
    parameters: &EdgeParameters,
    resolve_info: &ResolveEdgeInfo,
) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
    match edge_name {
        "parent" => block::parent(contexts, resolve_info),
        "statements" => block::statements(contexts, resolve_info),
        "type" => block::type_(contexts, resolve_info),
        _ => {
            unreachable!(
                "attempted to resolve unexpected edge '{edge_name}' on type 'Block'"
            )
        }
    }
}

mod block {
    use trustfall::provider::{
        resolve_neighbors_with, AsVertex, ContextIterator, ContextOutcomeIterator,
        ResolveEdgeInfo, VertexIterator,
    };

    use super::super::vertex::Vertex;

    pub(super) fn parent<'a, V: AsVertex<Vertex> + 'a>(
        contexts: ContextIterator<'a, V>,
        _resolve_info: &ResolveEdgeInfo,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
        resolve_neighbors_with(
            contexts,
            move |vertex| {
                let vertex = vertex
                    .as_block()
                    .expect("conversion failed, vertex was not a Block");
                todo!("get neighbors along edge 'parent' for type 'Block'")
            },
        )
    }

    pub(super) fn statements<'a, V: AsVertex<Vertex> + 'a>(
        contexts: ContextIterator<'a, V>,
        _resolve_info: &ResolveEdgeInfo,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
        resolve_neighbors_with(
            contexts,
            move |vertex| {
                let vertex = vertex
                    .as_block()
                    .expect("conversion failed, vertex was not a Block");
                todo!("get neighbors along edge 'statements' for type 'Block'")
            },
        )
    }

    pub(super) fn type_<'a, V: AsVertex<Vertex> + 'a>(
        contexts: ContextIterator<'a, V>,
        _resolve_info: &ResolveEdgeInfo,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
        resolve_neighbors_with(
            contexts,
            move |vertex| {
                let vertex = vertex
                    .as_block()
                    .expect("conversion failed, vertex was not a Block");
                todo!("get neighbors along edge 'type' for type 'Block'")
            },
        )
    }
}

pub(super) fn resolve_body_edge<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    edge_name: &str,
    parameters: &EdgeParameters,
    resolve_info: &ResolveEdgeInfo,
) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
    match edge_name {
        "parent" => body::parent(contexts, resolve_info),
        "value" => body::value(contexts, resolve_info),
        _ => {
            unreachable!(
                "attempted to resolve unexpected edge '{edge_name}' on type 'Body'"
            )
        }
    }
}

mod body {
    use trustfall::provider::{
        resolve_neighbors_with, AsVertex, ContextIterator, ContextOutcomeIterator,
        ResolveEdgeInfo, VertexIterator,
    };

    use super::super::vertex::Vertex;

    pub(super) fn parent<'a, V: AsVertex<Vertex> + 'a>(
        contexts: ContextIterator<'a, V>,
        _resolve_info: &ResolveEdgeInfo,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
        resolve_neighbors_with(
            contexts,
            move |vertex| {
                let vertex = vertex
                    .as_body()
                    .expect("conversion failed, vertex was not a Body");
                todo!("get neighbors along edge 'parent' for type 'Body'")
            },
        )
    }

    pub(super) fn value<'a, V: AsVertex<Vertex> + 'a>(
        contexts: ContextIterator<'a, V>,
        _resolve_info: &ResolveEdgeInfo,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
        resolve_neighbors_with(
            contexts,
            move |vertex| {
                let vertex = vertex
                    .as_body()
                    .expect("conversion failed, vertex was not a Body");
                todo!("get neighbors along edge 'value' for type 'Body'")
            },
        )
    }
}

pub(super) fn resolve_crate_edge<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    edge_name: &str,
    parameters: &EdgeParameters,
    resolve_info: &ResolveEdgeInfo,
) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
    match edge_name {
        "item" => crate_::item(contexts, resolve_info),
        _ => {
            unreachable!(
                "attempted to resolve unexpected edge '{edge_name}' on type 'Crate'"
            )
        }
    }
}

mod crate_ {
    use trustfall::provider::{
        resolve_neighbors_with, AsVertex, ContextIterator, ContextOutcomeIterator,
        ResolveEdgeInfo, VertexIterator,
    };

    use super::super::vertex::Vertex;

    pub(super) fn item<'a, V: AsVertex<Vertex> + 'a>(
        contexts: ContextIterator<'a, V>,
        _resolve_info: &ResolveEdgeInfo,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
        resolve_neighbors_with(
            contexts,
            move |vertex| {
                let vertex = vertex
                    .as_crate()
                    .expect("conversion failed, vertex was not a Crate");
                todo!("get neighbors along edge 'item' for type 'Crate'")
            },
        )
    }
}

pub(super) fn resolve_expr_edge<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    edge_name: &str,
    parameters: &EdgeParameters,
    resolve_info: &ResolveEdgeInfo,
) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
    match edge_name {
        "parent" => expr::parent(contexts, resolve_info),
        "type" => expr::type_(contexts, resolve_info),
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

    use super::super::vertex::Vertex;

    pub(super) fn parent<'a, V: AsVertex<Vertex> + 'a>(
        contexts: ContextIterator<'a, V>,
        _resolve_info: &ResolveEdgeInfo,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
        resolve_neighbors_with(
            contexts,
            move |vertex| {
                let vertex = vertex
                    .as_expr()
                    .expect("conversion failed, vertex was not a Expr");
                todo!("get neighbors along edge 'parent' for type 'Expr'")
            },
        )
    }

    pub(super) fn type_<'a, V: AsVertex<Vertex> + 'a>(
        contexts: ContextIterator<'a, V>,
        _resolve_info: &ResolveEdgeInfo,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
        resolve_neighbors_with(
            contexts,
            move |vertex| {
                let vertex = vertex
                    .as_expr()
                    .expect("conversion failed, vertex was not a Expr");
                todo!("get neighbors along edge 'type' for type 'Expr'")
            },
        )
    }
}

pub(super) fn resolve_fn_edge<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    edge_name: &str,
    parameters: &EdgeParameters,
    resolve_info: &ResolveEdgeInfo,
) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
    match edge_name {
        "body" => fn_::body(contexts, resolve_info),
        "parent" => fn_::parent(contexts, resolve_info),
        _ => {
            unreachable!(
                "attempted to resolve unexpected edge '{edge_name}' on type 'Fn'"
            )
        }
    }
}

mod fn_ {
    use trustfall::provider::{
        resolve_neighbors_with, AsVertex, ContextIterator, ContextOutcomeIterator,
        ResolveEdgeInfo, VertexIterator,
    };

    use super::super::vertex::Vertex;

    pub(super) fn body<'a, V: AsVertex<Vertex> + 'a>(
        contexts: ContextIterator<'a, V>,
        _resolve_info: &ResolveEdgeInfo,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
        resolve_neighbors_with(
            contexts,
            move |vertex| {
                let vertex = vertex
                    .as_fn()
                    .expect("conversion failed, vertex was not a Fn");
                todo!("get neighbors along edge 'body' for type 'Fn'")
            },
        )
    }

    pub(super) fn parent<'a, V: AsVertex<Vertex> + 'a>(
        contexts: ContextIterator<'a, V>,
        _resolve_info: &ResolveEdgeInfo,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
        resolve_neighbors_with(
            contexts,
            move |vertex| {
                let vertex = vertex
                    .as_fn()
                    .expect("conversion failed, vertex was not a Fn");
                todo!("get neighbors along edge 'parent' for type 'Fn'")
            },
        )
    }
}

pub(super) fn resolve_fn_body_edge<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    edge_name: &str,
    parameters: &EdgeParameters,
    resolve_info: &ResolveEdgeInfo,
) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
    match edge_name {
        "parent" => fn_body::parent(contexts, resolve_info),
        "value" => fn_body::value(contexts, resolve_info),
        _ => {
            unreachable!(
                "attempted to resolve unexpected edge '{edge_name}' on type 'FnBody'"
            )
        }
    }
}

mod fn_body {
    use trustfall::provider::{
        resolve_neighbors_with, AsVertex, ContextIterator, ContextOutcomeIterator,
        ResolveEdgeInfo, VertexIterator,
    };

    use super::super::vertex::Vertex;

    pub(super) fn parent<'a, V: AsVertex<Vertex> + 'a>(
        contexts: ContextIterator<'a, V>,
        _resolve_info: &ResolveEdgeInfo,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
        resolve_neighbors_with(
            contexts,
            move |vertex| {
                let vertex = vertex
                    .as_fn_body()
                    .expect("conversion failed, vertex was not a FnBody");
                todo!("get neighbors along edge 'parent' for type 'FnBody'")
            },
        )
    }

    pub(super) fn value<'a, V: AsVertex<Vertex> + 'a>(
        contexts: ContextIterator<'a, V>,
        _resolve_info: &ResolveEdgeInfo,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
        resolve_neighbors_with(
            contexts,
            move |vertex| {
                let vertex = vertex
                    .as_fn_body()
                    .expect("conversion failed, vertex was not a FnBody");
                todo!("get neighbors along edge 'value' for type 'FnBody'")
            },
        )
    }
}

pub(super) fn resolve_item_edge<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    edge_name: &str,
    parameters: &EdgeParameters,
    resolve_info: &ResolveEdgeInfo,
) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
    match edge_name {
        "parent" => item::parent(contexts, resolve_info),
        _ => {
            unreachable!(
                "attempted to resolve unexpected edge '{edge_name}' on type 'Item'"
            )
        }
    }
}

mod item {
    use trustfall::provider::{
        resolve_neighbors_with, AsVertex, ContextIterator, ContextOutcomeIterator,
        ResolveEdgeInfo, VertexIterator,
    };

    use super::super::vertex::Vertex;

    pub(super) fn parent<'a, V: AsVertex<Vertex> + 'a>(
        contexts: ContextIterator<'a, V>,
        _resolve_info: &ResolveEdgeInfo,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
        resolve_neighbors_with(
            contexts,
            move |vertex| {
                let vertex = vertex
                    .as_item()
                    .expect("conversion failed, vertex was not a Item");
                todo!("get neighbors along edge 'parent' for type 'Item'")
            },
        )
    }
}

pub(super) fn resolve_local_statement_edge<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    edge_name: &str,
    parameters: &EdgeParameters,
    resolve_info: &ResolveEdgeInfo,
) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
    match edge_name {
        "init" => local_statement::init(contexts, resolve_info),
        "parent" => local_statement::parent(contexts, resolve_info),
        _ => {
            unreachable!(
                "attempted to resolve unexpected edge '{edge_name}' on type 'LocalStatement'"
            )
        }
    }
}

mod local_statement {
    use trustfall::provider::{
        resolve_neighbors_with, AsVertex, ContextIterator, ContextOutcomeIterator,
        ResolveEdgeInfo, VertexIterator,
    };

    use super::super::vertex::Vertex;

    pub(super) fn init<'a, V: AsVertex<Vertex> + 'a>(
        contexts: ContextIterator<'a, V>,
        _resolve_info: &ResolveEdgeInfo,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
        resolve_neighbors_with(
            contexts,
            move |vertex| {
                let vertex = vertex
                    .as_local_statement()
                    .expect("conversion failed, vertex was not a LocalStatement");
                todo!("get neighbors along edge 'init' for type 'LocalStatement'")
            },
        )
    }

    pub(super) fn parent<'a, V: AsVertex<Vertex> + 'a>(
        contexts: ContextIterator<'a, V>,
        _resolve_info: &ResolveEdgeInfo,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
        resolve_neighbors_with(
            contexts,
            move |vertex| {
                let vertex = vertex
                    .as_local_statement()
                    .expect("conversion failed, vertex was not a LocalStatement");
                todo!("get neighbors along edge 'parent' for type 'LocalStatement'")
            },
        )
    }
}

pub(super) fn resolve_node_edge<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    edge_name: &str,
    parameters: &EdgeParameters,
    resolve_info: &ResolveEdgeInfo,
) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
    match edge_name {
        "parent" => node::parent(contexts, resolve_info),
        _ => {
            unreachable!(
                "attempted to resolve unexpected edge '{edge_name}' on type 'Node'"
            )
        }
    }
}

mod node {
    use trustfall::provider::{
        resolve_neighbors_with, AsVertex, ContextIterator, ContextOutcomeIterator,
        ResolveEdgeInfo, VertexIterator,
    };

    use super::super::vertex::Vertex;

    pub(super) fn parent<'a, V: AsVertex<Vertex> + 'a>(
        contexts: ContextIterator<'a, V>,
        _resolve_info: &ResolveEdgeInfo,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
        resolve_neighbors_with(
            contexts,
            move |vertex| {
                let vertex = vertex
                    .as_node()
                    .expect("conversion failed, vertex was not a Node");
                todo!("get neighbors along edge 'parent' for type 'Node'")
            },
        )
    }
}

pub(super) fn resolve_statement_edge<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    edge_name: &str,
    parameters: &EdgeParameters,
    resolve_info: &ResolveEdgeInfo,
) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
    match edge_name {
        "parent" => statement::parent(contexts, resolve_info),
        _ => {
            unreachable!(
                "attempted to resolve unexpected edge '{edge_name}' on type 'Statement'"
            )
        }
    }
}

mod statement {
    use trustfall::provider::{
        resolve_neighbors_with, AsVertex, ContextIterator, ContextOutcomeIterator,
        ResolveEdgeInfo, VertexIterator,
    };

    use super::super::vertex::Vertex;

    pub(super) fn parent<'a, V: AsVertex<Vertex> + 'a>(
        contexts: ContextIterator<'a, V>,
        _resolve_info: &ResolveEdgeInfo,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
        resolve_neighbors_with(
            contexts,
            move |vertex| {
                let vertex = vertex
                    .as_statement()
                    .expect("conversion failed, vertex was not a Statement");
                todo!("get neighbors along edge 'parent' for type 'Statement'")
            },
        )
    }
}

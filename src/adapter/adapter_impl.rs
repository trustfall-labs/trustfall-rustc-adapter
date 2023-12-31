use std::sync::{Arc, OnceLock};

use trustfall::{FieldValue, Schema, provider::{AsVertex, ContextIterator, ContextOutcomeIterator, EdgeParameters, ResolveEdgeInfo, ResolveInfo, Typename, VertexIterator, resolve_coercion_using_schema, resolve_property_with}};

use super::vertex::Vertex;

static SCHEMA: OnceLock<Schema> = OnceLock::new();

#[non_exhaustive]
#[derive(Debug)]
pub struct Adapter {}

impl Adapter {
    pub const SCHEMA_TEXT: &'static str = include_str!("./schema.graphql");

    pub fn schema() -> &'static Schema {
        SCHEMA
            .get_or_init(|| {
                Schema::parse(Self::SCHEMA_TEXT).expect("not a valid schema")
            })
    }

    pub fn new() -> Self {
        Self {}
    }
}

impl<'a> trustfall::provider::Adapter<'a> for Adapter {
    type Vertex = Vertex;

    fn resolve_starting_vertices(
        &self,
        edge_name: &Arc<str>,
        parameters: &EdgeParameters,
        resolve_info: &ResolveInfo,
    ) -> VertexIterator<'a, Self::Vertex> {
        match edge_name.as_ref() {
            "Crate" => super::entrypoints::crate_(resolve_info),
            _ => {
                unreachable!(
                    "attempted to resolve starting vertices for unexpected edge name: {edge_name}"
                )
            }
        }
    }

    fn resolve_property<V: AsVertex<Self::Vertex> + 'a>(
        &self,
        contexts: ContextIterator<'a, V>,
        type_name: &Arc<str>,
        property_name: &Arc<str>,
        resolve_info: &ResolveInfo,
    ) -> ContextOutcomeIterator<'a, V, FieldValue> {
        if property_name.as_ref() == "__typename" {
            return resolve_property_with(contexts, |vertex| vertex.typename().into());
        }
        match type_name.as_ref() {
            "Ty" => {
                super::properties::resolve_ty_property(
                    contexts,
                    property_name.as_ref(),
                    resolve_info,
                )
            }
            _ => {
                unreachable!(
                    "attempted to read property '{property_name}' on unexpected type: {type_name}"
                )
            }
        }
    }

    fn resolve_neighbors<V: AsVertex<Self::Vertex> + 'a>(
        &self,
        contexts: ContextIterator<'a, V>,
        type_name: &Arc<str>,
        edge_name: &Arc<str>,
        parameters: &EdgeParameters,
        resolve_info: &ResolveEdgeInfo,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Self::Vertex>> {
        match type_name.as_ref() {
            "Block" => {
                super::edges::resolve_block_edge(
                    contexts,
                    edge_name.as_ref(),
                    parameters,
                    resolve_info,
                )
            }
            "Body" => {
                super::edges::resolve_body_edge(
                    contexts,
                    edge_name.as_ref(),
                    parameters,
                    resolve_info,
                )
            }
            "Crate" => {
                super::edges::resolve_crate_edge(
                    contexts,
                    edge_name.as_ref(),
                    parameters,
                    resolve_info,
                )
            }
            "Expr" => {
                super::edges::resolve_expr_edge(
                    contexts,
                    edge_name.as_ref(),
                    parameters,
                    resolve_info,
                )
            }
            "Fn" => {
                super::edges::resolve_fn_edge(
                    contexts,
                    edge_name.as_ref(),
                    parameters,
                    resolve_info,
                )
            }
            "FnBody" => {
                super::edges::resolve_fn_body_edge(
                    contexts,
                    edge_name.as_ref(),
                    parameters,
                    resolve_info,
                )
            }
            "Item" => {
                super::edges::resolve_item_edge(
                    contexts,
                    edge_name.as_ref(),
                    parameters,
                    resolve_info,
                )
            }
            "LocalStatement" => {
                super::edges::resolve_local_statement_edge(
                    contexts,
                    edge_name.as_ref(),
                    parameters,
                    resolve_info,
                )
            }
            "Node" => {
                super::edges::resolve_node_edge(
                    contexts,
                    edge_name.as_ref(),
                    parameters,
                    resolve_info,
                )
            }
            "Statement" => {
                super::edges::resolve_statement_edge(
                    contexts,
                    edge_name.as_ref(),
                    parameters,
                    resolve_info,
                )
            }
            _ => {
                unreachable!(
                    "attempted to resolve edge '{edge_name}' on unexpected type: {type_name}"
                )
            }
        }
    }

    fn resolve_coercion<V: AsVertex<Self::Vertex> + 'a>(
        &self,
        contexts: ContextIterator<'a, V>,
        _type_name: &Arc<str>,
        coerce_to_type: &Arc<str>,
        _resolve_info: &ResolveInfo,
    ) -> ContextOutcomeIterator<'a, V, bool> {
        resolve_coercion_using_schema(contexts, Self::schema(), coerce_to_type.as_ref())
    }
}

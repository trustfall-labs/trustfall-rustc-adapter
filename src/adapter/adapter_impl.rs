use std::sync::{Arc, OnceLock};

use trustfall::{FieldValue, Schema, provider::{AsVertex, ContextIterator, ContextOutcomeIterator, EdgeParameters, ResolveEdgeInfo, ResolveInfo, Typename, VertexIterator, resolve_coercion_using_schema, resolve_property_with}};

use rustc_interface::Queries;

use super::{vertex::Vertex, util::is_subtype};

static SCHEMA: OnceLock<Schema> = OnceLock::new();

#[non_exhaustive]
pub struct Adapter<'a> {
    pub(crate) queries: &'a Queries<'a>
}

impl<'a> Adapter<'a> {
    pub const SCHEMA_TEXT: &'static str = include_str!("./schema.graphql");

    pub fn schema() -> &'static Schema {
        SCHEMA
            .get_or_init(|| {
                Schema::parse(Self::SCHEMA_TEXT).expect("not a valid schema")
            })
    }

    pub fn new(queries: &'a Queries<'a>) -> Self {
        Self { queries }
    }
}

impl<'a, 'b> trustfall::provider::Adapter<'a> for &'a Adapter<'b> {
    type Vertex = Vertex;

    fn resolve_starting_vertices(
        &self,
        edge_name: &Arc<str>,
        _parameters: &EdgeParameters,
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
        } else if is_subtype(Adapter::schema(), "Node", type_name) {
            match property_name.as_ref() {
                "inside_const_context" => {
                    return super::properties::resolve_inside_const_context_property(
                        contexts,
                        property_name.as_ref(),
                        resolve_info,
                        self,
                    );
                }
                _ => {}
            }
        }
        match type_name.as_ref() {
            "ConstStability" => {
                super::properties::resolve_const_stability_property(
                    contexts,
                    property_name.as_ref(),
                    resolve_info,
                    self,
                )
            }
            "Def" => super::properties::resolve_def_property(
                contexts,
                property_name.as_ref(),
                resolve_info,
                self,
            ),
            "Fn" => {
                super::properties::resolve_fn_property(
                    contexts,
                    property_name.as_ref(),
                    resolve_info,
                    self,
                )
            }
            "Item" => {
                super::properties::resolve_item_property(
                    contexts,
                    property_name.as_ref(),
                    resolve_info,
                    self,
                )
            }
            "Node" => {
                super::properties::resolve_node_property(
                    contexts,
                    property_name.as_ref(),
                    resolve_info,
                    self,
                )
            }
            "Stability" => {
                super::properties::resolve_stability_property(
                    contexts,
                    property_name.as_ref(),
                    resolve_info,
                    self,
                )
            }
            "Ty" => {
                super::properties::resolve_ty_property(
                    contexts,
                    property_name.as_ref(),
                    resolve_info,
                    self,
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
                    self,
                )
            }
            "Body" => {
                super::edges::resolve_body_edge(
                    contexts,
                    edge_name.as_ref(),
                    parameters,
                    resolve_info,
                    self,
                )
            }
            "Crate" => {
                super::edges::resolve_crate_edge(
                    contexts,
                    edge_name.as_ref(),
                    parameters,
                    resolve_info,
                    self,
                )
            }
            "Def" => {
                super::edges::resolve_def_edge(
                    contexts,
                    edge_name.as_ref(),
                    parameters,
                    resolve_info,
                    self,
                )
            }
            "Expr" => {
                super::edges::resolve_expr_edge(
                    contexts,
                    edge_name.as_ref(),
                    parameters,
                    resolve_info,
                    self,
                )
            }
            "Fn" => {
                super::edges::resolve_fn_edge(
                    contexts,
                    edge_name.as_ref(),
                    parameters,
                    resolve_info,
                    self,
                )
            }
            "FnBody" => {
                super::edges::resolve_fn_body_edge(
                    contexts,
                    edge_name.as_ref(),
                    parameters,
                    resolve_info,
                    self,
                )
            }
            "Item" => {
                super::edges::resolve_item_edge(
                    contexts,
                    edge_name.as_ref(),
                    parameters,
                    resolve_info,
                    self,
                )
            }
            "LocalStatement" => {
                super::edges::resolve_local_statement_edge(
                    contexts,
                    edge_name.as_ref(),
                    parameters,
                    resolve_info,
                    self,
                )
            }
            "MethodCall" => {
                super::edges::resolve_method_call_edge(
                    contexts,
                    edge_name.as_ref(),
                    parameters,
                    resolve_info,
                    self,
                )
            }
            "Node" => {
                super::edges::resolve_node_edge(
                    contexts,
                    edge_name.as_ref(),
                    parameters,
                    resolve_info,
                    self,
                )
            }
            "Statement" => {
                super::edges::resolve_statement_edge(
                    contexts,
                    edge_name.as_ref(),
                    parameters,
                    resolve_info,
                    self,
                )
            }
            "Ty" => {
                super::edges::resolve_ty_edge(
                    contexts,
                    edge_name.as_ref(),
                    parameters,
                    resolve_info,
                    self,
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
        resolve_coercion_using_schema(contexts, Adapter::schema(), coerce_to_type.as_ref())
    }
}

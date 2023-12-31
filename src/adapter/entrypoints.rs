use trustfall::provider::{ResolveInfo, VertexIterator};

use super::vertex::Vertex;

pub(super) fn crate_<'a>(_resolve_info: &ResolveInfo) -> VertexIterator<'a, Vertex> {
    todo!("implement resolving starting vertices for entrypoint edge 'Crate'")
}

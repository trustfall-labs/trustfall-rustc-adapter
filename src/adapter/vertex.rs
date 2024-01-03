use rustc_hir::{ItemId, HirId, BodyId};
use rustc_span::def_id::LocalDefId;

#[non_exhaustive]
#[derive(Debug, Clone, trustfall::provider::TrustfallEnumVertex)]
pub enum Vertex {
    Block(HirId),
    Body(BodyId),
    Crate(()),
    Expr(HirId),
    Fn(ItemId),
    FnBody(BodyId),
    Item(ItemId),
    LocalStatement(HirId),
    Node(HirId),
    Statement(HirId),
    Ty(LocalDefId, HirId),
}

impl Vertex {
    pub fn hir_id(&self) -> Option<HirId> {
        if let Some(item_id) = self.clone().item_id() {
            Some(item_id.hir_id())
        } else if let Some(BodyId { hir_id }) = self.clone().body_id() {
            Some(hir_id)
        } else {
            match self {
                Self::Node(hir_id)
                | Self::Block(hir_id)
                | Self::Expr(hir_id)
                | Self::Statement(hir_id)
                | Self::LocalStatement(hir_id) => Some(*hir_id),
                _ => None,
            }
        }
    }

    pub fn item_id(&self) -> Option<ItemId> {
        match self {
            Self::Item(item_id)
            | Self::Fn(item_id) => Some(*item_id),
            _ => None,
        }
    }

    pub fn body_id(&self) -> Option<BodyId> {
        match self {
            Self::Body(body_id)
            | Self::FnBody(body_id) => Some(*body_id),
            _ => None,
        }
    }
}

use rustc_hir::{ItemId, HirId};

#[non_exhaustive]
#[derive(Debug, Clone, trustfall::provider::TrustfallEnumVertex)]
pub enum Vertex {
    Block(()),
    Body(()),
    Crate(()),
    Expr(()),
    Fn(()),
    FnBody(()),
    Item(ItemId),
    LocalStatement(()),
    Node(HirId),
    Statement(()),
    Ty(()),
}

#[non_exhaustive]
#[derive(Debug, Clone, trustfall::provider::TrustfallEnumVertex)]
pub enum Vertex {
    Block(()),
    Body(()),
    Crate(()),
    Expr(()),
    Fn(()),
    FnBody(()),
    Item(()),
    LocalStatement(()),
    Node(()),
    Statement(()),
    Ty(()),
}

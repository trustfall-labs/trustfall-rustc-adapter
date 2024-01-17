use trustfall::Schema;

pub(super) fn is_subtype(schema: &Schema, super_: &str, sub: &str) -> bool {
    schema
        .subtypes(super_)
        .expect(format!("{super_:?} isn't a type in this schema").as_str())
        .any(|ty| ty == sub)
}

use std::{collections::BTreeMap, sync::Arc};

use itertools::Itertools;
use trustfall::{provider::check_adapter_invariants, execute_query, FieldValue};

use crate::compiler_config::CompilerConfig;

use super::Adapter;

#[test]
fn adapter_satisfies_trustfall_invariants() {
    let config = CompilerConfig::new("main.rs", "fn main() {}");
    let adapter = Adapter::new(config);
    let schema = Adapter::schema();
    check_adapter_invariants(schema, &adapter);
}

#[test]
fn ident_of_item() {
    let config = CompilerConfig::new("main.rs", "fn main() {}");
    let adapter = Adapter::new(config);
    let schema = Adapter::schema();
    let query = include_str!("./queries/ident_of_item.gql");
    let result = execute_query(
        schema,
        (&adapter).into(),
        query,
        BTreeMap::<Arc<str>, FieldValue>::new()
    )
        .unwrap()
        .map(|x| x["ident"].clone())
        .collect_vec();
    assert_eq!(result, vec![FieldValue::String("".into()), FieldValue::String("std".into()), FieldValue::String("main".into())]);
}


#[test]
/// Trustfall impl of https://rustc-dev-guide.rust-lang.org/rustc-driver-interacting-with-the-ast.html
fn ty_of_expression() {
    let config = CompilerConfig::new("main.rs", r#"fn main() {
    let i: i32 = 45;
    println!("Hello {i}!");
}"#);
    let adapter = Adapter::new(config);
    let schema = Adapter::schema();
    let query = include_str!("./queries/ty_of_expression.gql");
    let result = execute_query(
        schema,
        (&adapter).into(),
        query,
        BTreeMap::<Arc<str>, FieldValue>::new()
    )
        .unwrap()
        .collect_vec();
    assert_eq!(result.len(), 1);
    let result = result.first().unwrap();
    assert_eq!(result["as_string"], FieldValue::String("i32".into()));
}

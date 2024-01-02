use std::{collections::BTreeMap, sync::Arc};

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
    let result = execute_query(schema, (&adapter).into(), query, BTreeMap::<Arc<str>, FieldValue>::new()).unwrap();
    for x in result {
        println!("{x:#?}");
    }
}


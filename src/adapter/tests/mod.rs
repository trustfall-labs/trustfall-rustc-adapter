use std::{collections::BTreeMap, sync::Arc};

use itertools::Itertools;
use rustc_interface::run_compiler;
use trustfall::{provider::check_adapter_invariants, execute_query, FieldValue};

use crate::compiler_config::CompilerConfig;

use super::Adapter;

#[test]
fn adapter_satisfies_trustfall_invariants() {
    let config = CompilerConfig::new("main.rs", "fn main() {}");
    run_compiler(config.into(), |compiler| {
        compiler.enter(|queries| {
            let adapter = Adapter::new(queries);
            let schema = Adapter::schema();
            check_adapter_invariants(schema, &adapter);
        })
    });

}

#[test]
fn ident_of_item() {
    let config = CompilerConfig::new("main.rs", "fn main() {}");
    run_compiler(config.into(), |compiler| {
        compiler.enter(|queries| {
            let adapter = Adapter::new(queries);
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
        })
    })
}


#[test]
/// Trustfall impl of https://rustc-dev-guide.rust-lang.org/rustc-driver-interacting-with-the-ast.html
fn ty_of_expression() {
    let config = CompilerConfig::new("main.rs", r#"fn main() {
    let i: i32 = 45;
    println!("Hello {i}!");
}"#);
    run_compiler(config.into(), |compiler| {
        compiler.enter(|queries| {
            let adapter = Adapter::new(queries);
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
        })
    })
}

#[test]
/// Find the minimum supported Rust version of a crate based on the `std`
/// method calls it contains. 
fn msrv_methods() {
    let config = CompilerConfig::new("main.rs", r#"fn main() {
    let x = Some(42);
    x.is_some();
    x.as_slice();
}"#);
    run_compiler(config.into(), |compiler| {
        compiler.enter(|queries| {
            let adapter = Adapter::new(queries);
            let schema = Adapter::schema();
            let query = include_str!("./queries/msrv_methods.gql");
            let result = execute_query(
                schema,
                (&adapter).into(),
                query,
                BTreeMap::<Arc<str>, FieldValue>::new()
            )
                .unwrap()
                .collect_vec();
            println!("{result:#?}");
        })
    });
}

#[test]
/// Implements clippy's [`useless_conversion`](https://rust-lang.github.io/rust-clippy/rust-1.75.0/index.html#/useless_conversion)
/// lint.
fn useless_conversion() {
    let config = CompilerConfig::new("main.rs", r#"fn main() {
    let _: String = format!("hello").into();
    let _: Option<i32> = Some(42).into();
    let _: String = String::from(String::from(""));
}"#);
    run_compiler(config.into(), |compiler| {
        compiler.enter(|queries| {
            let adapter = Adapter::new(queries);
            let schema = Adapter::schema();
            let query = include_str!("./queries/useless_conversion.gql");
            let mut variables = BTreeMap::<Arc<str>, FieldValue>::new();
            // variables.insert("methods".into(), FieldValue::List([
            //     FieldValue::String("std::convert::Into::into".into()),
            //     FieldValue::String("std::convert::From::from".into()),
            // ].into()));
            let result = execute_query(
                schema,
                (&adapter).into(),
                query,
                variables,
            )
                .unwrap()
                .collect_vec();
            println!("{result:#?}");
        })
    });
}

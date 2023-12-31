extern crate rustc_ast_pretty;
extern crate rustc_driver;
extern crate rustc_error_codes;
extern crate rustc_errors;
extern crate rustc_hash;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_session;
extern crate rustc_span;

use rustc_interface::Config;
use trustfall::provider::TrustfallEnumVertex;

pub fn get_sysroot() -> String {
    use std::{process, str};

    let out = process::Command::new("rustc")
        .arg("--print=sysroot")
        .current_dir(".")
        .output()
        .unwrap();
    str::from_utf8(&out.stdout)
        .expect("TODO: Handle error")
        .trim()
        .into()
}

pub struct HirAdapter<'vertex> {
    config: &'vertex Config
}

impl<'vertex> HirAdapter<'vertex> {
    pub fn new(config: &'vertex Config) -> Self {
        Self { config }
    }
}

#[derive(Debug, Clone, Copy, TrustfallEnumVertex)]
pub(crate) enum Vertex<'vertex> {
    
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{path, process, str, sync::atomic::AtomicBool};

    use rustc_ast_pretty::pprust::item_to_string;
    use rustc_errors::registry;
    use rustc_session::config::{self, CheckCfg};

    fn get_config() -> Config {
        let sysroot = get_sysroot();
        Config {
            opts: config::Options {
                maybe_sysroot: Some(path::PathBuf::from(sysroot)),
                ..config::Options::default()
            },
            input: config::Input::Str {
                name: rustc_span::FileName::Custom("main.rs".to_string()),
                input: r#"
    fn main() {
        2 +
    }
    "#
                .to_string(),
            },
            crate_cfg: Default::default(),
            crate_check_cfg: Default::default(),
            output_dir: None,
            output_file: None,
            file_loader: None,
            locale_resources: rustc_driver::DEFAULT_LOCALE_RESOURCES,
            lint_caps: rustc_hash::FxHashMap::default(),
            parse_sess_created: None,
            register_lints: None,
            override_queries: None,
            make_codegen_backend: None,
            registry: registry::Registry::new(&rustc_error_codes::DIAGNOSTICS),
            expanded_args: Vec::new(),
            ice_file: None,
            hash_untracked_state: None,
            using_internal_features: AtomicBool::from(true).into(),
        }
    }
    
    fn get_type_of_expression() {
        let config = get_config();
        rustc_interface::run_compiler(config, |compiler| {
            compiler.enter(|queries| {
                let ast_krate = queries.parse().unwrap().get_mut().clone();
                for item in ast_krate.items {
                    println!("{}", item_to_string(&item));
                }
    
                // Analyze the crate and inspect the types under the cursor.
                queries.global_ctxt().unwrap().enter(|tcx| {
                    // Every compilation contains a single crate.
                    let hir_krate = tcx.hir();
                    // Iterate over the top-level items in the crate, looking for the main function.
                    for id in hir_krate.items() {
                        let item = hir_krate.item(id);
                        // Use pattern-matching to find a specific node inside the main function.
                        if let rustc_hir::ItemKind::Fn(_, _, body_id) = item.kind {
                            let expr = &tcx.hir().body(body_id).value;
                            println!("{expr:#?}");
                            if let rustc_hir::ExprKind::Block(block, _) = expr.kind {
                                if let rustc_hir::StmtKind::Local(local) = block.stmts[0].kind {
                                    if let Some(expr) = local.init {
                                        let hir_id = expr.hir_id; // hir_id identifies the string "Hello, world!"
                                        let def_id = item.hir_id().owner.def_id; // def_id identifies the main function
                                        let ty = tcx.typeck(def_id).node_type(hir_id);
                                        println!("{expr:#?}: {ty:?}");
                                    }
                                }
                            }
                        }
                    }
                })
            });
        });
    }

    #[test]
    fn it_works() {
        get_type_of_expression();
    }
}
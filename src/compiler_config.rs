use std::sync::atomic::AtomicBool;

use rustc_interface::Config;
use rustc_errors::registry;
use rustc_session::config;

use crate::util::get_sysroot;

/// A configuration struct for rustc that can be converted into
/// [`rustc_interface::interface::Config`] since the aformentioned type has really
/// annoying properties like lacking [`Clone`], [`Copy`] and [`std::fmt::Debug`] impls.
#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct CompilerConfig {
    code_filename: String,
    code_input: String,
}

impl CompilerConfig {
    pub fn new (filename: &str, input: &str) -> Self {
        Self { code_filename: filename.into(), code_input: input.into() }
    }
}

impl From<CompilerConfig> for Config {
    fn from(value: CompilerConfig) -> Self {
        let sysroot = get_sysroot();
        Config {
            opts: config::Options {
                maybe_sysroot: Some(sysroot.into()),
                ..config::Options::default()
            },
            input: config::Input::Str {
                name: rustc_span::FileName::Custom(value.code_filename),
                input: value.code_input,
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
}

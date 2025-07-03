use anyhow::Result;
use baad_core::errors::{ErrorContext, ErrorExt};
use clap::ValueEnum;
use std::path::PathBuf;
use std::process::Command;

pub struct FlatC {
    binary: PathBuf,
}

#[derive(Debug, Clone, ValueEnum, Copy)]
pub enum Language {
    Cpp,
    Java,
    Kotlin,
    CSharp,
    Go,
    Python,
    JavaScript,
    TypeScript,
    Php,
    Dart,
    Lua,
    Lobster,
    Rust,
    Swift,
    Nim,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum CppStd {
    Cpp0x,
    Cpp11,
    Cpp17,
}

#[derive(Default)]
pub struct FlatCOptions {
    pub languages: Vec<Language>,
    pub grpc: bool,
    pub output_path: Option<PathBuf>,
    pub include_paths: Vec<PathBuf>,
    pub binary: bool,
    pub json: bool,
    pub strict_json: bool,
    pub allow_non_utf8: bool,
    pub natural_utf8: bool,
    pub defaults_json: bool,
    pub no_prefix: bool,
    pub scoped_enums: bool,
    pub no_emit_min_max_enum_values: bool,
    pub no_includes: bool,
    pub gen_mutable: bool,
    pub gen_onefile: bool,
    pub gen_name_strings: bool,
    pub gen_object_api: bool,
    pub gen_compare: bool,
    pub gen_nullable: bool,
    pub gen_generated: bool,
    pub gen_jvmstatic: bool,
    pub gen_all: bool,
    pub cpp_include: Vec<String>,
    pub cpp_ptr_type: Option<String>,
    pub cpp_str_type: Option<String>,
    pub cpp_str_flex_ctor: bool,
    pub no_cpp_direct_copy: bool,
    pub cpp_std: Option<CppStd>,
    pub object_prefix: Option<String>,
    pub object_suffix: Option<String>,
    pub go_namespace: Option<String>,
    pub go_import: Option<String>,
    pub raw_binary: bool,
    pub size_prefixed: bool,
    pub proto: bool,
    pub oneof_union: bool,
    pub schema: bool,
    pub bfbs_comments: bool,
    pub conform: Option<PathBuf>,
    pub conform_includes: Vec<PathBuf>,
    pub filename_suffix: Option<String>,
    pub filename_ext: Option<String>,
    pub include_prefix: Option<PathBuf>,
    pub keep_prefix: bool,
    pub reflect_types: bool,
    pub reflect_names: bool,
    pub root_type: Option<String>,
    pub require_explicit_ids: bool,
    pub force_defaults: bool,
    pub force_empty: bool,
    pub force_empty_vectors: bool,
    pub flexbuffers: bool,
    pub no_warnings: bool,
    pub cs_global_alias: bool,
    pub json_nested_bytes: bool,
    pub python_no_type_prefix_suffix: bool,
    pub python_typing: bool,
    pub grpc_filename_suffix: Option<String>,
    pub grpc_additional_header: Vec<String>,
    pub grpc_search_path: Option<String>,
    pub grpc_use_system_headers: bool,
    pub grpc_python_typed_handlers: bool,
}


impl FlatC {
    pub fn new(binary: PathBuf) -> Result<Self> {
        if !binary.exists() {
            return None.error_context(&format!("FlatC binary not found at: {}", binary.display()));
        }
        Ok(Self { binary })
    }

    pub fn compile(&self, options: FlatCOptions, files: Vec<PathBuf>, binary_files: Vec<PathBuf>) -> Result<()> {
        let mut cmd = Command::new(&self.binary);

        for lang in &options.languages {
            match lang {
                Language::Cpp => cmd.arg("--cpp"),
                Language::Java => cmd.arg("--java"),
                Language::Kotlin => cmd.arg("--kotlin"),
                Language::CSharp => cmd.arg("--csharp"),
                Language::Go => cmd.arg("--go"),
                Language::Python => cmd.arg("--python"),
                Language::JavaScript => cmd.arg("--js"),
                Language::TypeScript => cmd.arg("--ts"),
                Language::Php => cmd.arg("--php"),
                Language::Dart => cmd.arg("--dart"),
                Language::Lua => cmd.arg("--lua"),
                Language::Lobster => cmd.arg("--lobster"),
                Language::Rust => cmd.arg("--rust"),
                Language::Swift => cmd.arg("--swift"),
                Language::Nim => cmd.arg("--nim"),
            };
        }

        if options.grpc {
            cmd.arg("--grpc");
        }

        if let Some(output_path) = &options.output_path {
            cmd.arg("-o").arg(output_path);
        }

        for include_path in &options.include_paths {
            cmd.arg("-I").arg(include_path);
        }

        if options.binary {
            cmd.arg("--binary");
        }
        if options.json {
            cmd.arg("--json");
        }

        if options.strict_json {
            cmd.arg("--strict-json");
        }
        if options.allow_non_utf8 {
            cmd.arg("--allow-non-utf8");
        }
        if options.natural_utf8 {
            cmd.arg("--natural-utf8");
        }
        if options.defaults_json {
            cmd.arg("--defaults-json");
        }

        if options.no_prefix {
            cmd.arg("--no-prefix");
        }
        if options.scoped_enums {
            cmd.arg("--scoped-enums");
        }
        if options.no_emit_min_max_enum_values {
            cmd.arg("--no-emit-min-max-enum-values");
        }
        if options.no_includes {
            cmd.arg("--no-includes");
        }
        if options.gen_mutable {
            cmd.arg("--gen-mutable");
        }
        if options.gen_onefile {
            cmd.arg("--gen-onefile");
        }
        if options.gen_name_strings {
            cmd.arg("--gen-name-strings");
        }
        if options.gen_object_api {
            cmd.arg("--gen-object-api");
        }
        if options.gen_compare {
            cmd.arg("--gen-compare");
        }
        if options.gen_nullable {
            cmd.arg("--gen-nullable");
        }
        if options.gen_generated {
            cmd.arg("--gen-generated");
        }
        if options.gen_jvmstatic {
            cmd.arg("--gen-jvmstatic");
        }
        if options.gen_all {
            cmd.arg("--gen-all");
        }

        for include in &options.cpp_include {
            cmd.arg("--cpp-include").arg(include);
        }
        if let Some(ptr_type) = &options.cpp_ptr_type {
            cmd.arg("--cpp-ptr-type").arg(ptr_type);
        }
        if let Some(str_type) = &options.cpp_str_type {
            cmd.arg("--cpp-str-type").arg(str_type);
        }
        if options.cpp_str_flex_ctor {
            cmd.arg("--cpp-str-flex-ctor");
        }
        if options.no_cpp_direct_copy {
            cmd.arg("--no-cpp-direct-copy");
        }
        if let Some(cpp_std) = &options.cpp_std {
            cmd.arg("--cpp-std").arg(match cpp_std {
                CppStd::Cpp0x => "c++0x",
                CppStd::Cpp11 => "c++11",
                CppStd::Cpp17 => "c++17",
            });
        }
        if let Some(prefix) = &options.object_prefix {
            cmd.arg("--object-prefix").arg(prefix);
        }
        if let Some(suffix) = &options.object_suffix {
            cmd.arg("--object-suffix").arg(suffix);
        }

        if let Some(go_namespace) = &options.go_namespace {
            cmd.arg("--go-namespace").arg(go_namespace);
        }
        if let Some(go_import) = &options.go_import {
            cmd.arg("--go-import").arg(go_import);
        }

        if options.raw_binary {
            cmd.arg("--raw-binary");
        }
        if options.size_prefixed {
            cmd.arg("--size-prefixed");
        }
        if options.proto {
            cmd.arg("--proto");
        }
        if options.oneof_union {
            cmd.arg("--oneof-union");
        }
        if options.schema {
            cmd.arg("--schema");
        }
        if options.bfbs_comments {
            cmd.arg("--bfbs-comments");
        }

        if let Some(conform) = &options.conform {
            cmd.arg("--conform").arg(conform);
        }
        for conform_include in &options.conform_includes {
            cmd.arg("--conform-includes").arg(conform_include);
        }

        if let Some(suffix) = &options.filename_suffix {
            cmd.arg("--filename-suffix").arg(suffix);
        }
        if let Some(ext) = &options.filename_ext {
            cmd.arg("--filename-ext").arg(ext);
        }
        if let Some(prefix) = &options.include_prefix {
            cmd.arg("--include-prefix").arg(prefix);
        }
        if options.keep_prefix {
            cmd.arg("--keep-prefix");
        }

        if options.reflect_types {
            cmd.arg("--reflect-types");
        }
        if options.reflect_names {
            cmd.arg("--reflect-names");
        }
        if let Some(root_type) = &options.root_type {
            cmd.arg("--root-type").arg(root_type);
        }

        if options.require_explicit_ids {
            cmd.arg("--require-explicit-ids");
        }
        if options.force_defaults {
            cmd.arg("--force-defaults");
        }
        if options.force_empty {
            cmd.arg("--force-empty");
        }
        if options.force_empty_vectors {
            cmd.arg("--force-empty-vectors");
        }
        if options.flexbuffers {
            cmd.arg("--flexbuffers");
        }
        if options.no_warnings {
            cmd.arg("--no-warnings");
        }
        if options.cs_global_alias {
            cmd.arg("--cs-global-alias");
        }
        if options.json_nested_bytes {
            cmd.arg("--json-nested-bytes");
        }
        if options.python_no_type_prefix_suffix {
            cmd.arg("--python-no-type-prefix-suffix");
        }
        if options.python_typing {
            cmd.arg("--python-typing");
        }

        if let Some(suffix) = &options.grpc_filename_suffix {
            cmd.arg("--grpc-filename-suffix").arg(suffix);
        }
        for header in &options.grpc_additional_header {
            cmd.arg("--grpc-additional-header").arg(header);
        }
        if let Some(search_path) = &options.grpc_search_path {
            cmd.arg("--grpc-search-path").arg(search_path);
        }
        if options.grpc_use_system_headers {
            cmd.arg("--grpc-use-system-headers");
        }
        if options.grpc_python_typed_handlers {
            cmd.arg("--grpc-python-typed-handlers");
        }

        for file in &files {
            cmd.arg(file);
        }

        if !binary_files.is_empty() {
            cmd.arg("--");
            for binary_file in &binary_files {
                cmd.arg(binary_file);
            }
        }

        let status = cmd.status()
            .handle_errors()
            .error_context(&format!("Failed to execute FlatC at {}", self.binary.display()))?;

        if !status.success() {
            return Err(anyhow::anyhow!(
                "FlatC failed with exit code {:?}",
                status.code()
            )).error_context("FlatC execution failed");
        }

        Ok(())
    }

    #[allow(dead_code)]
    pub fn compile_schema(&self, schema_files: Vec<PathBuf>, language: Language, output_path: PathBuf) -> Result<()> {
        let options = FlatCOptions {
            languages: vec![language],
            output_path: Some(output_path),
            ..Default::default()
        };
        self.compile(options, schema_files, vec![])
    }

    #[allow(dead_code)]
    pub fn json_to_binary(&self, schema_file: PathBuf, json_file: PathBuf, output_path: Option<PathBuf>) -> Result<()> {
        let options = FlatCOptions {
            binary: true,
            output_path,
            ..Default::default()
        };
        self.compile(options, vec![schema_file, json_file], vec![])
    }

    #[allow(dead_code)]
    pub fn binary_to_json(&self, schema_file: PathBuf, binary_file: PathBuf, output_path: Option<PathBuf>) -> Result<()> {
        let options = FlatCOptions {
            json: true,
            output_path,
            ..Default::default()
        };
        self.compile(options, vec![schema_file, binary_file], vec![])
    }
}
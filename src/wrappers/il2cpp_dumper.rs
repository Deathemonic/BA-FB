use anyhow::Result;
use baad_core::errors::{ErrorContext, ErrorExt};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::process::Command;

pub struct Il2CppDumper {
    binary: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Il2CppDumperOptions {
    pub binary_files: Vec<PathBuf>,
    pub metadata_file: Option<PathBuf>,
    pub image_base: Option<String>,
    pub select_outputs_only: bool,
    pub cs_out: Option<PathBuf>,
    pub py_out: Option<PathBuf>,
    pub cpp_out: Option<PathBuf>,
    pub json_out: Option<PathBuf>,
    pub dll_out: Option<PathBuf>,
    pub metadata_out: Option<PathBuf>,
    pub binary_out: Option<PathBuf>,
    pub excluded_namespaces: Vec<String>,
    pub layout: Option<String>,
    pub sort: Option<String>,
    pub flatten: bool,
    pub suppress_metadata: bool,
    pub suppress_dll_metadata: bool,
    pub must_compile: bool,
    pub separate_attributes: bool,
    pub create_project: bool,
    pub cpp_compiler: Option<String>,
    pub script_target: Option<String>,
    pub unity_path: Option<PathBuf>,
    pub unity_assemblies: Option<PathBuf>,
    pub unity_version: Option<String>,
    pub unity_version_from_asset: Option<PathBuf>,
    pub plugin_options: Vec<String>,
}

impl Default for Il2CppDumperOptions {
    fn default() -> Self {
        Self {
            binary_files: vec![PathBuf::from("libil2cpp.so")],
            metadata_file: Some(PathBuf::from("global-metadata.dat")),
            image_base: None,
            select_outputs_only: false,
            cs_out: None,
            py_out: None,
            cpp_out: None,
            json_out: None,
            dll_out: None,
            metadata_out: None,
            binary_out: None,
            excluded_namespaces: vec![
                "System".to_string(),
                "Mono".to_string(),
                "Microsoft.Reflection".to_string(),
                "Microsoft.Win32".to_string(),
                "Internal.Runtime".to_string(),
                "Unity".to_string(),
                "UnityEditor".to_string(),
                "UnityEngine".to_string(),
                "UnityEngineInternal".to_string(),
                "AOT".to_string(),
                "JetBrains.Annotations".to_string(),
            ],
            layout: Some("single".to_string()),
            sort: Some("index".to_string()),
            flatten: false,
            suppress_metadata: false,
            suppress_dll_metadata: false,
            must_compile: false,
            separate_attributes: false,
            create_project: false,
            cpp_compiler: None,
            script_target: Some("IDA".to_string()),
            unity_path: None,
            unity_assemblies: None,
            unity_version: None,
            unity_version_from_asset: None,
            plugin_options: vec![],
        }
    }
}

impl Il2CppDumper {
    pub fn new(binary: PathBuf) -> Result<Self> {
        if !binary.exists() {
            return None.error_context(&format!("IL2CPP dumper binary not found at: {}", binary.display()));
        }
        Ok(Self { binary })
    }

    pub fn run(&self, options: Il2CppDumperOptions) -> Result<()> {
        let mut cmd = Command::new(&self.binary);

        if !options.binary_files.is_empty() {
            cmd.arg("--bin");
            let binary_paths: Vec<String> = options.binary_files
                .iter()
                .map(|p| p.to_string_lossy().to_string())
                .collect();
            cmd.arg(binary_paths.join(","));
        }

        if let Some(metadata) = &options.metadata_file {
            cmd.arg("--metadata").arg(metadata);
        }

        if let Some(image_base) = &options.image_base {
            cmd.arg("--image-base").arg(image_base);
        }

        if options.select_outputs_only {
            cmd.arg("--select-outputs");
        }

        if let Some(cs_out) = &options.cs_out {
            cmd.arg("--cs-out").arg(cs_out);
        }
        if let Some(py_out) = &options.py_out {
            cmd.arg("--py-out").arg(py_out);
        }
        if let Some(cpp_out) = &options.cpp_out {
            cmd.arg("--cpp-out").arg(cpp_out);
        }
        if let Some(json_out) = &options.json_out {
            cmd.arg("--json-out").arg(json_out);
        }
        if let Some(dll_out) = &options.dll_out {
            cmd.arg("--dll-out").arg(dll_out);
        }
        if let Some(metadata_out) = &options.metadata_out {
            cmd.arg("--metadata-out").arg(metadata_out);
        }
        if let Some(binary_out) = &options.binary_out {
            cmd.arg("--binary-out").arg(binary_out);
        }

        if !options.excluded_namespaces.is_empty() {
            cmd.arg("--exclude-namespaces").arg(options.excluded_namespaces.join(","));
        }

        if let Some(layout) = &options.layout {
            cmd.arg("--layout").arg(layout);
        }
        if let Some(sort) = &options.sort {
            cmd.arg("--sort").arg(sort);
        }

        if options.flatten {
            cmd.arg("--flatten");
        }
        if options.suppress_metadata {
            cmd.arg("--suppress-metadata");
        }
        if options.suppress_dll_metadata {
            cmd.arg("--suppress-dll-metadata");
        }
        if options.must_compile {
            cmd.arg("--must-compile");
        }
        if options.separate_attributes {
            cmd.arg("--separate-attributes");
        }
        if options.create_project {
            cmd.arg("--project");
        }

        if let Some(cpp_compiler) = &options.cpp_compiler {
            cmd.arg("--cpp-compiler").arg(cpp_compiler);
        }
        if let Some(script_target) = &options.script_target {
            cmd.arg("--script-target").arg(script_target);
        }
        if let Some(unity_path) = &options.unity_path {
            cmd.arg("--unity-path").arg(unity_path);
        }
        if let Some(unity_assemblies) = &options.unity_assemblies {
            cmd.arg("--unity-assemblies").arg(unity_assemblies);
        }
        if let Some(unity_version) = &options.unity_version {
            cmd.arg("--unity-version").arg(unity_version);
        }
        if let Some(unity_version_asset) = &options.unity_version_from_asset {
            cmd.arg("--unity-version-from-asset").arg(unity_version_asset);
        }

        if !options.plugin_options.is_empty() {
            cmd.arg("--plugins");
            for plugin_opt in &options.plugin_options {
                cmd.arg(plugin_opt);
            }
        }

        let status = cmd.status()
            .handle_errors()
            .error_context(&format!("Failed to execute IL2CPP dumper at {}", self.binary.display()))?;

        if !status.success() {
            return Err(anyhow::anyhow!(
                "IL2CPP dumper failed with exit code {:?}",
                status.code()
            )).error_context("IL2CPP dumper execution failed");
        }

        Ok(())
    }
}
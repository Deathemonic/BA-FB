use eyre::{eyre, Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::process::Command;

pub struct Il2CppDumper {
    binary: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Il2CppDumperOptions {
    pub url: Option<String>,
    pub il2cpp: PathBuf,
    pub metadata: Option<PathBuf>,
    pub output: Option<PathBuf>,
    pub unity_version: Option<String>,
    pub compiler_type: Option<String>,
    pub output_csharp_stub: bool,
    pub layout: Option<String>,
    pub flatten_hierarchy: bool,
    pub output_disassembler_metadata: bool,
    pub disassembler: Option<String>,
    pub output_cpp_scaffolding: bool,
    pub sorting_mode: Option<String>,
    pub suppress_metadata: bool,
    pub compilable: bool,
    pub separate_assembly_attributes: bool,
    pub output_dummy_dlls: bool,
    pub output_vs_solution: bool,
    pub unity_path: Option<PathBuf>,
    pub unity_assemblies_path: Option<PathBuf>,
    pub extract_il2cpp_files: bool,
}

impl Default for Il2CppDumperOptions {
    fn default() -> Self {
        Self {
            url: None,
            il2cpp: PathBuf::new(),
            metadata: None,
            output: None,
            unity_version: None,
            compiler_type: None,
            output_csharp_stub: false,
            layout: None,
            flatten_hierarchy: false,
            output_disassembler_metadata: false,
            disassembler: None,
            output_cpp_scaffolding: false,
            sorting_mode: None,
            suppress_metadata: false,
            compilable: false,
            separate_assembly_attributes: false,
            output_dummy_dlls: false,
            output_vs_solution: false,
            unity_path: None,
            unity_assemblies_path: None,
            extract_il2cpp_files: false,
        }
    }
}

impl Il2CppDumper {
    pub fn new(binary: PathBuf) -> Result<Self> {
        if !binary.exists() {
            return Err(eyre!(format!(
                "IL2CPP dumper binary not found at: {}",
                binary.display()
            )));
        }
        Ok(Self { binary })
    }

    pub fn run(&self, options: Il2CppDumperOptions) -> Result<()> {
        let mut cmd = Command::new(&self.binary);

        cmd.arg("--il2cpp").arg(&options.il2cpp);

        if let Some(metadata) = &options.metadata {
            cmd.arg("--metadata").arg(metadata);
        }

        if let Some(output) = &options.output {
            cmd.arg("--output").arg(output);
        }

        if let Some(unity_version) = &options.unity_version {
            cmd.arg("--unity-version").arg(unity_version);
        }

        if let Some(compiler_type) = &options.compiler_type {
            cmd.arg("--compiler-type").arg(compiler_type);
        }

        if options.output_csharp_stub {
            cmd.arg("--output-csharp-stub");
        }

        if let Some(layout) = &options.layout {
            cmd.arg("--layout").arg(layout);
        }

        if options.flatten_hierarchy {
            cmd.arg("--flatten-hierarchy");
        }

        if options.output_disassembler_metadata {
            cmd.arg("--output-disassembler-metadata");
        }

        if let Some(disassembler) = &options.disassembler {
            cmd.arg("--disassembler").arg(disassembler);
        }

        if options.output_cpp_scaffolding {
            cmd.arg("--output-cpp-scaffolding");
        }

        if let Some(sorting_mode) = &options.sorting_mode {
            cmd.arg("--sorting-mode").arg(sorting_mode);
        }

        if options.suppress_metadata {
            cmd.arg("--suppress-metadata");
        }

        if options.compilable {
            cmd.arg("--compilable");
        }

        if options.separate_assembly_attributes {
            cmd.arg("--separate-assembly-attributes");
        }

        if options.output_dummy_dlls {
            cmd.arg("--output-dummy-dlls");
        }

        if options.output_vs_solution {
            cmd.arg("--output-vs-solution");
        }

        if let Some(unity_path) = &options.unity_path {
            cmd.arg("--unity-path").arg(unity_path);
        }

        if let Some(unity_assemblies_path) = &options.unity_assemblies_path {
            cmd.arg("--unity-assemblies-path")
                .arg(unity_assemblies_path);
        }

        if options.extract_il2cpp_files {
            cmd.arg("--extract-il2-cpp-files");
        }

        let status = cmd.status().wrap_err_with(|| {
            format!(
                "Failed to execute IL2CPP dumper at {}",
                self.binary.display()
            )
        })?;

        if !status.success() {
            return Err(eyre!(
                "IL2CPP dumper failed with exit code {:?}",
                status.code()
            ))
            .wrap_err_with(|| "IL2CPP dumper execution failed");
        }

        Ok(())
    }
}

use crate::wrappers::flatc::FlatCOptions;
use crate::wrappers::fbs_dumper::FbsDumperOptions;
use crate::wrappers::il2cpp_dumper::Il2CppDumperOptions;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Config {
    #[serde(default)]
    pub fbs_dumper: FbsDumperOptions,
    #[serde(default)]
    pub flatc: FlatCOptions,
    #[serde(default)]
    pub il2cpp_dumper: Il2CppDumperOptions,
}

impl Config {
    pub fn load(custom_path: Option<&Path>) -> Result<Self> {
        let config_path = match custom_path {
            Some(path) => {
                if !path.exists() {
                    return Err(anyhow::anyhow!("Config file not found: {}", path.display()));
                }
                Some(path)
            }
            None => {
                let default_path = Path::new("bafb.toml");
                if default_path.exists() {
                    Some(default_path)
                } else {
                    None
                }
            }
        };

        match config_path {
            Some(path) => {
                let content = std::fs::read_to_string(path)?;
                Ok(toml::from_str(&content)?)
            }
            None => Ok(Config::default()),
        }
    }

    pub fn merge_fbs_dumper_config(&self, options: &mut FbsDumperOptions) {
        if self.fbs_dumper.force_snake_case != FbsDumperOptions::default().force_snake_case {
            options.force_snake_case = self.fbs_dumper.force_snake_case;
        }
        if self.fbs_dumper.namespace_to_look_for != FbsDumperOptions::default().namespace_to_look_for {
            options.namespace_to_look_for = self.fbs_dumper.namespace_to_look_for.clone();
        }
    }

    pub fn merge_flatc_config(&self, options: &mut FlatCOptions) {
        let default_options = FlatCOptions::default();
        
        macro_rules! merge_field {
            ($field:ident) => {
                if self.flatc.$field != default_options.$field {
                    options.$field = self.flatc.$field.clone();
                }
            };
        }

        merge_field!(jsonschema);
        merge_field!(strict_json);
        merge_field!(allow_non_utf8);
        merge_field!(natural_utf8);
        merge_field!(defaults_json);
        merge_field!(unknown_json);
        merge_field!(no_prefix);
        merge_field!(scoped_enums);
        merge_field!(no_emit_min_max_enum_values);
        merge_field!(swift_implementation_only);
        merge_field!(no_includes);
        merge_field!(gen_mutable);
        merge_field!(gen_onefile);
        merge_field!(gen_name_strings);
        merge_field!(gen_object_api);
        merge_field!(gen_compare);
        merge_field!(gen_nullable);
        merge_field!(java_package_prefix);
        merge_field!(java_checkerframework);
        merge_field!(gen_generated);
        merge_field!(gen_jvmstatic);
        merge_field!(gen_all);
        merge_field!(gen_json_emit);
        merge_field!(cpp_include);
        merge_field!(cpp_ptr_type);
        merge_field!(cpp_str_type);
        merge_field!(cpp_str_flex_ctor);
        merge_field!(cpp_field_case_style);
        merge_field!(no_cpp_direct_copy);
        merge_field!(cpp_std);
        merge_field!(cpp_static_reflection);
        merge_field!(object_prefix);
        merge_field!(object_suffix);
        merge_field!(go_namespace);
        merge_field!(go_import);
        merge_field!(go_module_name);
        merge_field!(raw_binary);
        merge_field!(size_prefixed);
        merge_field!(proto);
        merge_field!(proto_namespace_suffix);
        merge_field!(oneof_union);
        merge_field!(keep_proto_id);
        merge_field!(proto_id_gap);
        merge_field!(schema);
        merge_field!(bfbs_filenames);
        merge_field!(bfbs_absolute_paths);
        merge_field!(bfbs_comments);
        merge_field!(bfbs_builtins);
        merge_field!(bfbs_gen_embed);
        merge_field!(filename_suffix);
        merge_field!(filename_ext);
        merge_field!(include_prefix);
        merge_field!(keep_prefix);
        merge_field!(reflect_types);
        merge_field!(reflect_names);
        merge_field!(rust_serialize);
        merge_field!(rust_module_root_file);
        merge_field!(root_type);
        merge_field!(require_explicit_ids);
        merge_field!(force_defaults);
        merge_field!(force_empty);
        merge_field!(force_empty_vectors);
        merge_field!(flexbuffers);
        merge_field!(no_warnings);
        merge_field!(warnings_as_errors);
        merge_field!(cs_global_alias);
        merge_field!(cs_gen_json_serializer);
        merge_field!(json_nested_bytes);
        merge_field!(ts_flat_files);
        merge_field!(ts_entry_points);
        merge_field!(annotate_sparse_vectors);
        merge_field!(annotate);
        merge_field!(no_leak_private_annotation);
        merge_field!(python_no_type_prefix_suffix);
        merge_field!(python_typing);
        merge_field!(python_version);
        merge_field!(python_gen_numpy);
        merge_field!(ts_omit_entrypoint);
        merge_field!(file_names_only);
        merge_field!(grpc_filename_suffix);
        merge_field!(grpc_additional_header);
        merge_field!(grpc_search_path);
        merge_field!(grpc_use_system_headers);
        merge_field!(grpc_python_typed_handlers);
    }

    pub fn merge_il2cpp_dumper_config(&self, options: &mut Il2CppDumperOptions) {
        let default_options = Il2CppDumperOptions::default();
        
        macro_rules! merge_field {
            ($field:ident) => {
                if self.il2cpp_dumper.$field != default_options.$field {
                    options.$field = self.il2cpp_dumper.$field.clone();
                }
            };
        }

        merge_field!(binary_files);
        merge_field!(metadata_file);
        merge_field!(image_base);
        merge_field!(select_outputs_only);
        merge_field!(cs_out);
        merge_field!(py_out);
        merge_field!(cpp_out);
        merge_field!(json_out);
        merge_field!(dll_out);
        merge_field!(metadata_out);
        merge_field!(binary_out);
        merge_field!(excluded_namespaces);
        merge_field!(layout);
        merge_field!(sort);
        merge_field!(flatten);
        merge_field!(suppress_metadata);
        merge_field!(suppress_dll_metadata);
        merge_field!(must_compile);
        merge_field!(separate_attributes);
        merge_field!(create_project);
        merge_field!(cpp_compiler);
        merge_field!(script_target);
        merge_field!(unity_path);
        merge_field!(unity_assemblies);
        merge_field!(unity_version);
        merge_field!(unity_version_from_asset);
        merge_field!(plugin_options);
    }
}
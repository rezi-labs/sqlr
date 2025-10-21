use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct GenerateRequest {
    pub settings: Settings,
    pub catalog: Catalog,
    pub queries: Vec<Query>,
    pub sqlc_version: String,
    pub plugin_options: serde_json::Value,
    pub global_options: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct GenerateResponse {
    pub files: Vec<File>,
}

#[derive(Debug, Serialize)]
pub struct File {
    pub name: String,
    pub contents: Vec<u8>,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub version: String,
    pub engine: String,
    pub schema: Vec<String>,
    pub queries: Vec<String>,
    pub codegen: Vec<Codegen>,
}

#[derive(Debug, Deserialize)]
pub struct Codegen {
    pub out: String,
    pub plugin: String,
    pub options: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct Catalog {
    pub comment: Option<String>,
    pub default_schema: String,
    pub name: String,
    pub schemas: Vec<Schema>,
}

#[derive(Debug, Deserialize)]
pub struct Schema {
    pub comment: Option<String>,
    pub name: String,
    pub tables: Vec<Table>,
    pub enums: Vec<Enum>,
    pub composite_types: Vec<CompositeType>,
}

#[derive(Debug, Deserialize)]
pub struct Table {
    pub rel: Identifier,
    pub columns: Vec<Column>,
    pub comment: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Column {
    pub name: String,
    pub not_null: bool,
    pub is_array: bool,
    pub comment: Option<String>,
    pub length: Option<i32>,
    pub is_named_param: bool,
    pub is_func_call: bool,
    pub scope: Option<String>,
    pub table: Option<Identifier>,
    pub table_alias: Option<String>,
    pub r#type: Option<Identifier>,
    pub is_sqlc_slice: bool,
    pub embed_table: Option<Identifier>,
}

#[derive(Debug, Deserialize)]
pub struct Identifier {
    pub catalog: String,
    pub schema: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Enum {
    pub name: String,
    pub vals: Vec<String>,
    pub comment: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CompositeType {
    pub name: String,
    pub comment: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Query {
    pub text: String,
    pub name: String,
    pub cmd: String,
    pub columns: Vec<Column>,
    pub params: Vec<Parameter>,
    pub comments: Vec<String>,
    pub filename: String,
}

#[derive(Debug, Deserialize)]
pub struct Parameter {
    pub number: i32,
    pub column: Column,
}

#[derive(Debug, Deserialize)]
pub struct PluginOptions {
    pub package: Option<String>,
    pub emit_json_tags: Option<bool>,
    pub emit_db_tags: Option<bool>,
    pub emit_prepared_queries: Option<bool>,
    pub emit_interface: Option<bool>,
    pub emit_exact_table_names: Option<bool>,
    pub emit_empty_slices: Option<bool>,
    pub emit_exported_queries: Option<bool>,
    pub emit_result_struct_pointers: Option<bool>,
    pub emit_params_struct_pointers: Option<bool>,
    pub emit_methods_with_db_argument: Option<bool>,
    pub emit_enum_valid_method: Option<bool>,
    pub emit_all_enum_values: Option<bool>,
    pub json_tags_case_style: Option<String>,
    pub output_batch_file_name: Option<String>,
    pub output_db_file_name: Option<String>,
    pub output_models_file_name: Option<String>,
    pub output_querier_file_name: Option<String>,
    pub output_files_suffix: Option<String>,
    pub inflection_exclude_table_names: Option<Vec<String>>,
    pub query_parameter_limit: Option<i32>,
    pub omit_unused_structs: Option<bool>,
    pub omit_sqlc_version: Option<bool>,
    pub build_tags: Option<String>,
    pub sql_package: Option<String>,
    pub sql_driver: Option<String>,
}

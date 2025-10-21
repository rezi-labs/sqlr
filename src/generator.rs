use crate::type_mapping::TypeMapper;
use crate::types::{Enum, File, GenerateRequest, GenerateResponse, PluginOptions, Query, Table};
use anyhow::Result;
use heck::{ToPascalCase, ToSnakeCase};

pub struct RustGenerator {
    request: GenerateRequest,
    options: PluginOptions,
}

impl RustGenerator {
    pub fn new(request: GenerateRequest) -> Self {
        let options = serde_json::from_value(request.plugin_options.clone())
            .unwrap_or_else(|_| PluginOptions::default());

        Self { request, options }
    }

    pub fn generate(&self) -> Result<GenerateResponse> {
        let mut files = Vec::new();

        // Generate models file
        let models_content = self.generate_models()?;
        files.push(File {
            name: self
                .options
                .output_models_file_name
                .clone()
                .unwrap_or_else(|| "models.rs".to_string()),
            contents: models_content.into_bytes(),
        });

        // Generate queries file
        let queries_content = self.generate_queries()?;
        files.push(File {
            name: self
                .options
                .output_db_file_name
                .clone()
                .unwrap_or_else(|| "queries.rs".to_string()),
            contents: queries_content.into_bytes(),
        });

        // Generate lib.rs file
        let lib_content = self.generate_lib()?;
        files.push(File {
            name: "lib.rs".to_string(),
            contents: lib_content.into_bytes(),
        });

        Ok(GenerateResponse { files })
    }

    fn generate_models(&self) -> Result<String> {
        let mut output = String::new();

        // Add imports
        for import in TypeMapper::get_rust_imports() {
            output.push_str(import);
            output.push('\n');
        }
        output.push('\n');

        // Generate structs for each table
        for schema in &self.request.catalog.schemas {
            for table in &schema.tables {
                output.push_str(&self.generate_table_struct(table)?);
                output.push_str("\n\n");
            }

            // Generate enums
            for enum_def in &schema.enums {
                output.push_str(&self.generate_enum(enum_def)?);
                output.push_str("\n\n");
            }
        }

        Ok(output)
    }

    fn generate_table_struct(&self, table: &Table) -> Result<String> {
        let struct_name = table.rel.name.to_pascal_case();
        let mut output = String::new();

        // Add comment if available
        if let Some(comment) = &table.comment {
            output.push_str(&format!("/// {comment}\n"));
        }

        // Add derives
        output.push_str("#[derive(Debug, Clone, Serialize, Deserialize");
        if self.options.emit_json_tags.unwrap_or(false) {
            output.push_str(", FromRow");
        }
        output.push_str(")]\n");

        // Add serde attributes
        if self.options.emit_json_tags.unwrap_or(false) {
            let case_style = self
                .options
                .json_tags_case_style
                .as_deref()
                .unwrap_or("snake_case");
            output.push_str(&format!("#[serde(rename_all = \"{case_style}\")]\n"));
        }

        output.push_str(&format!("pub struct {struct_name} {{\n"));

        for column in &table.columns {
            if let Some(comment) = &column.comment {
                output.push_str(&format!("    /// {comment}\n"));
            }

            let field_name = column.name.to_snake_case();
            let field_type =
                TypeMapper::sql_to_rust_type(&column.r#type, column.not_null, column.is_array);

            if self.options.emit_json_tags.unwrap_or(false) && field_name != column.name {
                output.push_str(&format!("    #[serde(rename = \"{}\")]\n", column.name));
            }

            output.push_str(&format!("    pub {field_name}: {field_type},\n"));
        }

        output.push_str("}\n");
        Ok(output)
    }

    fn generate_enum(&self, enum_def: &Enum) -> Result<String> {
        let enum_name = enum_def.name.to_pascal_case();
        let mut output = String::new();

        if let Some(comment) = &enum_def.comment {
            output.push_str(&format!("/// {comment}\n"));
        }

        output.push_str("#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]\n");
        output.push_str(&format!("pub enum {enum_name} {{\n"));

        for val in &enum_def.vals {
            let variant_name = val.to_pascal_case();
            output.push_str(&format!("    #[serde(rename = \"{val}\")]\n"));
            output.push_str(&format!("    {variant_name},\n"));
        }

        output.push_str("}\n");
        Ok(output)
    }

    fn generate_queries(&self) -> Result<String> {
        let mut output = String::new();

        // Add imports
        for import in TypeMapper::get_rust_imports() {
            output.push_str(import);
            output.push('\n');
        }
        for import in TypeMapper::get_sqlx_imports() {
            output.push_str(import);
            output.push('\n');
        }
        output.push_str("use crate::models::*;\n\n");

        // Generate database struct
        output.push_str("pub struct Database {\n");
        output.push_str("    pool: Pool<Postgres>,\n");
        output.push_str("}\n\n");

        output.push_str("impl Database {\n");
        output.push_str("    pub fn new(pool: Pool<Postgres>) -> Self {\n");
        output.push_str("        Self { pool }\n");
        output.push_str("    }\n\n");

        // Generate methods for each query
        for query in &self.request.queries {
            output.push_str(&self.generate_query_method(query)?);
            output.push('\n');
        }

        output.push_str("}\n");
        Ok(output)
    }

    fn generate_query_method(&self, query: &Query) -> Result<String> {
        let method_name = query.name.to_snake_case();
        let mut output = String::new();

        // Determine return type
        let return_type = self.get_query_return_type(query)?;

        // Generate method signature
        output.push_str(&format!("    pub async fn {method_name}(&self"));

        // Add parameters
        for param in &query.params {
            let param_name = format!("param_{}", param.number);
            let param_type = TypeMapper::sql_to_rust_type(
                &param.column.r#type,
                param.column.not_null,
                param.column.is_array,
            );
            output.push_str(&format!(", {param_name}: {param_type}"));
        }

        output.push_str(&format!(") -> Result<{return_type}, SqlxError> {{\n"));

        // Generate query execution
        output.push_str(&format!(
            "        let query = r#\"\n{}\n        \"#;\n\n",
            query.text
        ));

        match query.cmd.as_str() {
            ":one" => {
                output.push_str("        let row = sqlx::query(query)\n");
                for (i, _) in query.params.iter().enumerate() {
                    output.push_str(&format!("            .bind(param_{})\n", i + 1));
                }
                output.push_str("            .fetch_one(&self.pool)\n");
                output.push_str("            .await?;\n\n");
                output.push_str(&self.generate_row_mapping(query)?);
            }
            ":many" => {
                output.push_str("        let rows = sqlx::query(query)\n");
                for (i, _) in query.params.iter().enumerate() {
                    output.push_str(&format!("            .bind(param_{})\n", i + 1));
                }
                output.push_str("            .fetch_all(&self.pool)\n");
                output.push_str("            .await?;\n\n");
                output.push_str("        let mut results = Vec::new();\n");
                output.push_str("        for row in rows {\n");
                output.push_str(&format!(
                    "            {}",
                    self.generate_row_mapping(query)?
                        .replace("row", "&row")
                        .replace("Ok(", "results.push(")
                ));
                output.push_str("        }\n");
                output.push_str("        Ok(results)\n");
            }
            ":exec" => {
                output.push_str("        sqlx::query(query)\n");
                for (i, _) in query.params.iter().enumerate() {
                    output.push_str(&format!("            .bind(param_{})\n", i + 1));
                }
                output.push_str("            .execute(&self.pool)\n");
                output.push_str("            .await?;\n\n");
                output.push_str("        Ok(())\n");
            }
            _ => {
                output.push_str("        // Unknown command type\n");
                output.push_str("        Ok(())\n");
            }
        }

        output.push_str("    }\n");
        Ok(output)
    }

    fn get_query_return_type(&self, query: &Query) -> Result<String> {
        match query.cmd.as_str() {
            ":one" => {
                if query.columns.len() == 1 {
                    let col = &query.columns[0];
                    Ok(TypeMapper::sql_to_rust_type(
                        &col.r#type,
                        col.not_null,
                        col.is_array,
                    ))
                } else if query.columns.is_empty() {
                    Ok("()".to_string())
                } else {
                    // For multiple columns, use a tuple type
                    let types: Vec<String> = query
                        .columns
                        .iter()
                        .map(|col| {
                            TypeMapper::sql_to_rust_type(&col.r#type, col.not_null, col.is_array)
                        })
                        .collect();
                    Ok(format!("({})", types.join(", ")))
                }
            }
            ":many" => {
                let inner_type = if query.columns.len() == 1 {
                    let col = &query.columns[0];
                    TypeMapper::sql_to_rust_type(&col.r#type, col.not_null, col.is_array)
                } else if query.columns.is_empty() {
                    "()".to_string()
                } else {
                    let types: Vec<String> = query
                        .columns
                        .iter()
                        .map(|col| {
                            TypeMapper::sql_to_rust_type(&col.r#type, col.not_null, col.is_array)
                        })
                        .collect();
                    format!("({})", types.join(", "))
                };
                Ok(format!("Vec<{inner_type}>"))
            }
            ":exec" => Ok("()".to_string()),
            _ => Ok("()".to_string()),
        }
    }

    fn generate_row_mapping(&self, query: &Query) -> Result<String> {
        if query.columns.len() == 1 {
            let col = &query.columns[0];
            let rust_type = TypeMapper::sql_to_rust_type(&col.r#type, col.not_null, col.is_array);
            Ok(format!("        Ok(row.get::<{rust_type}, _>(0))\n"))
        } else if query.columns.len() > 1 {
            // For multiple columns, create a tuple or struct-like mapping
            let mut mapping = String::new();
            mapping.push_str("        Ok((\n");
            for (i, col) in query.columns.iter().enumerate() {
                let rust_type =
                    TypeMapper::sql_to_rust_type(&col.r#type, col.not_null, col.is_array);
                mapping.push_str(&format!("             row.get::<{rust_type}, _>({i}),\n"));
            }
            mapping.push_str("        ))\n");
            Ok(mapping)
        } else {
            Ok("        Ok(())\n".to_string())
        }
    }

    fn generate_lib(&self) -> Result<String> {
        let mut output = String::new();

        output.push_str("pub mod models;\n");
        output.push_str("pub mod queries;\n\n");
        output.push_str("pub use models::*;\n");
        output.push_str("pub use queries::Database;\n");

        Ok(output)
    }
}

impl Default for PluginOptions {
    fn default() -> Self {
        Self {
            package: None,
            emit_json_tags: Some(true),
            emit_db_tags: Some(true),
            emit_prepared_queries: Some(false),
            emit_interface: Some(false),
            emit_exact_table_names: Some(false),
            emit_empty_slices: Some(false),
            emit_exported_queries: Some(true),
            emit_result_struct_pointers: Some(false),
            emit_params_struct_pointers: Some(false),
            emit_methods_with_db_argument: Some(false),
            emit_enum_valid_method: Some(false),
            emit_all_enum_values: Some(false),
            json_tags_case_style: Some("snake_case".to_string()),
            output_batch_file_name: None,
            output_db_file_name: Some("queries.rs".to_string()),
            output_models_file_name: Some("models.rs".to_string()),
            output_querier_file_name: None,
            output_files_suffix: None,
            inflection_exclude_table_names: None,
            query_parameter_limit: None,
            omit_unused_structs: Some(false),
            omit_sqlc_version: Some(false),
            build_tags: None,
            sql_package: Some("sqlx".to_string()),
            sql_driver: Some("postgres".to_string()),
        }
    }
}

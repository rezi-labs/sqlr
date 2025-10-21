use crate::types::Identifier;

pub struct TypeMapper;

impl TypeMapper {
    pub fn sql_to_rust_type(sql_type: &Option<Identifier>, not_null: bool, is_array: bool) -> String {
        let base_type = match sql_type {
            Some(identifier) => Self::map_sql_type(&identifier.name),
            None => "String".to_string(),
        };

        let wrapped_type = if is_array {
            format!("Vec<{}>", base_type)
        } else {
            base_type
        };

        if not_null {
            wrapped_type
        } else {
            format!("Option<{}>", wrapped_type)
        }
    }

    fn map_sql_type(sql_type: &str) -> String {
        match sql_type.to_lowercase().as_str() {
            // Integer types
            "int2" | "smallint" => "i16",
            "int4" | "integer" | "int" => "i32", 
            "int8" | "bigint" => "i64",
            "serial2" | "smallserial" => "i16",
            "serial4" | "serial" => "i32",
            "serial8" | "bigserial" => "i64",

            // Floating point types
            "real" | "float4" => "f32",
            "double" | "float8" | "double precision" => "f64",
            "numeric" | "decimal" => "rust_decimal::Decimal",

            // String types
            "text" | "varchar" | "char" | "character varying" | "character" | "bpchar" => "String",

            // Boolean
            "bool" | "boolean" => "bool",

            // Binary data
            "bytea" => "Vec<u8>",

            // Date and time types
            "date" => "chrono::NaiveDate",
            "time" | "time without time zone" => "chrono::NaiveTime",
            "timetz" | "time with time zone" => "chrono::Time<chrono::FixedOffset>",
            "timestamp" | "timestamp without time zone" => "chrono::NaiveDateTime",
            "timestamptz" | "timestamp with time zone" => "chrono::DateTime<chrono::Utc>",
            "interval" => "chrono::Duration",

            // UUID
            "uuid" => "uuid::Uuid",

            // JSON types
            "json" | "jsonb" => "serde_json::Value",

            // Network types
            "inet" => "std::net::IpAddr",
            "cidr" => "String", // Could be a custom type
            "macaddr" => "String",

            // Geometric types (simplified)
            "point" | "line" | "lseg" | "box" | "path" | "polygon" | "circle" => "String",

            // Array types are handled in the caller
            _ if sql_type.ends_with("[]") => {
                let element_type = &sql_type[..sql_type.len() - 2];
                return format!("Vec<{}>", Self::map_sql_type(element_type));
            },

            // Unknown type, default to String
            _ => "String",
        }.to_string()
    }

    pub fn get_rust_imports() -> Vec<&'static str> {
        vec![
            "use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};",
            "use serde::{Deserialize, Serialize};",
            "use uuid::Uuid;",
            "use rust_decimal::Decimal;",
        ]
    }

    pub fn get_sqlx_imports() -> Vec<&'static str> {
        vec![
            "use sqlx::{FromRow, Row, Pool, Postgres, Error as SqlxError};",
            "use sqlx::postgres::PgRow;",
        ]
    }
}
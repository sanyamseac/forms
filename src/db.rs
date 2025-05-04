// src/db.rs
use anyhow::Result;
use scylla::{Session, FromRow};
use scylla::frame::response::result::CqlValue;
use serde_json::Value;
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;
use std::collections::HashMap;

use crate::{
    error::AppError,
    models::{FormResponse, FormSchema},
};

// Keep FromRow derive for potential future use or if parts still use it,
// but the manual parsing logic below overrides it for now.
#[derive(FromRow)]
struct FormSchemaRow {
    id: Uuid,
    name: String,
    description: Option<String>,
    fields: String,
    created_at: Option<chrono::DateTime<chrono::Utc>>,
    updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(FromRow)]
struct FormResponseRow {
    id: Uuid,
    form_id: Uuid,
    data: String,
    created_at: chrono::DateTime<chrono::Utc>,
}

pub async fn init_database(session: &Session) -> Result<()> {
    // Create keyspace
    session
        .query(
            "CREATE KEYSPACE IF NOT EXISTS form_portal WITH REPLICATION = {'class': 'SimpleStrategy', 'replication_factor': 1}",
            &[],
        )
        .await?;

    // Create form_schemas table
    session
        .query(
            "CREATE TABLE IF NOT EXISTS form_portal.form_schemas (
                id uuid PRIMARY KEY,
                name text,
                description text,
                fields text,
                created_at timestamp,
                updated_at timestamp
            )",
            &[],
        )
        .await?;

    Ok(())
}

pub async fn create_form_schema(session: &Arc<Session>, schema: FormSchema) -> Result<Uuid, AppError> {
    let id = schema.id.unwrap_or_else(Uuid::new_v4);

    let fields_json = serde_json::to_string(&schema.fields)
        .map_err(|e| AppError::InternalError(format!("Failed to serialize fields: {}", e)))?;

    let now = Utc::now();

    session
        .query(
            "INSERT INTO form_portal.form_schemas (id, name, description, fields, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?)",
            (
                id,
                schema.name,
                schema.description,
                fields_json,
                now,
                now,
            ),
        )
        .await
        .map_err(|e| AppError::DbError(format!("Failed to insert form schema: {}", e)))?;

    // Create a table for form responses
    let table_name = format!("form_portal.form_responses_{}", id.to_string().replace("-", ""));

    session
        .query(
            format!(
                "CREATE TABLE IF NOT EXISTS {} (
                    id uuid PRIMARY KEY,
                    form_id uuid,
                    data text,
                    created_at timestamp
                )",
                table_name
            ).as_str(),
            &[],
        )
        .await
        .map_err(|e| AppError::DbError(format!("Failed to create form responses table: {}", e)))?;

    Ok(id)
}

// Fix in get_form_schema function:
pub async fn get_form_schema(session: &Arc<Session>, id: Uuid) -> Result<FormSchema, AppError> {
    let result = session
        .query(
            "SELECT id, name, description, fields, created_at, updated_at FROM form_portal.form_schemas WHERE id = ?",
            (id,),
        )
        .await
        .map_err(|e| AppError::DbError(format!("Failed to fetch form schema: {}", e)))?;

    let row = result.first_row()
        .map_err(|_| AppError::NotFound(format!("Form schema with ID {} not found", id)))?;

    // Parse the row manually using pattern matching for timestamp
    let schema_row = FormSchemaRow {
        id: row.columns[0].as_ref().and_then(|v| v.as_uuid()).ok_or_else(||
            AppError::InternalError("Failed to get id column".to_string()))?,
        name: row.columns[1].as_ref().and_then(|v| v.as_text()).ok_or_else(||
            AppError::InternalError("Failed to get name column".to_string()))?.to_string(),
        description: row.columns[2].as_ref().and_then(|v| v.as_text()).map(|s| s.to_string()),
        fields: row.columns[3].as_ref().and_then(|v| v.as_text()).ok_or_else(||
            AppError::InternalError("Failed to get fields column".to_string()))?.to_string(),
        created_at: row.columns[4].as_ref().and_then(|v| match v {
            CqlValue::Timestamp(ts) => {
                // Convert TimeDelta to milliseconds i64
                let millis = ts.num_milliseconds();
                Some(chrono::DateTime::<chrono::Utc>::from_timestamp_millis(millis).unwrap_or_default())
            },
            _ => None,
        }),
        updated_at: row.columns[5].as_ref().and_then(|v| match v {
            CqlValue::Timestamp(ts) => {
                // Convert TimeDelta to milliseconds i64
                let millis = ts.num_milliseconds();
                Some(chrono::DateTime::<chrono::Utc>::from_timestamp_millis(millis).unwrap_or_default())
            },
            _ => None,
        }),
    };

    let fields = serde_json::from_str(&schema_row.fields)
        .map_err(|e| AppError::InternalError(format!("Failed to deserialize fields: {}", e)))?;

    let schema = FormSchema {
        id: Some(schema_row.id),
        name: schema_row.name,
        description: schema_row.description,
        fields,
        created_at: schema_row.created_at,
        updated_at: schema_row.updated_at,
    };

    Ok(schema)
}

pub async fn submit_form_response(
    session: &Arc<Session>,
    response: FormResponse,
) -> Result<Uuid, AppError> {
    let id = response.id.unwrap_or_else(Uuid::new_v4);
    let form_id = response.form_id;

    let table_name = format!("form_portal.form_responses_{}", form_id.to_string().replace("-", ""));

    let data_json = serde_json::to_string(&response.data)
        .map_err(|e| AppError::InternalError(format!("Failed to serialize form data: {}", e)))?;

    let now = Utc::now();

    session
        .query(
            format!(
                "INSERT INTO {} (id, form_id, data, created_at) VALUES (?, ?, ?, ?)",
                table_name
            ).as_str(),
            (id, form_id, data_json, now),
        )
        .await
        .map_err(|e| AppError::DbError(format!("Failed to insert form response: {}", e)))?;

    Ok(id)
}

pub async fn get_form_responses(
    session: &Arc<Session>,
    form_id: Uuid,
) -> Result<Vec<FormResponse>, AppError> {
    let table_name = format!("form_portal.form_responses_{}", form_id.to_string().replace("-", ""));

    let result = session
        .query(
            format!(
                "SELECT id, form_id, data, created_at FROM {}",
                table_name
            ).as_str(),
            &[],
        )
        .await
        .map_err(|e| AppError::DbError(format!("Failed to fetch form responses: {}", e)))?;

    let mut responses = Vec::new();

    // In get_form_responses function:
    for row in result.rows.unwrap_or_default() {
        // Parse the row manually using pattern matching for timestamp
        let response_row = FormResponseRow {
            id: row.columns[0].as_ref().and_then(|v| v.as_uuid()).ok_or_else(||
                AppError::InternalError("Failed to get id column".to_string()))?,
            form_id: row.columns[1].as_ref().and_then(|v| v.as_uuid()).ok_or_else(||
                AppError::InternalError("Failed to get form_id column".to_string()))?,
            data: row.columns[2].as_ref().and_then(|v| v.as_text()).ok_or_else(||
                AppError::InternalError("Failed to get data column".to_string()))?.to_string(),
            created_at: row.columns[3].as_ref().and_then(|v| match v {
                CqlValue::Timestamp(ts) => {
                    // Convert TimeDelta to milliseconds i64
                    let millis = ts.num_milliseconds();
                    Some(chrono::DateTime::<chrono::Utc>::from_timestamp_millis(millis).unwrap_or_default())
                },
                _ => None,
            }).ok_or_else(|| AppError::InternalError("Failed to get created_at column or invalid type".to_string()))?,
    };

        // Rest of the function as before
        let data: HashMap<String, Value> = serde_json::from_str(&response_row.data)
            .map_err(|e| AppError::InternalError(format!("Failed to deserialize response data: {}", e)))?;

        let response = FormResponse {
            id: Some(response_row.id),
            form_id: response_row.form_id,
            data,
            created_at: Some(response_row.created_at),
        };

        responses.push(response);
    }
    Ok(responses)
}
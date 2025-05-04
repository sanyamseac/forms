// src/models.rs
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum FieldType {
    Text,
    Number,
    Email,
    Date,
    Checkbox,
    Select,
    Radio,
    Textarea,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FieldOption {
    pub value: String,
    pub label: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FormField {
    pub id: String,
    pub label: String,
    pub field_type: FieldType,
    pub required: bool,
    pub placeholder: Option<String>,
    pub options: Option<Vec<FieldOption>>,
    pub validation: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FormSchema {
    pub id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub fields: Vec<FormField>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FormResponse {
    pub id: Option<Uuid>,
    pub form_id: Uuid,
    pub data: HashMap<String, serde_json::Value>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }
    
    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message),
        }
    }
}
// src/api.rs
use actix_web::{get, post, web, HttpResponse, Responder};
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    db,
    error::AppError,
    models::{ApiResponse, FormResponse, FormSchema},
    templates,
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(register_form_schema)
            .service(get_form_by_id)
            .service(render_form)
            .service(submit_form)
            .service(get_form_responses),
    );
}

#[post("/forms")]
async fn register_form_schema(
    session: web::Data<Arc<scylla::Session>>,
    form_schema: web::Json<FormSchema>,
) -> Result<impl Responder, AppError> {
    let form_id = db::create_form_schema(&session, form_schema.into_inner()).await?;
    
    Ok(HttpResponse::Created().json(ApiResponse::success(json!({
        "id": form_id,
        "message": "Form schema registered successfully"
    }))))
}

#[get("/forms/{id}")]
async fn get_form_by_id(
    session: web::Data<Arc<scylla::Session>>,
    path: web::Path<String>,
) -> Result<impl Responder, AppError> {
    let id = Uuid::parse_str(&path.into_inner())
        .map_err(|_| AppError::BadRequest("Invalid UUID format".to_string()))?;
    
    let schema = db::get_form_schema(&session, id).await?;
    
    Ok(HttpResponse::Ok().json(ApiResponse::success(schema)))
}

#[get("/forms/{id}/render")]
async fn render_form(
    session: web::Data<Arc<scylla::Session>>,
    path: web::Path<String>,
) -> Result<impl Responder, AppError> {
    let id = Uuid::parse_str(&path.into_inner())
        .map_err(|_| AppError::BadRequest("Invalid UUID format".to_string()))?;
    
    let schema = db::get_form_schema(&session, id).await?;
    
    let html = templates::generate_form_html(&schema);
    
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

#[post("/forms/{id}/submit")]
async fn submit_form(
    session: web::Data<Arc<scylla::Session>>,
    path: web::Path<String>,
    form: web::Form<HashMap<String, String>>,
) -> Result<impl Responder, AppError> {
    let form_id = Uuid::parse_str(&path.into_inner())
        .map_err(|_| AppError::BadRequest("Invalid UUID format".to_string()))?;
    
    // Validate that the form exists
    let schema = db::get_form_schema(&session, form_id).await?;
    
    // Convert form data to JSON values
    let mut data = HashMap::new();
    for (key, value) in form.into_inner() {
        // Find the field in the schema to determine the type
        if let Some(field) = schema.fields.iter().find(|f| f.id == key) {
            let json_value = match field.field_type {
                crate::models::FieldType::Number => {
                    if value.is_empty() {
                        json!(null)
                    } else {
                        match value.parse::<f64>() {
                            Ok(num) => json!(num),
                            Err(_) => json!(value),
                        }
                    }
                }
                crate::models::FieldType::Checkbox => {
                    if value == "on" {
                        json!(true)
                    } else {
                        json!(value)
                    }
                }
                _ => json!(value),
            };
            
            data.insert(key, json_value);
        }
    }
    
    let response = FormResponse {
        id: None,
        form_id,
        data,
        created_at: None,
    };
    
    let response_id = db::submit_form_response(&session, response).await?;
    
    Ok(HttpResponse::Created().json(ApiResponse::success(json!({
        "id": response_id,
        "message": "Form submitted successfully"
    }))))
}

#[get("/forms/{id}/responses")]
async fn get_form_responses(
    session: web::Data<Arc<scylla::Session>>,
    path: web::Path<String>,
) -> Result<impl Responder, AppError> {
    let form_id = Uuid::parse_str(&path.into_inner())
        .map_err(|_| AppError::BadRequest("Invalid UUID format".to_string()))?;
    
    // Validate that the form exists
    let _ = db::get_form_schema(&session, form_id).await?;
    
    let responses = db::get_form_responses(&session, form_id).await?;
    
    Ok(HttpResponse::Ok().json(ApiResponse::success(responses)))
}
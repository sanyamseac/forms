// src/templates.rs
use uuid::Uuid;
use handlebars::{Handlebars};
use serde_json::json;

use crate::models::{FieldType, FormSchema};

pub fn generate_form_html(schema: &FormSchema) -> String {
    let mut handlebars = Handlebars::new();
    
    // Register the form template
    handlebars
        .register_template_string(
            "form",
            r#"
            <!DOCTYPE html>
            <html lang="en">
            <head>
                <meta charset="UTF-8">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <title>{{name}}</title>
                <style>
                    body {
                        font-family: Arial, sans-serif;
                        max-width: 800px;
                        margin: 0 auto;
                        padding: 20px;
                    }
                    .form-group {
                        margin-bottom: 20px;
                    }
                    label {
                        display: block;
                        margin-bottom: 5px;
                        font-weight: bold;
                    }
                    input[type="text"],
                    input[type="number"],
                    input[type="email"],
                    input[type="date"],
                    textarea,
                    select {
                        width: 100%;
                        padding: 10px;
                        border: 1px solid #ddd;
                        border-radius: 4px;
                    }
                    button {
                        background-color: #4CAF50;
                        color: white;
                        padding: 10px 15px;
                        border: none;
                        border-radius: 4px;
                        cursor: pointer;
                    }
                    .checkbox-group label, .radio-group label {
                        font-weight: normal;
                        display: flex;
                        align-items: center;
                        margin-bottom: 5px;
                    }
                    .checkbox-group input, .radio-group input {
                        margin-right: 10px;
                    }
                </style>
            </head>
            <body>
                <h1>{{name}}</h1>
                {{#if description}}
                <p>{{description}}</p>
                {{/if}}
                
                <form action="/api/forms/{{id}}/submit" method="post">
                    {{#each fields}}
                    <div class="form-group">
                        <label for="{{id}}">{{label}}{{#if required}} *{{/if}}</label>
                        
                        {{#if_eq field_type "Text"}}
                        <input type="text" id="{{id}}" name="{{id}}" {{#if required}}required{{/if}} {{#if placeholder}}placeholder="{{placeholder}}"{{/if}}>
                        {{/if_eq}}
                        
                        {{#if_eq field_type "Number"}}
                        <input type="number" id="{{id}}" name="{{id}}" {{#if required}}required{{/if}} {{#if placeholder}}placeholder="{{placeholder}}"{{/if}}>
                        {{/if_eq}}
                        
                        {{#if_eq field_type "Email"}}
                        <input type="email" id="{{id}}" name="{{id}}" {{#if required}}required{{/if}} {{#if placeholder}}placeholder="{{placeholder}}"{{/if}}>
                        {{/if_eq}}
                        
                        {{#if_eq field_type "Date"}}
                        <input type="date" id="{{id}}" name="{{id}}" {{#if required}}required{{/if}}>
                        {{/if_eq}}
                        
                        {{#if_eq field_type "Textarea"}}
                        <textarea id="{{id}}" name="{{id}}" {{#if required}}required{{/if}} {{#if placeholder}}placeholder="{{placeholder}}"{{/if}}></textarea>
                        {{/if_eq}}
                        
                        {{#if_eq field_type "Select"}}
                        <select id="{{id}}" name="{{id}}" {{#if required}}required{{/if}}>
                            <option value="">-- Select --</option>
                            {{#each options}}
                            <option value="{{value}}">{{label}}</option>
                            {{/each}}
                        </select>
                        {{/if_eq}}
                        
                        {{#if_eq field_type "Checkbox"}}
                        <div class="checkbox-group">
                            {{#each options}}
                            <label>
                                <input type="checkbox" name="{{../id}}" value="{{value}}"> {{label}}
                            </label>
                            {{/each}}
                        </div>
                        {{/if_eq}}
                        
                        {{#if_eq field_type "Radio"}}
                        <div class="radio-group">
                            {{#each options}}
                            <label>
                                <input type="radio" name="{{../id}}" value="{{value}}" {{#if ../required}}required{{/if}}> {{label}}
                            </label>
                            {{/each}}
                        </div>
                        {{/if_eq}}
                    </div>
                    {{/each}}
                    
                    <button type="submit">Submit</button>
                </form>
            </body>
            </html>
            "#,
        )
        .expect("Failed to register template");
    
    // Replace the if_eq helper implementation with this version:
    handlebars.register_helper(
        "if_eq",
        Box::new(move |h: &handlebars::Helper,
                      _: &handlebars::Handlebars,
                      _: &handlebars::Context,
                      _: &mut handlebars::RenderContext,
                      out: &mut dyn handlebars::Output| {
            // Get parameters
            let param = h.param(0).and_then(|v| Some(v.value()));
            let value = h.param(1).and_then(|v| Some(v.value()));
            
            // Compare values if both are present
            let result = match (param, value) {
                (Some(p), Some(v)) => p == v,
                _ => false,
            };
            
            // Just output true or false instead of rendering templates
            out.write(if result { "true" } else { "false" })?;
            
            Ok(())
        })
    );
    
    // Convert fields to proper JSON representation
    let fields_json = schema
        .fields
        .iter()
        .map(|field| {
            let field_type = match field.field_type {
                FieldType::Text => "Text",
                FieldType::Number => "Number",
                FieldType::Email => "Email",
                FieldType::Date => "Date",
                FieldType::Checkbox => "Checkbox",
                FieldType::Select => "Select",
                FieldType::Radio => "Radio",
                FieldType::Textarea => "Textarea",
            };
            
            json!({
                "id": field.id,
                "label": field.label,
                "field_type": field_type,
                "required": field.required,
                "placeholder": field.placeholder,
                "options": field.options,
                "validation": field.validation
            })
        })
        .collect::<Vec<_>>();
    
    // Render the template
    handlebars
        .render(
            "form",
            &json!({
                "id": schema.id.unwrap_or_else(Uuid::new_v4).to_string(),
                "name": schema.name,
                "description": schema.description,
                "fields": fields_json
            }),
        )
        .unwrap_or_else(|_| "Failed to render form template".to_string())
}
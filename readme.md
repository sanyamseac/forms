# Form Portal API Usage Guide

This guide explains how to use the Form Portal API to create, render, submit, and retrieve form data.

## 1. Register a Form Schema

First, create a new form schema with the desired fields:

```bash
curl -X POST http://localhost:8080/api/forms \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Customer Feedback",
    "description": "Please share your feedback about our service",
    "fields": [
      {
        "id": "name",
        "label": "Your Name",
        "field_type": "Text",
        "required": true,
        "placeholder": "Enter your full name"
      },
      {
        "id": "email",
        "label": "Email Address",
        "field_type": "Email",
        "required": true,
        "placeholder": "your.email@example.com"
      },
      {
        "id": "rating",
        "label": "How would you rate our service?",
        "field_type": "Radio",
        "required": true,
        "options": [
          { "value": "5", "label": "Excellent" },
          { "value": "4", "label": "Good" },
          { "value": "3", "label": "Average" },
          { "value": "2", "label": "Poor" },
          { "value": "1", "label": "Very Poor" }
        ]
      },
      {
        "id": "comments",
        "label": "Additional Comments",
        "field_type": "Textarea",
        "required": false,
        "placeholder": "Please share any additional feedback..."
      }
    ]
  }'
```

This will return a response with the form ID:

```json
{
  "success": true,
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "message": "Form schema registered successfully"
  },
  "error": null
}
```

Save this ID as you'll need it to access your form later.

## 2. Render the Form

To get the HTML for your form, use the render endpoint:

```
GET http://localhost:8080/api/forms/{form_id}/render
```

For example:
```
http://localhost:8080/api/forms/550e8400-e29b-41d4-a716-446655440000/render
```

This will return an HTML page with your form rendered and ready to use.

## 3. Submit Form Data

Users will submit the form directly through the HTML form, which will POST to:

```
POST http://localhost:8080/api/forms/{form_id}/submit
```

The form submission will be handled automatically by the browser when the user clicks the Submit button.

## 4. Retrieve Form Responses

To get all responses for a specific form:

```bash
curl http://localhost:8080/api/forms/{form_id}/responses
```

For example:
```bash
curl http://localhost:8080/api/forms/550e8400-e29b-41d4-a716-446655440000/responses
```

This will return all responses in JSON format:

```json
{
  "success": true,
  "data": [
    {
      "id": "7f8d9e10-abcd-efgh-ijkl-123456789012",
      "form_id": "550e8400-e29b-41d4-a716-446655440000",
      "data": {
        "name": "John Doe",
        "email": "john.doe@example.com",
        "rating": "4",
        "comments": "Great service, but could be faster."
      },
      "created_at": "2025-05-04T10:30:15.123Z"
    },
    {
      "id": "2a3b4c5d-efgh-ijkl-mnop-987654321098",
      "form_id": "550e8400-e29b-41d4-a716-446655440000",
      "data": {
        "name": "Jane Smith",
        "email": "jane.smith@example.com",
        "rating": "5",
        "comments": "Excellent experience overall!"
      },
      "created_at": "2025-05-04T11:45:22.456Z"
    }
  ],
  "error": null
}
```

## 5. Get Form Schema Details

To retrieve the details of a form schema:

```bash
curl http://localhost:8080/api/forms/{form_id}
```

For example:
```bash
curl http://localhost:8080/api/forms/550e8400-e29b-41d4-a716-446655440000
```

This returns the complete form schema configuration:

```json
{
  "success": true,
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "name": "Customer Feedback",
    "description": "Please share your feedback about our service",
    "fields": [
      {
        "id": "name",
        "label": "Your Name",
        "field_type": "Text",
        "required": true,
        "placeholder": "Enter your full name",
        "options": null,
        "validation": null
      },
      // ... other fields
    ],
    "created_at": "2025-05-04T09:12:34.567Z",
    "updated_at": "2025-05-04T09:12:34.567Z"
  },
  "error": null
}
```

## 6. Field Types Reference

When creating form schemas, you can use the following field types:

- `Text`: Standard text input
- `Number`: Numeric input
- `Email`: Email address input with basic validation
- `Date`: Date picker
- `Checkbox`: Multiple-choice checkboxes
- `Select`: Dropdown select menu
- `Radio`: Single-choice radio buttons
- `Textarea`: Multi-line text input

For fields with options (Checkbox, Select, Radio), include an `options` array with value-label pairs.
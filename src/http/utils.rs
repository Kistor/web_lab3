use axum::Json;
use serde::Serialize;
use serde_json::{json, Value};

pub fn response<T: Serialize, E: ToString>(result: Result<T, E>) -> Json<Value> {
    Json(match result {
        Ok(response) => json!({
            "response": response,
            "success": true,
        }),
        Err(err) => json!({
            "error": err.to_string(),
            "success": false,
        }),
    })
}

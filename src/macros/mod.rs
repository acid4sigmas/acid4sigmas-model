#[macro_export]
macro_rules! error_response {
    ($status_code:expr, $message:expr) => {
        HttpResponse::build(actix_web::http::StatusCode::from_u16($status_code).unwrap())
            .json(serde_json::json!({ "error": $message }))
    };
}

#[macro_export]
macro_rules! message_response {
    ($message:expr) => {
        HttpResponse::build(actix_web::http::StatusCode::OK)
            .json(serde_json::json!({"message": $message}))
    }
}

#[macro_export]
macro_rules! token_response {
    ($token:expr) => {
        HttpResponse::build(actix_web::http::StatusCode::OK)
            .json(serde_json::json!({"token": $token}))
    }
}

#[macro_export]
macro_rules! to_string_ {
    ($s:expr) => {
        $s.to_string()
    };
}

use axum::{
    http::StatusCode,
    middleware::Next,
    response::Response,
    extract::Request,
};


pub async fn auth(req: Request, next: Next) -> Result<Response, StatusCode> {


println!("Auth middleware request detected");


    // let auth_header = req.headers()
    //     .get(axum::http::header::AUTHORIZATION)
    //     .and_then(|header| header.to_str().ok());

    // let auth_header = if let Some(auth_header) = auth_header {
    //     auth_header
    // } else {
    //     return Err(StatusCode::UNAUTHORIZED);
    // };

    // if let Some(current_user) = authorize_current_user(auth_header).await {
        // insert the current user into a request extension so the handler can
        // extract it
        // req.extensions_mut().insert(current_user);
        Ok(next.run(req).await)
    // } else {
    //     Err(StatusCode::UNAUTHORIZED)
    // }
}
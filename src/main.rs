mod utils;

use axum::{
    body::{Body, BoxBody},
    extract::{State, TypedHeader},
    routing::{get, post},
    headers::Cookie,
    http::{Request, StatusCode, Uri},
    response::{IntoResponse, Response, Result},
    Router,
};
use dotenv::dotenv;
use tokio::fs::read;
use tower::ServiceExt;
use tower_http::services::ServeDir;

use std::sync::Arc;

use crate::utils::*;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let config = Arc::new(Config::init().await);

    let app = Router::new()
        .route("/", get(root))
        .route("/auth", post(auth_handler))
        .fallback(fallback)
        .with_state(config.clone())
    ;

    axum::Server::bind(&config.run)
        .serve(app.into_make_service())
        .await
        .unwrap();
} 

async fn root(State(config): State<Arc<Config>>) -> (StatusCode, String) {
    let mut s = unsafe {
        String::from_utf8_unchecked(match (read(format!("{}/partial/index.html.top", config.static_path))).await {
            Ok(s) => s,
            Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "oops".to_string())
        })
    };
    
    // if let Some(token_value)

    s.push_str(&format!(r#"
<a href="{}">git</a> |
<a href="settings.html">settings</a> |
<button onclick="logout()">Log Out</button>
        "#,
        config.git_url,
    ));


    s.push_str( unsafe {
        String::from_utf8_unchecked(match read(format!("{}/partial/index.html.bottom", config.static_path)).await {
            Ok(s) => s,
            Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "oops".to_string())
        }).as_str()
    });


    (StatusCode::OK, s)
}

async fn auth_handler(
    State(config): State<Arc<Config>>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    payload: Option<String>, // TODO: accept formdata only?
) -> Response {
    if let Some(mut data) = payload {
        if data.len() <= 1 {
            return StatusCode::BAD_REQUEST.into_response()
        }

        match AuthRequests::from(data.remove(0)) {
            AuthRequests::RequestSalt => {
                let values = sqlx::query!(
                    "SELECT salt, status FROM user WHERE username = ?;",
                    data,
                )
                .fetch_optional(&config.db)
                .await;

                if let Ok(Some(record)) = values {
                    if UserStatus::from_int_unchecked(record.status.unsigned_abs() as u8) != UserStatus::None {
                        record.salt.into_response()
                    } else {
                        StatusCode::FORBIDDEN.into_response()
                    } 
                } else {
                    StatusCode::FORBIDDEN.into_response()
                }
            }
            AuthRequests::GenerateToken => {
                todo!()
            }
            AuthRequests::Register => {
                todo!()
            } 
            AuthRequests::Logout => {
                todo!()
            } 
            AuthRequests::None => StatusCode::BAD_REQUEST.into_response()
        }
    } else {
        StatusCode::BAD_REQUEST.into_response()
    }
} 

async fn fallback(State(config): State<Arc<Config>>, req: Request) -> Result<Response<BoxBody>, (StatusCode, String)> {
    let uri = req.uri();

    if uri == "/logout.html" || uri.to_string().starts_with("/partial/") {
        return Err((StatusCode::NOT_FOUND, format!("what is bro doing at {}", uri)))
    } 

    let res = get_static_file(config.clone(), req.clone()).await?;

    if res.status() == StatusCode::NOT_FOUND {
        // try with `.html`
        // TODO: handle if the Uri has query parameters
        get_static_file(config.clone(), match format!("{}.html", uri).parse() {
            Ok(u) => u,
            Err(_) => return Err((StatusCode::NOT_FOUND, "Invalid URI".to_string())),
        }.into_request()).await
    } else {
        Ok(res)
    }
}

async fn get_static_file(config: Arc<Config>, req: Request) -> Result<Response<BoxBody>, (StatusCode, String)> {
    // `ServeDir` implements `tower::Service` so we can call it with `tower::ServiceExt::oneshot`
    match ServeDir::new(config.static_path).oneshot(req).await {
        Ok(res) => Ok(res),
        Err(err) => Err((
            StatusCode::NOT_FOUND,
            format!("Something went wrong: {}", err),
        )),
    }
}

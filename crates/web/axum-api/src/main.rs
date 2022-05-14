use axum::{
    extract::{Path, Query},
    http::{Method, Request, StatusCode},
    middleware::{self, Next},
    response::IntoResponse,
    routing::{any, get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", any(root))
        .route("/user/:user_id", get(get_user))
        .route("/user", get(get_users))
        .route("/user", post(create_user))
        .route_layer(middleware::from_fn(req_log));

    let addr = SocketAddr::from(([0, 0, 0, 0], 80));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// request log
async fn req_log<B>(req: Request<B>, next: Next<B>) -> impl IntoResponse {
    println!(
        "path: {}, method: {}",
        req.uri().path(),
        req.method().to_string()
    );
    if *req.method() == Method::HEAD {
        Err(StatusCode::BAD_REQUEST)
    } else {
        Ok(next.run(req).await)
    }
}

// home handler
async fn root() -> &'static str {
    "Hello, World!"
}

// get user
async fn get_user(Path(user_id): Path<u64>) -> Json<User> {
    let user = User {
        id: user_id,
        username: "test".to_string(),
    };

    Json(user)
}

// get users
async fn get_users(Query(params): Query<HashMap<String, String>>) -> Json<Vec<User>> {
    if params.len() > 0 {
        println!("query params: {:?}", params)
    }

    let mut users = Vec::<User>::new();
    users.push(User {
        id: 10001,
        username: "test".to_string(),
    });

    Json(users)
}

// create user handler
async fn create_user(Json(payload): Json<CreateUser>) -> impl IntoResponse {
    let user = User {
        id: 1000,
        username: payload.username,
    };

    (StatusCode::CREATED, Json(user))
}

#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}

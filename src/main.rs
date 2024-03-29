use axum::{routing::get, Router};


#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new().route("/", get(hello_world));
    
    Ok(router.into())
}

async fn hello_world() -> &'static str {
    "Hello, world!"
}

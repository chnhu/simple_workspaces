mod db;
pub mod handler;
pub mod models;
pub mod repository;
pub mod schema;

use dotenv::dotenv;
use tracing_subscriber::{
    layer::SubscriberExt,
    Registry,
};
use axum::{
    routing::get, Router,
};
use crate::handler::{
    get_workspace_by_id_handler,
    list_all_workspace_hanlder,
    create_workspace_handler,
    update_workspace_handler
};


#[tokio::main]
async fn main() {

    dotenv().ok();

    // Start tracing.
    let subscriber = Registry::default().with(tracing_subscriber::fmt::layer());
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set logger");


    let pool = db::connect().await.expect("Failed to connect to database");

    let app = Router::new()
        .route("/workspace/:id", get(get_workspace_by_id_handler).patch(update_workspace_handler))
        .route("/workspaces", get(list_all_workspace_hanlder).post(create_workspace_handler))
        .with_state(pool);

    println!("Running on http://localhost:3000");
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
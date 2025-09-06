use axum::{
    extract::Path,
    response::Json,
    routing::get,
    Router,
};
use serde::Serialize;
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tokio::sync::RwLock;

// Define a simple Verse struct
#[derive(Serialize)]
struct Verse {
    text: String,
}

// Shared state type
type Bible = Arc<RwLock<HashMap<String, HashMap<String, HashMap<String, String>>>>>;

#[tokio::main]
async fn main() {
    // Load Bible JSON (example: KJV)
    let file = std::fs::read_to_string("kjv.json").expect("Failed to read Bible JSON");
    let kjv: HashMap<String, HashMap<String, HashMap<String, String>>> =
        serde_json::from_str(&file).expect("Invalid JSON format");

    let bible_state: Bible = Arc::new(RwLock::new(kjv));

    // Define routes
    let app = Router::new()
        .route("/v1/kjv/:book/:chapter/:verse", get(get_verse))
        .with_state(bible_state);

    // Run server
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("🚀 Bible API running on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Handler: /v1/kjv/:book/:chapter/:verse
async fn get_verse(
    Path((book, chapter, verse)): Path<(String, String, String)>,
    state: axum::extract::State<Bible>,
) -> Json<Verse> {
    let bible = state.read().await;
    let text = bible
        .get(&book)
        .and_then(|chapters| chapters.get(&chapter))
        .and_then(|verses| verses.get(&verse))
        .cloned()
        .unwrap_or_else(|| "Verse not found".to_string());

    Json(Verse { text })
}

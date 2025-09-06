use axum::{
    extract::{Path, State},
    response::Json,
    routing::get,
    Router,
};
use serde::Serialize;
use std::{collections::HashMap, fs, net::SocketAddr, path::PathBuf, sync::Arc};
use tokio::sync::RwLock;

#[derive(Serialize)]
struct Verse {
    text: String,
}

// Translation -> Book -> Chapter -> Verse -> Text
type BibleData = HashMap<String, HashMap<String, HashMap<String, String>>>;
type BibleStore = Arc<RwLock<HashMap<String, BibleData>>>;

#[tokio::main]
async fn main() {
    // Directory containing all translations (JSON files like kjv.json, asv.json, etc.)
    let bible_dir = PathBuf::from("./bibles");

    // Load all translations into memory
    let mut translations: HashMap<String, BibleData> = HashMap::new();
    for entry in fs::read_dir(&bible_dir).expect("Failed to read ./bibles directory") {
        let entry = entry.expect("Bad entry");
        let path = entry.path();

        if path.extension().map(|e| e == "json").unwrap_or(false) {
            let filename = path.file_stem().unwrap().to_string_lossy().to_string();
            println!("📖 Loading translation: {}", filename);

            let file = fs::read_to_string(&path).expect("Failed to read file");
            let bible: BibleData = serde_json::from_str(&file).expect("Invalid JSON format");

            translations.insert(filename, bible);
        }
    }

    let store: BibleStore = Arc::new(RwLock::new(translations));

    // Define routes
    let app = Router::new()
        .route("/v1/:translation/:book/:chapter/:verse", get(get_verse))
        .with_state(store);

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("🚀 Bible API running on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Handler: /v1/{translation}/{book}/{chapter}/{verse}
async fn get_verse(
    Path((translation, book, chapter, verse)): Path<(String, String, String, String)>,
    State(state): State<BibleStore>,
) -> Json<Verse> {
    let store = state.read().await;
    let text = store
        .get(&translation)
        .and_then(|books| books.get(&book))
        .and_then(|chapters| chapters.get(&chapter))
        .and_then(|verses| verses.get(&verse))
        .cloned()
        .unwrap_or_else(|| "Verse not found".to_string());

    Json(Verse { text })
}

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
// Language -> Translation -> BibleData
type BibleStoreData = HashMap<String, HashMap<String, BibleData>>;
type BibleStore = Arc<RwLock<BibleStoreData>>;

#[tokio::main]
async fn main() {
    let bible_dir = PathBuf::from("./bibles");

    let mut translations: BibleStoreData = HashMap::new();

    for entry in fs::read_dir(&bible_dir).expect("Failed to read ./bibles directory") {
        let entry = entry.expect("Bad entry");
        let path = entry.path();

        if path.extension().map(|e| e == "json").unwrap_or(false) {
            let filename = path.file_stem().unwrap().to_string_lossy().to_string();
            // Expect format: lang_translation (e.g., en_kjv, es_rvr)
            let parts: Vec<&str> = filename.splitn(2, '_').collect();
            if parts.len() != 2 {
                eprintln!("⚠️ Skipping {}, expected format lang_translation.json", filename);
                continue;
            }
            let lang = parts[0].to_string();
            let translation = parts[1].to_string();

            println!("📖 Loading {} ({})", translation, lang);

            let file = fs::read_to_string(&path).expect("Failed to read file");
            let bible: BibleData = serde_json::from_str(&file).expect("Invalid JSON format");

            translations
                .entry(lang)
                .or_default()
                .insert(translation, bible);
        }
    }

    let store: BibleStore = Arc::new(RwLock::new(translations));

    let app = Router::new()
        .route("/v1/:lang/:translation/:book/:chapter/:verse", get(get_verse))
        .route("/v1/translations", get(list_translations))
        .with_state(store);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("🚀 Bible API running on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_verse(
    Path((lang, translation, book, chapter, verse)): Path<(String, String, String, String, String)>,
    State(state): State<BibleStore>,
) -> Json<Verse> {
    let store = state.read().await;
    let text = store
        .get(&lang)
        .and_then(|t| t.get(&translation))
        .and_then(|books| books.get(&book))
        .and_then(|chapters| chapters.get(&chapter))
        .and_then(|verses| verses.get(&verse))
        .cloned()
        .unwrap_or_else(|| "Verse not found".to_string());

    Json(Verse { text })
}

#[derive(Serialize)]
struct TranslationList {
    languages: HashMap<String, Vec<String>>,
}

async fn list_translations(State(state): State<BibleStore>) -> Json<TranslationList> {
    let store = state.read().await;
    let mut langs: HashMap<String, Vec<String>> = HashMap::new();

    for (lang, trans) in store.iter() {
        langs.insert(lang.clone(), trans.keys().cloned().collect());
    }

    Json(TranslationList { languages: langs })
}

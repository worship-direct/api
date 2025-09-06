use axum::{
    extract::{Path, State},
    response::Json,
    routing::get,
    Router,
};
use serde::Serialize;
use std::{
    collections::HashMap,
    net::SocketAddr,
    path::PathBuf,
    sync::Arc,
    time::Duration,
};
use tokio::{fs, task, time};
use tokio::sync::RwLock;
use utoipa::ToSchema;
use utoipa_swagger_ui::SwaggerUi;

#[derive(Serialize, ToSchema, Clone)]
struct Verse {
    number: String,
    text: String,
}

#[derive(Serialize, ToSchema)]
struct VerseRange {
    verses: Vec<Verse>,
}

#[derive(Serialize, ToSchema)]
struct TranslationList {
    languages: HashMap<String, Vec<String>>,
}

type BibleData = HashMap<String, HashMap<String, HashMap<String, String>>>;
type BibleStore = Arc<RwLock<HashMap<String, HashMap<String, BibleData>>>>;

async fn get_verse_range(
    Path((lang, translation, book, chapter, verse)): Path<(String, String, String, String, String)>,
    State(state): State<BibleStore>,
) -> Json<VerseRange> {
    let store = state.read().await;
    let verses_map = store
        .get(&lang)
        .and_then(|t| t.get(&translation))
        .and_then(|books| books.get(&book))
        .and_then(|chapters| chapters.get(&chapter));

    let mut verses = Vec::new();

    if let Some(verses_map) = verses_map {
        if verse.contains('-') {
            let parts: Vec<&str> = verse.split('-').collect();
            if parts.len() == 2 {
                let start: usize = parts[0].parse().unwrap_or(0);
                let end: usize = parts[1].parse().unwrap_or(0);
                for i in start..=end {
                    if let Some(text) = verses_map.get(&i.to_string()) {
                        verses.push(Verse { number: i.to_string(), text: text.clone() });
                    }
                }
            }
        } else if let Some(text) = verses_map.get(&verse) {
            verses.push(Verse { number: verse.clone(), text: text.clone() });
        }
    }

    Json(VerseRange { verses })
}

async fn list_translations(State(state): State<BibleStore>) -> Json<TranslationList> {
    let store = state.read().await;
    let mut langs: HashMap<String, Vec<String>> = HashMap::new();
    for (lang, translations) in store.iter() {
        langs.insert(lang.clone(), translations.keys().cloned().collect());
    }
    Json(TranslationList { languages: langs })
}

async fn load_bibles(store: &BibleStore) {
    let bible_dir = PathBuf::from("./bibles");
    let mut new_store: HashMap<String, HashMap<String, BibleData>> = HashMap::new();

    if let Ok(lang_dirs) = fs::read_dir(&bible_dir).await {
        tokio::pin!(lang_dirs);
        while let Some(Ok(lang_entry)) = lang_dirs.next_entry().await {
            let lang_path = lang_entry.path();
            if lang_path.is_dir() {
                if let Some(lang_os) = lang_path.file_name() {
                    let lang = lang_os.to_string_lossy().to_string();
                    if let Ok(trans_files) = fs::read_dir(&lang_path).await {
                        tokio::pin!(trans_files);
                        while let Some(Ok(file_entry)) = trans_files.next_entry().await {
                            let path = file_entry.path();
                            if path.extension().map(|e| e == "json").unwrap_or(false) {
                                if let Some(trans_os) = path.file_stem() {
                                    let translation = trans_os.to_string_lossy().to_string();
                                    match fs::read_to_string(&path).await {
                                        Ok(file) => {
                                            if let Ok(bible) = serde_json::from_str::<BibleData>(&file) {
                                                new_store.entry(lang.clone())
                                                    .or_default()
                                                    .insert(translation, bible);
                                                println!("Loaded {} ({})", translation, lang);
                                            } else {
                                                eprintln!("Failed to parse JSON: {:?}", path);
                                            }
                                        }
                                        Err(err) => eprintln!("Failed to read {:?}: {}", path, err),
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let mut store_lock = store.write().await;
    *store_lock = new_store;
}

async fn watch_bibles(store: BibleStore) {
    loop {
        load_bibles(&store).await;
        time::sleep(Duration::from_secs(30)).await;
    }
}

#[tokio::main]
async fn main() {
    let store: BibleStore = Arc::new(RwLock::new(HashMap::new()));
    load_bibles(&store).await;

    let store_clone = store.clone();
    task::spawn(async move { watch_bibles(store_clone).await });

    let openapi = utoipa::openapi!();
    let app = Router::new()
        .route("/:lang/:translation/:book/:chapter/:verse", get(get_verse_range))
        .route("/translations", get(list_translations))
        .merge(SwaggerUi::new("/docs").url("/api-doc/openapi.json", openapi))
        .with_state(store);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("🚀 Bible API running at http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

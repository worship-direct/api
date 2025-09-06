Bible API

This project provides a REST API for Bible verses with multiple translations and languages. It supports:

- Single verse queries (e.g., /en/kjv/Genesis/1/1)
- Verse ranges (e.g., /en/kjv/Genesis/1/1-3)
- Multiple languages and translations (JSON files in bibles/{lang}/{translation}.json)
- Hot-reloading of Bible JSON modules every 30 seconds
- Verse numbers included in JSON for client display
- Static Swagger UI docs (docs/index.html and docs/openapi.json)

Project Structure

bible-api/
 ├── Cargo.toml
 ├── src/main.rs
 ├── bibles/         # Bible JSON modules
 │    ├── en/kjv.json
 │    ├── en/asv.json
 │    └── es/rvr.json
 ├── docs/           # Static API documentation
 │    ├── index.html
 │    └── openapi.json
 └── README.txt

Usage

1. Run API server:

cargo run

2. Access endpoints:

- Get a verse:
GET /en/kjv/Genesis/1/1

- Get verse range:
GET /en/kjv/Genesis/1/1-3

- List available translations:
GET /translations

- Swagger UI:
GET /docs/

Hot-reload

- Add or update Bible JSON files in ./bibles/{lang}/.
- The API will automatically load changes within 30 seconds.
- Swagger docs are static; regenerate openapi.json and index.html to update docs in GitHub Pages.

Static Docs Deployment

- Push docs/ folder to GitHub Pages or any static web server.
- The API can continue running independently with hot-reloads, ensuring no downtime.

Open-Source Contribution

- Add new translations by adding JSON files under the appropriate bibles/{lang}/ folder.
- Update openapi.json to reflect new endpoints if necessary.
- Pull requests can include both new Bible JSON and updated docs.

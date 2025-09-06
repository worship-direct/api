Bible API - Fully Automated Setup

This project provides a REST API for Bible verses with support for multiple languages and translations.
It automatically serves any JSON Bible modules you drop into the `bibles/{lang}/{translation}.json` folder.
It also generates static Swagger documentation automatically.

Features:

- Multi-language and multi-translation support
- Hot-reload Bible JSON modules every 30 seconds without downtime
- Single verse and verse range support
- Verse numbers included in JSON responses
- Automatic generation of static Swagger docs (docs/index.html + docs/openapi.json)
- Ready for GitHub Pages deployment for documentation

Folder Structure:

bible-api/
 ├── Cargo.toml
 ├── src/main.rs
 ├── bibles/                    # Bible JSON modules (add your files here)
 │    └── {lang}/{translation}.json
 ├── docs/                      # Automatically generated static API docs
 │    ├── index.html
 │    └── openapi.json
 └── README.txt                 # You can rename this to README.md

Usage:

1. Clone the repository.

2. Add your Bible JSON files under the appropriate folder structure:
   bibles/en/kjv.json
   bibles/en/asv.json
   bibles/es/rvr.json
   etc.

3. Run the API server:
   cargo run

4. Access endpoints:

   - Single verse:
     GET /en/kjv/Genesis/1/1

   - Verse range:
     GET /en/kjv/Genesis/1/1-3

   - List available translations:
     GET /translations

   - Swagger UI documentation:
     GET /docs/

Hot-Reload:

- The API automatically loads any new or updated JSON files in `./bibles/` every 30 seconds.
- No downtime is required for updates.
- Swagger docs are generated automatically at first run; if you add new endpoints, regenerate openapi.json by restarting the server.

Static Docs Deployment:

- The `docs/` folder is fully static and can be deployed to GitHub Pages or any static web server.
- API endpoints continue running independently while docs are hosted separately.
- Push `docs/` to GitHub Pages to provide documentation for users without impacting API availability.

Open-Source Contribution:

- Add new translations by creating JSON files under `bibles/{lang}/`.
- JSON structure should follow the format:
  {
    "Book": {
      "ChapterNumber": {
        "VerseNumber": "Text of verse",
        ...
      },
      ...
    },
    ...
  }
- Pull requests can include both new Bible JSON files and updated documentation.

Notes:

- No sample Bible files are required to run the API; it will automatically serve whatever JSON files exist in the `bibles/` folder.
- Verse numbers are included in the JSON response to allow clients to display them alongside verse text.
- Static Swagger documentation (`/docs/`) is generated automatically and can be served independently of the API.


# Bible API Documentation

Welcome to the Bible API documentation! This directory contains comprehensive guides to help you get started and make the most of the API.

## 📚 Available Documentation

### Getting Started

- **[Quick Start Guide](QUICKSTART.md)** - Get up and running in 5 minutes with ready-to-use examples

### Tutorials

- **[Bible API Tutorial - English Versions](TUTORIAL_BIBLE_EN.md)** - Complete, in-depth guide covering:
  - All API endpoints
  - Code examples in JavaScript, Python, and cURL
  - Common use cases and patterns
  - Best practices and troubleshooting

### API Reference

- **[Swagger UI](index.html)** - Interactive API documentation (available when server is running at `/docs`)

## 🚀 Quick Links

**For Beginners:**
Start with the [Quick Start Guide](QUICKSTART.md) to see the API in action immediately.

**For Developers:**
Read the [Full Tutorial](TUTORIAL_BIBLE_EN.md) for comprehensive code examples and integration patterns.

**For API Reference:**
Run the server and visit `http://localhost:8080/docs` for interactive Swagger documentation.

## 📖 What's Covered

- **English Bible Translations:** KJV (King James Version) and ASV (American Standard Version)
- **Single Verse Retrieval:** Get any verse by book, chapter, and verse number
- **Verse Range Retrieval:** Get multiple consecutive verses in one request
- **Translation Listing:** Discover all available Bible translations

## 🔧 Example Requests

```bash
# Get John 3:16 from KJV
curl http://localhost:8080/en/kjv/John/3/16

# Get Genesis 1:1-3 from ASV
curl http://localhost:8080/en/asv/Genesis/1/1-3

# List all available translations
curl http://localhost:8080/translations
```

## 💡 Need Help?

- Check the [Quick Start Guide](QUICKSTART.md) for common examples
- Read the [Full Tutorial](TUTORIAL_BIBLE_EN.md) for detailed explanations
- Visit the main [README](../README.md) for setup instructions

---

*Last updated: January 2026*

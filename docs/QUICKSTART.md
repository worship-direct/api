# Bible API Quick Start Guide

Get started with the Bible API in under 5 minutes!

## Installation & Running

```bash
# Clone the repository
git clone https://github.com/worship-direct/api.git
cd api

# Run the API server
cargo run

# The API will be available at http://localhost:8080
```

## Quick Examples

### Get a Single Verse
```bash
# John 3:16 in King James Version
curl http://localhost:8080/en/kjv/John/3/16
```

### Get Multiple Verses
```bash
# Genesis 1:1-3 in American Standard Version
curl http://localhost:8080/en/asv/Genesis/1/1-3
```

### List Available Translations
```bash
curl http://localhost:8080/translations
```

## Available English Translations

- **kjv** - King James Version
- **asv** - American Standard Version

## Response Format

```json
{
  "verses": [
    {
      "number": "1",
      "text": "In the beginning God created the heaven and the earth."
    }
  ]
}
```

## URL Pattern

```
GET /{language}/{translation}/{book}/{chapter}/{verse}
```

**Examples:**
- `/en/kjv/Psalms/23/1` - Single verse
- `/en/kjv/Romans/12/1-2` - Verse range

## Popular Verses to Try

```bash
# The Lord's Prayer
curl http://localhost:8080/en/kjv/Matthew/6/9-13

# Psalm 23
curl http://localhost:8080/en/kjv/Psalms/23/1-6

# The Ten Commandments (first few)
curl http://localhost:8080/en/kjv/Exodus/20/1-5

# The Great Commission
curl http://localhost:8080/en/kjv/Matthew/28/18-20

# The Beatitudes
curl http://localhost:8080/en/kjv/Matthew/5/3-12
```

## Next Steps

📖 **[Read the Full Tutorial](TUTORIAL_BIBLE_EN.md)** for detailed code examples in JavaScript, Python, and more!

🔧 **[View Swagger Documentation](http://localhost:8080/docs)** (when server is running)

## Troubleshooting

**Empty response?**
- Check book name spelling (case-sensitive: `Genesis`, not `genesis`)
- Verify chapter and verse numbers exist

**Server won't start?**
- Ensure Rust/Cargo is installed: `cargo --version`
- Check if port 8080 is available

**Need help?**
- See the [full tutorial](TUTORIAL_BIBLE_EN.md) for more examples
- Check the repository README for contribution guidelines

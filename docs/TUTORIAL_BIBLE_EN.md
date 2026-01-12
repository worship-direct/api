# Tutorial: Using the Bible API - English Versions

Welcome to the Bible API tutorial! This guide will walk you through using the API to access English Bible translations, including the King James Version (KJV) and American Standard Version (ASV).

## Table of Contents
- [Getting Started](#getting-started)
- [Available English Translations](#available-english-translations)
- [API Endpoints](#api-endpoints)
- [Example Usage](#example-usage)
- [Code Examples](#code-examples)
- [Response Format](#response-format)
- [Common Use Cases](#common-use-cases)

## Getting Started

The Bible API provides RESTful access to Bible verses across multiple translations. All endpoints return JSON responses and require no authentication.

**Base URL**: `http://localhost:8080` (when running locally)

## Available English Translations

Currently available English Bible translations:

- **KJV** - King James Version
- **ASV** - American Standard Version

To see all available translations across all languages, use the `/translations` endpoint.

## API Endpoints

### 1. List All Translations

Get a list of all available Bible translations grouped by language.

**Endpoint**: `GET /translations`

**Example Request**:
```bash
curl http://localhost:8080/translations
```

**Example Response**:
```json
{
  "languages": {
    "en": ["kjv", "asv"]
  }
}
```

### 2. Get a Single Verse

Retrieve a specific verse from a Bible translation.

**Endpoint**: `GET /{lang}/{translation}/{book}/{chapter}/{verse}`

**Parameters**:
- `lang` - Language code (e.g., `en` for English)
- `translation` - Translation abbreviation (e.g., `kjv`, `asv`)
- `book` - Book name (e.g., `Genesis`, `John`, `Psalms`)
- `chapter` - Chapter number
- `verse` - Verse number

**Example Request**:
```bash
curl http://localhost:8080/en/kjv/John/3/16
```

**Example Response**:
```json
{
  "verses": [
    {
      "number": "16",
      "text": "For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life."
    }
  ]
}
```

### 3. Get a Range of Verses

Retrieve multiple consecutive verses using the range syntax `start-end`.

**Endpoint**: `GET /{lang}/{translation}/{book}/{chapter}/{start}-{end}`

**Example Request**:
```bash
curl http://localhost:8080/en/kjv/Genesis/1/1-3
```

**Example Response**:
```json
{
  "verses": [
    {
      "number": "1",
      "text": "In the beginning God created the heaven and the earth."
    },
    {
      "number": "2",
      "text": "And the earth was without form, and void; and darkness [was] upon the face of the deep. And the Spirit of God moved upon the face of the waters."
    },
    {
      "number": "3",
      "text": "And God said, Let there be light: and there was light."
    }
  ]
}
```

## Example Usage

### Example 1: Getting a Famous Verse (John 3:16)

**KJV Version**:
```bash
curl http://localhost:8080/en/kjv/John/3/16
```

**ASV Version**:
```bash
curl http://localhost:8080/en/asv/John/3/16
```

### Example 2: Reading the Creation Story (Genesis 1:1-5)

```bash
curl http://localhost:8080/en/kjv/Genesis/1/1-5
```

### Example 3: Getting Psalm 23 (verses 1-6)

```bash
curl http://localhost:8080/en/kjv/Psalms/23/1-6
```

### Example 4: Comparing Translations

**KJV**:
```bash
curl http://localhost:8080/en/kjv/Matthew/5/3
```

**ASV**:
```bash
curl http://localhost:8080/en/asv/Matthew/5/3
```

## Code Examples

### JavaScript (Fetch API)

```javascript
// Get a single verse
async function getVerse(translation, book, chapter, verse) {
  const url = `http://localhost:8080/en/${translation}/${book}/${chapter}/${verse}`;
  const response = await fetch(url);
  const data = await response.json();
  return data.verses;
}

// Example usage
getVerse('kjv', 'John', '3', '16')
  .then(verses => {
    verses.forEach(verse => {
      console.log(`Verse ${verse.number}: ${verse.text}`);
    });
  });

// Get a range of verses
async function getVerseRange(translation, book, chapter, startVerse, endVerse) {
  const url = `http://localhost:8080/en/${translation}/${book}/${chapter}/${startVerse}-${endVerse}`;
  const response = await fetch(url);
  const data = await response.json();
  return data.verses;
}

// Example usage
getVerseRange('kjv', 'Genesis', '1', '1', '3')
  .then(verses => {
    verses.forEach(verse => {
      console.log(`${verse.number}. ${verse.text}`);
    });
  });
```

### JavaScript (Node.js with axios)

```javascript
const axios = require('axios');

// Get a single verse
async function getVerse(translation, book, chapter, verse) {
  try {
    const response = await axios.get(
      `http://localhost:8080/en/${translation}/${book}/${chapter}/${verse}`
    );
    return response.data.verses;
  } catch (error) {
    console.error('Error fetching verse:', error);
  }
}

// List all translations
async function getTranslations() {
  try {
    const response = await axios.get('http://localhost:8080/translations');
    return response.data.languages;
  } catch (error) {
    console.error('Error fetching translations:', error);
  }
}

// Example usage
(async () => {
  const verses = await getVerse('kjv', 'Psalms', '23', '1');
  console.log(verses[0].text);
  
  const translations = await getTranslations();
  console.log('English translations:', translations.en);
})();
```

### Python (requests library)

```python
import requests

def get_verse(translation, book, chapter, verse):
    """Get a single verse from the Bible API"""
    url = f"http://localhost:8080/en/{translation}/{book}/{chapter}/{verse}"
    response = requests.get(url)
    return response.json()['verses']

def get_verse_range(translation, book, chapter, start_verse, end_verse):
    """Get a range of verses from the Bible API"""
    url = f"http://localhost:8080/en/{translation}/{book}/{chapter}/{start_verse}-{end_verse}"
    response = requests.get(url)
    return response.json()['verses']

def get_translations():
    """Get all available translations"""
    url = "http://localhost:8080/translations"
    response = requests.get(url)
    return response.json()['languages']

# Example usage
if __name__ == "__main__":
    # Get John 3:16 in KJV
    verses = get_verse('kjv', 'John', '3', '16')
    for verse in verses:
        print(f"Verse {verse['number']}: {verse['text']}")
    
    # Get Genesis 1:1-3 in ASV
    verses = get_verse_range('asv', 'Genesis', '1', '1', '3')
    for verse in verses:
        print(f"{verse['number']}. {verse['text']}")
    
    # List all English translations
    translations = get_translations()
    print(f"Available English translations: {translations['en']}")
```

### Python (urllib - no dependencies)

```python
import urllib.request
import json

def get_verse(translation, book, chapter, verse):
    """Get a single verse using urllib (no external dependencies)"""
    url = f"http://localhost:8080/en/{translation}/{book}/{chapter}/{verse}"
    with urllib.request.urlopen(url) as response:
        data = json.loads(response.read())
        return data['verses']

# Example usage
verses = get_verse('kjv', 'Psalms', '119', '105')
print(verses[0]['text'])
```

### cURL Examples

```bash
# Get a single verse
curl http://localhost:8080/en/kjv/Proverbs/3/5

# Get a range of verses
curl http://localhost:8080/en/kjv/Romans/12/1-2

# Get all translations (pretty-printed)
curl http://localhost:8080/translations | python -m json.tool

# Save response to a file
curl http://localhost:8080/en/kjv/Psalms/23/1-6 -o psalm23.json
```

## Response Format

All verse endpoints return a consistent JSON structure:

```json
{
  "verses": [
    {
      "number": "verse_number",
      "text": "verse_text"
    }
  ]
}
```

- **verses**: Array of verse objects
- **number**: String containing the verse number
- **text**: String containing the verse text

### Empty Response

If a verse or book is not found, the API returns an empty verses array:

```json
{
  "verses": []
}
```

## Common Use Cases

### 1. Daily Verse Application

```javascript
// Display a random verse from Proverbs (simplified example)
async function getDailyVerse() {
  const chapter = Math.floor(Math.random() * 31) + 1; // Proverbs has 31 chapters
  const verse = Math.floor(Math.random() * 35) + 1; // Most chapters have at least 35 verses
  
  const response = await fetch(
    `http://localhost:8080/en/kjv/Proverbs/${chapter}/${verse}`
  );
  const data = await response.json();
  
  if (data.verses.length > 0) {
    return {
      reference: `Proverbs ${chapter}:${verse}`,
      text: data.verses[0].text
    };
  }
  // If verse doesn't exist, fall back to verse 1 of the chapter
  const fallbackResponse = await fetch(
    `http://localhost:8080/en/kjv/Proverbs/${chapter}/1`
  );
  const fallbackData = await fallbackResponse.json();
  return {
    reference: `Proverbs ${chapter}:1`,
    text: fallbackData.verses[0].text
  };
}
```

### 2. Scripture Memory Tool

```python
def quiz_verse(translation, book, chapter, verse):
    """Get a verse and hide it for memory practice"""
    verses = get_verse(translation, book, chapter, verse)
    if verses:
        reference = f"{book} {chapter}:{verse}"
        text = verses[0]['text']
        
        # Show reference, wait for user input, then reveal
        print(f"Reference: {reference}")
        input("Press Enter to reveal the verse...")
        print(f"\n{text}")
```

### 3. Bible Reading Plan

```python
def get_reading_plan_day(day_number):
    """Get entire chapter for a specific day reading plan
    
    Example: Read one chapter of Genesis per day (Genesis has 50 chapters)
    This fetches the full chapter by requesting a large verse range.
    The API will return only verses that exist.
    """
    chapter = day_number
    # Request verses 1-999 to get the entire chapter (API returns only existing verses)
    verses = get_verse_range('kjv', 'Genesis', str(chapter), '1', '999')
    
    print(f"Day {day_number} - Genesis {chapter}")
    print(f"Total verses: {len(verses)}\n")
    for verse in verses:
        print(f"{verse['number']}. {verse['text']}\n")
```

### 4. Translation Comparison

```python
def compare_translations(book, chapter, verse):
    """Compare the same verse across different English translations"""
    translations = ['kjv', 'asv']
    
    print(f"\n{book} {chapter}:{verse} - Translation Comparison\n")
    print("-" * 70)
    
    for trans in translations:
        verses = get_verse(trans, book, chapter, verse)
        if verses:
            print(f"\n{trans.upper()}:")
            print(verses[0]['text'])
    
    print("-" * 70)

# Example usage
compare_translations('John', '3', '16')
```

## Tips and Best Practices

1. **Book Names**: Use proper capitalization (e.g., `Genesis`, not `genesis`)
2. **Special Characters**: URL-encode book names with spaces or special characters
3. **Error Handling**: Always check if `verses` array is not empty before accessing
4. **Caching**: Consider caching frequently accessed verses to reduce API calls
5. **Rate Limiting**: While there's no enforced rate limit, be respectful with request volume

## Troubleshooting

### Common Issues

**Issue**: Empty verses array returned
- **Solution**: Check spelling of book name (must match exactly)
- **Solution**: Verify chapter and verse numbers exist in that book

**Issue**: Server not responding
- **Solution**: Ensure the API server is running on port 8080
- **Solution**: Check that bibles folder contains the JSON translation files

## Next Steps

- Explore the Swagger documentation at `http://localhost:8080/docs`
- Check available translations using the `/translations` endpoint
- Build your own Bible study application using these endpoints
- Contribute additional translations to the repository

## Support and Contributing

For issues, feature requests, or contributions, visit the GitHub repository.

---

*This tutorial covers English Bible translations. The API supports multiple languages - check the `/translations` endpoint for all available options.*

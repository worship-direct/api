import os
import json
import xml.etree.ElementTree as ET

# Configure directories
SWORD_DIR = "./modules"   # folder where downloaded Sword modules are
OUTPUT_DIR = "./bibles"   # folder where JSONs will be saved

os.makedirs(OUTPUT_DIR, exist_ok=True)

# Mapping Sword OSIS book codes to readable names
BOOK_MAP = {
    "Gen": "Genesis",
    "Exod": "Exodus",
    "Lev": "Leviticus",
    "Num": "Numbers",
    "Deut": "Deuteronomy",
    # Add remaining books...
}

def parse_osis_file(filepath):
    """Parse OSIS XML and return nested dict: Book -> Chapter -> Verse -> Text"""
    tree = ET.parse(filepath)
    root = tree.getroot()

    bible = {}
    for verse in root.iter('{http://www.bibletechnologies.net/2003/OSIS/namespace}verse'):
        osis_id = verse.attrib.get("osisID")
        if not osis_id:
            continue

        # Split osisID into Book.Chapter.Verse
        parts = osis_id.split(".")
        if len(parts) != 3:
            continue
        book_code, chapter, verse_num = parts
        book_name = BOOK_MAP.get(book_code, book_code)

        bible.setdefault(book_name, {}).setdefault(chapter, {})[verse_num] = "".join(verse.itertext()).strip()

    return bible

def convert_modules():
    for filename in os.listdir(SWORD_DIR):
        if filename.endswith(".xml"):
            path = os.path.join(SWORD_DIR, filename)
            
            # Extract language and translation from filename, e.g., en_kjv.xml
            base = os.path.splitext(filename)[0]
            lang, translation = base.split("_")

            bible_data = parse_osis_file(path)

            out_file = os.path.join(OUTPUT_DIR, f"{lang}_{translation}.json")
            with open(out_file, "w", encoding="utf-8") as f:
                json.dump(bible_data, f, ensure_ascii=False, indent=2)

            print(f"✅ Converted {filename} -> {out_file}")

if __name__ == "__main__":
    convert_modules()

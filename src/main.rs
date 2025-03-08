use wasm_bindgen::prelude::*;
use std::collections::HashMap;

#[wasm_bindgen]
pub fn transliterate(text: &str, script: &str) -> Result<String, String> {
    match script {
        "devanagari" => transliterate_devanagari(text),
        // Add other scripts like "bengali", "tamil", etc. here.
        _ => Err(format!("Unsupported script: {}", script)),
    }
}

fn transliterate_devanagari(text: &str) -> Result<String, String> {
    let devanagari_to_roman: HashMap<char, &str> = [
        ('अ', "a"), ('आ', "ā"), ('इ', "i"), ('ई', "ī"), ('उ', "u"), ('ऊ', "ū"),
        ('ऋ', "ṛ"), ('ॠ', "ṝ"), ('ऌ', "ḷ"), ('ॡ', "ḹ"), ('ए', "e"), ('ऐ', "ai"),
        ('ओ', "o"), ('औ', "au"), ('अं', "aṃ"), ('अः', "aḥ"), ('क', "ka"), ('ख', "kha"),
        ('ग', "ga"), ('घ', "gha"), ('ङ', "ṅa"), ('च', "ca"), ('छ', "cha"), ('ज', "ja"),
        ('झ', "jha"), ('ञ', "ña"), ('ट', "ṭa"), ('ठ', "ṭha"), ('ड', "ḍa"), ('ढ', "ḍha"),
        ('ण', "ṇa"), ('त', "ta"), ('थ', "tha"), ('द', "da"), ('ध', "dha"), ('न', "na"),
        ('प', "pa"), ('फ', "pha"), ('ब', "ba"), ('भ', "bha"), ('म', "ma"), ('य', "ya"),
        ('र', "ra"), ('ल', "la"), ('व', "va"), ('श', "śa"), ('ष', "ṣa"), ('स', "sa"),
        ('ह', "ha"), ('ळ', "ḷa"), ('क्ष', "kṣa"), ('ज्ञ', "jña"), ('त्र', "tra"),
        ('ा', "ā"), ('ि', "i"), ('ी', "ī"), ('ु', "u"), ('ू', "ū"), ('ृ', "ṛ"),
        ('ॄ', "ṝ"), ('ॢ', "ḷ"), ('ॣ', "ḹ"), ('े', "e"), ('ै', "ai"), ('ो', "o"),
        ('ौ', "au"), ('ं', "ṃ"), ('ः', "ḥ"), ('्', ""), ('ॐ', "oṃ"), ('ँ', "̃"),
        ('़',""),('।',".") //Add more characters as needed
    ]
    .iter().cloned().collect();

    let mut result = String::new();
    for c in text.chars() {
        if let Some(roman) = devanagari_to_roman.get(&c) {
            result.push_str(roman);
        } else {
            result.push(c); // Keep the original character if not found.
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_devanagari_transliteration() {
        let text = "नमस्ते";
        let result = transliterate(text, "devanagari").unwrap();
        assert_eq!(result, "namaste");

        let text = "भारत";
        let result = transliterate(text, "devanagari").unwrap();
        assert_eq!(result, "bhārata");

        let text = "विद्यालय";
        let result = transliterate(text, "devanagari").unwrap();
        assert_eq!(result, "vidyālaya");

        let text = "ॐ";
        let result = transliterate(text, "devanagari").unwrap();
        assert_eq!(result, "oṃ");

        let text = "चंद्र";
        let result = transliterate(text, "devanagari").unwrap();
        assert_eq!(result, "caṃdra");
    }

    #[test]
    fn test_unsupported_script() {
        let text = "some text";
        let result = transliterate(text, "invalid");
        assert_eq!(result, Err("Unsupported script: invalid".to_string()));
    }
      }

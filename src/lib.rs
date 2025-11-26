//! Emoji Smuggler - Educational demonstration of Unicode variation selector data smuggling
//!
//! This library demonstrates how Unicode variation selectors can be used to hide arbitrary
//! data within what appears to be a single emoji character. This technique exploits how
//! AI systems tokenize text and can bypass security filters.
//!
//! **SECURITY EDUCATIONAL PURPOSE ONLY**
//! This code is for security education and defensive purposes.

use wasm_bindgen::prelude::*;

/// Convert a byte value to its corresponding Unicode variation selector
///
/// # Unicode Variation Selector Ranges:
/// - Bytes 0-15: Use range U+FE00 to U+FE0F (Variation Selectors 1-16)
/// - Bytes 16-255: Use range U+E0100 to U+E01EF (Variation Selectors 17-256)
///
/// This gives us 256 possible values, encoding one full byte per selector.
///
/// # Example
/// ```
/// # use emoji_smuggler::byte_to_variation_selector;
/// let selector = byte_to_variation_selector(0x68); // 'h' in ASCII (104)
/// assert_eq!(selector, '\u{E0158}'); // E0100 + (104-16) = E0158
/// ```
#[wasm_bindgen]
pub fn byte_to_variation_selector(byte: u8) -> char {
    if byte < 16 {
        // Variation Selectors 1-16: U+FE00 to U+FE0F
        char::from_u32(0xFE00 + byte as u32).unwrap()
    } else {
        // Variation Selectors 17-256: U+E0100 to U+E01EF
        // Subtract 16 since byte 16 should map to E0100, byte 255 to E01EF
        char::from_u32(0xE0100 + (byte as u32 - 16)).unwrap()
    }
}

/// Convert a variation selector character back to its byte value
///
/// Returns `Some(byte)` if the character is a valid variation selector,
/// `None` otherwise.
///
/// # Example
/// ```
/// # use emoji_smuggler::variation_selector_to_byte;
/// let byte = variation_selector_to_byte('\u{E0158}'); // 'h' encoded
/// assert_eq!(byte, Some(0x68));
/// ```
#[wasm_bindgen]
pub fn variation_selector_to_byte(c: char) -> Option<u8> {
    let code_point = c as u32;

    if (0xFE00..=0xFE0F).contains(&code_point) {
        // Variation Selectors 1-16
        Some((code_point - 0xFE00) as u8)
    } else if (0xE0100..=0xE01EF).contains(&code_point) {
        // Variation Selectors 17-256
        // Variation Selectors 17-256
        // Add 16 since E0100 maps to byte 16, E01EF to byte 255
        Some((code_point - 0xE0100 + 16) as u8)
    } else {
        None
    }
}

/// Encode arbitrary bytes as hidden variation selectors after a base character
///
/// The output appears as a single character (usually an emoji), but contains
/// an invisible payload of variation selectors.
///
/// # Arguments
/// * `base` - The base character (typically an emoji like '🧁')
/// * `bytes` - The data to encode as variation selectors
///
/// # Example
/// ```
/// # use emoji_smuggler::encode;
/// let hidden = encode('🧁', &[0x68, 0x65, 0x6c, 0x6c, 0x6f]); // "hello"
/// println!("{}", hidden); // Visually displays as: 🧁
/// println!("{:?}", hidden); // Debug view shows hidden selectors
/// ```
#[wasm_bindgen]
pub fn encode(base: char, bytes: &[u8]) -> String {
    let mut result = String::new();
    result.push(base);
    for byte in bytes {
        result.push(byte_to_variation_selector(*byte));
    }
    result
}

/// Decode hidden data from a string containing variation selectors
///
/// Extracts all variation selectors from the input and converts them back
/// to their original byte values. Ignores non-variation-selector characters.
///
/// # Example
/// ```
/// # use emoji_smuggler::{encode, decode};
/// let hidden = encode('🧁', b"hello");
/// let revealed = decode(&hidden);
/// assert_eq!(revealed, b"hello");
/// assert_eq!(std::str::from_utf8(&revealed).unwrap(), "hello");
/// ```
#[wasm_bindgen]
pub fn decode(input: &str) -> Vec<u8> {
    input
        .chars()
        .filter_map(variation_selector_to_byte)
        .collect()
}

/// Returns statistics about an encoded string for analysis
///
/// # Returns
/// A tuple of (char_count, variation_selector_count, byte_length)
#[wasm_bindgen]
pub fn analyze_encoded(input: &str) -> Vec<usize> {
    let char_count = input.chars().count();
    let vs_count = input.chars().filter(|c| variation_selector_to_byte(*c).is_some()).count();
    let byte_length = input.len();

    vec![char_count, vs_count, byte_length]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_byte_to_variation_selector_low_range() {
        // Test bytes 0-15 map to FE00-FE0F
        assert_eq!(byte_to_variation_selector(0), '\u{FE00}');
        assert_eq!(byte_to_variation_selector(15), '\u{FE0F}');
    }

    #[test]
    fn test_byte_to_variation_selector_high_range() {
        // Test bytes 16-255 map to E0100-E01EF
        assert_eq!(byte_to_variation_selector(16), '\u{E0100}');
        assert_eq!(byte_to_variation_selector(255), '\u{E01EF}');
    }

    #[test]
    fn test_variation_selector_roundtrip() {
        // Test that all 256 byte values round-trip correctly
        for byte in 0..=255u8 {
            let selector = byte_to_variation_selector(byte);
            let recovered = variation_selector_to_byte(selector);
            assert_eq!(recovered, Some(byte));
        }
    }

    #[test]
    fn test_encode_hello() {
        // Test encoding "hello" as shown in the presentation
        let encoded = encode('🧁', &[0x68, 0x65, 0x6c, 0x6c, 0x6f]);

        // Visual representation looks like just an emoji
        assert!(encoded.starts_with('🧁'));

        // But contains 6 characters (emoji + 5 variation selectors)
        assert_eq!(encoded.chars().count(), 6);
    }

    #[test]
    fn test_decode_hello() {
        let encoded = encode('🧁', b"hello");
        let decoded = decode(&encoded);
        assert_eq!(decoded, b"hello");
        assert_eq!(std::str::from_utf8(&decoded).unwrap(), "hello");
    }

    #[test]
    fn test_roundtrip_binary_data() {
        // Test with arbitrary binary data
        let original: Vec<u8> = (0..=255).collect();
        let encoded = encode('🔒', &original);
        let decoded = decode(&encoded);
        assert_eq!(decoded, original);
    }

    #[test]
    fn test_analyze_encoded() {
        let encoded = encode('😊', b"hello");
        let stats = analyze_encoded(&encoded);

        // 1 base char + 5 variation selectors = 6 chars
        assert_eq!(stats[0], 6);
        // 5 variation selectors
        assert_eq!(stats[1], 5);
        // Byte length will be larger due to multibyte UTF-8 encoding
        assert!(stats[2] > 6);
    }
}

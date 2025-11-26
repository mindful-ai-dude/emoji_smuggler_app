//! Demonstration Tests for Pluralsight Presentation
//!
//! These tests are designed to be run during the presentation to demonstrate
//! emoji smuggling techniques in real-time.
//!
//! Run with: cargo test --test demo -- --nocapture

use emoji_smuggler::{encode, decode, byte_to_variation_selector, analyze_encoded};

#[test]
fn demo_1_basic_encoding() {
    println!("\n╔════════════════════════════════════════════════════════════╗");
    println!("║  DEMO 1: Basic Encoding - The 'hello' Example             ║");
    println!("║  (Matches Slides 12-13 from presentation)                  ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    let message = "hello";
    let base_emoji = '🧁';

    println!("📝 Original message: \"{}\"", message);
    println!("🎭 Base emoji: {}", base_emoji);

    let bytes = message.as_bytes();
    println!("\n🔢 Message as hex bytes:");
    for (i, &byte) in bytes.iter().enumerate() {
        println!("   Byte {}: 0x{:02X} ('{}')", i, byte, byte as char);
    }

    let encoded = encode(base_emoji, bytes);

    println!("\n✨ Encoded output:");
    println!("   Visual representation: {}", encoded);
    println!("   (Looks like a single emoji, right?)\n");

    println!("🔍 Debug view (reveals hidden characters):");
    println!("   {:?}", encoded);

    let stats = analyze_encoded(&encoded);
    println!("\n📊 Analysis:");
    println!("   • Total characters: {}", stats[0]);
    println!("   • Hidden variation selectors: {}", stats[1]);
    println!("   • UTF-8 byte length: {}", stats[2]);

    println!("\n⚠️  Token Explosion:");
    println!("   • Normal 'hello': ~2 tokens");
    println!("   • This emoji: {} characters = MANY MORE TOKENS", stats[0]);
    println!("   • Security filters fooled! ✓\n");

    assert_eq!(encoded.chars().count(), 6); // 1 base + 5 selectors
    assert_eq!(stats[1], 5); // 5 variation selectors
}

#[test]
fn demo_2_variation_selector_mapping() {
    println!("\n╔════════════════════════════════════════════════════════════╗");
    println!("║  DEMO 2: Variation Selector Byte Mapping                  ║");
    println!("║  (Matches Slide 10 from presentation)                      ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    println!("🔢 Demonstrating byte to variation selector conversion:\n");

    // Show examples from both ranges
    println!("Range 1 (Bytes 0-15 → U+FE00 to U+FE0F):");
    for byte in [0u8, 5, 10, 15] {
        let selector = byte_to_variation_selector(byte);
        println!("   Byte {:3} (0x{:02X}) → U+{:04X} → {:?}", byte, byte, selector as u32, selector);
    }

    println!("\nRange 2 (Bytes 16-255 → U+E0100 to U+E01EF):");
    for byte in [16u8, 50, 104, 255] {
        let selector = byte_to_variation_selector(byte);
        println!("   Byte {:3} (0x{:02X}) → U+{:05X} → {:?}", byte, byte, selector as u32, selector);
    }

    println!("\n📐 Key Insight:");
    println!("   • 256 variation selectors = 256 byte values");
    println!("   • Complete byte encoding capability!");
    println!("   • Can smuggle any binary data\n");

    // Verify the ranges
    assert_eq!(byte_to_variation_selector(0), '\u{FE00}');
    assert_eq!(byte_to_variation_selector(15), '\u{FE0F}');
    assert_eq!(byte_to_variation_selector(16), '\u{E0100}');
    assert_eq!(byte_to_variation_selector(255), '\u{E01EF}');
}

#[test]
fn demo_3_decoding_process() {
    println!("\n╔════════════════════════════════════════════════════════════╗");
    println!("║  DEMO 3: Decoding Hidden Messages                         ║");
    println!("║  (Matches Slide 14 from presentation)                      ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    let message = "hello";
    let encoded = encode('🧁', message.as_bytes());

    println!("🔒 Encoded string:");
    println!("   Visual: {}", encoded);
    println!("   Debug:  {:?}\n", encoded);

    println!("🔓 Decoding process:");
    let decoded_bytes = decode(&encoded);

    println!("   Step 1: Extract variation selectors");
    println!("   Step 2: Convert each to byte value");
    println!("   Step 3: Reconstruct original data\n");

    println!("   Decoded bytes: {:02X?}", decoded_bytes);

    let decoded_message = std::str::from_utf8(&decoded_bytes).unwrap();
    println!("   Decoded message: \"{}\"\n", decoded_message);

    println!("✅ SUCCESS: Hidden message recovered!");
    println!("   Original: \"{}\"", message);
    println!("   Recovered: \"{}\"\n", decoded_message);

    assert_eq!(decoded_bytes, message.as_bytes());
    assert_eq!(decoded_message, message);
}

#[test]
fn demo_4_security_filter_bypass() {
    println!("\n╔════════════════════════════════════════════════════════════╗");
    println!("║  DEMO 4: Security Filter Bypass Simulation                ║");
    println!("║  Demonstrating how this bypasses AI safety filters        ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    // Simulate encoding a "malicious" payload
    let malicious_payload = "jailbreak";
    let innocent_looking = encode('🎨', malicious_payload.as_bytes());

    println!("🎭 What the security filter sees:");
    println!("   Input text: {}", innocent_looking);
    println!("   Visual length: 1 character");
    println!("   Assessment: SAFE ✓ (just an emoji)\n");

    println!("🤖 What the AI model receives:");
    let stats = analyze_encoded(&innocent_looking);
    println!("   Actual characters: {}", stats[0]);
    println!("   Hidden selectors: {}", stats[1]);
    println!("   Hidden payload: {:?}", std::str::from_utf8(&decode(&innocent_looking)).unwrap());
    println!("   Token count: EXPLODED 💥\n");

    println!("⚠️  SECURITY IMPLICATION:");
    println!("   ❌ Filter bypassed!");
    println!("   ❌ Malicious payload delivered to model!");
    println!("   ❌ AI safety system circumvented!\n");

    println!("🛡️  DEFENSE (Slide 15):");
    println!("   ✓ Implement variation selector filtering");
    println!("   ✓ Use Unicode normalization");
    println!("   ✓ Deploy multi-layer inspection");
    println!("   ✓ Anomaly detection for character count vs visual length\n");

    assert_eq!(innocent_looking.chars().count(), 10); // 1 + 9 chars
    assert_ne!(innocent_looking.len(), 1); // Many UTF-8 bytes
}

#[test]
fn demo_5_binary_data_smuggling() {
    println!("\n╔════════════════════════════════════════════════════════════╗");
    println!("║  DEMO 5: Binary Data Smuggling                            ║");
    println!("║  Proving any data can be hidden in an emoji               ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    // Create binary payload (simulated command bytes)
    let binary_payload = vec![0xDE, 0xAD, 0xBE, 0xEF, 0xCA, 0xFE, 0xBA, 0xBE];

    println!("💾 Original binary data:");
    println!("   Hex: {:02X?}", binary_payload);
    println!("   (Could be malware, shellcode, encrypted commands, etc.)\n");

    let smuggled = encode('📦', &binary_payload);

    println!("📦 Smuggled in emoji:");
    println!("   Visual: {}", smuggled);
    println!("   (Looks harmless, right?)\n");

    let recovered = decode(&smuggled);
    println!("🔓 Recovered binary data:");
    println!("   Hex: {:02X?}", recovered);
    println!("   Match: {}\n", if recovered == binary_payload { "✅ PERFECT" } else { "❌ FAILED" });

    println!("🚨 THREAT SCENARIOS:");
    println!("   • Malware C2 channel in social media posts");
    println!("   • Exfiltrate data through chat applications");
    println!("   • Bypass DLP systems with 'innocent' emojis");
    println!("   • Inject payloads into AI prompts\n");

    assert_eq!(recovered, binary_payload);
}

#[test]
fn demo_6_roundtrip_all_bytes() {
    println!("\n╔════════════════════════════════════════════════════════════╗");
    println!("║  DEMO 6: Complete Byte Range Verification                 ║");
    println!("║  Testing all 256 possible byte values                     ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    let all_bytes: Vec<u8> = (0..=255).collect();

    println!("🔬 Testing round-trip encoding for all 256 byte values...\n");

    let encoded = encode('🔐', &all_bytes);
    let decoded = decode(&encoded);

    println!("   Original bytes: {} values (0x00 to 0xFF)", all_bytes.len());
    println!("   Encoded to: {} characters", encoded.chars().count());
    println!("   Decoded to: {} bytes", decoded.len());

    let stats = analyze_encoded(&encoded);
    println!("\n📊 Statistics:");
    println!("   • Total characters: {}", stats[0]);
    println!("   • Variation selectors: {}", stats[1]);
    println!("   • UTF-8 byte size: {} bytes", stats[2]);
    println!("   • Expansion ratio: {}x", stats[2] / all_bytes.len());

    println!("\n✅ Verification:");
    let matches = all_bytes == decoded;
    println!("   All bytes recovered correctly: {}\n", if matches { "YES ✓" } else { "NO ✗" });

    assert_eq!(decoded, all_bytes);
    assert_eq!(stats[1], 256); // 256 variation selectors
}

#[test]
fn demo_7_visual_vs_actual_comparison() {
    println!("\n╔════════════════════════════════════════════════════════════╗");
    println!("║  DEMO 7: Visual Deception - The Core Attack               ║");
    println!("║  Comparing what humans see vs what computers process      ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    let messages = vec![
        ("hi", '👋'),
        ("secret", '🔒'),
        ("malware", '🦠'),
        ("payload", '🧁'),
    ];

    println!("👁️  HUMAN PERCEPTION vs 🤖 MACHINE PROCESSING:\n");

    for (msg, emoji) in messages {
        let encoded = encode(emoji, msg.as_bytes());
        let stats = analyze_encoded(&encoded);

        println!("─────────────────────────────────────────────────────");
        println!("Visual (what humans see):     {}", encoded);
        println!("Actual character count:       {}", stats[0]);
        println!("Hidden variation selectors:   {}", stats[1]);
        println!("Hidden message:               \"{}\"", msg);
        println!("UTF-8 bytes:                  {}", stats[2]);
        println!();
    }

    println!("🎯 KEY TAKEAWAY:");
    println!("   What you SEE ≠ What the computer PROCESSES");
    println!("   This is the foundation of the emoji attack!\n");
}

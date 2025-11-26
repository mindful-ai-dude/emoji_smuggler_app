//! Emoji Smuggler CLI - Interactive demonstration tool
//!
//! This binary provides an interactive command-line interface for demonstrating
//! emoji-based data smuggling techniques.

use emoji_smuggler::{encode, decode, analyze_encoded};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        return;
    }

    match args[1].as_str() {
        "encode" => {
            if args.len() < 4 {
                eprintln!("Usage: emoji-smuggler encode <base_emoji> <message>");
                eprintln!("Example: emoji-smuggler encode 🧁 \"hello\"");
                return;
            }
            let base = args[2].chars().next().unwrap_or('🧁');
            let message = args[3..].join(" ");
            run_encode(base, &message);
        }
        "decode" => {
            if args.len() < 3 {
                eprintln!("Usage: emoji-smuggler decode <encoded_string>");
                return;
            }
            let encoded = &args[2];
            run_decode(encoded);
        }
        "demo" => {
            run_full_demo();
        }
        "analyze" => {
            if args.len() < 3 {
                eprintln!("Usage: emoji-smuggler analyze <encoded_string>");
                return;
            }
            let encoded = &args[2];
            run_analyze(encoded);
        }
        "--help" | "-h" => {
            print_help();
        }
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            print_help();
        }
    }
}

fn print_help() {
    println!("Emoji Smuggler - Unicode Variation Selector Data Hiding");
    println!();
    println!("USAGE:");
    println!("    emoji-smuggler <COMMAND> [OPTIONS]");
    println!();
    println!("COMMANDS:");
    println!("    encode <emoji> <message>   Encode a message into an emoji");
    println!("    decode <encoded>           Decode a message from an emoji");
    println!("    analyze <encoded>          Analyze an encoded string");
    println!("    demo                       Run full demonstration");
    println!("    --help, -h                 Print this help message");
    println!();
    println!("EXAMPLES:");
    println!("    emoji-smuggler encode 🧁 \"hello\"");
    println!("    emoji-smuggler decode \"<encoded-output>\"");
    println!("    emoji-smuggler demo");
}

fn run_encode(base: char, message: &str) {
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║           EMOJI SMUGGLER - ENCODING DEMONSTRATION          ║");
    println!("╚════════════════════════════════════════════════════════════╝");
    println!();

    println!("📝 Original Message:");
    println!("   \"{}\"", message);
    println!();

    let bytes = message.as_bytes();
    println!("🔢 Message as Hex Bytes:");
    print!("   ");
    for (i, byte) in bytes.iter().enumerate() {
        print!("{:02X}", byte);
        if i < bytes.len() - 1 {
            print!(" ");
        }
    }
    println!();
    println!();

    let encoded = encode(base, bytes);

    println!("✨ Encoded Output (Visual):");
    println!("   {}", encoded);
    println!();

    println!("🔍 Encoded Output (Debug View):");
    println!("   {:?}", encoded);
    println!();

    let stats = analyze_encoded(&encoded);
    println!("📊 Analysis:");
    println!("   Characters: {}", stats[0]);
    println!("   Variation Selectors: {}", stats[1]);
    println!("   UTF-8 Bytes: {}", stats[2]);
    println!();

    println!("⚠️  WARNING: The visual output looks like a single emoji,");
    println!("   but contains {} hidden variation selectors!", stats[1]);
}

fn run_decode(encoded: &str) {
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║           EMOJI SMUGGLER - DECODING DEMONSTRATION          ║");
    println!("╚════════════════════════════════════════════════════════════╝");
    println!();

    println!("🔒 Encoded Input:");
    println!("   {}", encoded);
    println!();

    let bytes = decode(encoded);

    if bytes.is_empty() {
        println!("❌ No hidden data found!");
        return;
    }

    println!("🔓 Decoded Bytes (Hex):");
    print!("   ");
    for (i, byte) in bytes.iter().enumerate() {
        print!("{:02X}", byte);
        if i < bytes.len() - 1 {
            print!(" ");
        }
    }
    println!();
    println!();

    if let Ok(message) = std::str::from_utf8(&bytes) {
        println!("✅ Decoded Message:");
        println!("   \"{}\"", message);
    } else {
        println!("⚠️  Decoded data (not valid UTF-8):");
        println!("   {:?}", bytes);
    }
    println!();
}

fn run_analyze(encoded: &str) {
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║           EMOJI SMUGGLER - STRING ANALYSIS                 ║");
    println!("╚════════════════════════════════════════════════════════════╝");
    println!();

    println!("🔍 Input String:");
    println!("   Visual: {}", encoded);
    println!("   Debug:  {:?}", encoded);
    println!();

    let stats = analyze_encoded(encoded);
    let decoded = decode(encoded);

    println!("📊 Statistics:");
    println!("   Total Characters: {}", stats[0]);
    println!("   Variation Selectors: {}", stats[1]);
    println!("   UTF-8 Byte Length: {}", stats[2]);
    println!("   Hidden Data Size: {} bytes", decoded.len());
    println!();

    if stats[1] > 0 {
        println!("⚠️  ALERT: This string contains {} hidden variation selectors!", stats[1]);
        println!("   Potential data smuggling detected!");
    } else {
        println!("✅ No variation selectors detected - string is clean.");
    }
}

fn run_full_demo() {
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║      EMOJI SMUGGLING: FULL DEMONSTRATION                   ║");
    println!("║      (As seen in Pluralsight Author Audition)              ║");
    println!("╚════════════════════════════════════════════════════════════╝");
    println!();

    // Demo 1: The "hello" example from the presentation
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("DEMO 1: Encoding 'hello' (Slides 12-13)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();

    let message = "hello";
    let base = '🧁';
    println!("📝 Message: \"{}\"", message);
    println!("🎭 Base Emoji: {}", base);
    println!();

    let bytes = message.as_bytes();
    println!("🔢 Hex Bytes: {:02X} {:02X} {:02X} {:02X} {:02X}",
             bytes[0], bytes[1], bytes[2], bytes[3], bytes[4]);
    println!("   (h=0x68, e=0x65, l=0x6C, l=0x6C, o=0x6F)");
    println!();

    let encoded = encode(base, bytes);
    println!("✨ Encoded Result:");
    println!("   Visual:  {}", encoded);
    println!("   Debug:   {:?}", encoded);
    println!();

    let stats = analyze_encoded(&encoded);
    println!("📊 Token Explosion:");
    println!("   Normal text 'hello' would be: ~2 tokens");
    println!("   This emoji has: {} characters (1 base + {} selectors)", stats[0], stats[1]);
    println!("   AI tokenizer sees: MANY MORE TOKENS");
    println!();

    // Demo 2: Decoding
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("DEMO 2: Decoding (Slide 14)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();

    let decoded = decode(&encoded);
    println!("🔓 Extracted Bytes: {:02X?}", decoded);
    println!("✅ Decoded Message: \"{}\"", std::str::from_utf8(&decoded).unwrap());
    println!();

    // Demo 3: Security Implications
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("DEMO 3: Security Implications");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();

    println!("🚨 Attack Vector:");
    println!("   1. Attacker encodes malicious payload in emoji");
    println!("   2. Security filter sees: {} (looks innocent)", base);
    println!("   3. AI model receives: {} hidden bytes", stats[1]);
    println!("   4. Filter bypassed! Payload delivered to model.");
    println!();

    println!("🛡️  Defense Strategies (Slide 15):");
    println!("   ✓ Unicode normalization");
    println!("   ✓ Variation selector filtering");
    println!("   ✓ Multi-layered content inspection");
    println!("   ✓ Anomaly detection for suspicious patterns");
    println!();

    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║  Demo Complete! This demonstrates emoji hacking in action. ║");
    println!("╚════════════════════════════════════════════════════════════╝");
}

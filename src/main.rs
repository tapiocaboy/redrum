use clap::Parser;
use redrum::{
    hash_string,
    hash_individual_chars,
    calculate_frequency,
    analyze_combined_hashes,
    analyze_character_shifts,
};

const ASCII_ART: &str = r#"
.______       _______  _______     .______       __    __  .___  ___. 
|   _  \     |   ____||       \    |   _  \     |  |  |  | |   \/   | 
|  |_)  |    |  |__   |  .--.  |   |  |_)  |    |  |  |  | |  \  /  | 
|      /     |   __|  |  |  |  |   |      /     |  |  |  | |  |\/|  | 
|  |\  \----.|  |____ |  '--'  |   |  |\  \----.|  `--'  | |  |  |  | 
| _| `._____||_______||_______/    | _| `._____| \______/  |__|  |__| 
"#;

/// A program to analyze SHA-256 hashes of words
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The word to analyze (default: REDRUM)
    #[arg(short, long, default_value = "REDRUM")]
    word: String,
}

fn main() {
    println!("{}", ASCII_ART);
    
    let args = Args::parse();
    let word = args.word.to_uppercase();
    
    // 1. Hash the entire word
    println!("# Hash Analysis of \"{}\"\n", word);
    
    println!("## 1. Full SHA-256 Hash");
    let full_hash = hash_string(&word);
    println!("```\n{}\n```\n", full_hash);
    
    // 2. Hash individual characters
    println!("## 2. Individual Character Hashes");
    println!("| Character | SHA-256 Hash |");
    println!("|-----------|--------------|");
    for (c, hash) in hash_individual_chars(&word) {
        println!("| {} | {} |", c, hash);
    }
    println!();
    
    // 3. Frequency analysis
    println!("## 3. Frequency Analysis of Full Hash");
    println!("| Character | Frequency |");
    println!("|-----------|-----------|");
    let frequencies = calculate_frequency(&full_hash);
    for (c, count) in frequencies.iter() {
        println!("| {} | {} |", c, count);
    }
    println!();
    
    // 4. Combined Hash Analysis
    println!("## 4. Combined Hash Analysis");
    let (word_hash, _char_hashes, combined_hash) = analyze_combined_hashes(&word);
    
    println!("### Word Hash vs. Combined Hash");
    println!("Word Hash: `{}`", word_hash);
    println!("Combined Hash: `{}`", combined_hash);
    println!();
    
    // 5. Character Shifting Analysis
    println!("## 5. Character Shifting Analysis");
    println!("| Character | Alphabet Index | Word Hash Char | Combined Hash Char |");
    println!("|-----------|----------------|----------------|-------------------|");
    
    let shifts = analyze_character_shifts(&word, &word_hash, &combined_hash);
    for (c, index, word_char, combined_char) in shifts {
        println!("| {} | {} | {} | {} |", c, index, word_char, combined_char);
    }
}

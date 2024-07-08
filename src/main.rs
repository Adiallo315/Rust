use rand::Rng;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, Write};
use chrono::Local;
use anyhow::Result;

// Main function which handles user input and controls the program flow
fn main() -> Result<()> {
    println!("Welcome to the password management program!");
    println!("Choose an option:");
    println!("1. Generate passwords");
    println!("2. Crack a password");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Error reading input");
    let choice = choice.trim();

    match choice {
        "1" => {
            generate_passwords();
            Ok(())
        }
        "2" => crack_password_menu(),
        _ => {
            println!("Invalid option, please try again.");
            Ok(())
        }
    }
}

// Function to handle password generation based on user input
fn generate_passwords() {
    println!("How many passwords would you like to generate?");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error reading input");
    let num_passwords: usize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input, using default of 1 password.");
            1
        }
    };

    println!("Enter the number of characters for the passwords:");
    input.clear();
    io::stdin().read_line(&mut input).expect("Error reading input");
    let password_length: usize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input, using default length of 12 characters.");
            12
        }
    };

    println!("Include uppercase letters? (y/n)");
    let mut use_upper = String::new();
    io::stdin().read_line(&mut use_upper).expect("Error reading input");
    let use_upper = use_upper.trim().to_lowercase() == "y";

    println!("Include lowercase letters? (y/n)");
    let mut use_lower = String::new();
    io::stdin().read_line(&mut use_lower).expect("Error reading input");
    let use_lower = use_lower.trim().to_lowercase() == "y";

    println!("Include numbers? (y/n)");
    let mut use_numbers = String::new();
    io::stdin().read_line(&mut use_numbers).expect("Error reading input");
    let use_numbers = use_numbers.trim().to_lowercase() == "y";

    println!("Include symbols? (y/n)");
    let mut use_symbols = String::new();
    io::stdin().read_line(&mut use_symbols).expect("Error reading input");
    let use_symbols = use_symbols.trim().to_lowercase() == "y";

    let mut passwords = Vec::new();
    
    for i in 0..num_passwords {
        let password = generate_password(password_length, use_upper, use_lower, use_numbers, use_symbols);
        let security = evaluate_password(&password);
        println!("Password {}: {} - Security: {}", i + 1, password, security);
        passwords.push((password, security));
    }

    println!("Would you like to save the generated passwords to a file? (y/n)");
    let mut save_to_file = String::new();
    io::stdin().read_line(&mut save_to_file).expect("Error reading input");
    if save_to_file.trim().to_lowercase() == "y" {
        save_passwords_to_file(passwords);
    }
}

// Function to display menu for password cracking and handle user input
fn crack_password_menu() -> Result<()> {
    println!("Please enter the MD5 hash of the password to crack:");
    let mut hash = String::new();
    io::stdin().read_line(&mut hash).expect("Error reading input");
    let hash = hash.trim();

    println!("Please enter the path to the dictionary file:");
    let mut dictionary_path = String::new();
    io::stdin().read_line(&mut dictionary_path).expect("Error reading input");
    let dictionary_path = dictionary_path.trim();

    if let Some(password) = crack_password(hash, dictionary_path)? {
        println!("Password found: {}", password);
    } else {
        println!("Password not found in the dictionary.");
    }

    Ok(())
}

// Function to generate a password based on specified criteria
fn generate_password(length: usize, use_upper: bool, use_lower: bool, use_numbers: bool, use_symbols: bool) -> String {
    let mut charset = String::new();

    if use_upper {
        charset.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    }
    if use_lower {
        charset.push_str("abcdefghijklmnopqrstuvwxyz");
    }
    if use_numbers {
        charset.push_str("0123456789");
    }
    if use_symbols {
        charset.push_str(")(*&^%$#@!~");
    }

    let charset = charset.as_bytes();
    let mut rng = rand::thread_rng();

    let password: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset[idx] as char
        })
        .collect();
    password
}

// Function to evaluate the security level of a password
fn evaluate_password(password: &str) -> String {
    let length = password.len();
    let has_upper = password.chars().any(|c| c.is_uppercase());
    let has_lower = password.chars().any(|c| c.is_lowercase());
    let has_number = password.chars().any(|c| c.is_numeric());
    let has_symbol = password.chars().any(|c| !c.is_alphanumeric());

    let mut score = 0;

    if length >= 8 {
        score += 1;
    }
    if length >= 12 {
        score += 1;
    }
    if length >= 16 {
        score += 1;
    }
    if has_upper {
        score += 1;
    }
    if has_lower {
        score += 1;
    }
    if has_number {
        score += 1;
    }
    if has_symbol {
        score += 1;
    }

    match score {
        0..=2 => "Weak".to_string(),
        3..=5 => "Medium".to_string(),
        6..=8 => "Strong".to_string(),
        _ => "Very Strong".to_string(),
    }
}

// Function to save generated passwords to a file
fn save_passwords_to_file(passwords: Vec<(String, String)>) {
    let file_name = "generated_passwords.txt";
    let now = Local::now();
    let datetime = now.format("%Y-%m-%d %H:%M:%S").to_string();

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_name)
        .expect("Error opening file");

    writeln!(file, "Passwords generated on : {}", datetime).expect("Error writing to file");
    for (password, security) in passwords {
        writeln!(file, "Password : {}, Security : {}", password, security).expect("Error writing to file");
    }
    println!("Passwords have been saved to the file '{}'.", file_name);
}

// Function to crack a password using a dictionary file and an MD5 hash
fn crack_password(hash: &str, dictionary_path: &str) -> Result<Option<String>> {
    let file = File::open(dictionary_path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let password = line?;
        if md5::compute(password.as_bytes()).to_hex() == hash {
            return Ok(Some(password));
        }
    }

    Ok(None)
}

// Trait implementation to convert an MD5 hash to a hexadecimal string
trait ToHex {
    fn to_hex(&self) -> String;
}

impl ToHex for [u8; 16] {
    fn to_hex(&self) -> String {
        self.iter().map(|byte| format!("{:02x}", byte)).collect()
    }
}

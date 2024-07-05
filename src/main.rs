use rand::Rng;
use std::fs::OpenOptions;
use std::io::{self, Write};
use chrono::Local;

// func to generate password
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

// func to evaluate password
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
        0..=2 => "Faible".to_string(),
        3..=5 => "Moyenne".to_string(),
        6..=8 => "Haute".to_string(),
        _ => "Très Haute".to_string(),
    }
}

// main function
fn main() {
    println!("Bienvenue dans le générateur de mots de passe !");
    println!("Longueurs recommandées pour un mot de passe sécurisé :");
    println!("1. Faible sécurité (8-11 caractères)");
    println!("2. Sécurité moyenne (12-15 caractères)");
    println!("3. Haute sécurité (16+ caractères)");

    println!("Combien de mots de passe souhaitez-vous générer ?");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Erreur de lecture de l'entrée");
    let num_passwords: usize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Entrée invalide, utilisation de 1 mot de passe par défaut.");
            1
        }
    };

    println!("Veuillez entrer le nombre de caractères pour les mots de passe:");
    input.clear();
    io::stdin().read_line(&mut input).expect("Erreur de lecture de l'entrée");
    let password_length: usize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Entrée invalide, utilisation de la longueur par défaut de 12 caractères.");
            12
        }
    };

    println!("Inclure des majuscules ? (y/n)");
    let mut use_upper = String::new();
    io::stdin().read_line(&mut use_upper).expect("Erreur de lecture de l'entrée");
    let use_upper = use_upper.trim().to_lowercase() == "y";

    println!("Inclure des minuscules ? (y/n)");
    let mut use_lower = String::new();
    io::stdin().read_line(&mut use_lower).expect("Erreur de lecture de l'entrée");
    let use_lower = use_lower.trim().to_lowercase() == "y";

    println!("Inclure des chiffres ? (y/n)");
    let mut use_numbers = String::new();
    io::stdin().read_line(&mut use_numbers).expect("Erreur de lecture de l'entrée");
    let use_numbers = use_numbers.trim().to_lowercase() == "y";

    println!("Inclure des symboles ? (y/n)");
    let mut use_symbols = String::new();
    io::stdin().read_line(&mut use_symbols).expect("Erreur de lecture de l'entrée");
    let use_symbols = use_symbols.trim().to_lowercase() == "y";

    let mut passwords = Vec::new();
    
    for i in 0..num_passwords {
        let password = generate_password(password_length, use_upper, use_lower, use_numbers, use_symbols);
        let security = evaluate_password(&password);
        println!("Mot de passe {}: {} - Sécurité: {}", i + 1, password, security);
        passwords.push((password, security));
    }

    println!("Souhaitez-vous sauvegarder les mots de passe générés dans un fichier ? (y/n)");
    let mut save_to_file = String::new();
    io::stdin().read_line(&mut save_to_file).expect("Erreur de lecture de l'entrée");
    if save_to_file.trim().to_lowercase() == "y" {
        save_passwords_to_file(passwords);
    }
}

// func to save passwords to file
fn save_passwords_to_file(passwords: Vec<(String, String)>) {
    let file_name = "generated_passwords.txt";
    let now = Local::now();
    let datetime = now.format("%Y-%m-%d %H:%M:%S").to_string();

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_name)
        .expect("Erreur lors de l'ouverture du fichier");

    writeln!(file, "Mots de passe générés le : {}", datetime).expect("Erreur d'écriture dans le fichier");
    for (password, security) in passwords {
        writeln!(file, "Mot de passe : {}, Sécurité : {}", password, security).expect("Erreur d'écriture dans le fichier");
    }
    println!("Les mots de passe ont été sauvegardés dans le fichier '{}'.", file_name);
}

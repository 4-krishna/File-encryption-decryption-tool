use clap::{Arg, Command};
use fernet::{Fernet, Key};
use std::{fs::File, io::{self, Read, Write}, path::Path};

fn generate_key() -> Key {
    Fernet::generate_key()
}

fn encrypt_file(input_file: &str, output_file: &str) -> io::Result<()> {
    let key = generate_key();
    let cipher_suite = Fernet::new(&key).unwrap();
    
    let mut file = File::open(input_file)?;
    let mut plaintext = Vec::new();
    file.read_to_end(&mut plaintext)?;
    
    let encrypted_data = cipher_suite.encrypt(&plaintext);
    
    let mut output = File::create(output_file)?;
    output.write_all(&encrypted_data)?;
    
    println!("File encrypted successfully.");
    println!("Encryption key: {}", key.to_base64());
    
    Ok(())
}

fn decrypt_file(input_file: &str, output_file: &str, key: &str) -> io::Result<()> {
    let cipher_suite = Fernet::new(key.as_bytes()).unwrap();
    
    let mut file = File::open(input_file)?;
    let mut encrypted_data = Vec::new();
    file.read_to_end(&mut encrypted_data)?;
    
    let decrypted_data = cipher_suite.decrypt(&encrypted_data).unwrap();
    
    let mut output = File::create(output_file)?;
    output.write_all(&decrypted_data)?;
    
    println!("File decrypted successfully.");
    
    Ok(())
}

fn main() {
    let matches = Command::new("File Encryption and Decryption Tool")
        .version("1.0")
        .author("Author <author@example.com>")
        .about("Encrypts and decrypts files")
        .arg(Arg::new("action")
            .about("'encrypt' or 'decrypt'")
            .required(true)
            .index(1)
            .possible_values(&["encrypt", "decrypt"]))
        .arg(Arg::new("input_file")
            .about("Input file path")
            .required(true)
            .index(2))
        .arg(Arg::new("output_file")
            .about("Output file path")
            .short('o')
            .long("output")
            .takes_value(true))
        .get_matches();
    
    let action = matches.value_of("action").unwrap();
    let input_file = matches.value_of("input_file").unwrap();
    
    let output_file = matches.value_of("output_file");
    
    if action == "encrypt" {
        if !Path::new(input_file).exists() {
            println!("Error: Input file '{}' not found.", input_file);
            return;
        }
        
        if output_file.is_none() {
            println!("Error: Output file path required.");
            return;
        }
        
        let output_file = output_file.unwrap();
        if let Err(e) = encrypt_file(input_file, output_file) {
            eprintln!("Error: {}", e);
        }
    } else if action == "decrypt" {
        if output_file.is_none() {
            println!("Error: Output file path required.");
            return;
        }
        
        let key = rpassword::prompt_password("Enter encryption key: ").unwrap();
        let output_file = output_file.unwrap();
        if let Err(e) = decrypt_file(input_file, output_file, &key) {
            eprintln!("Error: {}", e);
        }
    }
}

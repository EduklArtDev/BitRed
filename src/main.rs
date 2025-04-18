use bip39::{Language, Mnemonic};
use std::io;

fn main() {
    let mut inptsenha = String::new();

    println!("Digita sua senha ai: ");
    io::stdin()
        .read_line(&mut inptsenha)
        .expect("Erro ao ler a linha");

    //remove \n
    let inptsenha = inptsenha.trim();

    println!("Digitou: {}", inptsenha);

    //.............................

    let mnemonic = Mnemonic::from_entropy(&[0; 32]).unwrap();

    let senha = inptsenha;

    // Gera a seed com base na mnemônica + senha (BIP-39)
    let seed = mnemonic.to_seed(senha);

    println!("Seed (hex): {}", hex::encode(seed));
}

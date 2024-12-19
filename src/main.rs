// src/main.rs
mod token_wallet;
use token_wallet::Wallet;

fn main() {
    let alice_wallet = Wallet::new();
    let bob_wallet = Wallet::new();

    alice_wallet.receive_tokens(100);  // Add initial balance to Alice's wallet
    match alice_wallet.send_tokens(50, &bob_wallet) {  // Send 50 tokens to Bob's wallet
        Ok(_) => println!("Tokens sent successfully!"),
        Err(e) => println!("Failed to send tokens: {}", e),
    }

    // Get balance tokens present in Alice's and Bob's wallets
    println!("Alice's balance: {}", alice_wallet.get_balance());
    println!("Bob's balance: {}", bob_wallet.get_balance());
}

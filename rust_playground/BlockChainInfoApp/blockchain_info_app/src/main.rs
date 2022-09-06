// Enable macros use from the external crate serde_derive

use core::time;
use std::{io, thread};

use dialoguer::{theme::ColorfulTheme, Select};
use ethereum::eth_info::latest_block;
#[macro_use]
extern crate serde_derive;
extern crate chrono;

// in order to make the module in this crate available to the crate, we need to make the crate aware of them
mod now_nodes {
    pub mod blockchain_address;
    pub mod blockchain_info;
    pub mod blockchain_status;
    pub mod blockchain_transaction;
}

mod ethereum {
    pub mod eth_info;
    pub mod utils;
}
// using the contents of the modules in main
use crate::now_nodes::{
    blockchain_address::BlockchainAddress,
    blockchain_info::{
        blockchain_address_request, blockchain_status_request, calculate_wallet_balance, Chain,
    },
    blockchain_status::BlockchainStatus,
};

fn now_nodes_app(chain_str:&str, address: &str) {
    
    let blockchain = match chain_str {
        "ethereum" => Chain::Ethereum,
        "bitcoin" => Chain::Bitcoin,
        _ => Chain::Bitcoin
    };
    
    let status: BlockchainStatus = blockchain_status_request(&blockchain);
    println!(
        "\n\nQuerying {} - chain: {}\n\n",
        &status.blockbook.coin, // accessing the attributes of the struct
        &status.backend.chain
    );

    let bc_address: BlockchainAddress = blockchain_address_request(&blockchain, address);

    println!(
        "\n\nAnalyzing transactions from Bitcoin adress {}\n\n",
        &bc_address.address
    ); // accessing the attributes of the struct

    let sleep_time = time::Duration::from_millis(2500);
    // sleep or 2.5s for user friendly print statements
    thread::sleep(sleep_time);
    println!(
        "\nYou have a total of {} transactions!",
        &bc_address.txids.len()
    );

    println!("\nDo you want to query these transactions? (y/n)\n");
    let mut command = String::new();
    io::stdin().read_line(&mut command);

    if command.trim().eq("y") {
        println!("\nWe will look up the following transactions:\n");
        thread::sleep(sleep_time);
        println!("{:#?}", &bc_address.txids); // {:#?} creates a prettier output presentation of the vector
        let balance: i32 = calculate_wallet_balance(&blockchain,&bc_address, address);
        println!("Current BALANCE:           {}", &balance);
        println!(
            "         IN BTC:           {}\n\n",
            balance as f32 * 0.00000001
        );
    }
}

fn main() {
    let selections = &[
        "BTC (NOWNodes)",
        "Ethereum (NOWNodes)",
        "Ethereum (Localhost)",
        "Ethereum (Infura)",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Which data source would you like to query?")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    match selection {
        0 => now_nodes_app("bitcoin",&dotenv::var("BC_WALLET").expect("Error reading env var.")),
        1 => now_nodes_app("ethereum",&dotenv::var("ETH_WALLET").expect("Error reading env var.")),
        2 => latest_block("ws://127.0.0.1:8546"),
        3 => latest_block(
            &[
                "wss://mainnet.infura.io/ws/v3/",
                &dotenv::var("INFURA_API_KEY").expect("Could not find key: INFURA_API_KEY"),
            ]
            .join(""),
        ),
        _ => println!("No idea what you entered.."),
    }
}

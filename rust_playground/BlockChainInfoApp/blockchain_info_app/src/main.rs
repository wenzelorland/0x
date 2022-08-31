// Enable macros use from the external crate serde_derive

use core::time;
use std::{io, thread};

use dialoguer::{theme::ColorfulTheme, Select};
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

mod geth {
    pub mod geth_info;
}
// using the contents of the modules in main
use crate::now_nodes::blockchain_address::BlockchainAddress;

fn bc_now_nodes_app(address: &str) {
    let status: now_nodes::blockchain_status::BlockchainStatus =
        now_nodes::blockchain_info::blockchain_status_request();
    println!(
        "\n\nQuerying {} - chain: {}\n\n",
        &status.blockbook.coin, // accessing the attributes of the struct
        &status.backend.chain
    );
    let bc_address: BlockchainAddress =
        now_nodes::blockchain_info::blockchain_address_request(address);

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
        let balance: i32 =
            now_nodes::blockchain_info::calculate_wallet_balance(&bc_address, address);
        /*
        let mut balance: i32 = 0;
        for tx_id in &bc_address.txids {
            // vout are actually transactions that went in
            // vin are transactions that went out
            let mut subtotal_vin: i32 = 0;
            let mut subtotal_vout: i32 = 0;

            // reading the transaction details
            let bc_transaction: BlockchainTransaction =  blockchain_info::blockchain_transaction_request(&tx_id);

            let match_address = String::from(address);

            // tx is represented by each item in the vin vector
            for tx in &bc_transaction.vin {
                // each tx will be a Vin struct
                // since addresses is a vector, we can call the .contains method on it
                if tx.addresses.contains(&match_address) {
                    // since the value attribute is stored as string, we need to parse it to an integer first
                    // only append the value where the address matches
                    subtotal_vin += tx.value.parse::<i32>().unwrap();
                }
            }

            for tx in &bc_transaction.vout {
                if tx.addresses.contains(&match_address) {
                    subtotal_vout += tx.value.parse::<i32>().unwrap();
                }
            }
            balance += &subtotal_vout - &subtotal_vin;

            println!("-------------------------------------------------------");
            println!("TX ID:                     {}", &bc_transaction.txid);
            println!("Satoshis IN:               {}", &subtotal_vout);
            println!("Satoshis OUT:              {}", &subtotal_vin);
            println!("BALANCE:                   {}", &balance);
            println!("-------------------------------------------------------");
        }*/
        println!("Current BALANCE:           {}", &balance);
        println!(
            "         IN BTC:           {}\n\n",
            balance as f32 * 0.00000001
        );
    }
}

fn main() {
    let selections = &["NOWNodes - BTC", "Go Ethereum - Localhost"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Which data source would you like to query?")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();
    if selection == 0 {
        let entered_address = dotenv::var("BC_WALLET").expect("Error reading env var.");
        bc_now_nodes_app(&entered_address);
    } else if selection == 1 {
        geth::geth_info::latest_block();
    } else {
        println!("No idea what you entered..")
    }
}

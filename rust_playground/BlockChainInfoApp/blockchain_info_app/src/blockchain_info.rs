use reqwest;
use dotenv;
use tokio;
// use the Result trait from the serde crate
use serde_json::Result; 

use chrono::prelude::DateTime;
use chrono::Utc;
use std::time::{SystemTime, UNIX_EPOCH, Duration};


// importing from another file in this source folder
use crate::blockchain_status::BlockchainStatus;
use crate::blockchain_address::BlockchainAddress;
use crate::blockchain_transaction::BlockchainTransaction;

const HOST_ROOT: &str = "https://btcbook.nownodes.io/api/";

#[tokio::main]
pub async fn send_request(url: &str) -> String {
    let client = reqwest::Client::new();
    client
        .get(url) // build basic get request
        .header("api-key", dotenv::var("API_KEY").expect("Could not find key: API_KEY")) // the expect method handles the Optional when None is returned
        .send()
        .await // for async runtimes
        .expect("Failed to get response")
        .text() // this is goign to produce a String representation of JSON
        .await // another await for the parsing into text
        .expect("Failed to convert payload")

}

pub fn blockchain_status_request() -> BlockchainStatus{
    let response = send_request(&HOST_ROOT);
    // specifying the return 
    serde_json::from_str(&response)
        .expect("Failed to parse JSON") // option return type handling
}

pub fn blockchain_address_request(address:&str) -> BlockchainAddress{
    let response = send_request(&[HOST_ROOT, "v2/address/", &address].join("")); // joining mutliple strings through array creationa and join method on the array
    // specifying the return 
    serde_json::from_str(&response)
        .expect("Failed to parse JSON") // option return type handling
}

pub fn blockchain_transaction_request(transaction:&str) -> BlockchainTransaction{
    // retrieve transaction details by hash
    let response = send_request(&[HOST_ROOT, "v2/tx/", &transaction].join("")); // joining mutliple strings through array creationa and join method on the array
    // specifying the return 
    serde_json::from_str(&response)
        .expect("Failed to parse JSON") // option return type handling
}

fn convert_unix_ts(unix_int:&u64) -> String {
    let d = UNIX_EPOCH + Duration::from_secs(*unix_int);
    let datetime = DateTime::<Utc>::from(d);
    datetime.format("%Y-%m-%d %H:%M:%S.%f").to_string()
}

pub fn calculate_wallet_balance(blockchain_address: &BlockchainAddress, address:&str ) -> i32 {
    let mut balance: i32 = 0;
    blockchain_address.txids.iter().rev().for_each(|tx_id| {
        // vout are actually transactions that went in 
        // vin are transactions that went out
        let mut subtotal_vin: i32 = 0;
        let mut subtotal_vout: i32 = 0;

        // reading the transaction details
        let bc_transaction: BlockchainTransaction =  blockchain_transaction_request(&tx_id);
            
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
        println!("Block Time:                {}", convert_unix_ts(&bc_transaction.block_time));
        println!("Satoshis IN:               {}", &subtotal_vout);
        println!("Satoshis OUT:              {}", &subtotal_vin);
        println!("BALANCE:                   {}", &balance);
        println!("-------------------------------------------------------");

    });
    balance
}
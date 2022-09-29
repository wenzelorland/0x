use dialoguer::theme::ColorfulTheme;
use dialoguer::Confirm;
use web3::helpers as w3h;

use web3::types::{H160, U64, BlockNumber};

use crate::ethereum::utils::{
    get_func_signature, get_token_name, get_transaction, is_smart_contract, wei_to_eth,
};

use super::utils::{get_block, choose_block};

#[tokio::main]
pub async fn ethereum_info(host: &str) {
    dotenv::dotenv().ok();

    let websocket = web3::transports::WebSocket::new(host).await.unwrap();
    let web3s = web3::Web3::new(websocket);
    let block_no = choose_block();
    let block = get_block(&web3s, block_no).await;
    

    if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want to query all transations in this block?")
        .default(true)
        .interact()
        .unwrap()
    {
        for transaction_hash in block.transactions {
            let tx = get_transaction(&web3s, transaction_hash).await;
            let mut token_name: String = "".to_string();
            if is_smart_contract(&web3s, tx.to.unwrap()).await {
                token_name = get_token_name(&web3s, tx.to.unwrap()).await;
            }
            let func_signature = get_func_signature(tx.input);

            let from_addr = tx.from.unwrap_or(H160::zero());
            let to_addr = tx.to.unwrap_or(H160::zero());

            let eth_value = wei_to_eth(tx.value);
            println!("-------------------------------------------------------");
            println!(
                "TX Index:              {}",
                tx.transaction_index.unwrap_or(U64::from(0))
            );
            println!(
                "TX Hash:               {}",
                w3h::to_string(&transaction_hash)
            );
            println!("Token Name:            {}", token_name);
            println!("Method:                {}", &func_signature);
            println!("From:                  {}", w3h::to_string(&from_addr));
            println!("To:                    {}", w3h::to_string(&to_addr));
            println!("Value:                 {}", eth_value);
            println!("Gas:                   {}", tx.gas);
            println!("Gas Price:             {}", tx.gas_price);
            println!("-------------------------------------------------------");
        }
    }
}

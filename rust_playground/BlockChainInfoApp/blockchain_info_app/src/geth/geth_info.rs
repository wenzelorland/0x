use web3::types::{BlockId, BlockNumber};
#[tokio::main]
pub async fn latest_block() {
    dotenv::dotenv().ok();
    let websocket = web3::transports::WebSocket::new("ws://127.0.0.1:8546")
        .await
        .unwrap();
    let web3s = web3::Web3::new(websocket);
    let latest_block = web3s
        .eth()
        .block(BlockId::Number(BlockNumber::Latest))
        .await
        .unwrap()
        .unwrap();
    println!(
        "block number {}, number of transactions: {}, difficulty {}",
        latest_block.number.unwrap(),
        &latest_block.transactions.len(),
        &latest_block.total_difficulty.unwrap()
    );
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BlockchainAddress {
    page: u64,
    total_pages: u64,
    items_on_page: u64,
    pub address: String,
    balance: String,
    total_received: String,
    total_sent: String,
    unconfirmed_balance: String,
    unconfirmed_txs: u64,
    txs: u64,
    // Vector are very similar to lists, and since the response in the json field in this case represents a list structure
    // which in Rust is resembled by Vectors, thus create a Vector of Strings for that
    pub txids: Vec<String>,
}

/*
For Message get_address
https://documenter.getpostman.com/view/13630829/TVmFkLwy#cebd6a63-13bc-4ba1-81f7-360c88871b90

{
  "page": 1,
  "totalPages": 1,
  "itemsOnPage": 1000,
  "address": "bc1qkwk40g4shd3suh6h0vw00l27pzqlmuamp0mpfv",
  "balance": "0",
  "totalReceived": "0",
  "totalSent": "0",
  "unconfirmedBalance": "36281",
  "unconfirmedTxs": 1,
  "txs": 0,
  "txids": [
    "e9af0964a511954dd1a8c66e358707eba670bf0489ead22e771d7c8a1b108784"
  ]
}
*/
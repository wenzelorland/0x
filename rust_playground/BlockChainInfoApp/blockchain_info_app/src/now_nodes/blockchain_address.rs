#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    
    #[serde(rename = "type")]
    pub token_type: String,
    pub name: String,
    pub contract: String,
    pub transfers: u64,
    pub symbol: String,
    pub decimals: u16,
    pub balance: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BlockchainAddress {
    page: u64,
    total_pages: u64,
    items_on_page: u64,
    pub address: String,
    pub balance: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    total_received: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    total_sent: Option<String>,

    unconfirmed_balance: String,
    unconfirmed_txs: u64,


    pub txs: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_token_txs: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce:Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tokens: Option<Vec<Token>>,
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

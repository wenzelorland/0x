// Message Structs
#[derive(Deserialize, Debug)]
// because of this rust will be able to recognize as deserializable from JSON
// Since the message response fields are in camelCase, we need to tell Rust to raccount 
// for the naming convention when trying to match with Rust specific conventions, like the snake_case that are typically used within rust
#[serde(rename_all = "camelCase")]
pub struct Blockbook {
    pub coin: String,
    host: String,
    version: String,
    git_commit: String,
    build_time: String,
    sync_mode: bool,
    initial_sync: bool,
    in_sync: bool,
    best_height: u64,
    last_block_time: String,
    in_sync_mempool: bool,
    last_mempool_time: String,
    mempool_size: u64,
    decimals: u64,
    db_size: u64,
    about: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Backend {
    pub chain: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    blocks: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    headers: Option<u64>,
    best_block_hash: String,
    difficulty: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    size_on_disk: Option<u64>,
    version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    subversion: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protocol_version: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BlockchainStatus {
    // the fields are datatypes of other structs
    pub blockbook: Blockbook,
    pub backend: Backend,
}

//for the json body:
/*{
  "blockbook": {
    "coin": "Bitcoin",
    "host": "1a2fd4d97e40",
    "version": "devel",
    "gitCommit": "4188e73",
    "buildTime": "2020-12-25T18:22:47+00:00",
    "syncMode": true,
    "initialSync": false,
    "inSync": true,
    "bestHeight": 703054,
    "lastBlockTime": "2021-10-01T17:14:48.861079741Z",
    "inSyncMempool": true,
    "lastMempoolTime": "2021-10-01T17:24:45.89833923Z",
    "mempoolSize": 17202,
    "decimals": 8,
    "dbSize": 331167505510,
    "about": "Blockbook - blockchain indexer for Trezor wallet https://trezor.io/. Do not use for any other purpose."
  },
  "backend": {
    "chain": "main",
    "blocks": 703054,
    "headers": 703054,
    "bestBlockHash": "000000000000000000092520079e3e4436df7b51aa398f669c330e8e173de67c",
    "difficulty": "18997641161758.95",
    "sizeOnDisk": 416919758736,
    "version": "200100",
    "subversion": "/Satoshi:0.20.1/",
    "protocolVersion": "70015",
    "warnings": "Warning: unknown new rules activated (versionbit 2) "
  }
} */
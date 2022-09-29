use chrono::{DateTime, NaiveDateTime, Utc};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufReader, self};
use web3;
use web3::contract::{Contract, Options};
use web3::helpers as w3h;
use web3::transports::WebSocket;
use web3::types::{Block, BlockNumber, Bytes, Transaction, TransactionId, H160, H256, U256, U64};
use web3::Web3;

pub fn wei_to_eth(wei_val: U256) -> f64 {
    let res = wei_val.as_u128() as f64;
    let res = res / 1_000_000_000_000_000_000.0;
    res
}

pub fn choose_block() -> BlockNumber {
    println!("\nWhich blocknumber would you like to query? ('latest' or '' for latest, int for specific block)\n");
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    match trimmed {
        "" => return BlockNumber::Latest,
        "latest" => return BlockNumber::Latest,
        _ => match trimmed.parse::<u64>() {
            Ok(i) => {println!("{}", i);return BlockNumber::Number(U64::from(i))},
            Err(..) => {
                println!("unknown input: {} using latest", trimmed);
                return BlockNumber::Latest
            },
        }
    } 
    }

pub async fn get_block(web3s: &Web3<WebSocket>, block_no: BlockNumber) -> Block<H256> {
    let block = web3s
        .eth()
        .block(web3::types::BlockId::Number(block_no))
        .await
        .unwrap()
        .unwrap();
    let timestamp = block.timestamp.as_u64() as i64;
    let naive = NaiveDateTime::from_timestamp(timestamp, 0);
    let utc_dt: DateTime<Utc> = DateTime::from_utc(naive, Utc);
    println!(
        "[{}] block num {}, parent {}, transactions: {}, gas used {}, gas limit {}, base fee {}, difficulty {}, total difficulty {}",
        utc_dt.format("%Y-%m-%d %H:%M:%S"),
        block.number.unwrap(),
        block.parent_hash,
        block.transactions.len(),
        block.gas_used,
        block.gas_limit,
        block.base_fee_per_gas.unwrap(),
        block.difficulty,
        block.total_difficulty.unwrap()
    );
    block
}

pub async fn is_smart_contract(web3s: &Web3<WebSocket>, to_addr: H160) -> bool {
    let found_smart_contract = match to_addr {
        addr => match web3s.eth().code(addr, None).await {
            Ok(code) => {
                if code == web3::types::Bytes::from([]) {
                    // println!("Empty code, skipping.");
                    false
                } else {
                    // println!("Non empty code, returning address.");
                    true
                }
            }
            _ => {
                // println!("Unable to retrieve code, skipping.");
                false
            }
        },
    };
    found_smart_contract
}

pub async fn get_token_name(web3s: &Web3<WebSocket>, smart_contract_addr: H160) -> String {
    let smart_contract = Contract::from_json(
        web3s.eth(),
        smart_contract_addr,
        include_bytes!("resources/erc20_abi.json"),
    )
    .unwrap();

    let token_name: String = match smart_contract
        .query("name", (), None, Options::default(), None)
        .await
    {
        Ok(result) => result,
        _ => {
            // println!("Could not get name, skipping.");
            "[unknown_erc20_token]".to_string()
        }
    };
    token_name
}

pub async fn get_transaction(web3s: &Web3<WebSocket>, tx_hash: H256) -> Transaction {
    let tx = web3s
        .eth()
        .transaction(TransactionId::Hash(tx_hash))
        .await
        .unwrap()
        .unwrap();
    tx
}

pub fn get_func_signature(tx_input: Bytes) -> String {
    let file = File::open("src/ethereum/resources/signatures.json").unwrap();
    let reader = BufReader::new(file);
    let code_sig_lookup: BTreeMap<String, Vec<String>> = serde_json::from_reader(reader).unwrap();
    let input_str: String = w3h::to_string(&tx_input);
    if input_str.len() < 12 {
        return "[unknown_func]".to_string();
    }
    let func_code = input_str[3..11].to_string();
    let func_signature: String = match code_sig_lookup.get(&func_code) {
        Some(func_sig) => format!("{:?}", func_sig),
        _ => {
            // println!("Function not found.");
            "[unknown_func]".to_string()
        }
    };
    func_signature
}

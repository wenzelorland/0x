// Parsing the transaction json payload
// where the response body has a repetetive pattern
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Vin {
    txid: String,
    vout: u64,
    sequence: u64,
    n: u64,
    pub addresses: Vec<String>,
    is_address: bool,
    pub value: String,
    hex: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Vout {
    pub value: String,
    n: u64, 
    hex: String,
    pub addresses: Vec<String>,
    is_address: bool,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BlockchainTransaction {
    pub txid: String,
    version: u64,
    //lock_time: u64,
    // since the pattern is repetitive, we can use structs and 
    // assign them to the struct fields to reflect the message pattern
    pub vin: Vec<Vin>,
    // the objects in vout list are of the type Vout (wich the struct Vout reflects)
    pub vout: Vec<Vout>,
    block_hash: String,
    block_height: u64,
    confirmations: u64,
    pub block_time: u64,
    value: String,
    value_in: String,
    fees: String,
    hex: String,
}

/*
Transaction response
{
  "txid": "e9af0964a511954dd1a8c66e358707eba670bf0489ead22e771d7c8a1b108784",
  "version": 2,
  "lockTime": 703054,
  "vin": [
    {
      "txid": "718bc862423ccac4f6568e5aff4eaf443f60e7d6cde7888e893a64e4048897a5",
      "vout": 1,
      "sequence": 4294967293,
      "n": 0,
      "addresses": [
        "bc1q96qxee235gcmp58v055l8whhh2mujjh3refjqp"
      ],
      "isAddress": true,
      "value": "37831"
    }
  ],
  "vout": [
    {
      "value": "36281",
      "n": 0,
      "hex": "0014b3ad57a2b0bb630e5f577b1cf7fd5e0881fdf3bb",
      "addresses": [
        "bc1qkwk40g4shd3suh6h0vw00l27pzqlmuamp0mpfv"
      ],
      "isAddress": true
    }
  ],
  "blockHeight": -1,
  "confirmations": 0,
  "blockTime": 1633109991,
  "value": "36281",
  "valueIn": "37831",
  "fees": "1550",
  "hex": "02000000000101a5978804e4643a898e88e7cdd6e7603f44af4eff5a8e56f6c4ca3c4262c88b710100000000fdffffff01b98d000000000000160014b3ad57a2b0bb630e5f577b1cf7fd5e0881fdf3bb02473044022070114262df47e1f9cd3f8b73aa1d9a23e3d990116693b7d98bebfe75d236a69a02204cfd7c7b3af1d058cf5e8309ce3b793fb8989ce130027c524c80a07e5f18dd4e012103e5cd1fbd0c264b32fec6557be5b67d6af194c98f323ca365ac488cf0c15b226d4eba0a00",
  "rbf": true,
  "coinSpecificData": {
    "txid": "e9af0964a511954dd1a8c66e358707eba670bf0489ead22e771d7c8a1b108784",
    "hash": "18a902591156cfd8a4272d13023046508de6393243b33c09a5e16e48c561c41b",
    "version": 2,
    "size": 191,
    "vsize": 110,
    "weight": 437,
    "locktime": 703054,
    "vin": [
      {
        "txid": "718bc862423ccac4f6568e5aff4eaf443f60e7d6cde7888e893a64e4048897a5",
        "vout": 1,
        "scriptSig": {
          "asm": "",
          "hex": ""
        },
        "txinwitness": [
          "3044022070114262df47e1f9cd3f8b73aa1d9a23e3d990116693b7d98bebfe75d236a69a02204cfd7c7b3af1d058cf5e8309ce3b793fb8989ce130027c524c80a07e5f18dd4e01",
          "03e5cd1fbd0c264b32fec6557be5b67d6af194c98f323ca365ac488cf0c15b226d"
        ],
        "sequence": 4294967293
      }
    ],
    "vout": [
      {
        "value": 0.00036281,
        "n": 0,
        "scriptPubKey": {
          "asm": "0 b3ad57a2b0bb630e5f577b1cf7fd5e0881fdf3bb",
          "hex": "0014b3ad57a2b0bb630e5f577b1cf7fd5e0881fdf3bb",
          "reqSigs": 1,
          "type": "witness_v0_keyhash",
          "addresses": [
            "bc1qkwk40g4shd3suh6h0vw00l27pzqlmuamp0mpfv"
          ]
        }
      }
    ],
    "hex": "02000000000101a5978804e4643a898e88e7cdd6e7603f44af4eff5a8e56f6c4ca3c4262c88b710100000000fdffffff01b98d000000000000160014b3ad57a2b0bb630e5f577b1cf7fd5e0881fdf3bb02473044022070114262df47e1f9cd3f8b73aa1d9a23e3d990116693b7d98bebfe75d236a69a02204cfd7c7b3af1d058cf5e8309ce3b793fb8989ce130027c524c80a07e5f18dd4e012103e5cd1fbd0c264b32fec6557be5b67d6af194c98f323ca365ac488cf0c15b226d4eba0a00"
  }
}
*/
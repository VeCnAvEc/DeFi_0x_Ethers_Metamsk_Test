use std::fs::File;
use std::io::Read;

use ethers::addressbook::Address;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Token {
    pub address: Address,
    pub symbol: String,
    pub decimals: u8
}
#[derive(Debug, Deserialize, Serialize)]
pub struct TokenPairs {
    token0: Token,
    token1: Token,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct PairsConfig {
    pairs_bsc: Vec<TokenPairs>
}

impl TokenPairs {
    fn get_sell_token(&self) -> &Token {
        &self.token0
    }

    fn get_buy_token(&self) -> &Token {
        &self.token1
    }
}

impl PairsConfig {
    pub fn serialize_pairs_json() -> Self {
        let mut file = File::open("./src/config/Pairs.json").expect("couldn't find open Pairs.json");

        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("couldn't read the file");

        let config_pairs: Self = serde_json::from_str(contents.as_str()).unwrap();
        config_pairs
    }

    pub fn get_bsc_pair(&self, index: usize) -> Option<&TokenPairs> {
        self.pairs_bsc.get(index)
    }
}
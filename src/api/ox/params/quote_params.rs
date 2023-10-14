use serde::{Deserialize, Serialize};

use super::{params_inerface::OxRequestParams, input_params_type::input_params_type::QuoteInputParams}; 

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct QuoteParams {  
    pub sellToken: Option<String>,
    pub buyToken: Option<String>,
    pub sellAmount: Option<String>,
    pub takerAddress: Option<String>
}

impl <'a>OxRequestParams<'a, QuoteParams> for QuoteParams {
    type Output = Self;

    fn get_output(&self) -> &Self::Output {&self}
}

impl QuoteParams {
    pub fn new() -> Self {
        Self {
            sellToken: None,
            buyToken: None,
            sellAmount: None,
            takerAddress: None
        }
    }

    pub fn set_params(
        &mut self,
        input_params: &QuoteInputParams, 
    ) {
        // formatting `sell amount` to a readable value for the blockchain
        let amount: Option<String> = match input_params.1 {
            Some(amount_params) => {
                let amount = ::ethers::utils::parse_units(
                    amount_params.0, amount_params.1 as i32
                );
                let mut result = None;
                
                if let Ok(res) = amount {
                    result = Some(res.to_string());
                }

                result
            }
            None => None
        };

        if let Some(sell_token) = &input_params.0 {self.sellToken = Some(sell_token.clone())}
        if let Some(buy_token) = &input_params.2 {self.buyToken = Some(buy_token.clone())}
        if let Some(taker_addr) = &input_params.3 {self.takerAddress = Some(taker_addr.clone())}
        self.sellAmount = amount;
    }
}
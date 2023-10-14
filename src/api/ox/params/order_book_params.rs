use super::{params_inerface::OxRequestParams, input_params_type::input_params_type::OrderBookInputParams}; 
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderBookParams {
    baseToken: Option<String>,
    quoteToken: Option<String>
}

impl <'a>OxRequestParams<'a, OrderBookParams> for OrderBookParams {
    type Output = OrderBookParams;

    fn get_output(&self) -> &Self::Output {self}
}

impl OrderBookParams {
    pub fn new() -> Self {
        Self {
            baseToken: None,
            quoteToken: None
        }
    }
    
    pub fn set_params(&mut self, params: &OrderBookInputParams) {
        if let Some(base_token) = &params.0 { self.baseToken = Some(base_token.clone()) }
        if let Some(quote_token) = &params.1 { self.quoteToken = Some(quote_token.clone()) }
    }
}
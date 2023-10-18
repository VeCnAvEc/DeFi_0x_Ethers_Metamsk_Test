use serde_json::Value;

pub fn log_order_info(token_num: u8, token_info: &Value) -> String {
    format!(
        "{}: {}", token_num, token_info
    )
}

pub fn log_token_info(
    token_num: u8,
    dex_name: &Vec<Value>, symbol: &str,
    price: f64, with_price_impact: f64,
    token_purchased_quantity: f64,
    token_purchased_quantity_with_price_impact: f64,
    gas_price: f64,
    gas_price_in_coin: f64,
    gas_token_name: String
) -> String {
    format!(
        "dex name: {:?}\n\
        symbol: {symbol}\n\
        price: {price}\n\
        with price impact: {with_price_impact}\n\
        token {token_num}: {token_purchased_quantity}\n\
        token {token_num} with price impact: {token_purchased_quantity_with_price_impact}\n\
        gas price: {gas_price}\n\
        gas price in coin: {gas_token_name} {gas_price_in_coin}
        ", dex_name
    )
}
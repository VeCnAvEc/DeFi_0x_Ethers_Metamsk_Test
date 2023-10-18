use serde_json::Value;
use crate::api::ox::params::input_params_type::input_params_type::QuoteInputParams;
use crate::api::ox::request::quote;

// -> Result<, String>
pub async fn get_price(params: &QuoteInputParams) -> Result<Value, (i32, String)>{
    let swap_price_result = quote::quote::swap_quote(
        &params,
        "/swap/v1/quote?"
    ).await;

    if let Err(error) = swap_price_result {
        return Err((1468030, error));
    }

    let response = swap_price_result.unwrap();
    let text = response.text().await;
    if let Err(error) = text {
        return Err((1468031, error.to_string()));
    }

    let text_to_value: serde_json::Result<Value> = serde_json::from_str(text.unwrap().as_str());
    if let Err(error) = text_to_value {
        return Err((1468032, format!("Failed to convert text to value: {}", error.to_string())))
    }

    Ok(text_to_value.unwrap())
}
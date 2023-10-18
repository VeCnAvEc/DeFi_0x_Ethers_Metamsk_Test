pub fn format_price(price_token: Option<&serde_json::Value>) -> Result<f64, (i32, String)>{
    let price_token = price_token.unwrap().as_str().map(|n_price| {
        n_price.parse::<f64>()
            .map_or_else(|error| Err(error.to_string()), |num| Ok(num))
    });

    if let None = price_token {
        return Err((24857300, "Couldn't get the price".to_string()));
    }
    if let Some(Err(error)) = price_token {
        return Err((24857301, format!("Couldn't parse the price: {error}")));
    }

    Ok(price_token.unwrap().unwrap())
}
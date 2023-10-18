use ethers::types::U256;
use serde_json::Value;

pub mod look_for_arbitration_opportunity;
pub mod helper;

pub const ONE_ETH: u128 = 10u128.pow(18);

#[derive(Debug)]
pub struct CommissionAndPaymentParameters {
    token_price: Option<f64>,
    sources: Vec<Value>,
    estimated_price_impact: Option<f64>,
    estimated_gas: Option<f64>,
    gas_in_coin: Option<(String, f64)>
}

impl CommissionAndPaymentParameters {
    pub fn new() -> Self {
        CommissionAndPaymentParameters {
            token_price: None,
            sources: Vec::new(),
            estimated_price_impact: None,
            estimated_gas: None,
            gas_in_coin: None,
        }
    }

    pub fn build(&mut self, value: &Value) -> Result<(), (i32, String)> {
        let token_price = value.get("price");
        let sources = value.get("sources");
        let estimated_price_impact = value.get("estimatedPriceImpact");
        let estimated_gas = value.get("estimatedGas");

        if let None = token_price {
            return Err((3568320, "Couldn't to get token_price".to_string()))
        }
        if let None = sources {
            return Err((3568321, "Couldn't to get sources".to_string()))
        }
        if let None = estimated_price_impact {
            return Err((3568322, "Couldn't to get estimated_price_impact".to_string()))
        }
        if let None = estimated_gas {
            return Err((3568323, "Couldn't to get estimated_gas".to_string()))
        }

        self.set_token_price(token_price);
        self.set_sources(sources);
        self.set_estimated_price_impact(estimated_price_impact);
        self.set_estimated_gas(estimated_gas);

        Ok(())
    }

    fn set_token_price(&mut self, token_price: Option<&Value>) {
        token_price.and_then(|token_price_val| {
            token_price_val.as_str().and_then(|token_price_str| {
                let res = token_price_str.parse::<f64>().and_then(|est_price| {
                    self.token_price = Some(est_price);
                    Ok(())
                });
                res.ok()
            })
        });
    }

    fn set_sources(&mut self, sources: Option<&Value>) {
        sources.map(|sources_val| {
            for dex in sources_val.as_array().unwrap() {
                if dex.get("proportion").unwrap() != "0" {
                    self.sources.push(dex.clone());
                }
            }
        });
    }

    fn set_estimated_price_impact(&mut self, estimated_price_impact: Option<&Value>) {
        estimated_price_impact.and_then(|est_price_val| {
            est_price_val.as_str().and_then(|est_price_str| {
                let res = est_price_str.parse::<f64>().and_then(|est_price| {
                    self.estimated_price_impact = Some(est_price);
                    Ok(())
                });
                res.ok()
            })
        });
    }

    fn set_estimated_gas(&mut self, estimated_gas: Option<&Value>) {
        estimated_gas.and_then(|est_gas_val| {
            est_gas_val.as_str().and_then(|est_gas_str| {
                let res = est_gas_str.parse::<f64>().and_then(|est_gas| {
                    self.estimated_gas = Some(est_gas);
                    Ok(())
                });
                res.ok()
            })
        });
    }

    pub fn set_gas_in_coin(&mut self, price_in_coin: f64, token_name: String) {
        self.gas_in_coin = Some((token_name, price_in_coin));
    }

    fn get_token_price(&self) -> Option<f64> {
        self.token_price
    }

    fn get_estimated_gas(&self) -> Option<f64> {
        self.estimated_gas
    }
    fn get_estimated_price_impact(&self) -> Option<f64> {
        self.estimated_price_impact
    }
    fn get_sources(&self) -> &Vec<Value> {
        &self.sources
    }

    fn get_gas_in_coin(&self) -> Option<&(String, f64)> {
        return self.gas_in_coin.as_ref();
    }

    pub fn estimated_with_price_impact(&self) -> Result<f64, (i32, String)> {
        if let None = self.token_price {
            return Err((5468351, "Not found token_price".to_string()))
        }
        if let None = self.estimated_price_impact {
            return Err((5468351, "Not found estimated_price_impact".to_string()))
        }
        Ok(self.token_price.unwrap() / (1.0 - (self.estimated_price_impact.unwrap() / 10000.0)))
    }

    fn estimated_gas_price_in_coin(&mut self, network_gas_price: U256) -> f64 {
        (self.estimated_gas.unwrap() * network_gas_price.to_string().parse::<f64>().unwrap() / ONE_ETH  as f64)
    }
}
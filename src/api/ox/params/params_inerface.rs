use serde::{Serialize, Deserialize};
pub trait OxRequestParams<'de, T: Serialize + Deserialize<'de>> {
    type Output: Serialize + Deserialize<'de> ;
    
    fn get_output(&self) -> &Self::Output;

    fn build_quary_for_request(&self) -> Result<String, String> {
        let quary = serde_qs::to_string(self.get_output());
        if let Err(error) = &quary {
            return Err(error.to_string());
        }

        Ok(quary.unwrap())
    }
}
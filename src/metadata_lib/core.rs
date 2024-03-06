use serde_json::{json, Value};

#[derive(Default)]
pub struct CoreMetadata {
    name: String,
    symbol: String,
    description: String,
    seller_fee_basis_points: usize,
    image: String,
    animation_url: String,
    external_url: String,
}

impl CoreMetadata {
    pub fn as_json(&self)->Value{
        json!({
            "name":self.name,
            "symbol":self.symbol,
            "description":self.description,
            "seller_fee_basis_points":self.seller_fee_basis_points,
            "image":self.image,
            "animation_url":self.animation_url,
            "external_url":self.external_url
        })
    }
}

impl std::fmt::Display for CoreMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Implementación para mostrar Formulario
        write!(
            f,
        r#"
    "name": "{}",
    "symbol": "{}",
    "description": "{}",
    "seller_fee_basis_points": {},
    "image": "{}",
    "animation_url": "{}",
    "external_url": "{}"
        "#,
            self.name,
            self.symbol,
            self.description,
            self.seller_fee_basis_points,
            self.image,
            self.animation_url,
            self.external_url,
        )
    }
}

#[cfg(test)]
mod tets {
    use super::*;
    #[test]
    fn test_core_metadata() {
        let core_metadata = CoreMetadata::default();
        println!("{}", core_metadata);
        println!("{}",core_metadata.as_json());
    }
}

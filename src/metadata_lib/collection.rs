#[derive(Default)]
pub struct Collection {
    name: String,
    family: String,
}

impl std::fmt::Display for Collection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fmt_coll = format!(
            r#" "collection": {{ "name" : {}, "family" : {} }}"#,
            &self.name, &self.family
        );

        write!(f, "{}", fmt_coll)
    }
}

#[cfg(test)]
mod tets {
    // cargo test --package metadata-builder-sol --bin metadata-builder-sol -- metadata_lib::collection::tets::test_fmt_coll --exact --nocapture 
    use super::*;
    #[test]
    fn test_fmt_coll(){
        let collection = Collection::default();
        println!("{}",collection); 
    }
}
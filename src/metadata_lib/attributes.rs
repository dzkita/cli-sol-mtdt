// use serde::ser::Serialize;

#[derive(Default, Clone)]
pub struct Attribute {
    trait_type: String,
    value: String,
    display_type: String,
    trait_count: u64,
    max_value: u64,
}

pub struct Attributes {
    atrs: Vec<Attribute>,
}

impl Attributes {
    fn new(amount_atrs: usize) -> Self {
        let mut vector = Vec::<Attribute>::new();
        let def_atr = Attribute::default();
        for _x in 0..amount_atrs {
            vector.push(def_atr.clone());
        }
        Self { atrs: vector }
    }
}

impl std::fmt::Display for Attributes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let atr_len = self.atrs.len();
        if atr_len == 0 {
            write!(f, "")
        } else {
            let fmt_atrs: String;
            if atr_len == 1 {
                fmt_atrs = format!(r#" "attributes": [ {} ]"#, &self.atrs.get(0).unwrap());
            } else {
                fmt_atrs = format!(
                    r#" "attributes": [ \n{}\n ,"#,
                    &self
                        .atrs
                        .iter()
                        .enumerate()
                        .map(|(index, linea)| {
                            if index == atr_len - 1 {
                                format!("  {}\n]", linea)
                            } else {
                                format!("  {},\n", linea)
                            }
                        })
                        .collect::<String>()
                );
            }
            write!(f, "{}", fmt_atrs)
        }
    }
}

impl std::fmt::Display for Attribute {
    /// TODO: agregar el campo de wallet para que se fea formateado
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut instr = -1;
        let formated_str;

        if self.max_value > 0 && self.trait_count > 0 && !self.display_type.is_empty() {
            instr = 0
        } else if !self.display_type.is_empty() {
            // display type solo
            // display type solo + max_value
            // display type solo + trait_count
            if self.max_value > 0 {
                instr = 5
            } else if self.trait_count > 0 {
                instr = 6
            } else {
                instr = 2
            }
        } else {
            // ambos            -> 1 ok
            // solo trait_count -> 3 ok
            // solo max_value   -> 4 ok
            if self.max_value > 0 && self.trait_count > 0 {
                instr = 1
            } else if self.trait_count > 0 {
                instr = 3
            } else if self.max_value > 0 {
                instr = 4
            }
        }

        match instr {
            0 => {
                formated_str = format!(
                    r#"{{ "trait_type": {} , "value" : "{}" , "max_value" : {}, "trait_count" : {}, "display_type" : {} }}"#,
                    self.trait_type,
                    self.value,
                    self.max_value,
                    self.trait_count,
                    self.display_type
                );
            }
            1 => {
                formated_str = format!(
                    r#"{{ "trait_type": {} , "value" : "{}" , "max_value" : {}, "trait_count" : {} }}"#,
                    self.trait_type, self.value, self.max_value, self.max_value
                );
            }
            2 => {
                formated_str = format!(
                    r#"{{ "trait_type": {} , "value" : "{}" , "display_type" : {} }}"#,
                    self.trait_type, self.value, self.display_type
                );
            }
            3 => {
                formated_str = format!(
                    r#"{{ "trait_type": {} , "value" : "{}" ,"trait_count" : {} }}"#,
                    self.trait_type, self.value, self.trait_count
                );
            }
            4 => {
                formated_str = format!(
                    r#"{{ "trait_type": {} , "value" : "{}" , "max_value" : {} }}"#,
                    self.trait_type, self.value, self.max_value
                );
            }
            5 => {
                formated_str = format!(
                    r#"{{ "trait_type": {} , "value" : "{}" , "max_value" : {}, "display_type" : {} }}"#,
                    self.trait_type, self.value, self.max_value, self.display_type
                );
            }
            6 => {
                formated_str = format!(
                    r#"{{ "trait_type": {} , "value" : "{}" , "trait_count" : {}, "display_type" : {} }}"#,
                    self.trait_type, self.value, self.trait_count, self.display_type
                );
            }
            _ => {
                formated_str = format!(
                    r#"{{ "trait_type": {} ,"value" : "{}" }}"#,
                    self.trait_type, self.value
                );
            }
        }

        write!(f, "{}", formated_str)
    }
}

#[cfg(test)]
mod tets {
    use serde_json::json;

    use super::*;
    #[test]
    fn test_core_metadata() {
        let mut core_metadata = Attribute::default();
        core_metadata.trait_type = "Test".to_string();
        core_metadata.value = "test_atr".to_string();
        println!("{}", core_metadata.max_value);
        println!("{}", core_metadata.trait_count);
        println!("{}", core_metadata);

        core_metadata.display_type = "Date".to_string();
        println!("Con Date :\n{}", core_metadata);
        core_metadata.max_value = 4;
        println!("Con Date & Max Value:\n{}", core_metadata);
        core_metadata.trait_count = 22;
        println!("Con Todo\n{}", core_metadata);
        core_metadata.display_type = "".to_string();
        println!("Con MaxValue & Count\n{}", core_metadata);
        core_metadata.max_value = 0;
        println!("Con Count\n{}", core_metadata);
        let atr = Attributes::new(1);
        println!("{}", atr);
        // json!(core_metadata);
    }
}

#[derive(Default, Clone)]
/// `type` is a reserved wors so we use `tipo` as is the translation in spanish
pub struct File {
    uri: String,
    tipo: String,
    cdn: bool,
}
#[derive(Default, Clone)]
pub struct Creator {
    address: String,
    share: usize,
}

pub type Creators = Vec<Creator>;
pub type Files = Vec<File>;

#[derive(Clone)]
pub struct Properties {
    files: Files,
    category: String,
    creators: Creators,
}

impl std::fmt::Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fmt_file: String;
        if self.cdn {
            fmt_file = format!(
                r#"{{ "uri": {}, "type" : {} , "cdn" : {} }}"#,
                &self.uri, &self.tipo, &self.cdn
            );
        } else {
            fmt_file = format!(r#"{{ "uri": {}, "type" : {} }}"#, &self.uri, &self.tipo);
        }
        write!(f, "{}", fmt_file)
    }
}
impl std::fmt::Display for Creator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fmt_creator = format!(
            r#"{{ "address": {}, "share" : {} }}"#,
            &self.address, &self.share
        );
        write!(f, "{}", fmt_creator)
    }
}

impl Properties {
    fn fmt_properties(&self) -> String {
        let files_len = &self.files.len();
        let mut fmted_files: String = String::new();
        if files_len != &0 {
            if files_len == &1 {
                fmted_files = format!(r#" "files": [ {} ]"#, &self.files.get(0).unwrap());
            } else {
                fmted_files = format!(
                    r#" "files": [ \n{}\n ,"#,
                    &self
                        .files
                        .iter()
                        .enumerate()
                        .map(|(index, linea)| {
                            if index == files_len - 1 {
                                format!("  {}\n]", linea)
                            } else {
                                format!("  {},\n", linea)
                            }
                        })
                        .collect::<String>()
                );
            }
        }
        fmted_files
    }

    fn fmt_creators(&self) -> String {
        let creators_len = &self.creators.len();
        let mut fmted_creators: String = String::new();
        if creators_len != &0 {
            if creators_len == &1 {
                fmted_creators = format!(r#" "creators": [ {} ]"#, &self.creators.get(0).unwrap());
            } else {
                fmted_creators = format!(
                    r#" "creators": [ \n{}\n ,"#,
                    &self
                        .creators
                        .iter()
                        .enumerate()
                        .map(|(index, linea)| {
                            if index == creators_len - 1 {
                                format!("  {}\n]", linea)
                            } else {
                                format!("  {},\n", linea)
                            }
                        })
                        .collect::<String>()
                );
            }
        }
        fmted_creators
    }

    pub fn new(am_files: usize, am_creators: usize) -> Self {
        let mut empty_files = Files::new();
        let mut empty_creators = Creators::new();
        match am_files {
            0 => {}
            1 => {
                empty_files.push(File::default());
            }
            _ => {
                let def = File::default();
                for _x in 0..am_files {
                    empty_files.push(def.clone());
                }
            }
        }
        match am_creators {
            0 => {}
            1 => {
                empty_creators.push(Creator::default());
            }
            _ => {
                let def = Creator::default();
                for _x in 0..am_creators {
                    empty_creators.push(def.clone());
                }
            }
        }

        Self {
            files: empty_files,
            category: String::new(),
            creators: empty_creators,
        }
    }
}

impl std::fmt::Display for Properties {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fmted_files = Self::fmt_properties(&self);

        let fmted_creators = Self::fmt_creators(&self);
        let mut fmt_properties: String = String::new();
        if self.category.is_empty() {
            if fmted_files != "" && fmted_creators != "" {
                fmt_properties = format!(
                    r#" "properties" : {{ {}, {} }} "#,
                    fmted_files, fmted_creators
                );
            } else if fmted_files != "" {
                fmt_properties = format!(r#" "properties" : {{ {} }} "#, fmted_files);
            } else if fmted_creators != "" {
                fmt_properties = format!(r#" "properties" : {{ {} }} "#, fmted_creators);
            } else {
                fmt_properties = format!("");
            }
        } else {
            if fmted_files != "" && fmted_creators != "" {
                fmt_properties = format!(
                    r#" "properties" : {{ {}, "category" : {}, {} }} "#,
                    fmted_files, &self.category, fmted_creators
                );
            } else if fmted_files != "" {
                fmt_properties = format!(
                    r#" "properties" : {{ {}, "category" : {} }} "#,
                    fmted_files, &self.category
                );
            } else if fmted_creators != "" {
                fmt_properties = format!(
                    r#" "properties" : {{ "category" : {}, {} }} "#,
                    &self.category, fmted_creators
                );
            } else {
                format!(r#" "properties" : {{ "category" : {} }} "#, &self.category);
            }
        }
        write!(f, "{}", fmt_properties)
    }
}

#[cfg(test)]
mod tets {
    // cargo test --package metadata-builder-sol --bin metadata-builder-sol -- metadata_lib::properties::tets::test_properties_metadata --exact --nocapture
    use super::*;
    #[test]
    fn test_properties_metadata() {
        let mut properties_metadata = Properties::new(1, 0);
        let mtdt2 = Properties::new(1, 1);
        println!("{}", properties_metadata);
        
        properties_metadata.category = "film".to_string();
        
        println!("{}", properties_metadata);
        println!("{}", mtdt2);
        
        // json!(core_metadata);
    }
}

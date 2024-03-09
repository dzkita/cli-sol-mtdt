use proc_macro2::TokenStream;
use quote::quote;

pub fn gen_core_logic() -> TokenStream {
    quote! {
        // Código específico para #[mtdt].
        // Puedes agregar aquí la lógica para la sección core.
        // Código específico para #[mtdt].
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

        #[derive(Default)]
        pub struct Collection {
            name: String,
            family: String,
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
        impl std::fmt::Display for Collection {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let fmt_coll = format!(
                    r#" "collection": {{ "name" : {}, "family" : {} }}"#,
                    &self.name, &self.family
                );

                write!(f, "{}", fmt_coll)
            }
        }
    }
}

pub fn gen_properties_logic() -> TokenStream {
    quote! {
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

        }
}

pub fn gen_attrs_logic() -> TokenStream {
    quote! {
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

    }
}

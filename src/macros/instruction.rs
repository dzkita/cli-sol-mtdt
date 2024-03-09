use syn::{Item, ItemStruct, Attribute,Meta, meta::{ParseNestedMeta,parser}, parse_quote};
use proc_macro2::TokenStream;
use quote::quote;
pub fn generate_metadata(attrs: &[Attribute]) -> TokenStream {
    // Parseamos los argumentos de las instrucciones.
    let (generate_core, generate_attributes) = parse_metadata_instructions(attrs);

    // Generamos la metadata según las instrucciones.
    let generated_code = quote! {
        // Código común que deseas generar para todos los structs.

        // Verificación de la instrucción #[mtdt(full)] con argumentos.
        mod metadata {
            // Código específico para #[mtdt].
            pub fn generate() {
                // Implementación específica para #[mtdt].
                // Puedes utilizar los argumentos aquí, por ejemplo: #(#metadata_args)*
                #generate_core
                #generate_attributes
            }
        }

        // Resto del código que deseas generar para todos los structs.
    };

    generated_code
}

fn parse_metadata_instructions(attrs: &[Attribute]) -> (TokenStream, TokenStream) {
    // Variables para controlar qué partes de la metadata se deben generar.
    let mut generate_core = quote! {};
    let mut generate_attributes = quote! {};

    // Iteramos sobre los atributos para identificar las instrucciones.
    for attr in attrs {
        //  #[mtdt(kind = "EarlGrey")]
        if attr.path().is_ident("mtdt") {         // this parses the `tea`
            attr.parse_nested_meta(|meta| {      // this parses the `(`
                if meta.path.is_ident("full") {  // this parses the `full`
                    // // #[mtdt(full)]
                    // generate_core = quote! {
                    //     // Código específico para #[mtdt(full)].
                    //     // Puedes agregar aquí la lógica para la sección core.
                    // };
                    // #[mtdt(full)]
                    generate_core = quote! {
                        // Código específico para #[mtdt(full)].
                        // Puedes agregar aquí la lógica para la sección core.
                    };
                    generate_attributes = quote! {
                        // Código específico para #[mtdt(full)].
                        // Puedes agregar aquí la lógica para la sección attributes.
                    };
                    
                    Ok(())
                }else if meta.path.is_ident(""){
                    Ok(())
                }else if meta.path.is_ident("") {
                    Ok(())
                
                    // generate_attributes = quote! {
                    //     // Código específico para #[mtdt(full)].
                    //     // Puedes agregar aquí la lógica para la sección attributes.
                    // };

                    // Some("core") => {
                    //     generate_core = quote! {
                    //         // Código específico para #[mtdt(core)].
                    //         // Puedes agregar aquí la lógica para la sección core.
                    //     };
                    // }
                    // Some("atrs") => {
                    //     generate_attributes = quote! {
                    //         // Código específico para #[mtdt(atrs)].
                    //         // Puedes agregar aquí la lógica para la sección attributes.
                    //     };
                    // }
                } else {
                    // #[mtdt]
                    generate_core = quote! {
                        // Código específico para #[mtdt].
                        // Puedes agregar aquí la lógica para la sección core.
                    };
                    Ok(())
                }
            })?;
        }
            if let Some(syn::Meta::List(meta)) = meta_list.nested.first() {
                match meta {
                    Meta::Path(path) => {
                        match path.get_ident().map(|ident| ident.to_string().as_str()) {
                            Some("mtdt") => {
                                // #[mtdt]
                                generate_core = quote! {
                                    // Código específico para #[mtdt].
                                    // Puedes agregar aquí la lógica para la sección core.
                                };
                            }
                            Some("mtdt") if meta_list.nested.len() == 1 => {
                                // #[mtdt(full)]
                                generate_core = quote! {
                                    // Código específico para #[mtdt(full)].
                                    // Puedes agregar aquí la lógica para la sección core.
                                };
                                generate_attributes = quote! {
                                    // Código específico para #[mtdt(full)].
                                    // Puedes agregar aquí la lógica para la sección attributes.
                                };
                            }
                            Some("mtdt") if meta_list.nested.len() == 2 => {
                                // #[mtdt(core,atrs)]
                                for nested_meta in &meta_list.nested {
                                    if let syn::NestedMeta::Meta(nested_meta) = nested_meta {
                                        match nested_meta {
                                            Meta::Path(nested_path) => {
                                                match nested_path.get_ident().map(|ident| ident.to_string().as_str()) {
                                                    Some("core") => {
                                                        generate_core = quote! {
                                                            // Código específico para #[mtdt(core)].
                                                            // Puedes agregar aquí la lógica para la sección core.
                                                        };
                                                    }
                                                    Some("atrs") => {
                                                        generate_attributes = quote! {
                                                            // Código específico para #[mtdt(atrs)].
                                                            // Puedes agregar aquí la lógica para la sección attributes.
                                                        };
                                                    }
                                                    _ => {
                                                        // Manejar otros casos si es necesario.
                                                    }
                                                }
                                            }
                                            _ => {
                                                // Manejar otros casos si es necesario.
                                            }
                                        }
                                    }
                                }
                            }
                            _ => {
                                // Manejar otros casos si es necesario.
                            }
                        }
                    }
                    _ => {
                        // Manejar otros casos si es necesario.
                    }
                }
            }
        }
    }

    (generate_core, generate_attributes)
}
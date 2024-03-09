use super::gen_logic::{gen_attrs_logic, gen_core_logic, gen_properties_logic};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    meta::{parser, ParseNestedMeta},
    parse_quote, Attribute, Item, ItemStruct, LitStr, Meta,
};

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
    let mut generate_properties = quote! {};

    // Iteramos sobre los atributos para identificar las instrucciones.
    for attr in attrs {
        //  #[mtdt(kind = "EarlGrey")]
        if attr.path().is_ident("mtdt") {
            // this parses the `tea`
            attr.parse_nested_meta(|meta| {
                // this parses the `(`
                if meta.path.is_ident("full") {
                    // this parses the `full`
                    // // #[mtdt(full)]
                    // generate_core = quote! {
                    //     // Código específico para #[mtdt(full)].
                    //     // Puedes agregar aquí la lógica para la sección core.
                    // };
                    // #[mtdt(full)]
                    generate_core = gen_core_logic();
                    // Código específico para #[mtdt(full)].
                    // Puedes agregar aquí la lógica para la sección attributes.
                    generate_attributes = gen_attrs_logic();

                    // Código específico para #[mtdt(full)].
                    // Puedes agregar aquí la lógica para la sección attributes.
                    generate_properties = gen_properties_logic();
                    Ok(())
                } else if meta.path.is_ident("core") {
                    // Este genera los casos :
                    // #[mtdt(core)]
                    // #[mtdt(core,atrs)]
                    // #[mtdt(core,properties)]
                    let value = meta.value()?; // this parses the `=`
                    let s: LitStr = value.parse()?;
                    if s.value() == "atrs" {
                        generate_attributes = gen_attrs_logic();
                    } else if s.value() == "properites" {
                        generate_properties = gen_properties_logic();
                    }
                    generate_core = gen_core_logic();

                    Ok(())
                } else if meta.path.is_ident("atrs") {
                    // Este genera los casos :
                    // #[mtdt(atrs)]
                    // #[mtdt(atrs,core)]
                    // #[mtdt(atrs,properties)]
                    let value = meta.value()?; // this parses the `=`
                    let s: LitStr = value.parse()?;
                    if s.value() == "core" {
                        generate_core = gen_core_logic();
                    } else if s.value() == "properites" {
                        generate_properties = gen_properties_logic();
                    }
                    generate_attributes = gen_attrs_logic();

                    Ok(())
                } else if meta.path.is_ident("properites") {
                    // Este genera los casos :
                    // #[mtdt(properites)]
                    // #[mtdt(properites,core)]
                    // #[mtdt(properites,atrs)]
                    let value = meta.value()?; // this parses the `=`
                    let s: LitStr = value.parse()?;
                    if s.value() == "core" {
                        generate_core = gen_core_logic();
                    } else if s.value() == "atrs" {
                        generate_attributes = gen_attrs_logic();
                    }
                    generate_properties = gen_properties_logic();

                    Ok(())
                } else {
                    // #[mtdt] : CoreMetadata, Collection
                    generate_core = gen_core_logic();
                    Ok(())
                }
            })
            .unwrap();
        }
    }
    (generate_core, generate_attributes)
}

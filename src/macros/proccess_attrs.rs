use crate::macros::gen_logic::{gen_attrs_logic, gen_core_logic, gen_properties_logic};
use proc_macro2::TokenStream;

use quote::quote;
use syn::{Attribute, LitStr};

pub fn proccess_attr(attr: &Attribute) -> TokenStream {
    let mut proccessed_code = quote! {};
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
            proccessed_code = quote! {
                // #[mtdt(full)]
                // generate_core = gen_core_logic();
                gen_core_logic();
                // Código específico para #[mtdt(full)].
                // Puedes agregar aquí la lógica para la sección attributes.
                // generate_attributes = gen_attrs_logic();
                gen_attrs_logic();

                // Código específico para #[mtdt(full)].
                // Puedes agregar aquí la lógica para la sección attributes.
                // generate_properties = gen_properties_logic();
                gen_properties_logic();

            };
            Ok(())
        } else if meta.path.is_ident("core") {
            // Este genera los casos :
            // #[mtdt(core)]
            // #[mtdt(core,atrs)]
            // #[mtdt(core,properties)]
            let value = meta.value()?; // this parses the `=`
            let s: LitStr = value.parse()?;
            if s.value() == "atrs" {
                proccessed_code = quote! {
                    gen_attrs_logic();
                    gen_core_logic();
                };
            } else if s.value() == "properites" {
                proccessed_code = quote! {
                    gen_properties_logic();
                    gen_core_logic();
                };
            } else {
                proccessed_code = gen_core_logic();
            }
            Ok(())
        } else if meta.path.is_ident("atrs") {
            // Este genera los casos :
            // #[mtdt(atrs)]
            // #[mtdt(atrs,core)]
            // #[mtdt(atrs,properties)]
            let value = meta.value()?; // this parses the `=`
            let s: LitStr = value.parse()?;
            if s.value() == "core" {
                proccessed_code = quote! {
                    gen_core_logic();
                    gen_attrs_logic();
                };
            } else if s.value() == "properites" {
                proccessed_code = quote! {
                    gen_attrs_logic();
                    gen_properties_logic();
                };
            } else {
                proccessed_code = gen_attrs_logic();
            }
            Ok(())
        } else if meta.path.is_ident("properites") {
            // Este genera los casos :
            // #[mtdt(properites)]
            // #[mtdt(properites,core)]
            // #[mtdt(properites,atrs)]
            let value = meta.value()?; // this parses the `=`
            let s: LitStr = value.parse()?;
            if s.value() == "core" {
                proccessed_code = quote! {
                    gen_core_logic();
                    gen_properties_logic();
                };
            } else if s.value() == "atrs" {
                proccessed_code = quote! {
                    gen_attrs_logic();
                    gen_properties_logic();
                };
            } else {
                proccessed_code = gen_properties_logic();
            }

            Ok(())
        } else {
            // #[mtdt] : CoreMetadata, Collection
            proccessed_code = gen_core_logic();
            Ok(())
        }
    })
    .unwrap();
    proccessed_code
}

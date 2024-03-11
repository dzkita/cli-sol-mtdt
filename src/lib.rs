extern crate proc_macro;
mod macros;
mod mtdt_attributes;
use macros::{proccess_attrs::proccess_attr};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

/// Ahora quiero agregarle una forma de interprear la seccion instruction:
///#[mtdt] : esta se guardaria solo para los tokens comunes (solo la parte CoreMetadata)
/// #[mtdt(full)] : pide implementar todos los parametros y arma la cuenta como default al maximo espacio
/// ----------------------------------------
///  Tambien permite seleccionar a los usarios las propiedades que necesiten, por ejemplo :
/// #[mtdt(core,atrs)] : pide implementar SOLO cuentas para `CoreMetadata` && para `Attributes`
/// ----------------------------------------
/// Otra particularidad es que va a permitir limitar el espacio que ocupe la cuenta, esto esta bueno porque el espacio cuesta dinero y el dinero hay que cuidarlo pai
/// #[mtdt(core == <cant_bytes>)] : esto es lo mismo que el caso anterior solo que el usuario regula el espacio de las cuentas para cada parametro. Tengamos en cuenta que esto es mas complejo pero es mas barato porque no hay espacio al pedo

#[proc_macro_derive(Mtdt, attributes(mtdt))]
pub fn mtdt_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as ItemStruct);
    let struct_name = &input.ident;
    let fields = &input.fields;
    let attributes = &input.attrs;

    if let Some(impl_tokens) = generate_impl(struct_name, fields, attributes) {
        let result = quote::quote! {
            #input
            #impl_tokens
        };
        return result.into();
    } else {
        // No hay código generado.
        return proc_macro::TokenStream::new();
    }
}

use syn::{parse_quote, Attribute, Fields, LitStr};

fn generate_impl(
    struct_name: &syn::Ident,
    fields: &Fields,
    attributes: &[Attribute],
) -> Option<proc_macro2::TokenStream> {
    // Verificar si el struct tiene el atributo #[mtdt].
    let has_mtdt_attr = attributes.iter().any(|attr| attr.path().is_ident("mtdt"));

    if has_mtdt_attr {
        // Aquí puedes realizar la lógica específica para el atributo #[mtdt].
        // Puedes acceder a los atributos del struct y realizar implementaciones personalizadas.

        // aca se tiene que fijar cual/ cuales son los atributes que tiene 
        // Y generar la logica 
        // Y despues segun lo generado, implementar el trait `OnchainMetadata` 
        let struct_impl = quote::quote! {
            impl SomeTrait for #struct_name {
                fn some_method(&self) {
                    // Implementación específica para todo el struct.
                }
            }
        };

        // Puedes incluir más lógica aquí según sea necesario.

        Some(struct_impl)
    } else {
        // El struct no tiene el atributo #[mtdt], no generamos código.
        None
    }
}

// // Nuevos atributos personalizados para Mtdt.
// #[proc_macro_attribute]
// pub fn mtdt(args: TokenStream, input: TokenStream) -> TokenStream {
//     // Si no se proporcionan argumentos, asume que es equivalente a #[mtdt(full)].
//     let input = parse_macro_input!(input as ItemStruct);

//     // Obtenemos el nombre de la estructura.
//     let struct_name = &input.ident;

//     if args.is_empty() {
//         // Implementación para el caso predeterminado (equivalente a #[mtdt(full)]).
//         let expanded = quote! {
//             // Accede a opciones predeterminadas para #[mtdt(full)].
//             // ...

//             // Implementación específica bajo #[mtdt(full)].
//             // ...

//             // Aquí puedes incluir el uso de #[derive(Mtdt)] si es necesario.
//             #[derive(Mtdt)]
//             struct #struct_name {
//                 gen_core_logic(),
//                 gen_attrs_logic(),
//                 gen_properties_logic(),
//             }

//         };

//         return TokenStream::from(expanded);
//     }
//     else{
//         let op =quote!();
//         return TokenStream::from(op);
//     }

//     // // Parsea los argumentos para obtener opciones específicas.
//     // let options = syn::parse_macro_input!(args as syn::AttributeArgs);

//     // // Ahora, puedes procesar options para manejar diferentes opciones.
//     // let mut full_options = None;
//     // for option in options {
//     //     if let syn::NestedMeta::Meta(meta) = option {
//     //         match meta {
//     //             syn::Meta::NameValue(nv) if nv.path.is_ident("full") => {
//     //                 if let syn::Lit::Str(lit) = nv.lit {
//     //                     full_options = Some(lit.value());
//     //                 }
//     //             }
//     //             // Añade más casos para manejar otros atributos si es necesario.
//     //             _ => {}
//     //         }
//     //     }
//     // }

//     // // Procesa input según las necesidades específicas de #[mtdt].
//     // let expanded = quote! {
//     //     // Implementación específica bajo #[mtdt].
//     //     // Accede a full_options para obtener las opciones específicas de #[mtdt(full)].
//     //     // ...

//     //     // También puedes procesar otras opciones según sea necesario.
//     //     // ...

//     //     // Ahora, aquí puedes incluir el uso de #[derive(Mtdt)].
//     //     // Asegúrate de ajustar esto según la estructura real de tu implementación.
//     //     #[derive(Mtdt)]
//     //     #input
//     // };

//     // TokenStream::from(expanded)
// }

// // Nuevos atributos específicos bajo #[mtdt].
// #[proc_macro_attribute]
// pub fn mtdt_full(args: TokenStream, input: TokenStream) -> TokenStream {
//     // Implementación del atributo #[mtdt(full)]...
//     // Accede a args para obtener las opciones y realiza las acciones necesarias.
//     // ...

//     // Procesa input según las necesidades específicas de #[mtdt(full)].
//     let expanded = quote! {
//         // Implementación específica para #[mtdt(full)].
//         // ...
//     };

//     TokenStream::from(expanded)
// }

#[proc_macro_attribute]
pub fn mtdt_core(args: TokenStream, input: TokenStream) -> TokenStream {
    // Implementación del atributo #[mtdt(otra_opcion)]...
    // Accede a args para obtener las opciones y realiza las acciones necesarias.
    // ...

    // Procesa input según las necesidades específicas de #[mtdt(otra_opcion)].
    let expanded = quote! {
        // Implementación específica para #[mtdt(otra_opcion)].
        // ...
    };

    TokenStream::from(expanded)
}

// /// Ahora quiero generar los siguientes escenarios
// #[proc_macro]
// pub fn instruction(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
//     // Implementación de tu macro para la anotación `instruction`.
//     // Puedes usar `parse_macro_input!(input as SomeType)` para analizar la entrada.
//     // Aquí deberías procesar la información relevante de la sección `instruction`.
//     // Luego, puedes generar el código necesario y devolverlo como un nuevo `TokenStream`.
//     // Por ejemplo:
//     // let output = quote! { /* tu código generado */ };
//     // output.into()
//     let expanded = quote::quote! {
//         // Aquí colocarías el código generado.
//         // Puedes usar `name` para referenciar el nombre de la estructura o enum.
//         // Ejemplo: println!("Deriving Mtdt for {}", stringify!(#name));
//     };
//     expanded.into()
// }

// #[proc_macro]
// pub fn account(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
//     // Implementación de tu macro para la anotación `instruction`.
//     // Puedes usar `parse_macro_input!(input as SomeType)` para analizar la entrada.
//     // Aquí deberías procesar la información relevante de la sección `instruction`.
//     // Luego, puedes generar el código necesario y devolverlo como un nuevo `TokenStream`.
//     // Por ejemplo:
//     // let output = quote! { /* tu código generado */ };
//     // output.into()
//     let expanded = quote::quote! {
//         // Aquí colocarías el código generado.
//         // Puedes usar `name` para referenciar el nombre de la estructura o enum.
//         // Ejemplo: println!("Deriving Mtdt for {}", stringify!(#name));
//     };
//     expanded.into()
// }

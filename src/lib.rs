extern crate proc_macro;
mod metadata_lib;


use proc_macro::TokenStream;
use quote::{self,ToTokens};
use syn::{parse_macro_input, Attribute,DeriveInput};


// #[proc_macro_derive(Mtdt, attributes(account, instruction))]
// pub fn derive_anchor_deserialize(item: TokenStream) -> TokenStream {
//     parse_macro_input!(item as anchor_lib::anchor_syn::AccountsStruct)
//         .to_token_stream()
//         .into()
// }


/// Ahora quiero agregarle una forma de interprear la seccion instruction:
///#[mtdt] : esta se guardaria solo para los tokens comunes (solo la parte CoreMetadata)
/// #[mtdt(full)] : pide implementar todos los parametros y arma la cuenta como default al maximo espacio 
/// ----------------------------------------
///  Tambien permite seleccionar a los usarios las propiedades que necesiten, por ejemplo :
/// #[mtdt(core,atrs)] : pide implementar SOLO cuentas para `CoreMetadata` && para `Attributes`
/// ----------------------------------------
/// Otra particularidad es que va a permitir limitar el espacio que ocupe la cuenta, esto esta bueno porque el espacio cuesta dinero y el dinero hay que cuidarlo pai
/// #[mtdt(core == <cant_bytes>)] : esto es lo mismo que el caso anterior solo que el usuario regula el espacio de las cuentas para cada parametro. Tengamos en cuenta que esto es mas complejo pero es mas barato porque no hay espacio al pedo

#[proc_macro_derive(Mtdt, attributes(account, instruction))]
pub fn derive_onchain_mtdt(input: TokenStream) -> TokenStream {
    // Parseamos la entrada como una estructura o enum que estamos derivando.
    let input = parse_macro_input!(input as DeriveInput);

    // Extraemos el nombre de la estructura o enum.
    let name = &input.ident;

    // Aquí puedes generar el código que desees para la derivación.
    let expanded = quote::quote! {
        // Aquí colocarías el código generado.
        // Puedes usar `name` para referenciar el nombre de la estructura o enum.
        // Ejemplo: println!("Deriving Mtdt for {}", stringify!(#name));
    };

    // Devolvemos el código generado como un TokenStream.
    expanded.into()
}



#[proc_macro]
pub fn instruction(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as Item);
    let expanded = generate_code(input);
    expanded.into()
}

// use syn::{Item, ItemEnum, ItemStruct};

fn generate_code(item: Item) -> proc_macro2::TokenStream {
    match item {
        Item::Struct(s) => generate_struct_code(&s),
        Item::Enum(e) => generate_enum_code(&e),
        _ => {
            // Manejar otros casos o generar un error si es necesario.
            quote! {
                compile_error!("Unsupported item type. Only structs and enums are supported.");
            }
        }
    }
}

fn generate_struct_code(s: &ItemStruct) -> proc_macro2::TokenStream {
    // Implementa la lógica para generar código para structs.
    // ...
}

fn generate_enum_code(e: &ItemEnum) -> proc_macro2::TokenStream {
    // Implementa la lógica para generar código para enums.
    // ...
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
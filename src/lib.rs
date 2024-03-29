extern crate proc_macro;
mod macros;
use macros::instruction::generate_metadata;
use proc_macro::TokenStream;

use syn::ItemStruct;
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

#[proc_macro_derive(Mtdt, attributes(instruction))]
pub fn derive_onchain_mtdt(input: TokenStream) -> TokenStream {
    // Parseamos la entrada como una estructura o enum que estamos derivando.
    // let input = parse_macro_input!(input as DeriveInput);

    // Extraemos el nombre de la estructura o enum.
    // let name = &input.ident;

    // // Aquí puedes generar el código que desees para la derivación.
    // let expanded = quote::quote! {
    //     // Aquí colocarías el código generado.
    //     // Puedes usar `name` para referenciar el nombre de la estructura o enum.
    //     // Ejemplo: println!("Deriving Mtdt for {}", stringify!(#name));
    // };

    // Aplicamos el macro instruction a la estructura o enum.
    let expanded = instruction(input);
    // Devolvemos el código generado como un TokenStream.
    expanded.into()
}

#[proc_macro]
pub fn instruction(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as ItemStruct);
    // let input = syn::parse_macro_input!(input as ItemStruct);

    // Obtener el nombre del struct.
    let struct_name = &input.ident;

    // Llamar a la función que genera la metadata.
    let expanded = generate_metadata(&input.attrs, struct_name);
    // let expanded = generate_metadata(&input.attrs );
    expanded.into()
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

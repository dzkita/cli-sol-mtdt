use proc_macro::TokenStream;
use quote::quote;

// Nuevos atributos específicos bajo #[mtdt].
#[proc_macro_attribute]
pub fn mtdt_full(args: TokenStream, input: TokenStream) -> TokenStream {
    // Implementación del atributo #[mtdt(full)]...
    // Accede a args para obtener las opciones y realiza las acciones necesarias.
    // ...

    // Procesa input según las necesidades específicas de #[mtdt(full)].
    let expanded = quote! {
        // Implementación específica para #[mtdt(full)].
        // ...
    };

    TokenStream::from(expanded)
}


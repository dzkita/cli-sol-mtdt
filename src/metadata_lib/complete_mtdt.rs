use super::{
    attributes::{Attribute, Attributes},
    collection::Collection,
    core::CoreMetadata,
    image,
    properties::{Creator, Creators, File, Files, Properties},
};



/// Cosas a gestionar aca :
/// 1. Crear una implementacion para el display
/// 2. Crear una implementacion para crear una metadata
/// 3. Estarioa bueno poder armar un macro para que se pueda implementar de forma opciona como hace el parametro `account` en Anchor, de esta forma se podria usar :
/// 
/// #[mtdt] : esta se guardaria solo para los tokens comunes (solo la parte CoreMetadata)
/// #[mtdt(full)] : pide implementar todos los parametros y arma la cuenta como default al maximo espacio 
/// ----------------------------------------
///  Tambien permite seleccionar a los usarios las propiedades que necesiten, por ejemplo :
/// #[mtdt(core,atrs)] : pide implementar SOLO cuentas para `CoreMetadata` && para `Attributes`
/// ----------------------------------------
/// Otra particularidad es que va a permitir limitar el espacio que ocupe la cuenta, esto esta bueno porque el espacio cuesta dinero y el dinero hay que cuidarlo pai
/// #[mtdt(core == <cant_bytes>)] : esto es lo mismo que el caso anterior solo que el usuario regula el espacio de las cuentas para cada parametro. Tengamos en cuenta que esto es mas complejo pero es mas barato porque no hay espacio al pedo
/// ----------------------------------------
/// #[mtdt('nft-img-onchain')] : esto abre la opcion de que la imagen se renderee dentro del programa de rendereo, aca van a haber `assets` y tiene que existir un orden de `apilamiento`
/// ----------------------------------------
/// 
/// 
/// 
/// Diagrama de las cuentas :
///
///   id : mtdt_id -> properties | acc(properties)
///                -> attributes | acc(atrs)
///                -> core       | Capaz core & collection
///                -> collection | pueden compartir cuenta
/// ----------------------------------------
/// En `anchor` se veria algo asi :
/// 
/// #[derive(Accounts)]
/// pub struct TokenMetadata {
///     pub token_account:<Account>,
///     #[mtdt]
///     pub mtdt:Account<Metadata>,
///     pub name: String,// esto lo requiere el atributo `mtdt`
///     pub symbol: String,// esto lo requiere el atributo `mtdt`
///     pub description: String,// esto lo requiere el atributo `mtdt`
///     pub seller_fee_basis_points: usize,// esto lo requiere el atributo `mtdt`
///     pub image: String,// esto lo requiere el atributo `mtdt`
///     pub animation_url: String,// esto lo requiere el atributo `mtdt`
///     pub external_url: String,// esto lo requiere el atributo `mtdt`
/// }
/// 
/// ----------------------------------------
/// Otra particularidad es que tiene que existir la capacidad de `migrar` cuentas, por ejemplo, una cuenta tiene que ser capaz de achicarse y agrandarse segun lo necesite. Un ejemplo puede ser si se decide borrar las propiedades (esto tiene que tener un control de acceso) o si se quiere añadir alguna otra, por lo tanto esto tiene que ser 

pub struct FullMetadata {
    properties: Properties,
    attributes: Attributes,
    
    core:CoreMetadata,
    collection:Collection
}
// #[derive(Default)]
// pub struct CoreMetadata {
//     name: String,
//     symbol: String,
//     description: String,
//     seller_fee_basis_points: usize,
//     image: String,
//     animation_url: String,
//     external_url: String,
// }
// #[derive(Default, Clone)]
// pub struct Attribute {
//     trait_type: String,
//     value: String,
//     display_type: String,
//     trait_count: u64,
//     max_value: u64,
// }

// pub struct Attributes {
//     atrs: Vec<Attribute>,
// }

// #[derive(Default)]
// pub struct Collection {
//     name: String,
//     family: String,
// }

// #[derive(Default, Clone)]
// /// `type` is a reserved wors so we use `tipo` as is the translation in spanish
// pub struct File {
//     uri: String,
//     tipo: String,
//     cdn: bool,
// }
// #[derive(Default, Clone)]
// pub struct Creator {
//     address: String,
//     share: usize,
// }

// pub type Creators = Vec<Creator>;
// pub type Files = Vec<File>;

// #[derive(Clone)]
// pub struct Properties {
//     files: Files,
//     category: String,
//     creators: Creators,
// }
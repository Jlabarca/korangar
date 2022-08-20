use proc_macro::TokenStream as InterfaceTokenStream;
use syn::{ Ident, DataStruct, Generics, Attribute };
use quote::quote;

use super::helper::prototype_element_helper;

pub fn derive_prototype_element_struct(data_struct: DataStruct, generics: Generics, attributes: Vec<Attribute>, name: Ident) -> InterfaceTokenStream {

    let (initializers, _window_title, _window_class) = prototype_element_helper(data_struct, attributes);
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl #impl_generics crate::interface::traits::PrototypeElement for #name #type_generics #where_clause {
            fn to_element(&self, display: String) -> crate::interface::types::ElementCell {
                let elements: Vec<crate::interface::types::ElementCell> = vec![#(#initializers),*];
                std::rc::Rc::new(std::cell::RefCell::new(crate::interface::elements::Expandable::new(display, elements, false)))
            }
        }
    }.into()
}
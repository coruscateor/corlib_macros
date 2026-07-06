#![doc = include_str!("../README.md")] 

#![cfg_attr(docsrs, feature(doc_cfg))]

use quote::quote;

use syn::{DeriveInput, parse_macro_input};

///
/// A derive macro for the corlib::any::AsAnyRef trait.
/// 
/// The aforementioned trait is required to be in scope for this derive macro to be used.
/// 
#[proc_macro_derive(AsAnyRef)]
pub fn as_any_ref(input: proc_macro::TokenStream) -> proc_macro::TokenStream
{

    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let as_any_ref_impl = quote!
    {

        impl #impl_generics corlib::any::AsAnyRef for #name #ty_generics #where_clause
        {

            fn as_any_ref(&self) -> &dyn core::any::Any
            {

                self
                
            }

        }

    };

    as_any_ref_impl.into()

}

///
/// A derive macro for the corlib::any::AsAnyMut trait.
///
/// The aforementioned trait as well as the corlib::any::AsAnyRef trait are required to be in scope for this derive macro to be used.
/// 
#[proc_macro_derive(AsAnyMut)]
pub fn as_any_mut(input: proc_macro::TokenStream) -> proc_macro::TokenStream
{

    //let cloned_input = input.clone();

    //let as_any_ref_res: proc_macro::TokenStream = as_any_ref(cloned_input); //crate::

    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let as_any_mut_impl = quote!
    {

        impl #impl_generics corlib::any::AsAnyRef for #name #ty_generics #where_clause
        {

            fn as_any_ref(&self) -> &dyn core::any::Any
            {

                self
                
            }

        }

        impl #impl_generics corlib::any::AsAnyMut for #name #ty_generics #where_clause
        {

            fn as_any_mut(&mut self) -> &mut dyn core::any::Any
            {

                self
                
            }

        }

    };

    //as_any_ref_res.

    as_any_mut_impl.into()

}

///
/// A derive macro for the corlib::rc::RcDefault trait.
/// 
/// The aforementioned trait is required to be in scope for this derive macro to be used.
/// 
/// Also Self needs to have the core::default::Default trait implemented or otherwise have a static default method returning Self.
/// 
#[proc_macro_derive(RcDefault)]
pub fn rc_default(input: proc_macro::TokenStream) -> proc_macro::TokenStream
{

    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let rc_default_impl = quote!
    {

        impl #impl_generics corlib::rc::RcDefault for #name #ty_generics #where_clause
        {

            fn rc_default() -> std::rc::Rc<Self>
            {

                let value = Self::default();

                std::rc::Rc::new(value)
                
            }

        }

    };

    rc_default_impl.into()

}

///
/// A derive macro for the corlib::rc::ArcDefault trait.
/// 
/// The aforementioned trait is required to be in scope for this derive macro to be used.
/// 
/// Also Self needs to have the core::default::Default trait implemented or otherwise have a static default method returning Self.
/// 
#[proc_macro_derive(ArcDefault)]
pub fn arc_default(input: proc_macro::TokenStream) -> proc_macro::TokenStream
{

    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let arc_default_impl = quote!
    {

        impl #impl_generics corlib::rc::ArcDefault for #name #ty_generics #where_clause
        {

            fn arc_default() -> std::sync::Arc<Self>
            {

                let value = Self::default();

                std::sync::Arc::new(value)
                
            }

        }

    };

    arc_default_impl.into()

}

///
/// A derive macro for the corlib::WeakSelf trait.
/// 
/// The aforementioned trait is required to be in scope for this derive macro to be used.
/// 
/// This macro must decorate a struct and that struct must have a field named “weak_self” which is of the type: std::rc::Weak<Self>.
/// 
#[proc_macro_derive(WeakSelf)]
pub fn weak_self(input: proc_macro::TokenStream) -> proc_macro::TokenStream
{

    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let weak_self_impl = quote!
    {

        impl #impl_generics corlib::WeakSelf for #name #ty_generics #where_clause
        {

            fn weak_self(&self) -> std::rc::Weak<Self>
            {

                self.weak_self.clone()
                
            }

            fn weak_self_ref(&self) -> &std::rc::Weak<Self>
            {

                &self.weak_self
                
            }

        }

    };

    weak_self_impl.into()

}

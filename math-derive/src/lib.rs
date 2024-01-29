use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{ext::IdentExt, parse_macro_input, DeriveInput, Fields};

#[proc_macro_derive(Vector)]
pub fn vector_derive(input: TokenStream) -> TokenStream {
    let input = &parse_macro_input!(input as DeriveInput);
    match generate(input) {
        Ok(generated) => generated,
        Err(err) => err.to_compile_error().into(),
    }
}

fn generate(derive_input: &DeriveInput) -> Result<TokenStream, syn::Error> {
    let struct_data = match &derive_input.data {
        syn::Data::Struct(v) => v,
        _ => {
            return Err(syn::Error::new_spanned(
                &derive_input.ident,
                "Must be struct type",
            ));
        }
    };

    let struct_name = &derive_input.ident;
    let (impl_generics, type_generics, where_clause) = &derive_input.generics.split_for_impl();
    let fields: Vec<_> = match &struct_data.fields {
        Fields::Named(fields) => fields.named.clone().into_iter().collect(),
        Fields::Unnamed(fields) => fields.unnamed.clone().into_iter().collect(),
        _ => panic!("Vector type must have 1 or more elements."),
    };

    let field_types = fields
        .iter()
        .cloned()
        .map(|field| field.ty)
        .collect::<Vec<_>>();
    let field_type = field_types[0].clone();
    assert!(
        field_types
            .iter()
            .skip(1)
            .cloned()
            .all(|ty| field_type == ty),
        "Vector type must have the same element type."
    );

    assert!(fields.len() >= 1);
    let dimension = fields.len();
    let field_names: Vec<_> = match &struct_data.fields {
        Fields::Named(_) => fields
            .iter()
            .cloned()
            .map(|field| format_ident!("{}", field.ident.as_ref().unwrap().unraw().to_string()))
            .collect(),
        Fields::Unnamed(_) => fields
            .iter()
            .enumerate()
            .map(|(index, _v)| format_ident!("{index}"))
            .collect(),
        _ => panic!("Vector type must have the same element type."),
    };

    let mut token_streams = Vec::new();

    let ops_settings = [
        (quote! {Add}, quote! {add}, quote! {+}),
        (quote! {Sub}, quote! {sub}, quote! {-}),
        (quote! {Mul}, quote! {mul}, quote! {*}),
        (quote! {Div}, quote! {div}, quote! {/}),
    ];

    for (ops_trait_name, ops_trait_func_name, op) in ops_settings {
        let ops_elements: Vec<proc_macro2::TokenStream> = field_names
            .iter()
            .enumerate()
            .map(|(i, name)| format!("{name}: self.{name} {op} rhs.get({i}),"))
            .map(|s| s.parse().unwrap())
            .collect();

        let assign_ops_trait_name = format_ident!("{}Assign", ops_trait_name.to_string());
        let assign_ops_trait_func_name =
            format_ident!("{}_assign", ops_trait_func_name.to_string());
        let impl_trait = quote! {
            impl #impl_generics std::ops::#ops_trait_name for #struct_name #type_generics #where_clause {
                type Output = Self;

                fn #ops_trait_func_name(self, rhs: Self) -> Self::Output {
                    Self {
                        #(#ops_elements)*
                    }
                }
            }

            impl #impl_generics std::ops::#assign_ops_trait_name for #struct_name #type_generics #where_clause {
                fn #assign_ops_trait_func_name(&mut self, rhs: Self) {
                    *self = *self #op rhs;
                }
            }
        };
        token_streams.push(impl_trait);
    }

    {
        // impl Mul<scalar>
        let ops_elements: Vec<proc_macro2::TokenStream> = field_names
            .iter()
            .map(|name| quote!(#name: self.#name * scalar,))
            .collect();

        let impl_trait = quote! {
            impl #impl_generics std::ops::Mul<#field_type> for #struct_name #type_generics #where_clause {
                type Output = Self;

                fn mul(self, scalar: #field_type) -> Self::Output {
                    Self {
                        #(#ops_elements)*
                    }
                }
            }
        };
        token_streams.push(impl_trait);
    }
    {
        // impl Div<scalar>
        let ops_elements: Vec<proc_macro2::TokenStream> = field_names
            .iter()
            .map(|name| quote!(#name: self.#name * one_over_scalar,))
            .collect();

        let impl_trait = quote! {
            impl #impl_generics std::ops::Div<#field_type> for #struct_name #type_generics #where_clause {
                type Output = Self;

                fn div(self, scalar: #field_type) -> Self::Output {
                    let one_over_scalar = T::one() / scalar;
                    Self {
                        #(#ops_elements)*
                    }
                }
            }
        };
        token_streams.push(impl_trait);
    }

    let index_access: Vec<proc_macro2::TokenStream> = field_names
        .iter()
        .enumerate()
        .map(|(i, name)| quote! {#i => &self.#name,})
        .collect();
    let impl_index = quote! {
        impl #impl_generics std::ops::Index<usize> for #struct_name #type_generics #where_clause {
            type Output = #field_type;

            fn index(&self, index: usize) -> &Self::Output {
                match index {
                    #(#index_access)*
                    _ => panic!("out of range"),
                }
            }
        }
    };
    token_streams.push(impl_index);

    let index_mut_access: Vec<proc_macro2::TokenStream> = field_names
        .iter()
        .enumerate()
        .map(|(i, name)| quote! {#i => &mut self.#name,})
        .collect();
    let impl_index_mut = quote! {
        impl #impl_generics std::ops::IndexMut<usize> for #struct_name #type_generics #where_clause {
            fn index_mut(&mut self, index: usize) -> &mut <Self as std::ops::Index::<usize>>::Output {
                match index {
                    #(#index_mut_access)*
                    _ => panic!("out of range"),
                }
            }
        }
    };
    token_streams.push(impl_index_mut);

    {
        let neg_elements: Vec<proc_macro2::TokenStream> = field_names
            .iter()
            .map(|name| quote! { #name: -self.#name, })
            .collect();
        let where_clause = if where_clause.is_some() {
            quote! { #where_clause, where #field_type: std::ops::Neg<Output = #field_type> }
        } else {
            quote! { where #field_type: std::ops::Neg<Output = #field_type> }
        };
        let impl_neg = quote! {
            impl #impl_generics std::ops::Neg for #struct_name #type_generics #where_clause {
                type Output = Self;

                fn neg(self) -> Self::Output {
                    Self { #(#neg_elements)* }
                }
            }
        };
        token_streams.push(impl_neg);
    }

    {
        // impl length_squared
        let calc_length_squared: proc_macro2::TokenStream = field_names
            .iter()
            .map(|name| quote! { self.#name * self.#name })
            .fold(proc_macro2::TokenStream::new(), |ts, v| {
                if ts.is_empty() {
                    v
                } else {
                    quote! { #ts + #v }
                }
            });
        let impl_length_squared = quote! {
            impl #impl_generics #struct_name #type_generics #where_clause {
                pub fn length_squared(&self) -> #field_type {
                    #calc_length_squared
                }
            }
        };
        token_streams.push(impl_length_squared);
    }
    {
        // impl length
        let where_clause = if where_clause.is_some() {
            quote! { #where_clause, where #field_type: num::Float }
        } else {
            quote! { where #field_type: num::Float }
        };
        let impl_length = quote! {
            impl #impl_generics #struct_name #type_generics #where_clause {
                pub fn length(&self) -> #field_type {
                    self.length_squared().sqrt()
                }
            }
        };
        token_streams.push(impl_length);
    }
    {
        // impl dot
        let calc_dot: proc_macro2::TokenStream = field_names
            .iter()
            .enumerate()
            .map(|(i, name)| quote! { self.#name * rhs.get(#i) })
            .fold(proc_macro2::TokenStream::new(), |ts, v| {
                if ts.is_empty() {
                    v
                } else {
                    quote! { #ts + #v }
                }
            });
        let impl_dot = quote! {
            impl #impl_generics #struct_name #type_generics #where_clause {
                pub fn dot<V: VectorLike<#field_type, #dimension>>(&self, rhs: V) -> #field_type {
                    #calc_dot
                }
            }
        };
        token_streams.push(impl_dot);
    }
    {
        let zero_elements: Vec<proc_macro2::TokenStream> = field_names
            .iter()
            .map(|name| quote!{ #name: <#field_type as num::Zero>::zero(), })
            .collect();

        // impl normalize and normalized
        let where_clause = if where_clause.is_some() {
            quote! { #where_clause, where #field_type: num::Float }
        } else {
            quote! { where #field_type: num::Float }
        };
        let impl_normalize = quote! {
            impl #impl_generics #struct_name #type_generics #where_clause {
                pub fn normalized(&self) -> Self {
                    let len = self.length();
                    if len != T::zero() {
                        let one_over_len = T::one() / len;
                        *self * one_over_len
                    } else {
                        Self {
                            #(#zero_elements)*
                        }
                    }
                }

                pub fn normalize(&mut self) {
                    *self = self.normalized()
                }
            }
        };
        token_streams.push(impl_normalize);
    }

    let expanded = quote! {
        #(#token_streams)*
    };

    Ok(expanded.into())
}

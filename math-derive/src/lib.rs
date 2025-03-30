use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{ext::IdentExt, parse_macro_input, DeriveInput, Fields, Ident};

#[proc_macro_derive(Vector)]
pub fn vector_derive(input: TokenStream) -> TokenStream {
    let input = &parse_macro_input!(input as DeriveInput);
    match generate(input) {
        Ok(generated) => generated,
        Err(err) => err.to_compile_error().into(),
    }
}

struct TypeData {
    derive_input: DeriveInput,
    field_names: Vec<Ident>,
    field_type: syn::Type,
    dimension: usize,
}

impl TypeData {
    fn new(derive_input: DeriveInput) -> Result<Self, syn::Error> {
        let struct_data = match &derive_input.data {
            syn::Data::Struct(v) => v,
            _ => {
                return Err(syn::Error::new_spanned(
                    &derive_input.ident,
                    "Must be struct type",
                ));
            }
        };
        let fields: Vec<_> = match &struct_data.fields {
            Fields::Named(fields) => fields.named.clone().into_iter().collect(),
            Fields::Unnamed(fields) => fields.unnamed.clone().into_iter().collect(),
            _ => panic!("Vector type must have 1 or more elements."),
        };
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

        Ok(Self {
            derive_input,
            field_names,
            field_type,
            dimension,
        })
    }

    fn name(&self) -> &Ident {
        &self.derive_input.ident
    }

    fn field_names(&self) -> &Vec<Ident> {
        &self.field_names
    }

    fn field_type(&self) -> &syn::Type {
        &self.field_type
    }

    fn dimension(&self) -> usize {
        self.dimension
    }

    fn generics(&self) -> &syn::Generics {
        &self.derive_input.generics
    }

    fn type_generics(&self) -> syn::TypeGenerics {
        self.derive_input.generics.split_for_impl().1
    }

    fn make_where_clause(
        &self,
        additional: Option<proc_macro2::TokenStream>,
    ) -> proc_macro2::TokenStream {
        let where_clause = self.derive_input.generics.where_clause.as_ref();
        match (where_clause, additional) {
            (Some(where_clause), Some(additional)) => quote! { where #where_clause, #additional },
            (Some(where_clause), None) => quote! { where #where_clause },
            (None, Some(additional)) => quote! { where #additional },
            (None, None) => quote! {},
        }
    }
}

fn generate(derive_input: &DeriveInput) -> Result<TokenStream, syn::Error> {
    let mut token_streams = Vec::new();

    let type_data = TypeData::new(derive_input.clone())?;
    token_streams.push(impl_convert(&type_data));
    token_streams.push(impl_display(&type_data));
    token_streams.push(impl_ops_vec(&type_data));
    token_streams.push(impl_zero_one(&type_data));
    token_streams.push(impl_mul_scalar(&type_data));
    token_streams.push(impl_mul_assign_scalar(&type_data));
    token_streams.push(impl_div_scalar(&type_data));
    token_streams.push(impl_div_assign_scalar(&type_data));
    token_streams.push(impl_mul_scalar_vec(&type_data));
    token_streams.push(impl_index(&type_data));
    token_streams.push(impl_index_mut(&type_data));
    token_streams.push(impl_neg(&type_data));
    token_streams.push(impl_as_float_vec(&type_data));
    token_streams.push(impl_length(&type_data));
    token_streams.push(impl_dot(&type_data));
    token_streams.push(impl_normalized(&type_data));
    token_streams.push(impl_distance(&type_data));

    let expanded = quote! {
        #(#token_streams)*
    };

    Ok(expanded.into())
}

fn impl_trait(
    type_data: &TypeData,
    trait_name: proc_macro2::TokenStream,
    inner: proc_macro2::TokenStream,
    additional_where_clause: Option<proc_macro2::TokenStream>,
) -> proc_macro2::TokenStream {
    let generics = type_data.generics();
    let struct_name = type_data.name();
    let type_generics = type_data.type_generics();
    let where_clause = type_data.make_where_clause(additional_where_clause);

    quote! {
        impl #generics #trait_name for #struct_name #type_generics #where_clause {
            #inner
        }
    }
}

fn impl_convert(type_data: &TypeData) -> proc_macro2::TokenStream {
    let field_type = type_data.field_type();
    let dimension = type_data.dimension();
    let field_names = type_data.field_names();
    let indices: Vec<proc_macro2::TokenStream> = (0..dimension)
        .map(|i| i.to_string().parse().unwrap())
        .collect();
    let from_array = impl_trait(
        type_data,
        quote! { std::convert::From<[#field_type; #dimension]> },
        quote! {
            fn from(array: [#field_type; #dimension]) -> Self {
                Self {
                    #(#field_names: array[#indices]),*
                }
            }
        },
        None,
    );

    let tuple_elements = (0..dimension).map(|_| quote! {T}).collect::<Vec<_>>();
    let tuple_type = quote! { (#(#tuple_elements,)*) };
    let from_tuple = impl_trait(
        type_data,
        quote! { std::convert::From<#tuple_type> },
        quote! {
            fn from(tuple: #tuple_type) -> Self {
                Self {
                    #(#field_names: tuple.#indices),*
                }
            }
        },
        None,
    );

    let into_array = impl_trait(
        type_data,
        quote! { std::convert::Into<[#field_type; #dimension]> },
        quote! {
            fn into(self) -> [#field_type; #dimension] {
                [ #(self.#field_names),* ]
            }
        },
        None,
    );

    let into_tuple = impl_trait(
        type_data,
        quote! { std::convert::Into<#tuple_type> },
        quote! {
            fn into(self) -> #tuple_type {
                (#(self.#field_names,)*)
            }
        },
        None,
    );

    quote! {
        #from_array
        #from_tuple
        #into_array
        #into_tuple
    }
}

fn impl_zero_one(type_data: &TypeData) -> proc_macro2::TokenStream {
    let field_names = type_data.field_names();
    let zero_elements: Vec<proc_macro2::TokenStream> = field_names
        .iter()
        .map(|name| quote! { #name: T::ZERO })
        .collect();
    let one_elements: Vec<proc_macro2::TokenStream> = field_names
        .iter()
        .map(|name| quote! { #name: T::ONE })
        .collect();
    let is_near_zero_elements: Vec<proc_macro2::TokenStream> = field_names
        .iter()
        .map(|name| quote! { self.#name.is_near_zero() })
        .collect();

    let impl_zero = impl_trait(
        type_data,
        quote! { crate::num::Zero },
        quote! {
            const ZERO: Self = Self {
                #(#zero_elements),*
            };

            fn is_near_zero(&self) -> bool {
                #(#is_near_zero_elements) && *
            }
        },
        None,
    );

    let impl_one = impl_trait(
        type_data,
        quote! { crate::num::One },
        quote! {
            const ONE: Self = Self {
                #(#one_elements),*
            };
        },
        None,
    );

    quote! {
        #impl_zero
        #impl_one
    }
}

fn impl_display(type_data: &TypeData) -> proc_macro2::TokenStream {
    let field_names = type_data.field_names();
    let field_type = type_data.field_type();
    let display_elements: Vec<proc_macro2::TokenStream> = field_names
        .iter()
        .map(|name| quote! { self.#name })
        .collect();
    let display_format = (0..field_names.len())
        .map(|_| "{}")
        .collect::<Vec<_>>()
        .join(", ");
    let display_format = format!("[{}]", display_format);
    impl_trait(
        type_data,
        quote! { std::fmt::Display },
        quote! {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, #display_format, #(#display_elements),*)
            }
        },
        Some(quote! { #field_type: std::fmt::Display }),
    )
}

fn impl_ops_vec(type_data: &TypeData) -> proc_macro2::TokenStream {
    let ops_settings = [
        (quote! {Add}, quote! {add}, quote! {+}),
        (quote! {Sub}, quote! {sub}, quote! {-}),
        (quote! {Mul}, quote! {mul}, quote! {*}),
        (quote! {Div}, quote! {div}, quote! {/}),
    ];

    let mut token_streams = Vec::new();
    for (ops_trait_name, ops_trait_func_name, op) in ops_settings {
        let ops_elements: Vec<proc_macro2::TokenStream> = type_data
            .field_names()
            .iter()
            .enumerate()
            .map(|(i, name)| format!("{name}: self.{name} {op} *rhs.get({i}),"))
            .map(|s| s.parse().unwrap())
            .collect();

        let assign_ops_trait_name = format_ident!("{}Assign", ops_trait_name.to_string());
        let assign_ops_trait_func_name =
            format_ident!("{}_assign", ops_trait_func_name.to_string());

        let ops_inner = quote! {
            type Output = Self;

            fn #ops_trait_func_name(self, rhs: Self) -> Self::Output {
                Self {
                    #(#ops_elements)*
                }
            }
        };

        let assign_ops_inner = quote! {
            fn #assign_ops_trait_func_name(&mut self, rhs: Self) {
                *self = *self #op rhs;
            }
        };

        let it = impl_trait(
            type_data,
            quote! {std::ops::#ops_trait_name},
            ops_inner,
            None,
        );
        token_streams.push(it);

        let it = impl_trait(
            type_data,
            quote! { std::ops::#assign_ops_trait_name },
            assign_ops_inner,
            None,
        );
        token_streams.push(it);
    }

    quote! {
        #(#token_streams)*
    }
}

fn impl_mul_scalar(type_data: &TypeData) -> proc_macro2::TokenStream {
    let ops_elements: Vec<proc_macro2::TokenStream> = type_data
        .field_names()
        .iter()
        .map(|name| quote!(#name: self.#name * scalar,))
        .collect();
    let field_type = type_data.field_type();
    let inner = quote! {
        type Output = Self;

        fn mul(self, scalar: #field_type) -> Self::Output {
            Self {
                #(#ops_elements)*
            }
        }
    };
    impl_trait(
        type_data,
        quote! { std::ops::Mul<#field_type> },
        inner,
        None,
    )
}

fn impl_mul_assign_scalar(type_data: &TypeData) -> proc_macro2::TokenStream {
    let field_type = type_data.field_type();
    let inner = quote! {
        fn mul_assign(&mut self, scalar: #field_type) {
            *self = *self * scalar;
        }
    };
    impl_trait(
        type_data,
        quote! { std::ops::MulAssign<#field_type> },
        inner,
        None,
    )
}

fn impl_div_scalar(type_data: &TypeData) -> proc_macro2::TokenStream {
    let ops_elements: Vec<proc_macro2::TokenStream> = type_data
        .field_names()
        .iter()
        .map(|name| quote!(#name: self.#name / scalar,))
        .collect();
    let field_type = type_data.field_type();
    let inner = quote! {
        type Output = Self;

        fn div(self, scalar: #field_type) -> Self::Output {
            Self {
                #(#ops_elements)*
            }
        }
    };
    impl_trait(
        type_data,
        quote! { std::ops::Div<#field_type> },
        inner,
        None,
    )
}

fn impl_div_assign_scalar(type_data: &TypeData) -> proc_macro2::TokenStream {
    let field_type = type_data.field_type();
    let inner = quote! {
        fn div_assign(&mut self, scalar: #field_type) {
            *self = *self / scalar;
        }
    };
    impl_trait(
        type_data,
        quote! { std::ops::DivAssign<#field_type> },
        inner,
        None,
    )
}

fn impl_mul_scalar_vec(type_data: &TypeData) -> proc_macro2::TokenStream {
    let types = vec![
        quote! { i8 },
        quote! { i16 },
        quote! { i32 },
        quote! { i64 },
        quote! { i128 },
        quote! { isize },
        quote! { u8 },
        quote! { u16 },
        quote! { u32 },
        quote! { u64 },
        quote! { u128 },
        quote! { usize },
        quote! { f32 },
        quote! { f64 },
    ];
    let struct_name = type_data.name();
    let mul_scalar_v_collection: Vec<proc_macro2::TokenStream> = types
        .iter()
        .map(|t| {
            quote!(
                impl std::ops::Mul<#struct_name<#t>> for #t {
                    type Output = #struct_name<#t>;

                    fn mul(self, v: #struct_name<#t>) -> Self::Output {
                        v * self
                    }
                }
            )
        })
        .collect();

    quote! {
        #(#mul_scalar_v_collection)*
    }
}

fn impl_index(type_data: &TypeData) -> proc_macro2::TokenStream {
    let field_names = type_data.field_names();
    let field_type = type_data.field_type();
    let index_access: Vec<proc_macro2::TokenStream> = field_names
        .iter()
        .enumerate()
        .map(|(i, name)| quote! {#i => &self.#name,})
        .collect();
    impl_trait(
        type_data,
        quote! { std::ops::Index<usize> },
        quote! {
            type Output = #field_type;

            fn index(&self, index: usize) -> &Self::Output {
                match index {
                    #(#index_access)*
                    _ => panic!("out of range"),
                }
            }
        },
        None,
    )
}

fn impl_index_mut(type_data: &TypeData) -> proc_macro2::TokenStream {
    let field_names = type_data.field_names();
    let index_access: Vec<proc_macro2::TokenStream> = field_names
        .iter()
        .enumerate()
        .map(|(i, name)| quote! {#i => &mut self.#name,})
        .collect();
    impl_trait(
        type_data,
        quote! { std::ops::IndexMut<usize> },
        quote! {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                match index {
                    #(#index_access)*
                    _ => panic!("out of range"),
                }
            }
        },
        None,
    )
}

fn impl_neg(type_data: &TypeData) -> proc_macro2::TokenStream {
    let field_type = type_data.field_type();
    let neg_elements: Vec<proc_macro2::TokenStream> = type_data
        .field_names()
        .iter()
        .map(|name| quote! { #name: -self.#name, })
        .collect();
    impl_trait(
        type_data,
        quote! { std::ops::Neg },
        quote! {
            type Output = Self;

            fn neg(self) -> Self::Output {
                Self { #(#neg_elements)* }
            }
        },
        Some(quote! { #field_type: std::ops::Neg<Output = #field_type> }),
    )
}

fn impl_as_float_vec(type_data: &TypeData) -> proc_macro2::TokenStream {
    let field_type = type_data.field_type();
    let struct_name = type_data.name();
    let field_cast: Vec<_> = type_data
        .field_names()
        .iter()
        .map(|name| quote! { #name: self.#name.as_floating_point() })
        .collect();
    let where_clause = type_data.make_where_clause(Some(
        quote! {<#field_type as crate::num::AsFloatingPoint>::Output: VectorElement},
    ));

    quote! {
        impl<#field_type: VectorElement> #struct_name<#field_type> #where_clause {
            fn as_float_vec(&self) -> #struct_name<<#field_type as crate::num::AsFloatingPoint>::Output> {
                use crate::num::AsFloatingPoint as _;
                #struct_name::<_> {
                    #(#field_cast),*
                }
            }
        }
    }
}

// impl length_squared and length
fn impl_length(type_data: &TypeData) -> proc_macro2::TokenStream {
    let field_type = type_data.field_type();
    let field_names = type_data.field_names();
    let struct_name = type_data.name();
    let type_generics = type_data.type_generics();
    let where_clause = type_data.make_where_clause(None);
    let generics = type_data.generics();
    quote! {
        impl #generics #struct_name #type_generics #where_clause {
            pub fn length_squared(&self) -> #field_type {
                #(self.#field_names * self.#field_names)+*
            }

            pub fn length(&self) -> <#field_type as crate::num::AsFloatingPoint>::Output {
                use num::Float as _;
                use crate::num::AsFloatingPoint as _;
                (#(self.#field_names.as_floating_point().powi(2))+*).sqrt()
            }
        }
    }
}

fn impl_dot(type_data: &TypeData) -> proc_macro2::TokenStream {
    let field_type = type_data.field_type();
    let field_names = type_data.field_names();
    let struct_name = type_data.name();
    let dimension = type_data.dimension();
    let type_generics = type_data.type_generics();
    let where_clause = type_data.make_where_clause(None);
    let generics = type_data.generics();
    let calc_elements = field_names
        .iter()
        .enumerate()
        .map(|(i, name)| quote! {self.#name * *rhs.get(#i)})
        .collect::<Vec<_>>();

    quote! {
        impl #generics #struct_name #type_generics #where_clause {
            pub fn dot(&self, rhs: impl VectorLike<#dimension, ElementType = #field_type>) -> #field_type {
                #(#calc_elements)+*
            }
        }
    }
}

fn impl_normalized(type_data: &TypeData) -> proc_macro2::TokenStream {
    let field_type = type_data.field_type();
    let struct_name = type_data.name();
    let type_generics = type_data.type_generics();
    let where_clause = type_data.make_where_clause(Some(quote! {
    <#field_type as crate::num::AsFloatingPoint>::Output: crate::math::traits::FloatVectorElement,
    }));
    let generics = type_data.generics();
    quote! {
        impl #generics #struct_name #type_generics #where_clause {
            pub fn normalized(&self) -> #struct_name<<#field_type as crate::num::AsFloatingPoint>::Output> {
                use crate::num::Zero;
                let v = self.as_float_vec();
                let length = v.length();
                if length.is_near_zero() {
                    return <#struct_name<<#field_type as crate::num::AsFloatingPoint>::Output> as crate::num::Zero>::ZERO;
                }
                v / length
            }

            pub fn normalize(&mut self)
            where
                T: crate::math::vec::traits::FloatVectorElement,
            {
                *self = self.normalized();
            }
        }
    }
}

fn impl_distance(type_data: &TypeData) -> proc_macro2::TokenStream {
    let field_type = type_data.field_type();
    let struct_name = type_data.name();
    let type_generics = type_data.type_generics();
    let where_clause = type_data.make_where_clause(Some(quote! {
    <#field_type as crate::num::AsFloatingPoint>::Output: crate::math::traits::FloatVectorElement,
    }));
    let generics = type_data.generics();
    quote! {
        impl #generics #struct_name #type_generics #where_clause {
            pub fn distance(&self, other: impl Into<Self>) -> <#field_type as crate::num::AsFloatingPoint>::Output {
                let v1 = self.as_float_vec();
                let other: Self = other.into();
                let v2 = other.as_float_vec();
                (v1 - v2).length()
            }
        }
    }
}

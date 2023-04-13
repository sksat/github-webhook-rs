use proc_macro2::{TokenStream, TokenTree};
use quote::{quote, ToTokens, TokenStreamExt};

use crate::ir::{
    RustAlias, RustContainerAttrs, RustEnum, RustEnumMember, RustFieldAttr, RustFieldAttrs,
    RustMemberType, RustSegment, RustStruct, RustStructAttr, RustStructMember, RustType,
    SerdeContainerAttr, SerdeFieldAttr, TypeName,
};

macro_rules! id {
    ($($tt:tt)*) => {
        proc_macro2::Ident::new($($tt)*, proc_macro2::Span::call_site())
    };
}

impl RustSegment {
    pub fn into_token_stream(self) -> TokenStream {
        match self {
            RustSegment::Struct(s) => s.into_token_stream(),
            RustSegment::Enum(e) => e.into_token_stream(),
            RustSegment::Alias(a) => a.into_token_stream(),
        }
    }
}

impl ToTokens for TypeName {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { name, is_borrowed } = self;
        let name = id!(name);
        let p = if *is_borrowed {
            quote! { <'a> }
        } else {
            quote!()
        };
        tokens.extend(
            quote! {
                #name #p
            }
            .into_iter(),
        )
    }
}

impl ToTokens for RustStructAttr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(
            match self {
                RustStructAttr::Serde(s) => quote! {
                    #[serde(#s)]
                },
            }
            .into_iter(),
        )
    }
}

impl ToTokens for SerdeContainerAttr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(
            match self {
                SerdeContainerAttr::RenameAll(r) => {
                    let r = r.to_string();
                    quote! {
                        rename_all = #r
                    }
                }
                SerdeContainerAttr::Tag(name) => quote! {
                    tag = #name
                },
            }
            .into_iter(),
        )
    }
}

impl ToTokens for RustFieldAttr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(
            match self {
                RustFieldAttr::Serde(s) => quote! {
                    #[serde(#s)]
                },
            }
            .into_iter(),
        )
    }
}

impl ToTokens for SerdeFieldAttr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(
            match self {
                SerdeFieldAttr::Rename(s) => quote! {
                    rename = #s
                },
                SerdeFieldAttr::Borrow => quote! {
                    borrow = "'a"
                },
            }
            .into_iter(),
        )
    }
}

impl ToTokens for RustType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let s = match self {
            RustType::String { is_borrowed } => {
                if *is_borrowed {
                    tokens.extend(
                        quote! {
                            &'a str
                        }
                        .into_iter(),
                    );
                    return;
                }
                "String"
            }
            RustType::Number => "usize",
            RustType::Boolean => "bool",
            RustType::Custom(TypeName { name, is_borrowed }) => {
                let name = id!(name);
                let p = if *is_borrowed {
                    quote! { <'a> }
                } else {
                    quote!()
                };
                tokens.extend(
                    quote! {
                        #name #p
                    }
                    .into_iter(),
                );
                return;
            }
            RustType::Array(t) => {
                tokens.extend(
                    quote! {
                        Vec<#t>
                    }
                    .into_iter(),
                );
                return;
            }
            RustType::Empty => {
                tokens.append(TokenTree::Group(proc_macro2::Group::new(
                    proc_macro2::Delimiter::Parenthesis,
                    Default::default(),
                )));
                return;
            }
            RustType::Unknown => "Unknown",
            RustType::UnknownLiteral => "UnknownLiteral",
            RustType::UnknownIntersection => "UnknownIntersection",
            RustType::UnknownUnion => "UnknownUnion",
        };
        tokens.append(TokenTree::Ident(id!(s)));
    }
}

impl ToTokens for RustMemberType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let inner_ty = &self.ty;
        tokens.extend(
            if self.is_optional {
                quote! {
                    Option<#inner_ty>
                }
            } else {
                quote! {
                    #inner_ty
                }
            }
            .into_iter(),
        )
    }
}

impl ToTokens for RustStructMember {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            attr,
            name,
            ty,
            comment,
        } = self;
        let name = id!(name);

        tokens.extend(
            if self.ty.is_unknown() {
                quote! {
                    /* unknown type */
                }
            } else {
                let mut attrs = TokenStream::new();
                if let Some(comment) = comment {
                    attrs = quote! {
                        #[doc=#comment]
                    }
                }
                match attr {
                    RustFieldAttrs::Default => (),
                    RustFieldAttrs::With(w) => {
                        attrs = quote! {
                            #attrs
                            #(#w)*
                        }
                    }
                }
                quote! {
                    #attrs
                    pub #name: #ty,
                }
            }
            .into_iter(),
        );
    }
}

impl ToTokens for RustStruct {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            name,
            member,
            attr,
            is_borrowed,
        } = self;
        let name = id!(name);
        tokens.extend(
            match attr {
                RustContainerAttrs::Default => quote! {
                    #[derive(Debug, Deserialize)]
                },
                RustContainerAttrs::With(w) => quote! {
                    #[derive(Debug, Deserialize)]
                    #(#w)*
                },
            }
            .into_iter(),
        );
        let p = if *is_borrowed {
            quote! { <'a> }
        } else {
            quote!()
        };

        tokens.extend(
            quote! {
                pub struct #name #p {
                    #(#member)*
                }
            }
            .into_iter(),
        );
    }
}

impl ToTokens for RustEnum {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            name,
            member,
            attr,
            is_borrowed,
        } = self;
        let name = id!(name);
        tokens.extend(
            if attr.is_tagged_enum() || member.iter().all(|m| m.is_nullary()) {
                quote! {
                    #[derive(Debug, Deserialize)]
                }
            } else {
                quote! {
                    #[derive(Debug)]
                }
            }
            .into_iter(),
        );
        match attr {
            RustContainerAttrs::Default => (),
            RustContainerAttrs::With(w) => {
                tokens.extend(
                    quote! {
                        #(#w)*
                    }
                    .into_iter(),
                );
            }
        }

        let p = if *is_borrowed {
            quote! { <'a> }
        } else {
            quote!()
        };
        tokens.extend(
            quote! {
                pub enum #name #p {
                    #(#member)*
                }
            }
            .into_iter(),
        );
    }
}

impl ToTokens for RustAlias {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            name,
            ty: typ,
            is_borrowed,
        } = self;
        let ident = id!(name);
        let p = if *is_borrowed {
            quote! { <'a> }
        } else {
            quote!()
        };
        tokens.extend(
            quote! {
                pub type #ident #p = #typ;
            }
            .into_iter(),
        )
    }
}

impl ToTokens for RustEnumMember {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(
            match self {
                RustEnumMember::Nullary(v) => {
                    let v = id!(&v.name);
                    quote!(#v,)
                }
                RustEnumMember::Unary(a) => {
                    let n = id!(&a.name);
                    quote!(#n(#a),)
                }
                RustEnumMember::UnaryNamed {
                    variant_name,
                    type_name,
                } => {
                    let variant_name = id!(variant_name);
                    quote!(#variant_name(#type_name),)
                }
            }
            .into_iter(),
        )
    }
}

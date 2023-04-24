use proc_macro2::{TokenStream, TokenTree};
use quote::{quote, ToTokens, TokenStreamExt};

use crate::ir::{
    Attrs, RustAlias, RustEnum, RustEnumMember, RustEnumMemberKind, RustFieldAttr, RustMemberType,
    RustSegment, RustStruct, RustStructAttr, RustStructMember, RustType, RustVariantAttr,
    SerdeContainerAttr, SerdeFieldAttr, SerdeVariantAttr, TypeName,
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
                SerdeContainerAttr::Untagged => quote! {
                    untagged
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
                SerdeFieldAttr::Flatten => quote! {
                    flatten
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
            RustType::Unit => {
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

        if !self.ty.is_unknown() {
            let mut c = TokenStream::new();
            if let Some(comment) = comment {
                c = quote! {
                    #[doc=#comment]
                }
            }
            tokens.extend(
                quote! {
                    #c
                    #attr
                    pub #name: #ty,
                }
                .into_iter(),
            );
        }
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
        tokens.extend({
            quote! {
                #[derive(Debug, Deserialize)]
                #attr
            }
            .into_iter()
        });
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
            if attr.as_inner().iter().any(|a| a.as_serde().is_some())
                || member.iter().all(|m| m.kind.is_nullary())
            {
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

        let p = if *is_borrowed {
            quote! { <'a> }
        } else {
            quote!()
        };
        tokens.extend(
            quote! {
                #attr
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

impl ToTokens for RustEnumMemberKind {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(
            match self {
                RustEnumMemberKind::Nullary(v) => {
                    let v = id!(v);
                    quote!(#v,)
                }
                RustEnumMemberKind::Unary(a) => {
                    if a.is_unknown() {
                        quote! {
                            #[serde(other)]
                            Other,
                        }
                    } else {
                        let n = a.to_ident();
                        let n = id!(n);
                        quote!(#n(#a),)
                    }
                }
                RustEnumMemberKind::UnaryNamed {
                    variant_name,
                    type_name,
                } => {
                    if type_name.is_unknown() {
                        quote! {
                            #[serde(other)]
                            Other,
                        }
                    } else {
                        let variant_name = id!(variant_name);
                        quote!(#variant_name(#type_name),)
                    }
                }
            }
            .into_iter(),
        )
    }
}

impl<Field: ToTokens> ToTokens for Attrs<Field> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if !self.as_inner().is_empty() {
            let ws = self.as_inner();
            tokens.extend(
                quote! {
                    #(#ws)*
                }
                .into_iter(),
            )
        }
    }
}

impl ToTokens for RustEnumMember {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { attr, kind } = self;
        tokens.extend(
            quote! {
                #attr
                #kind
            }
            .into_iter(),
        )
    }
}

impl ToTokens for RustVariantAttr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(
            match self {
                RustVariantAttr::Serde(s) => quote! {
                    #[serde(#s)]
                },
            }
            .into_iter(),
        )
    }
}

impl ToTokens for SerdeVariantAttr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(
            match self {
                SerdeVariantAttr::Rename(s) => quote! {
                    rename = #s
                },
            }
            .into_iter(),
        )
    }
}

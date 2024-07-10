use proc_macro2::{TokenStream, TokenTree};
use quote::{quote, ToTokens, TokenStreamExt};

use crate::ir::{
    Attrs, RustAlias, RustComment, RustEnum, RustEnumMember, RustEnumMemberKind, RustFieldAttr,
    RustMemberType, RustSegment, RustStruct, RustStructAttr, RustStructMember, RustType,
    RustVariantAttr, SerdeContainerAttr, SerdeFieldAttr, SerdeVariantAttr, TypeName,
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
            },
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
            },
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
            },
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
            },
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
            },
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
                            ::std::borrow::Cow<'a, str>
                        },
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
                    },
                );
                return;
            }
            RustType::Array(t) => {
                tokens.extend(
                    quote! {
                        Vec<#t>
                    },
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
            // serde_json::Value
            RustType::Unknown => "Value",
            RustType::UnknownLiteral => "UnknownLiteral",
            RustType::UnknownIntersection => "UnknownIntersection",
            RustType::Map(t1, t2) => {
                tokens.extend(
                    quote! {
                        HashMap<#t1, #t2>
                    },
                );
                return;
            }
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
            },
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
            tokens.extend(
                quote! {
                    #comment
                    #attr
                    pub #name: #ty,
                },
            );
        }
    }
}

impl ToTokens for RustComment {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let c = &self.0;

        tokens.extend(
            quote! {
                #[doc=#c]
            },
        );
    }
}

impl ToTokens for RustStruct {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            name,
            member,
            comment,
            attr,
            is_borrowed,
        } = self;
        let name = id!(name);
        tokens.extend({
            quote! {
                #[derive(Debug, Deserialize)]
                #attr
                #comment
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
            },
        );
    }
}

impl ToTokens for RustEnum {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            name,
            member,
            comment,
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
            },
        );

        let p = if *is_borrowed {
            quote! { <'a> }
        } else {
            quote!()
        };
        tokens.extend(
            quote! {
                #attr
                #comment
                pub enum #name #p {
                    #(#member)*
                }
            },
        );
    }
}

impl ToTokens for RustAlias {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            name,
            ty: typ,
            comment,
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
                #comment
                pub type #ident #p = #typ;
            },
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
                            Other,
                        }
                    } else {
                        let variant_name = id!(variant_name);
                        quote!(#variant_name(#type_name),)
                    }
                }
            },
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
                },
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
            },
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
            },
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
                SerdeVariantAttr::Borrow => quote! {
                    borrow = "'a"
                },
            },
        )
    }
}

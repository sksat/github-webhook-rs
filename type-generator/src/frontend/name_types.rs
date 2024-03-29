use std::collections::HashMap;

use crate::ir::{
    RustAlias, RustComment, RustEnum, RustEnumMember, RustEnumMemberKind, RustSegment, RustStruct,
    RustType, RustVariantAttr, RustVariantAttrs, SerdeVariantAttr, TypeName,
};

use super::{ts_index_signature, ts_prop_signature, FrontendState, TypeConvertContext};

#[derive(Default)]
pub struct State<'a> {
    /// set of literals -> rust type name
    literal_map: HashMap<Vec<&'a str>, String>,
}

pub fn string_literal_union<'input>(
    st: &mut FrontendState<'input, '_>,
    variants: Vec<&'input str>,
    comment: Option<RustComment>,
    path: &mut TypeConvertContext,
) -> TypeName {
    let name = path.create_ident();

    TypeName::new(match st.name_types.literal_map.get(&variants) {
        Some(s) => {
            // create new alias from union
            create_alias(st, comment, &name, s.to_owned());
            name
        }
        None => {
            // create new enum from union
            create_enum(st, comment, &name, &variants);

            st.name_types.literal_map.insert(variants, name.clone());
            name
        }
    })
}

fn create_alias(
    st: &mut FrontendState,
    comment: Option<RustComment>,
    name: &String,
    old_name: String,
) {
    st.segments.push(RustSegment::Alias(RustAlias {
        name: name.to_owned(),
        is_borrowed: false,
        comment,
        ty: RustType::Custom(TypeName::new(old_name)),
    }));
}

fn create_enum(st: &mut FrontendState, comment: Option<RustComment>, name: &String, vs: &[&str]) {
    st.segments.push(RustSegment::Enum(RustEnum::from_members(
        name.to_owned(),
        comment,
        vs.iter().map(|&v| {
            let renamed = rename_to_valid_ident(v);
            let mut attr = RustVariantAttrs::new();
            if v != renamed {
                attr.add_attr(RustVariantAttr::Serde(SerdeVariantAttr::Rename(
                    v.to_owned(),
                )));
            }
            RustEnumMember {
                attr,
                kind: RustEnumMemberKind::Nullary(renamed),
            }
        }),
    )));
}

fn rename_to_valid_ident(s: &str) -> String {
    s.split(&['-', ' ', '_'])
        .map(|term| {
            let mut term = term
                .chars()
                .filter_map(|c| {
                    if c.is_ascii_alphanumeric() {
                        Some(c.to_ascii_lowercase())
                    } else {
                        None
                    }
                })
                .collect::<String>();
            if let Some(c) = term.chars().next() {
                let capital_ch = c.to_ascii_uppercase();
                let replace_with = if capital_ch.is_alphabetic() {
                    capital_ch.to_string()
                } else if capital_ch.is_numeric() {
                    format!("N{capital_ch}")
                } else {
                    unimplemented!()
                };
                term.replace_range(..1, &replace_with);
            }
            term
        })
        .collect::<Vec<_>>()
        .concat()
}

pub fn type_literal<'input>(
    st: &mut FrontendState<'input, '_>,
    type_literal: impl Iterator<Item = &'input swc_ecma_ast::TsTypeElement>,
    comment: Option<RustComment>,
    ctxt: &mut TypeConvertContext<'input>,
    lkm: &mut HashMap<String, HashMap<String, String>>,
) -> RustStruct {
    let name = ctxt.create_ident();
    RustStruct::from_members(
        name.to_owned(),
        comment,
        type_literal.into_iter().flat_map(|m| match m {
            swc_ecma_ast::TsTypeElement::TsPropertySignature(p) => {
                Some(ts_prop_signature(p, st, ctxt, &name, lkm))
            }
            swc_ecma_ast::TsTypeElement::TsIndexSignature(i) => {
                Some(ts_index_signature(i, None, st, ctxt, lkm))
            }
            _ => None,
        }),
    )
}

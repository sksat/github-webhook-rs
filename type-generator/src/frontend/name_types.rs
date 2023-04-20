use std::collections::HashMap;

use crate::{
    ir::{
        RenameRule, RustAlias, RustEnum, RustEnumMember, RustEnumMemberKind, RustSegment, RustType,
        RustVariantAttr, RustVariantAttrs, SerdeVariantAttr, TypeName,
    },
    FrontendState,
};

#[derive(Default)]
pub struct State {
    /// set of literals -> rust type name
    name_map: HashMap<Vec<String>, String>,
}

pub fn string_literal_union(
    st: &mut FrontendState,
    variants: Vec<String>,
    struct_name: &str,
    prop_name: &str,
) -> TypeName {
    let mut prop = prop_name.to_owned();
    RenameRule::SnakeCase.convert_to_pascal(&mut prop);
    let name = format!("{struct_name}{prop}LiteralUnion");

    TypeName::new(match st.name_types.name_map.get(&variants) {
        Some(s) => {
            // create new alias from union
            create_alias(st, &name, s.to_owned());
            name
        }
        None => {
            // create new enum from union
            create_enum(st, &name, &variants);

            st.name_types.name_map.insert(variants, name.clone());
            name
        }
    })
}

fn create_alias(st: &mut FrontendState, name: &String, old_name: String) {
    st.segments.push(RustSegment::Alias(RustAlias {
        name: name.to_owned(),
        is_borrowed: false,
        ty: RustType::Custom(TypeName::new(old_name)),
    }));
}

fn create_enum(st: &mut FrontendState, name: &String, vs: &[String]) {
    st.segments.push(RustSegment::Enum(RustEnum::from_members(
        name.to_owned(),
        vs.iter().map(|v| {
            let renamed = v
                .split(&['-', ' ', '_'])
                .map(|term| {
                    let mut term = term.to_ascii_lowercase();
                    if let Some(c) = term.chars().next() {
                        let capital_ch = c.to_ascii_uppercase();
                        term.replace_range(..1, &capital_ch.to_string());
                    }
                    term
                })
                .collect::<Vec<_>>()
                .concat();
            let mut attr = RustVariantAttrs::Default;
            if v != &renamed {
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

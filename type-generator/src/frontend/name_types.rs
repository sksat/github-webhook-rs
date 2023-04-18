use std::collections::HashMap;

use crate::{
    ir::{
        RenameRule, RustEnum, RustEnumMember, RustEnumMemberKind, RustSegment, RustVariantAttr,
        RustVariantAttrs, SerdeVariantAttr, TypeName,
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
    TypeName::new(match st.name_types.name_map.get(&variants) {
        Some(s) => s.to_owned(),
        None => {
            // create new enum from union
            let mut prop = prop_name.to_owned();
            RenameRule::SnakeCase.convert_to_pascal(&mut prop);
            let name = format!("{struct_name}{prop}LiteralUnion");

            create_enum(st, &name, &variants);

            st.name_types.name_map.insert(variants, name.clone());
            name
        }
    })
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

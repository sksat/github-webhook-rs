use std::collections::HashMap;

use crate::ir::{TypeName, *};

/// add unnamed types to segments, and move references
pub fn name_types(segments: &mut Vec<RustSegment>) {
    // set of literals -> rust type name
    let mut name_map: HashMap<Vec<String>, String> = HashMap::new();
    let mut extra_segments = Vec::new();
    for segment in segments.iter_mut() {
        match segment {
            RustSegment::Struct(s) => {
                for memb in &mut s.member {
                    let ty = &mut memb.ty.ty;
                    match ty {
                        RustType::String { .. }
                        | RustType::Number
                        | RustType::Boolean
                        | RustType::Custom(_)
                        | RustType::Array(_)
                        | RustType::Unit
                        | RustType::Unknown
                        | RustType::UnknownLiteral
                        | RustType::UnknownIntersection
                        | RustType::UnknownUnion => (),

                        RustType::StringLiteralUnion(vs) => {
                            *ty = RustType::Custom(TypeName::new(
                                name_map
                                    .entry(vs.to_vec())
                                    .or_insert_with(|| {
                                        // create new enum from union
                                        let mut name = memb.name.to_owned();
                                        RenameRule::SnakeCase.convert_to_pascal(&mut name);
                                        name.push_str("LiteralUnion");

                                        create_enum(&mut extra_segments, &name, vs);

                                        name
                                    })
                                    .to_owned(),
                            ));
                        }
                    }
                }
            }
            RustSegment::Enum(_) | RustSegment::Alias(_) => (),
        }
    }
    segments.extend(extra_segments.into_iter());
}

fn create_enum(extra_segments: &mut Vec<RustSegment>, name: &String, vs: &mut Vec<String>) {
    extra_segments.push(RustSegment::Enum(RustEnum::from_members(
        name.to_owned(),
        std::mem::take(vs).into_iter().map(|v| {
            // FIXME: `v` is possibly not valid rust ident
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
            if v != renamed {
                attr.add_attr(RustVariantAttr::Serde(SerdeVariantAttr::Rename(v)));
            }
            RustEnumMember {
                attr,
                kind: RustEnumMemberKind::Nullary(TypeName::new(renamed)),
            }
        }),
    )));
}

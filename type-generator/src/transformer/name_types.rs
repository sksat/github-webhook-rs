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

                                        extra_segments.push(RustSegment::Enum(
                                            RustEnum::from_members(
                                                name.to_owned(),
                                                std::mem::take(vs).into_iter().map(|v| {
                                                    // FIXME: `v` is possibly not valid rust ident
                                                    RustEnumMember::Nullary(TypeName::new(v))
                                                }),
                                            ),
                                        ));

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

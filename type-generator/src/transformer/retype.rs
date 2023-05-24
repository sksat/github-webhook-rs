use std::collections::HashMap;

use crate::ir::{RustEnumMemberKind, RustSegment, RustType};

pub fn retype(segments: &mut Vec<RustSegment>, map: HashMap<String, RustType>) {
    trait Retype {
        fn retype_custom(&mut self, map: &HashMap<String, RustType>);
    }
    impl Retype for RustType {
        fn retype_custom(&mut self, map: &HashMap<String, RustType>) {
            match self {
                RustType::Array(t) => {
                    t.retype_custom(map);
                }
                RustType::Map(t1, t2) => {
                    t1.retype_custom(map);
                    t2.retype_custom(map);
                }
                RustType::Custom(n) => {
                    if let Some(ty) = map.get(&n.name) {
                        *self = ty.to_owned();
                    }
                }
                RustType::String { .. }
                | RustType::Number
                | RustType::Boolean
                | RustType::Unit
                | RustType::Unknown
                | RustType::UnknownLiteral
                | RustType::UnknownIntersection => (),
            }
        }
    }
    for segment in segments {
        match segment {
            RustSegment::Struct(s) => {
                for m in &mut s.member {
                    m.ty.ty.retype_custom(&map);
                }
            }
            RustSegment::Enum(e) => {
                for m in &mut e.member {
                    match &mut m.kind {
                        RustEnumMemberKind::Nullary(..) => (),
                        RustEnumMemberKind::Unary(u) => {
                            u.retype_custom(&map);
                        }
                        RustEnumMemberKind::UnaryNamed { type_name, .. } => {
                            type_name.retype_custom(&map);
                        }
                    }
                }
            }
            RustSegment::Alias(a) => {
                a.ty.retype_custom(&map);
            }
        }
    }
}

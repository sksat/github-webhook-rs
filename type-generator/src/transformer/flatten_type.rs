use std::collections::HashMap;

use crate::ir::{RustFieldAttr, RustMemberType, RustSegment, RustType, SerdeFieldAttr};

use super::retype;

/// flattens type with only one field attributed `#[serde(flatten)]`
pub fn flatten_type(segments: &mut Vec<RustSegment>) {
    let mut retype_map: HashMap<String, RustType> = HashMap::new();
    segments.retain_mut(|segment| match segment {
        RustSegment::Struct(s) => {
            if s.member.len() == 1 {
                let r = s.member.first().unwrap();
                if r.attr
                    .as_inner()
                    .contains(&RustFieldAttr::Serde(SerdeFieldAttr::Flatten))
                {
                    let RustMemberType { ty, is_optional } = &r.ty;
                    assert!(!is_optional);
                    retype_map.insert(s.name.to_owned(), ty.to_owned());
                    return false;
                }
            }
            true
        }
        _ => true,
    });
    retype::retype(segments, retype_map)
}

use std::collections::HashMap;

use crate::ir::{LiteralKeyMap, RustSegment, RustStructAttr, SerdeContainerAttr};

/// find tag from rust enum and attr to it.
pub fn adapt_internal_tag(segment: &mut RustSegment, lkm: &LiteralKeyMap) -> Option<()> {
    if let RustSegment::Enum(re) = segment {
        let mut cand_props: HashMap<String, String> = Default::default();
        for memb in &re.member {
            let tname = &memb.kind.as_unary()?.as_custom()?.name;
            let props = lkm.get(tname)?;
            if cand_props.is_empty() {
                cand_props = props.clone();
                continue;
            }
            // calc intersection of all enum members
            cand_props.retain(|k, _| props.contains_key(k));
            if cand_props.is_empty() {
                return None;
            }
        }
        assert!(!cand_props.is_empty());
        if cand_props.len() != 1 {
            return None;
        }
        let tag_name = cand_props.keys().next().unwrap().to_owned();

        // validate and collect tag name
        let mut variant_names = Vec::new();

        for memb in &re.member {
            let inter = &memb.kind.as_unary().unwrap().as_custom().unwrap().name;
            let variant_name = lkm.get(inter).unwrap().get(&tag_name).unwrap().to_owned();
            if variant_names.contains(&variant_name) {
                return None;
            }
            variant_names.push(variant_name);
        }

        for (memb, variant_name) in re.member.iter_mut().zip(variant_names) {
            memb.kind.name_unary(variant_name);
        }
        re.attr
            .retain(|r| !matches!(r, RustStructAttr::Serde(SerdeContainerAttr::Untagged)));
        re.attr
            .add_attr(RustStructAttr::Serde(SerdeContainerAttr::Tag(tag_name)));
    }
    Some(())
}

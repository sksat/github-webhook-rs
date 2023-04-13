use std::collections::HashMap;

use crate::ir::*;

pub fn adapt_internal_tag(segment: &mut RustSegment, lkm: &LiteralKeyMap) -> Option<()> {
    if let RustSegment::Enum(re) = segment {
        let mut props: HashMap<String, String> = Default::default();
        for memb in &re.member {
            let inter = memb.as_unary()?;
            let map = lkm.get(inter)?;
            if props.is_empty() {
                props = map.clone();
                continue;
            }
            props.retain(|k, _| map.contains_key(k));
            if props.is_empty() {
                return None;
            }
        }
        // assert !props.is_empty()
        assert_eq!(props.len(), 1);
        let tag_name = props.keys().next().unwrap().to_owned();
        for memb in &mut re.member {
            let inter = memb.as_unary().unwrap();
            let variant_name = lkm.get(inter).unwrap().get(&tag_name).unwrap().to_owned();
            memb.name_unary(variant_name);
        }
        re.attr
            .add_attr(RustStructAttr::Serde(SerdeContainerAttr::Tag(tag_name)));
    }
    Some(())
}

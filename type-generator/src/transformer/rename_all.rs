use crate::{
    case::{detect_case, CaseConvention},
    ir::{RustEnumMemberKind, RustSegment, RustStructAttr, SerdeContainerAttr},
};

pub fn adapt_rename_all(segment: &mut RustSegment) -> Option<()> {
    if let RustSegment::Enum(re) = segment {
        let mut conv: Option<CaseConvention> = None;
        for memb in &re.member {
            let s = match &memb.kind {
                RustEnumMemberKind::Nullary(v) => v,
                RustEnumMemberKind::Unary(v) => &v.name,
                RustEnumMemberKind::UnaryNamed { variant_name, .. } => variant_name,
            };
            match conv.as_mut() {
                Some(conv) => {
                    let new = detect_case(s);
                    conv.cast(new)?
                }
                None => {
                    conv = Some(detect_case(s));
                }
            };
        }
        let rr = conv?.into_rename_rule();
        if rr.is_pascal_case() {
            return None;
        }
        for memb in &mut re.member {
            match &mut memb.kind {
                RustEnumMemberKind::Unary(v) => {
                    let type_name = v.to_owned();
                    rr.convert_to_pascal(&mut v.name);
                    memb.kind = RustEnumMemberKind::UnaryNamed {
                        variant_name: v.name.to_owned(),
                        type_name,
                    };
                }
                RustEnumMemberKind::Nullary(variant_name) => {
                    rr.convert_to_pascal(variant_name);
                }
                RustEnumMemberKind::UnaryNamed { variant_name, .. } => {
                    rr.convert_to_pascal(variant_name);
                }
            };
        }
        re.attr
            .add_attr(RustStructAttr::Serde(SerdeContainerAttr::RenameAll(rr)));
    }
    Some(())
}

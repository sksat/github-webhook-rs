use swc_common::EqIgnoreSpan;

pub struct Merged<'a> {
    pub intersection: Vec<&'a swc_ecma_ast::TsTypeElement>,
    pub diffs: Vec<Vec<&'a swc_ecma_ast::TsTypeElement>>,
}

pub fn merge_union_type_lits<'input>(
    variants: &[&'input swc_ecma_ast::TsTypeLit],
) -> Merged<'input> {
    let mut intersection: Vec<_> = variants.first().unwrap().members.iter().collect();
    let mut diffs = vec![vec![]];
    for variant in variants[1..].iter() {
        let mut diff: Vec<_> = variant.members.iter().collect();
        intersection.retain(|i| {
            if let Some(index) = diff.iter().position(|d| i.eq_ignore_span(d)) {
                // `i` remains in common, so remove it from `diff`
                diff.remove(index);
                true
            } else {
                // `i` turns out to be specific to former variants
                for diff in diffs.iter_mut() {
                    diff.push(*i)
                }
                false
            }
        });
        diffs.push(diff);
    }
    Merged {
        intersection,
        diffs,
    }
}

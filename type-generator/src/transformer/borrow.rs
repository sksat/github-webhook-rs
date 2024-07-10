use std::collections::HashSet;

use crate::{
    dag::CoDirectedAcyclicGraph,
    ir::{
        RustFieldAttr, RustSegment, RustType, RustVariantAttr, SerdeFieldAttr, SerdeVariantAttr,
        TypeName,
    },
};

pub fn adapt_borrow(segments: &mut [RustSegment], type_deps: &CoDirectedAcyclicGraph<usize>) {
    let mut decorated: HashSet<String> = HashSet::new();
    let sorted = match type_deps.co_topo_sort() {
        Ok(s) => s,
        Err(cy) => {
            let mut msg = segments.get(cy[0]).unwrap().name().to_owned();
            for index in cy {
                let seg = segments.get(index).unwrap().name();
                msg.push_str(&format!("\n -> {}", seg));
            }
            panic!("cyclic dependency detected (this is a bug):\n{}", msg);
        }
    };
    for index in sorted {
        let seg = segments.get_mut(index).unwrap();
        fn borrow_typename(
            TypeName { name, is_borrowed }: &mut TypeName,
            did_borrow: &mut bool,
            decorated: &HashSet<String>,
        ) {
            if decorated.contains(name) {
                *is_borrowed = true;
                *did_borrow = true;
            }
        }
        fn borrow_type(ty: &mut RustType, did_borrow: &mut bool, decorated: &HashSet<String>) {
            match ty {
                RustType::String { is_borrowed } => {
                    *is_borrowed = true;
                    *did_borrow = true;
                }
                RustType::Number => (),
                RustType::Boolean => (),
                RustType::Custom(t) => {
                    borrow_typename(t, did_borrow, decorated);
                }
                RustType::Array(t) => borrow_type(t, did_borrow, decorated),
                RustType::Unit => (),
                RustType::Unknown => (),
                RustType::UnknownLiteral => (),
                RustType::UnknownIntersection => (),
                RustType::Map(t1, t2) => {
                    borrow_type(t1, did_borrow, decorated);
                    borrow_type(t2, did_borrow, decorated);
                }
            }
        }
        let mut did_borrow = false;
        match seg {
            RustSegment::Struct(s) => {
                for mem in &mut s.member {
                    borrow_type(&mut mem.ty.ty, &mut did_borrow, &decorated);
                }
                if did_borrow {
                    for mem in &mut s.member {
                        if mem.ty.ty.is_borrowed() {
                            mem.attr
                                .add_attr(RustFieldAttr::Serde(SerdeFieldAttr::Borrow));
                        }
                    }
                    s.is_borrowed = true;
                    decorated.insert(s.name.to_owned());
                }
            }
            RustSegment::Enum(e) => {
                for mem in &mut e.member {
                    if let Some(t) = mem.kind.as_type_mut() {
                        borrow_type(t, &mut did_borrow, &decorated);
                    }
                }
                if did_borrow {
                    for mem in &mut e.member {
                        if let Some(t) = mem.kind.as_type() {
                            if t.is_borrowed() {
                                mem.attr
                                    .add_attr(RustVariantAttr::Serde(SerdeVariantAttr::Borrow));
                            }
                        }
                    }
                    e.is_borrowed = true;
                    decorated.insert(e.name.to_owned());
                }
            }
            RustSegment::Alias(a) => {
                let ty = &mut a.ty;
                borrow_type(ty, &mut did_borrow, &decorated);
                if did_borrow {
                    a.is_borrowed = true;
                    decorated.insert(a.name.to_owned());
                }
            }
        }
    }
}

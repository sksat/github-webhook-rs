pub mod merge_union_type_lits;
pub mod name_types;

use once_cell::sync::Lazy;
use std::{borrow::Cow, collections::HashMap};

use crate::{
    case,
    frontend::merge_union_type_lits::Merged,
    ir::{
        LiteralKeyMap, RustAlias, RustComment, RustContainerAttrs, RustEnum, RustEnumMember,
        RustEnumMemberKind, RustFieldAttr, RustFieldAttrs, RustMemberType, RustSegment, RustStruct,
        RustStructAttr, RustStructMember, RustType, RustVariantAttrs, SerdeContainerAttr,
        SerdeFieldAttr, TypeName,
    },
};

pub fn interface2struct<'input>(
    st: &mut FrontendState<'input, '_>,
    interface: &'input swc_ecma_ast::TsInterfaceDecl,
    comment: Option<RustComment>,
    lkm: &mut LiteralKeyMap,
) {
    let name = interface.id.sym.as_ref();
    let ibody = &interface.body.body;
    let mut ctxt = TypeConvertContext::from_path(Path::from_iter([Cow::Borrowed(name)]));

    let member = ibody
        .iter()
        .map(|m| m.as_ts_property_signature().unwrap())
        .map(|prop| ts_prop_signature(prop, st, &mut ctxt, name, lkm))
        .collect();

    let name = name.to_owned();
    let s = RustStruct {
        attr: RustContainerAttrs::new(),
        name,
        comment,
        is_borrowed: false,
        member,
    };
    st.segments.push(RustSegment::Struct(s));
}

pub fn ts_prop_signature<'input>(
    prop: &'input swc_ecma_ast::TsPropertySignature,
    st: &mut FrontendState<'input, '_>,
    ctxt: &mut TypeConvertContext<'input>,
    name: &str,
    lkm: &mut HashMap<String, HashMap<String, String>>,
) -> RustStructMember {
    let comment = st.get_comment(prop.span.lo);
    let mut is_optional = prop.optional;
    let mut pkey: &str = match &*prop.key {
        swc_ecma_ast::Expr::Ident(pkey) => &pkey.sym,
        swc_ecma_ast::Expr::Lit(swc_ecma_ast::Lit::Str(k)) => &k.value,
        _ => unreachable!(),
    };
    let mut attr = RustFieldAttrs::new();
    // avoid conflict to Rust reserved word
    static RENAME_RULES: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
        HashMap::from_iter([
            ("type", "type_"),
            ("ref", "ref_"),
            ("self", "self_"),
            ("+1", "plus_1"),
            ("-1", "minus_1"),
        ])
    });
    if let Some(renamed) = RENAME_RULES.get(pkey) {
        attr.add_attr(RustFieldAttr::Serde(SerdeFieldAttr::Rename(
            pkey.to_owned(),
        )));
        pkey = renamed;
    }
    let ptype = &prop.type_ann.as_ref().unwrap().type_ann;
    let mut ctxt = ctxt.clone();
    ctxt.projection(Cow::Borrowed(pkey));

    let (is_optional2, ty) = ts_type_to_rs(st, &mut Some(ctxt), ptype, None, lkm);
    is_optional |= is_optional2;

    fn extract_literal_type(ptype: &swc_ecma_ast::TsType) -> Option<&str> {
        ptype.as_ts_lit_type()?.lit.as_str()?.raw.as_deref()
    }
    if let Some(lit) = extract_literal_type(ptype) {
        lkm.entry(name.to_owned()).or_default().insert(
            pkey.to_owned(),
            lit.strip_prefix('\"')
                .unwrap()
                .strip_suffix('\"')
                .unwrap()
                .to_owned(),
        );
    }
    RustStructMember {
        ty: RustMemberType { ty, is_optional },
        name: pkey.to_string(),
        attr,
        comment,
    }
    //dbg!(prop);

    //let pkey = if let Some(pkey) = &prop.key.as_ident() {
    //    // ident
    //    &pkey.sym
    //} else {
    //    // interface { "+1": number; "-1": number; }
    //    // TODO: parse
    //    dbg!(prop);
    //    "+1"
    //};
}

pub fn ts_index_signature<'input>(
    index: &'input swc_ecma_ast::TsIndexSignature,
    comment: Option<RustComment>,
    st: &mut FrontendState<'input, '_>,
    ctxt: &mut TypeConvertContext<'input>,
    lkm: &mut HashMap<String, HashMap<String, String>>,
) -> RustStructMember {
    assert!(index.params.len() == 1);
    let param = index.params.first().unwrap();
    let ident = param.as_ident().expect("key is string");
    let mut ctxt = Some(ctxt.clone());
    let (_, key_ty) = ts_type_to_rs(
        st,
        &mut ctxt,
        &ident.type_ann.as_ref().unwrap().type_ann,
        None,
        lkm,
    );
    let (_, value_ty) = ts_type_to_rs(
        st,
        &mut ctxt,
        &index.type_ann.as_ref().unwrap().type_ann,
        None,
        lkm,
    );
    RustStructMember {
        ty: RustMemberType {
            ty: RustType::Map(Box::new(key_ty), Box::new(value_ty)),
            is_optional: false,
        },
        name: ident.sym.to_string(),
        attr: RustFieldAttrs::from_attr(RustFieldAttr::Serde(SerdeFieldAttr::Flatten)),
        comment,
    }
}

pub fn tunion2enum<'input>(
    st: &mut FrontendState<'input, '_>,
    name: &'input str,
    tsuoi: &'input swc_ecma_ast::TsUnionOrIntersectionType,
    comment: Option<RustComment>,
    lkm: &mut LiteralKeyMap,
    from_alias: bool,
) {
    union_or_intersection(
        st,
        Some(TypeConvertContext {
            path: vec![Cow::Borrowed(name)],
            granted_name: Some(name),
            from_alias,
            ..Default::default()
        }),
        tsuoi,
        comment,
        &mut false,
        lkm,
    );
}

fn union_or_intersection<'input>(
    st: &mut FrontendState<'input, '_>,
    mut ctxt: Option<TypeConvertContext<'input>>,
    tsuoi: &'input swc_ecma_ast::TsUnionOrIntersectionType,
    comment: Option<RustComment>,
    nullable: &mut bool,
    lkm: &mut LiteralKeyMap,
) -> RustType {
    use swc_ecma_ast::TsKeywordTypeKind;
    use swc_ecma_ast::TsUnionOrIntersectionType;

    match tsuoi {
        TsUnionOrIntersectionType::TsUnionType(tunion) => {
            // nullable check
            let mut types: Vec<&Box<swc_ecma_ast::TsType>> = tunion.types.iter().collect();
            // obtain non-null types within union, judging if there is null keyword
            types.retain(|t| {
                if let Some(tkey) = t.as_ts_keyword_type() {
                    if tkey.kind == TsKeywordTypeKind::TsNullKeyword {
                        *nullable = true;
                        return false;
                    }
                }
                true
            });

            assert!(!types.is_empty());
            if types.len() == 1 {
                let (n, t) = ts_type_to_rs(st, &mut ctxt, types[0], comment, lkm);
                *nullable |= n;
                return t;
            }

            // strings check: "Bot" | "User" | "Organization"
            if let Some(mut variants) = types
                .iter()
                .map(|t| Some(t.as_ts_lit_type()?.lit.as_str()?.value.as_ref()))
                .collect::<Option<Vec<&str>>>()
            {
                variants.sort();
                let ct = ctxt.as_mut().expect("provide ctxt");
                let tn = name_types::string_literal_union(st, variants, comment, ct);
                return RustType::Custom(tn);
                //TODO: comment strs  // {:?}", strs));
            }

            if types.len() >= 2 {
                if let Some(variants) = types
                    .iter()
                    .map(|t| t.as_ts_type_lit())
                    .collect::<Option<Vec<_>>>()
                {
                    let ctxt = ctxt.as_mut().unwrap();
                    let Merged {
                        intersection,
                        diffs,
                    } = merge_union_type_lits::merge_union_type_lits(&variants);
                    let mut s =
                        name_types::type_literal(st, intersection.into_iter(), None, ctxt, lkm);
                    let member = diffs
                        .into_iter()
                        .map(|d| {
                            let s = name_types::type_literal(st, d.into_iter(), None, ctxt, lkm);
                            RustEnumMemberKind::Unary(st.push_segment(RustSegment::Struct(s)))
                                .into()
                        })
                        .collect();
                    let ty = st.push_segment(RustSegment::Enum(RustEnum {
                        name: ctxt.create_ident_with(Some(vec!["DistinctUnion".to_string()])),
                        attr: RustContainerAttrs::from_attr(RustStructAttr::Serde(
                            SerdeContainerAttr::Untagged,
                        )),
                        comment: None,
                        is_borrowed: false,
                        member,
                    }));
                    s.member.push(RustStructMember {
                        ty: RustMemberType {
                            ty,
                            is_optional: false,
                        },
                        name: "distinct".to_owned(),
                        attr: RustFieldAttrs::from_attr(RustFieldAttr::Serde(
                            SerdeFieldAttr::Flatten,
                        )),
                        comment,
                    });
                    return st.push_segment(RustSegment::Struct(s));
                }
            }

            let type_convert_context = ctxt.as_mut().unwrap();
            let mut name = type_convert_context.create_ident();
            if !type_convert_context.from_alias {
                name.push_str("Union");
            }
            let variants: Vec<_> = types
                .iter()
                .map(|t| {
                    let (_, t) = ts_type_to_rs(st, &mut ctxt, t, None, lkm);
                    RustEnumMember {
                        attr: RustVariantAttrs::new(),
                        kind: RustEnumMemberKind::Unary(t),
                    }
                })
                .collect();

            st.segments.push(RustSegment::Enum(RustEnum {
                attr: RustContainerAttrs::from_attr(RustStructAttr::Serde(
                    SerdeContainerAttr::Untagged,
                )),
                name: name.to_owned(),
                comment,
                is_borrowed: false,
                member: variants,
            }));
            RustType::Custom(TypeName::new(name))
        }
        TsUnionOrIntersectionType::TsIntersectionType(tints) => {
            if tints.types.len() == 2 {
                let mut iter = tints.types.iter();
                // if types consist of type literal and type ref, create new struct with serde flatten attribute
                let tref = iter.next().unwrap().as_ts_type_ref();
                let tlit = iter.next().unwrap().as_ts_type_lit();
                if let (Some(tref), Some(tlit)) = (tref, tlit) {
                    let name = tref.type_name.as_ident().unwrap().sym.as_ref();
                    let mut str = name_types::type_literal(
                        st,
                        tlit.members.iter(),
                        None,
                        &mut ctxt.unwrap(),
                        lkm,
                    );

                    if str.member.iter().all(|m| m.ty.is_unknown()) {
                        let struct_name = str.name.to_owned();
                        let a = RustAlias {
                            name: struct_name.to_owned(),
                            is_borrowed: false,
                            comment: None,
                            ty: RustType::Custom(TypeName::new(name.to_owned())),
                        };
                        st.segments.push(RustSegment::Alias(a));
                        return RustType::Custom(TypeName::new(struct_name));
                    } else {
                        // add flatten attributed field to struct
                        let mut field_name = name.to_owned();
                        case::CaseConvention::Pascal
                            .into_rename_rule()
                            .convert_to_snake(&mut field_name);
                        str.member.push(RustStructMember {
                            attr: RustFieldAttrs::from_attr(RustFieldAttr::Serde(
                                SerdeFieldAttr::Flatten,
                            )),
                            name: field_name,
                            ty: RustMemberType {
                                is_optional: false,
                                ty: RustType::Custom(TypeName::new(name.to_owned())),
                            },
                            comment,
                        });
                        let struct_name = str.name.to_owned();
                        st.segments.push(RustSegment::Struct(str));
                        return RustType::Custom(TypeName::new(struct_name));
                    }
                }
            }
            // dbg!(tints);
            //todo!();
            RustType::UnknownIntersection
        }
    }
}

pub struct FrontendState<'input, 'output> {
    pub segments: &'output mut Vec<RustSegment>,
    pub comments: &'input swc_common::comments::SingleThreadedComments,
    pub name_types: name_types::State<'input>,
}

impl<'input, 'output> FrontendState<'input, 'output> {
    pub fn push_segment(&mut self, value: RustSegment) -> RustType {
        let name = value.name().to_owned();
        self.segments.push(value);
        RustType::Custom(TypeName::new(name))
    }
    pub fn get_comment(&self, pos: swc_common::BytePos) -> Option<RustComment> {
        self.comments
            .with_leading(pos, |cs| cs.last().map(|c| strip_docs(&c.text)))
    }
}

fn ts_keyword_type_to_rs(typ: &swc_ecma_ast::TsKeywordType) -> RustType {
    use swc_ecma_ast::TsKeywordTypeKind;
    match typ.kind {
        TsKeywordTypeKind::TsStringKeyword => RustType::String { is_borrowed: false },
        TsKeywordTypeKind::TsNumberKeyword => RustType::Number,
        TsKeywordTypeKind::TsBooleanKeyword => RustType::Boolean,
        TsKeywordTypeKind::TsNullKeyword => RustType::Unit,
        TsKeywordTypeKind::TsUnknownKeyword => RustType::Unknown,
        _ => {
            unimplemented!("{:?}", typ.kind);
        }
    }
}

pub type Path<'a> = Vec<Cow<'a, str>>;

#[derive(Clone, Default)]
pub struct TypeConvertContext<'a> {
    path: Path<'a>,
    granted_name: Option<&'a str>,
    from_alias: bool,
    /// prevent duplicate field name
    duplicate_counter: usize,
}

impl<'a> TypeConvertContext<'a> {
    pub fn from_path(path: Path<'a>) -> Self {
        Self {
            path,
            ..Default::default()
        }
    }

    /// struct field
    pub fn projection(&mut self, field: Cow<'a, str>) {
        self.path.push(field);
        self.granted_name = None;
        self.from_alias = false;
        self.duplicate_counter = 0;
    }

    fn to_pascal(&self) -> Vec<String> {
        self.path
            .iter()
            .map(|p| {
                let mut p = p.to_string();
                case::detect_case(&p)
                    .into_rename_rule()
                    .convert_to_pascal(&mut p);
                p
            })
            .collect()
    }

    /// create identifier from path
    pub fn create_ident(&mut self) -> String {
        self.create_ident_with(None)
    }

    pub fn create_ident_with(&mut self, additional: Option<Vec<String>>) -> String {
        if let Some(name) = self.granted_name.take() {
            return name.to_owned();
        }
        let mut v = self.to_pascal();
        if let Some(additional) = additional {
            v.extend(additional);
        } else {
            if self.from_alias || self.duplicate_counter != 0 {
                let suffix = self.duplicate_counter + self.from_alias as usize;
                v.push(suffix.to_string());
            }
            self.duplicate_counter += 1;
        }
        v.concat()
    }
}

pub fn ts_type_to_rs<'input>(
    st: &mut FrontendState<'input, '_>,
    ctxt: &mut Option<TypeConvertContext<'input>>,
    mut typ: &'input swc_ecma_ast::TsType,
    comment: Option<RustComment>,
    lkm: &mut HashMap<String, HashMap<String, String>>,
) -> (bool, RustType) {
    let mut nullable = false;

    // peel off parenthesis (that only exist for precedence)
    while let swc_ecma_ast::TsType::TsParenthesizedType(t) = typ {
        typ = &*t.type_ann;
    }

    let typ = match typ {
        swc_ecma_ast::TsType::TsKeywordType(tk) => ts_keyword_type_to_rs(tk),
        swc_ecma_ast::TsType::TsUnionOrIntersectionType(tsuoi) => {
            union_or_intersection(st, ctxt.to_owned(), tsuoi, comment, &mut nullable, lkm)
        }
        swc_ecma_ast::TsType::TsLitType(_tslit) => RustType::UnknownLiteral,
        swc_ecma_ast::TsType::TsTypeRef(tref) => {
            let id = tref.type_name.as_ident().unwrap().sym.as_ref();
            RustType::Custom(TypeName {
                name: id.to_owned(),
                is_borrowed: false,
            })
        }
        swc_ecma_ast::TsType::TsArrayType(tarray) => {
            let (_n, etype) = ts_type_to_rs(st, ctxt, &tarray.elem_type, comment, lkm);
            //format!("Vec<{etype}>")
            RustType::Array(Box::new(etype))
        }
        swc_ecma_ast::TsType::TsTypeLit(tlit) => {
            let s = name_types::type_literal(
                st,
                tlit.members.iter(),
                comment,
                ctxt.as_mut().unwrap(),
                lkm,
            );
            let name = s.name.clone();
            st.segments.push(RustSegment::Struct(s));

            RustType::Custom(TypeName::new(name))
        }
        swc_ecma_ast::TsType::TsTupleType(t) => {
            // empty array type is treated as tuple type with 0 elements
            if t.elem_types.is_empty() {
                RustType::Unit
            } else {
                RustType::Unknown
            }
        }
        _ => {
            //dbg!(typ);
            //todo!();
            RustType::Unknown
        }
    };

    (nullable, typ)
}

pub fn strip_docs(comment: &str) -> RustComment {
    let comment = comment.trim_start_matches('*');
    let comment = comment.trim_start();
    let comment = comment.trim_end();
    RustComment(
        comment
            .split('\n')
            .map(|s| s.trim_start().trim_start_matches("* "))
            .collect::<Vec<_>>()
            .join(" "),
    )
}

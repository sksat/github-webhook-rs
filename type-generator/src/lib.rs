use std::{fmt::Write, path::PathBuf};

use std::path::Path;

use anyhow::Result;

use swc_common::{
    self,
    errors::{ColorConfig, Handler},
    sync::Lrc,
    SourceMap,
};

use swc_ecma_parser::{lexer::Lexer, Capturing, Parser, StringInput, Syntax};

pub enum RustType {
    String,
    Number,
    Boolean,
    Custom(String),
    Array(Box<RustType>),
    Empty, // ()
    Unknwon,
    UnknwonLiteral,
    UnknwonIntersection,
    UnknwonUnion,
}

impl RustType {
    pub fn is_unknown(&self) -> bool {
        match &self {
            RustType::Unknwon
            | RustType::UnknwonLiteral
            | RustType::UnknwonIntersection
            | RustType::UnknwonUnion => true,
            RustType::Array(t) => t.is_unknown(),
            _ => false,
        }
    }
}

pub struct RustStruct {
    pub name: String,
    pub member: Vec<RustStructMember>,
}

pub struct RustEnum {
    pub name: String,
    pub member: Vec<RustEnumMember>,
}

pub struct RustStructMember {
    pub name: String,
    pub typ: RustType,
    pub optional: bool,
    pub comment: Option<String>,
}

pub enum RustEnumMember {
    TypeAlias(String),
    EnumVariant(String),
}

impl std::string::ToString for RustType {
    fn to_string(&self) -> String {
        match self {
            RustType::String => "String".to_string(),
            RustType::Number => "usize".to_string(),
            RustType::Boolean => "bool".to_string(),
            RustType::Custom(s) => s.clone(),
            RustType::Array(t) => format!("Vec<{}>", t.to_string()),
            RustType::Empty => "()".to_string(),
            RustType::Unknwon => "Unknwon".to_string(),
            RustType::UnknwonLiteral => "UnknwonLiteral".to_string(),
            RustType::UnknwonIntersection => "UnknwonIntersection".to_string(),
            RustType::UnknwonUnion => "UnknwonUnion".to_string(),
        }
    }
}

impl std::fmt::Display for RustStructMember {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let name = &self.name;

        // comment unknwon
        if self.typ.is_unknown() {
            write!(f, "// ")?;
        }

        let typ = self.typ.to_string();

        let typ = if self.optional {
            format!("Option<{}>", typ)
        } else {
            typ
        };

        if let Some(comment) = &self.comment {
            write!(f, "pub {name}: {typ}, // {comment}")
        } else {
            write!(f, "pub {name}: {typ},")
        }
    }
}

impl std::fmt::Display for RustStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "#[derive(Debug, Deserialize)]")?;
        writeln!(f, "pub struct {} {{", self.name)?;

        for m in &self.member {
            writeln!(f, "  {}", m)?;
        }

        writeln!(f, "}}")
    }
}

impl std::fmt::Display for RustEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "#[derive(Debug, Deserialize)]")?;
        writeln!(f, "pub enum {} {{", self.name)?;

        for m in &self.member {
            write!(f, "  ")?;
            match m {
                RustEnumMember::EnumVariant(v) => write!(f, "{v}")?,
                RustEnumMember::TypeAlias(a) => write!(f, "{a}({a})")?,
            }
            writeln!(f, ",")?;
        }

        writeln!(f, "}}")
    }
}

fn interface2struct(interface: &swc_ecma_ast::TsInterfaceDecl) -> Result<RustStruct> {
    let name = interface.id.sym.to_string();
    let ibody = &interface.body.body;

    let mut rmember = Vec::new();

    for member in ibody {
        let prop = member.as_ts_property_signature().unwrap();
        if prop.optional {
            // TODO: optional or skip
            continue;
        }
        //dbg!(prop);

        let pkey: &str = match &*prop.key {
            swc_ecma_ast::Expr::Ident(pkey) => &pkey.sym,
            swc_ecma_ast::Expr::Lit(pkey) => match pkey {
                swc_ecma_ast::Lit::Str(_pkey) => {
                    // TODO: use &pkey.value.as_ref()
                    continue;
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        };
        //let pkey = if let Some(pkey) = &prop.key.as_ident() {
        //    // ident
        //    &pkey.sym
        //} else {
        //    // interface { "+1": number; "-1": number; }
        //    // TODO: parse
        //    dbg!(prop);
        //    "+1"
        //};

        // avoid conflict to Rust reserved word
        let pkey = match pkey {
            "type" => "type_",
            "ref" => "ref_",
            _ => pkey,
        };

        let ptype = &prop.type_ann.as_ref().unwrap().type_ann;
        let (optional, typ) = ts_type_to_rs(ptype);

        rmember.push(RustStructMember {
            typ,
            name: pkey.to_string(),
            optional,
            comment: None,
        });
    }

    Ok(RustStruct {
        name,
        member: rmember,
    })
}

fn tunion2enum(name: &str, tunion: &swc_ecma_ast::TsUnionType) -> Result<RustEnum> {
    let mut member = Vec::new();
    for t in &tunion.types {
        match &**t {
            swc_ecma_ast::TsType::TsTypeRef(tref) => {
                // export type Hoge = Fuga | Fuge;
                let i = &tref.type_name.as_ident().unwrap();
                let sym = i.sym.to_string();
                member.push(RustEnumMember::TypeAlias(sym));
                //write!(out, "{}({})", &i.sym, &i.sym)?;
            }
            swc_ecma_ast::TsType::TsLitType(tlit) => {
                // export type BranchProtectionRuleEnforcementLevel =
                //   | "off" | "none_admins" | "everyone";
                let s = match tlit.lit {
                    swc_ecma_ast::TsLit::Str(ref s) => s.value.as_ref().to_string(),
                    _ => todo!(),
                };
                //write!(out, "{}", s)?;
                member.push(RustEnumMember::EnumVariant(s));
            }
            swc_ecma_ast::TsType::TsArrayType(_tarray) => {
                // export WebhookEvents = | ( | "a" | "b" | "c" )[] | ["*"];
                // (| "a" | "b" | "c")[] <-
                //dbg!(tarray);
            }
            swc_ecma_ast::TsType::TsTupleType(_ttuple) => {
                // export WebhookEvents = | ( | "a" | "b" | "c" )[] | ["*"];
                // ["*"] <-
                //dbg!(ttuple);
            }
            swc_ecma_ast::TsType::TsTypeLit(_ttlit) => {
                // export PullRequestReviewRequestRemovedEvent =
                // | {
                //     action: "review_request_removed";
                //     requested_reviewer: User;
                //    }
                //  | {
                //     action: "review_request_removed";
                //     requested_reviewer: Team;
                //  };
                //dbg!(ttlit);
            }
            _ => {
                dbg!(t);
                unreachable!()
            }
        }
        //writeln!(out, ",")?;
    }

    Ok(RustEnum {
        name: name.to_string(),
        member,
    })
}

pub fn dts2rs(dts_file: &PathBuf) -> Result<String> {
    let mut out = String::new();

    let cm: Lrc<SourceMap> = Default::default();
    let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));

    // Real usage
    let fm = cm
        .load_file(Path::new(dts_file))
        .unwrap_or_else(|_| panic!("failed to load {}", &dts_file.display()));

    let lexer = Lexer::new(
        Syntax::Typescript(Default::default()),
        Default::default(),
        StringInput::from(&*fm),
        None,
    );

    let capturing = Capturing::new(lexer);

    let mut parser = Parser::new_from(capturing);

    for e in parser.take_errors() {
        e.into_diagnostic(&handler).emit();
    }

    let module = parser
        .parse_typescript_module()
        .map_err(|e| e.into_diagnostic(&handler).emit())
        .expect("Failed to parse module.");

    //println!("Tokens: {:?}", parser.input().take());

    let ice = &module.body[1]
        .as_module_decl()
        .unwrap()
        .as_export_decl()
        .unwrap()
        .decl
        .as_ts_type_alias()
        .unwrap()
        .type_ann
        .as_ts_union_or_intersection_type()
        .unwrap()
        .as_ts_union_type()
        .unwrap()
        .types[0] // IssueCommentEvent
        .as_ts_type_ref()
        .unwrap()
        .type_name
        .as_ident()
        .unwrap()
        .as_ref();

    writeln!(out, "// {ice}")?;

    for b in &module.body {
        // skip statement
        if b.is_stmt() {
            continue;
        }
        let b = b.as_module_decl().unwrap();

        // skip not export decl
        let b = b.as_export_decl().unwrap();
        let decl = &b.decl;

        //dbg!(&decl);
        match decl {
            swc_ecma_ast::Decl::TsInterface(interface) => {
                //let name = interface.id.sym.as_ref();
                //match name {
                //    "CheckRunCreatedEvent" | "GollumEvent" => continue,
                //    _ => {}
                //}

                writeln!(out, "// ts interface")?;

                let rstruct = interface2struct(interface).unwrap();

                write!(out, "{}", rstruct)?;
            }
            swc_ecma_ast::Decl::TsTypeAlias(talias) => {
                writeln!(out, "// ts type alias")?;

                let a = talias.id.sym.as_ref();

                // lazy skip
                match a {
                    "WebhookEvents"
                    | "PullRequestReviewRequestRemovedEvent"
                    | "PullRequestReviewRequestedEvent" => {
                        continue; //return Err(anyhow!("lazy skip"));
                    }
                    _ => {}
                }

                let typ = &talias.type_ann;
                match &**typ {
                    swc_ecma_ast::TsType::TsTypeRef(tref) => {
                        writeln!(
                            out,
                            "pub type {a} = {};",
                            &tref.type_name.as_ident().unwrap().sym
                        )?;
                    }
                    swc_ecma_ast::TsType::TsUnionOrIntersectionType(tuoi) => {
                        let tunion = tuoi.as_ts_union_type().unwrap();

                        let renum = tunion2enum(a, tunion).unwrap();
                        writeln!(out, "{}", renum)?;
                    }
                    swc_ecma_ast::TsType::TsKeywordType(_tkey) => {
                        // export type Hoge = number;
                        let (_, typ) = ts_type_to_rs(typ);
                        writeln!(out, "pub type {a} = {};", typ.to_string())?;
                    }
                    swc_ecma_ast::TsType::TsArrayType(tarray) => {
                        // export type BranchProtectionRuleArray = string[];
                        //dbg!(tarray);
                        let (_, typ) = ts_type_to_rs(&tarray.elem_type);
                        writeln!(out, "pub type {a} = Vec<{}>;", typ.to_string())?;
                    }
                    swc_ecma_ast::TsType::TsTypeOperator(_toperator) => {
                        // export type WebhookEventName = keyof EventPayloadMap;
                        //dbg!(toperator);
                    }
                    _ => {
                        dbg!(typ);
                        unreachable!()
                    }
                }
            }
            _ => unreachable!(),
        }

        //println!("{}", b.is_export_decl());
    }

    Ok(out)
}

fn ts_type_to_rs(typ: &swc_ecma_ast::TsType) -> (bool, RustType) {
    use swc_ecma_ast::TsKeywordTypeKind;
    use swc_ecma_ast::TsUnionOrIntersectionType;

    let mut nullable = false;

    let typ = match typ {
        swc_ecma_ast::TsType::TsKeywordType(tk) => match tk.kind {
            TsKeywordTypeKind::TsStringKeyword => RustType::String,
            TsKeywordTypeKind::TsNumberKeyword => RustType::Number,
            TsKeywordTypeKind::TsBooleanKeyword => RustType::Boolean,
            TsKeywordTypeKind::TsNullKeyword => {
                nullable = true;
                RustType::Empty
            }
            _ => {
                dbg!(tk.kind);
                todo!()
            }
        },
        swc_ecma_ast::TsType::TsUnionOrIntersectionType(tsuoi) => {
            match tsuoi {
                TsUnionOrIntersectionType::TsUnionType(tunion) => {
                    // nullable check
                    let mut types = tunion.types.clone();
                    types.retain(|t| {
                        if let Some(tkey) = t.as_ts_keyword_type() {
                            if tkey.kind == TsKeywordTypeKind::TsNullKeyword {
                                nullable = true;
                                return false;
                            }
                        }
                        true
                    });

                    assert!(!types.is_empty());
                    if types.len() == 1 {
                        let (n, t) = ts_type_to_rs(&types[0]);
                        return (n || nullable, t);
                    }

                    // strings check: "Bot" | "User" | "Organization"
                    if types.iter().all(|t| {
                        if let Some(t) = t.as_ts_lit_type() {
                            if t.lit.is_str() {
                                return true;
                            }
                        }
                        false
                    }) {
                        let _strs = types
                            .iter()
                            .map(|t| {
                                t.as_ts_lit_type()
                                    .unwrap()
                                    .lit
                                    .as_str()
                                    .unwrap()
                                    .value
                                    .as_ref()
                            })
                            .collect::<Vec<&str>>();
                        return (nullable, RustType::String);
                        //TODO: comment strs  // {:?}", strs));
                    }

                    // other types
                    dbg!(types);

                    RustType::UnknwonUnion
                }
                TsUnionOrIntersectionType::TsIntersectionType(tints) => {
                    dbg!(tints);
                    //todo!();
                    RustType::UnknwonIntersection
                }
            }
        }
        swc_ecma_ast::TsType::TsLitType(_tslit) => RustType::UnknwonLiteral,
        swc_ecma_ast::TsType::TsTypeRef(tref) => {
            RustType::Custom(tref.type_name.as_ident().unwrap().sym.as_ref().to_string())
        }
        swc_ecma_ast::TsType::TsArrayType(tarray) => {
            let (_n, etype) = ts_type_to_rs(&tarray.elem_type);
            //format!("Vec<{etype}>")
            RustType::Array(Box::new(etype))
        }
        swc_ecma_ast::TsType::TsTypeLit(tlit) => {
            for m in &tlit.members {
                dbg!(m);
            }
            RustType::Unknwon
        }
        _ => {
            //dbg!(typ);
            //todo!();
            RustType::Unknwon
        }
    };

    (nullable, typ)
}

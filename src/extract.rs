use resast::{prelude::*, stmt::Stmt, ProgramPart};
use ressa::*;

#[cfg(not(feature = "hashbrown"))]
use std::collections::HashSet;


#[cfg(feature = "hashbrown")]
use hashbrown::HashSet;

/// extract all links that are created from a js script
pub fn extract_links<T: PartialEq + Eq + std::hash::Hash + From<String>>(
    script: &str,
) -> HashSet<T> {
    let mut map = HashSet::new();

    match Parser::new(&script) {
        Ok(p) => {
            for part in p {
                match part {
                    Ok(program) => {
                        // entry script program
                        match program {
                            ProgramPart::Decl(Decl::Var(VarKind::Var, _var_decl)) => {
                                // for variable in var_decl {
                                //     // start of variables capture into tree backed collection
                                //     match variable.init {
                                //         indent => match indent {
                                //             Some(exp) => match exp {
                                //                 Expr::Array(_) => {
                                //                 }
                                //                 _ => (),
                                //             },
                                //             _ => (),
                                //         },
                                //     }
                                // }
                            }
                            ProgramPart::Decl(Decl::Class(_)) => {}
                            ProgramPart::Stmt(stmt) => {
                                match stmt {
                                    Stmt::Expr(expression) => {
                                        match expression {
                                            resast::expr::Expr::Assign(assign) => {
                                                match assign {
                                                    AssignExpr {
                                                        operator,
                                                        left,
                                                        right,
                                                    } => {
                                                        // assignment to create node
                                                        if operator == AssignOp::Equal {
                                                            match left {
                                                                AssignLeft::Expr(exp) => {
                                                                    match *exp {
                                                                        Expr::Ident(_idt) => {}
                                                                        Expr::Member(mexp) => {
                                                                            match mexp {
                                                                                MemberExpr {
                                                                                    object: _,
                                                                                    property,
                                                                                    computed,
                                                                                } => {
                                                                                    if !computed {
                                                                                        match *property {
                                                                                            Expr::Ident(
                                                                                                ident,
                                                                                            ) => {
                                                                                                // links
                                                                                                if ident.name == "href" {
                                                                                                    match *right {
                                                                                                        Expr::Lit(value) => {
                                                                                                            match value {
                                                                                                                Lit::String(string_lit) => {
                                                                                                                    match string_lit {
                                                                                                                        StringLit::Double(value) | StringLit::Single(value) => {
                                                                                                                            map.insert(value.to_string().into());
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                                _ => ()
                                                                                                            }
                                                                                                        },
                                                                                                        _ => ()
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                            _ => (),
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        _ => (),
                                                                    }
                                                                }
                                                                _ => (),
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            _ => (),
                                        }
                                    }
                                    _ => (),
                                }
                            }
                            _ => (),
                        }
                    }
                    _ => (),
                }
            }
        }
        _ => (),
    };

    map
}

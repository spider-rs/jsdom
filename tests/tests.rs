use resast::{prelude::*};
use ressa::*;

const SCRIPT: &str = r###"
var ele = document.createElement('a');
ele.href = "https://a11ywatch.com";
"###;

#[test]
fn parse_links() {
    let p = Parser::new(&SCRIPT).unwrap();

    // build tree with elements created from the nodes todo

    for part in p {
        match part {
            Ok(program) => {
                // entry script program
                match program {
                    resast::ProgramPart::Decl(Decl::Var(VarKind::Var, var_decl)) => {
                        for variable in var_decl {

                            // start of variables capture into tree backed collection
                            match variable.init {
                                indent => match indent {
                                    Some(exp) => {
                                        println!("{:?}", exp);

                                        match exp {
                                            Expr::Array(item) => {
                                                println!("{:?}", item);
                                            }
                                            _ => (),
                                        }
                                    }
                                    _ => (),
                                },
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

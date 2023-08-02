use resast::{prelude::*, stmt::Stmt, ProgramPart};
use ressa::*;
use std::borrow::Cow;

#[cfg(not(feature = "hashbrown"))]
use std::collections::HashSet;

#[cfg(feature = "hashbrown")]
use hashbrown::HashSet;

/// extract all links that are created from a js script
#[cfg(not(feature = "tokio"))]
pub fn extract_links<T: PartialEq + Eq + std::hash::Hash + From<String>>(
    script: &str,
) -> HashSet<T> {
    let mut map = HashSet::new();

    match Parser::new(&Box::new(script)) {
        Ok(p) => {
            for part in p {
                let _ = _extract_links(part, &mut map); // can match to know if insert succeeded
            }
        }
        _ => (),
    };

    map
}

/// extract all links that are created from a js script
#[cfg(feature = "tokio")]
pub async fn extract_links<T: PartialEq + Eq + std::hash::Hash + From<String>>(
    script: &str,
) -> HashSet<T> {
    use tokio_stream::StreamExt;
    let mut map = HashSet::new();

    match Parser::new(&Box::new(script)) {
        Ok(p) => {
            let mut stream = tokio_stream::iter(Box::new(p));

            while let Some(part) = stream.next().await {
                _extract_links(part, &mut map);
            }
        }
        _ => (),
    };

    map
}

/// parse and extract links
/// using option instead of result as we do not care why the operation failed
fn _extract_links<T: PartialEq + Eq + std::hash::Hash + From<String>>(
    part: Result<ProgramPart<Cow<str>>, Error>,
    map: &mut HashSet<T>,
) -> Option<()> {
    let program = part.ok()?;

    let stmt = match program {
        ProgramPart::Stmt(stmt) => Some(stmt),
        _ => None,
    }?;

    let expr = match stmt {
        Stmt::Expr(expr) => Some(expr),
        _ => None,
    }?;

    let assign = match expr {
        resast::expr::Expr::Assign(a) => Some(a),
        _ => None,
    }?;

    if assign.operator != AssignOp::Equal {
        return None;
    }

    let exp = match assign.left {
        AssignLeft::Expr(exp) => Some(exp),
        _ => None,
    }?;

    let mexp = match *exp {
        Expr::Member(mexp) => Some(mexp),
        _ => None,
    }?;

    if mexp.computed {
        return None;
    }

    match *mexp.property {
        Expr::Ident(ident) if ident.name == "href" => {
            let literal = match *assign.right {
                Expr::Lit(value) => Some(value),
                _ => None,
            }?;
            let s = match literal {
                Lit::String(s) => Some(s),
                _ => None,
            }?;

            match s {
                StringLit::Double(v) | StringLit::Single(v) => {
                    map.insert(v.to_string().into());
                    Some(())
                }
            }
        }
        _ => None,
    }
}

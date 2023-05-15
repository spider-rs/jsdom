use jsdom::extract::extract_links;

const SCRIPT: &str = r###"
var ele = document.createElement('a');
ele.href = 'https://a11ywatch.com';
"###;

const SCRIPT_E: &str = r###"
(function() {  
    var Choosealicense, 
    LicenseSuggestion, 
    bind = function(fn, me){ 
        return function(){ 
            return fn.apply(me, arguments); 
        }; 
    };
    Choosealicense = (function() {
        Choosealicense.prototype.selectText = function(element) {
          var range, selection;

          if (document.body.createTextRange) {

          } else if (window.getSelection) {

          }

        };

    });
    
    })()
"###;

#[test]
#[cfg(all(not(feature = "hashbrown"), not(feature = "tokio")))]
fn parse_links() {
    use std::collections::HashSet;
    // build tree with elements created from the nodes todo
    let links: HashSet<String> = extract_links(SCRIPT);

    assert!(links.contains("https://a11ywatch.com"))
}

#[test]
#[cfg(not(feature = "tokio"))]
fn parse_e() {
    use std::collections::HashSet;
    // build tree with elements created from the nodes todo
    let links: HashSet<String> = extract_links(SCRIPT_E);

    assert!(!links.contains("https://a11ywatch.com"))
}


#[cfg(all(feature = "hashbrown", feature = "tokio"))]
#[tokio::test]
async fn parse_e() {
    use hashbrown::HashSet;
    // build tree with elements created from the nodes todo
    let links: HashSet<String> = extract_links(SCRIPT_E).await;

    assert!(!links.contains("https://a11ywatch.com"))
}

#[cfg(all(not(feature = "hashbrown"), feature = "tokio"))]
#[tokio::test]
async fn parse_e() {
    use std::collections::HashSet;
    // build tree with elements created from the nodes todo
    let links: HashSet<String> = extract_links(SCRIPT_E).await;

    assert!(!links.contains("https://a11ywatch.com"))
}

#[test]
#[cfg(all(feature = "hashbrown", not(feature = "tokio")))]
fn parse_links() {
    use hashbrown::HashSet;
    // build tree with elements created from the nodes todo
    let links: HashSet<String> = extract_links(SCRIPT);

    assert!(links.contains("https://a11ywatch.com"))
}

#[cfg(all(feature = "tokio", not(feature = "hashbrown")))]
#[tokio::test]
async fn parse_links() {
    use std::collections::HashSet;
    // build tree with elements created from the nodes todo
    let links: HashSet<String> = extract_links(SCRIPT).await;

    assert!(links.contains("https://a11ywatch.com"))
}

#[cfg(all(feature = "tokio", feature = "hashbrown"))]
#[tokio::test]
async fn parse_links() {
    use hashbrown::HashSet;
    // build tree with elements created from the nodes todo
    let links: HashSet<String> = extract_links(SCRIPT).await;

    assert!(links.contains("https://a11ywatch.com"))
}

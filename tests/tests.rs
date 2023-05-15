use jsdom::extract::extract_links;
use std::collections::HashSet;

const SCRIPT: &str = r###"
var ele = document.createElement('a');
ele.href = 'https://a11ywatch.com';
"###;

#[test]
fn parse_links() {
    // build tree with elements created from the nodes todo
    let links: HashSet<String> = extract_links(SCRIPT);

    assert!(links.contains("https://a11ywatch.com"))
}

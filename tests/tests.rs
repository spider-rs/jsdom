use jsdom::extract::extract_links;

const SCRIPT: &str = r###"
var ele = document.createElement('a');
ele.href = 'https://a11ywatch.com';
"###;

#[test]
#[cfg(not(feature = "hashbrown"))]
fn parse_links() {
    use std::collections::HashSet;
    // build tree with elements created from the nodes todo
    let links: HashSet<String> = extract_links(SCRIPT);

    assert!(links.contains("https://a11ywatch.com"))
}

#[test]
#[cfg(feature = "hashbrown")]
fn parse_links() {
    use hashbrown::HashSet;
    // build tree with elements created from the nodes todo
    let links: HashSet<String> = extract_links(SCRIPT);

    assert!(links.contains("https://a11ywatch.com"))
}

# jsdom

A fast javascript dom parser for web scraping.

`cargo add jsdom`

```rust
use std::collections::HashSet;
use jsdom::extract::extract_links;

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
```

## Features

This package will rollout features that are most important for web scraping first.

1. `hashbrown`: Enable the hashbrown crate.
1. `tokio`: Enable tokio streaming utils.

### Stage 0.1

Intro stage can handle elements created in statements and expressions.
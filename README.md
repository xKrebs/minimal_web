# minimal
A minimal tool for build web-page easier using Rust

# Install
minimal_web = "0.1.0"

# Usage/Examples

```RUST

use minimal_web::*;

fn main() {
    let document = document();
    let container = document.query_selector_html(".container"); //HtmlElement
    let all_p = container.query_selector_list("p"); //NodeList
    let all_p_clone = all_p.clone();
    for i in 0..all_p.length(){
        let element = all_p_clone.get_html(i); //HtmlElement
        //do something
    }
}
```

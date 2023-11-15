# Minimal
A minimal tool for build web-page easier using Rust

# Features
I started to built this library not just for classic call such as query_selector, document etc.

What are the features of minimal?

- Shorter call for some function
- Easier way for closure
- More readable exceptions
- Animation (soon)
and more...

Version 0.1.2 implement

- syntax change for macros : from Fun! to fnv! (function void) and from FunMut! to fnmut (function mut)
- Parallax animation

```
(RUST)
use minimal_web::{animation::use_parallax, *};
fn main(){
    use_parallax();
    html!{
        <div class="parallax-effect" data-max="10">
        // max value is 2 but you can change it with data-max, the value is the max of parallax effect when you over
        ...
        </div>
    }
}
...
You can also build your own glow with this syntax
<div class="parallax-effect">
    ...
    <div class="wrapper"> <- a wrapper of parent (with same parent dimension)
        <div class="glow"></div>
    </div>
</div>
```
# What's next version ?
Some animations built by me in rust that you can use directly with minimal_web !

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

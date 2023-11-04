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

Version 0.1.1 implement

-> custom_expect for Option<T> and Result<T> (Build your own custom error message)
-> macros for closure :
    FunMut!(EventType, function) -> mutable closure
    Fun!(EventType, function) -> classic closure

    ```Rust
    let onclick = Fun!(Event, move |e|{
        gloo::console::log!("Hi, nice to meet you");
    });
    document().query_selector_html("button").set_onclick(onclick.as_ref().dyn_ref());

    ///Instead of
    let onclick = Closure::<dyn FnMut(Event)>::new(move |e|{
        gloo::console::log!("Hi, nice to meet you");
    });
    document().query_selector_html("button").set_onclick(onclick.as_ref().dyn_ref());
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

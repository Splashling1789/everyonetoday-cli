#![allow(non_snake_case)]

use dioxus::html::style;
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;

fn main() {
    // launch the web app
    dioxus_web::launch(App);
}

// create a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        style {
            include_str!("../src/style.css")
        }
        nav {
            "Hello wol"
        }
        div {

        }
    })
}
#![allow(non_snake_case)]

// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;

const EVTD_SUBTITLE: &str = "Your title is here";
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
            div {
                class: "title",
                div {
                    class: "supertitle",
                    "EveryoneToday CLI"
                }
                div {
                    class: "subtitle",
                    format!("- {EVTD_SUBTITLE} -")
                }
            }
        }
        div {
            class: "main-content",
            section {
                class: "main-section",
                h2 {
                    "Welcome to the wall!"
                },
                p {
                    "This is what everyone is thinking"
                }
            }
            section {
                class: "posts-section",
                for i in 0..6{
                article {
                    i {
                            p {
                                class: "post-quote",
                                "This is a test message. Lorem few fjw efojw efoj fojef ojf joe foje", br{}, "foje foje foj f bla bla bla and bla. Cause bla is lifef weiojfijw oi fweinioj fwioejjijij fifi"
                            }
                        }
                    i {
                        class: "post-sign",
                        "- Pablo"
                    }
                    span {
                        class: "post-time",
                        "08:10"
                    }
                }
                    }
            }
        }
    })
}
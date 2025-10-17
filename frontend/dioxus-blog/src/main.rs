use std::rc::Rc;

use anchor_lang::prelude::*;
use dioxus::{prelude::*, web::HashHistory};

mod apis;
mod components;
mod contexts;
mod pages;

use components::*;
use contexts::*;
use pages::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: String },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::LaunchBuilder::new()
        .with_cfg(dioxus::web::Config::new().history(Rc::new(HashHistory::new(true))))
        .launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        RpcProvider {
            WalletProvider {
                Header {}
                Router::<Route> {}
            }
        }
    }
}

use dioxus::{
    prelude::*,
};
mod components;
use components::*;

mod assets;
use assets::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TW_CSS }

        Navbar {}
        Hero {}
    }
}

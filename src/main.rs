use dioxus::{
    html::{source, video},
    prelude::*,
};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TW_CSS: Asset = asset!("/assets/tailwind.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const BLACKHOLE_WEBM: Asset = asset!("/assets/blackhole.webm");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TW_CSS }
        Hero {}

    }
}

#[component]
pub fn Hero() -> Element {
    rsx! {
        main {
            class: "h-full w-full",
            div {
                class: "flex flex-col gap-20",
                div {
                    class: "relative flex flex-col h-full w-full",
                    video {
                        class: "rotate-180 absolute top-[-340px] left-0 w-full h-full object-cover -z-20",
                        autoplay: "",
                        muted: "",
                        loop: "",
                        source {
                            src: BLACKHOLE_WEBM,
                            r#type: "video/webm"
                        }
                    }
                }
            }
        }
    }
}

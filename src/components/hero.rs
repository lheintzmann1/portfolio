
use dioxus::prelude::*;
use crate::assets::*;
use crate::components::hero_content::HeroContent;

#[component]
pub fn Hero() -> Element {
    rsx! {
        main {
            class: "h-full w-full ",
            div {
                class: "flex flex-col gap-20",
                div {
                    class: "flex flex-col h-full w-full", // "relative" cause issues with video positioning
                    video {
                        class: "rotate-180 absolute top-[-400px] left-0 w-full h-full object-cover -z-20", // originally: "top-[-360px]"
                        autoplay: true,
                        muted: true,
                        loop: true,
                        source {
                            src: BLACKHOLE_WEBM,
                            r#type: "video/webm"
                        }
                    }
                    HeroContent {}
                }
            }
        }
    }
}

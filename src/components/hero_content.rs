use dioxus::prelude::*;
use crate::assets::*;
use dioxus_heroicons::{Icon, solid::Shape};

#[component]
pub fn HeroContent() -> Element {
    rsx! {
        div {
            class: "flex flex-row items-center justify-center px-20 mt-60 w-full z-[20]", // originally: "mt-40"
            div {
                class: "h-full w-full flex flex-col gap-5 justify-center m-auto text-start",
                div {
                    class: "Welcome-box py-[8px] px-[7px] border border-[#7042f88b] opacity-[0.9]]",
                    style: "opacity: 1; transform: none;",
                    Icon {
                        icon: Shape::Sparkles,
                        class: "text-[#b49bff] mr-[10px] h-5 w-5",
                    }
                    h1 {
                        class: "Welcome-text text-[13px]",
                        style: "font-family: 'Inter', sans-serif; font-weight: 500;",
                        "Full-Stack Developer"
                    }
                }
            }
        }
    }
}
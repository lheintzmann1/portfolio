use dioxus::prelude::*;
use dioxus_free_icons::icons::hi_solid_icons::HiSparkles;
use dioxus_free_icons::Icon;

#[component]
pub fn HeroContent() -> Element {
    rsx! {
        div {
            class: "w-full px-8 sm:px-12 md:px-16 lg:px-20 xl:px-24 2xl:px-32",
            div {
                class: "flex flex-col gap-5 text-start",

                div {
                    class: "Welcome-box py-2 px-[7px] border border-[#7042f88b]",
                    Icon {
                        icon: HiSparkles,
                        class: "text-[#b49bff] mr-2.5 ml-1.5 h-5 w-5",
                    }
                    h1 {
                        class: "Welcome-text text-[13px]",
                        style: "font-family: 'Inter', sans-serif; font-weight: 500;",
                        "Lorem ipsum dolor"
                    }
                }

                h1 {
                    class: "text-5xl sm:text-6xl md:text-7xl lg:text-8xl font-bold text-white mt-4 max-w-[600px]",
                    style: "font-family: 'Inter', sans-serif; font-weight: 400; font-size: 3.75rem;",
                    "Lorem ipsum dolor sit amet, consectetur."
                }
                p {
                    class: "text-xl md:text-2xl text-gray-400 mt-4 max-w-[600px]",
                    style: "font-family: 'Inter', sans-serif; font-weight: 400; font-size: 1.125rem; line-height: 1.75rem;",
                    "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Quisque non risus pellentesque, mattis ipsum vitae, elementum arcu fusce."
                }
            }
        }
    }
}

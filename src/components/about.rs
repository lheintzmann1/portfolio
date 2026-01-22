// About section with personal introduction and statistics.
// Features a two-column layout with avatar card and biography.

use dioxus::prelude::*;

use crate::assets::ME_PNG;
use crate::data::{load_profile, ProfileData};
use crate::styles::{FONT_INTER, GLASS_CONTAINER_BG};

#[component]
pub fn About() -> Element {
    let profile = load_profile();

    rsx! {
        section {
            id: "about",
            class: "min-h-screen w-full py-20 px-8 sm:px-12 md:px-16 lg:px-20 xl:px-24 2xl:px-32",

            div { class: "max-w-7xl mx-auto",
                SectionTitle { title: "About Me" }

                div { class: "grid md:grid-cols-2 gap-12 items-center",
                    AvatarCard {}

                    div { class: "space-y-6",
                        for paragraph in &profile.bio {
                            p { class: "text-gray-300 text-lg leading-relaxed", "{paragraph}" }
                        }

                        StatsGrid { profile }
                    }
                }
            }
        }
    }
}

#[component]
fn SectionTitle(title: &'static str) -> Element {
    rsx! {
        div { class: "text-center mb-16",
            h2 {
                class: "text-4xl md:text-5xl font-bold text-white mb-4",
                style: "{FONT_INTER}",
                "{title}"
            }
            div { class: "h-0.5 w-20 bg-blue-400 mx-auto" }
        }
    }
}

#[component]
fn AvatarCard() -> Element {
    rsx! {
        div { class: "relative",
            div { class: "aspect-square max-w-md mx-auto rounded-[32px] {GLASS_CONTAINER_BG} backdrop-blur-xl border border-white/[0.08] overflow-hidden shadow-2xl shadow-black/40 hover:shadow-blue-500/10 transition-all duration-700 ease-in-out hover:border-white/[0.12]",
                img {
                    src: ME_PNG,
                    alt: "Profile photo",
                    class: "w-full h-full object-cover",
                }
            }
        }
    }
}

#[component]
fn StatsGrid(profile: ProfileData) -> Element {
    rsx! {
        div { class: "grid grid-cols-3 gap-8 mt-12",
            for stat in &profile.stats {
                StatItem { value: stat.value.clone(), label: stat.label.clone() }
            }
        }
    }
}

#[component]
fn StatItem(value: String, label: String) -> Element {
    rsx! {
        div { class: "text-center",
            div { class: "text-4xl font-light text-white mb-2", "{value}" }
            div { class: "text-gray-500 text-xs mt-1 tracking-wider uppercase", "{label}" }
        }
    }
}

//! # About Section Component
//!
//! Personal introduction section featuring a visual avatar card,
//! biography paragraphs, and key statistics.

use dioxus::prelude::*;

use crate::assets::ME_PNG;
use crate::data::{load_profile, ProfileData};
use crate::styles::{FONT_INTER, GLASS_CONTAINER_BG};

// ============================================================================
// COMPONENTS
// ============================================================================

/// About section with personal introduction and statistics.
///
/// Displays a two-column layout:
/// - Left: Visual avatar card with profile photo
/// - Right: Biography and key statistics
#[component]
pub fn About() -> Element {
    let profile = load_profile();

    rsx! {
        section {
            id: "about",
            class: "min-h-screen w-full py-20 px-8 sm:px-12 md:px-16 lg:px-20 xl:px-24 2xl:px-32",

            div { class: "max-w-7xl mx-auto",
                // Section header
                SectionTitle { title: "About Me" }

                // Two-column content layout
                div { class: "grid md:grid-cols-2 gap-12 items-center",
                    // Left: Avatar card
                    AvatarCard {}

                    // Right: Bio and stats
                    div { class: "space-y-6",
                        // Biography paragraphs
                        for paragraph in &profile.bio {
                            p { class: "text-gray-300 text-lg leading-relaxed", "{paragraph}" }
                        }

                        // Statistics grid
                        StatsGrid { profile }
                    }
                }
            }
        }
    }
}

/// Reusable section title component with underline accent.
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

/// Glassmorphism avatar card with profile photo.
#[component]
fn AvatarCard() -> Element {
    rsx! {
        div { class: "relative",
            div { class: "aspect-square max-w-md mx-auto rounded-[32px] {GLASS_CONTAINER_BG} backdrop-blur-xl border border-white/[0.08] overflow-hidden shadow-2xl shadow-black/40 hover:shadow-blue-500/10 transition-all duration-700 ease-in-out hover:border-white/[0.12]",
                // Profile photo
                img {
                    src: ME_PNG,
                    alt: "Profile photo",
                    class: "w-full h-full object-cover",
                }
            }
        }
    }
}

/// Grid displaying key statistics.
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

/// Individual statistic item with value and label.
#[component]
fn StatItem(value: String, label: String) -> Element {
    rsx! {
        div { class: "text-center",
            div { class: "text-4xl font-light text-white mb-2", "{value}" }
            div { class: "text-gray-500 text-xs mt-1 tracking-wider uppercase", "{label}" }
        }
    }
}

//! # Hero Content Component
//!
//! The main textual content displayed in the hero section,
//! including the role badge, headline, and introduction paragraph.

use dioxus::prelude::*;
use dioxus_free_icons::icons::hi_solid_icons::HiSparkles;
use dioxus_free_icons::Icon;

use crate::data::load_profile;
use crate::styles::FONT_INTER;

// ============================================================================
// COMPONENTS
// ============================================================================

/// Hero section content with role badge, headline, and introduction.
///
/// Displays a decorative badge with the developer role, a bold headline,
/// and a brief introduction paragraph.
#[component]
pub fn HeroContent() -> Element {
    let profile = load_profile();

    rsx! {
        div { class: "w-full px-8 sm:px-12 md:px-16 lg:px-20 xl:px-24 2xl:px-32",
            div { class: "flex flex-col gap-5 text-start",
                // Role badge with sparkle icon
                RoleBadge { role: profile.role.clone() }

                // Main headline
                h1 {
                    class: "text-5xl sm:text-6xl md:text-7xl lg:text-8xl font-bold text-white mt-4 max-w-[600px]",
                    style: "{FONT_INTER} font-weight: 400; font-size: 3.75rem;",
                    "{profile.headline}"
                }

                // Introduction paragraph
                p {
                    class: "text-xl md:text-2xl text-gray-400 mt-4 max-w-[600px]",
                    style: "{FONT_INTER} font-weight: 400; font-size: 1.125rem; line-height: 1.75rem;",
                    "{profile.intro}"
                }
            }
        }
    }
}

/// Decorative badge displaying the developer role.
#[component]
fn RoleBadge(role: String) -> Element {
    rsx! {
        div { class: "Welcome-box py-2 px-[7px] border border-[#7042f88b]",
            Icon {
                icon: HiSparkles,
                class: "text-[#b49bff] mr-2.5 ml-1.5 h-5 w-5",
            }
            h1 {
                class: "Welcome-text text-[13px]",
                style: "{FONT_INTER} font-weight: 500;",
                "{role}"
            }
        }
    }
}

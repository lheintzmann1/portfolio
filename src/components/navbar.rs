//! # Navbar Component
//!
//! Fixed navigation header with logo, site name, and section links.
//! Features a glassmorphism design with backdrop blur effect.

use dioxus::prelude::*;

use crate::assets::LH_WHITE_SVG;
use crate::data::{load_navigation, load_profile};
use crate::styles::FONT_OUTFIT;

// ============================================================================
// COMPONENTS
// ============================================================================

/// Fixed navigation bar with glassmorphism styling.
///
/// Layout:
/// - Left: Logo and site name (name hidden on mobile)
/// - Center: Navigation links to page sections
/// - Right: Spacer (reserved for future features)
#[component]
pub fn Navbar() -> Element {
    let profile = load_profile();
    let nav_links = load_navigation();

    rsx! {
        nav { class: "fixed top-0 left-0 w-full z-50 backdrop-blur-md bg-black/20 border-b border-white/10",
            div { class: "max-w-8xl mx-auto px-4 sm:px-6 lg:px-8",
                div { class: "flex items-center h-16",
                    // Logo and brand name (left)
                    BrandSection { name: profile.name }

                    // Navigation links (center)
                    nav { class: "flex-1 flex justify-center items-baseline space-x-8",
                        for link in nav_links {
                            NavLink { href: link.href, text: link.text }
                        }
                    }

                    // Right spacer (reserved for social icons or language toggle)
                    div { class: "flex-1" }
                }
            }
        }
    }
}

/// Brand section with logo and site owner name.
#[component]
fn BrandSection(name: String) -> Element {
    rsx! {
        div { class: "flex-1 flex items-center space-x-3",
            // Logo image
            div { class: "w-10 h-10",
                img {
                    class: "w-full h-full hover:brightness-110 transition-all duration-300",
                    src: "{LH_WHITE_SVG}",
                    alt: "{name} Logo",
                }
            }

            // Owner name (hidden on mobile)
            a {
                href: "#",
                class: "hidden md:flex ml-2.5 text-white hover:text-blue-400 transition-colors duration-300",
                style: "{FONT_OUTFIT} font-weight: 400;",
                "{name}"
            }
        }
    }
}

/// Individual navigation link with animated underline effect.
#[component]
fn NavLink(href: String, text: String) -> Element {
    rsx! {
        a {
            href: "{href}",
            class: "text-white/90 hover:text-white hover:bg-white/10 px-3 py-2 rounded-md text-sm font-medium transition-all duration-300 relative group",
            "{text}"

            // Animated underline on hover
            span { class: "absolute bottom-0 left-0 w-0 h-0.5 bg-blue-400 group-hover:w-full transition-all duration-300" }
        }
    }
}
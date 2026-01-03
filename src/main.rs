//! # Portfolio Application
//!
//! A modern, responsive portfolio website built with Dioxus and Tailwind CSS.
//! This application showcases projects, skills, and professional experience
//! through an elegant single-page design.
//!
//! ## Architecture
//!
//! - `main.rs`: Application entry point and root component
//! - `components/`: Reusable UI components (Hero, Navbar, Skills, etc.)
//! - `assets.rs`: Static asset declarations (images, stylesheets, media)
//! - `styles.rs`: Shared CSS class constants for visual consistency
//! - `data.rs`: JSON data loading and parsing

use dioxus::prelude::*;

mod assets;
mod components;
mod data;
mod styles;

use assets::*;
use components::*;

/// Application entry point.
///
/// Initializes and launches the Dioxus application with the root `App` component.
fn main() {
    dioxus::launch(App);
}

/// Root application component.
///
/// Defines the overall page structure including:
/// - Document head configuration (favicon, stylesheets)
/// - Custom cursor styling
/// - Main content layout with all section components
#[component]
fn App() -> Element {
    rsx! {
        // Document head configuration
        document::Title { "Lucas HEINTZMANN" }
        document::Link { rel: "icon", r#type: "image/x-icon", href: FAVICON_ICO }
        document::Link {
            rel: "icon",
            r#type: "image/png",
            sizes: "16x16",
            href: FAVICON_16,
        }
        document::Link {
            rel: "icon",
            r#type: "image/png",
            sizes: "32x32",
            href: FAVICON_32,
        }
        document::Link {
            rel: "apple-touch-icon",
            sizes: "180x180",
            href: APPLE_TOUCH_ICON,
        }
        document::Link { rel: "manifest", href: SITE_WEBMANIFEST }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TW_CSS }

        // Global custom cursor style
        document::Style {
            r#"
            * {{
                cursor: url("{CURSOR_PNG}") 18 18, auto !important;
            }}
            "#
        }

        div { class: "bg-[#030014] relative",
            // Add starfield before all content
            Starfield {}

            // Wrap content in relative container
            div { class: "relative z-10",
                Navbar {}
                Hero {}
                About {}
                Skills {}
                Projects {}
                Experience {}
                Contact {}
            }
        }
    }
}

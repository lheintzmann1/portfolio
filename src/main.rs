// Portfolio application built with Dioxus and Tailwind CSS.
// A modern, responsive single-page site showcasing projects, skills, and experience.

use dioxus::prelude::*;

mod assets;
mod components;
mod data;
mod styles;

use assets::*;
use components::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        // Document head: favicons, stylesheets, and metadata
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

        // Custom cursor style
        document::Style {
            r#"
            * {{
                cursor: url("{CURSOR_PNG}") 18 18, auto !important;
            }}
            "#
        }

        div { class: "bg-[#030014] relative",
            Starfield {}

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

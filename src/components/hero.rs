//! # Hero Section Component
//!
//! The landing section featuring an animated video background
//! with the main portfolio introduction content overlaid.

use dioxus::prelude::*;

use crate::assets::BLACKHOLE_WEBM;
use crate::components::HeroContent;

/// Hero section with animated video background.
///
/// Displays a fullscreen video background (black hole animation)
/// with the main introduction content positioned on top.
#[component]
pub fn Hero() -> Element {
    rsx! {
        main { class: "relative min-h-screen w-full overflow-hidden",
            // Video background layer (z-index: auto)
            video {
                class: "absolute inset-0 w-full h-full object-cover",
                style: "transform: rotate(180deg) translateY(43%);",
                autoplay: true,
                muted: true,
                r#loop: true,
                playsinline: true,
                preload: "auto",
                source { src: "{BLACKHOLE_WEBM}", r#type: "video/webm" }
            }

            // Content layer positioned above video
            div { class: "relative z-20 min-h-screen flex items-center justify-center",
                HeroContent {}
            }
        }
    }
}

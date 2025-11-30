use dioxus::prelude::*;
use crate::assets::*;
use crate::components::hero_content::HeroContent;

#[component]
pub fn Hero() -> Element {
    rsx! {
        main {
            class: "relative min-h-screen w-full overflow-hidden",
            
            // Video background - absolutely positioned
            video {
                class: "absolute inset-0 w-full h-full object-cover",
                style: "transform: rotate(180deg) translateY(43%);",
                autoplay: true,
                muted: true,
                loop: true,
                playsinline: true,
                source {
                    src: BLACKHOLE_WEBM,
                    r#type: "video/webm"
                }
            }
            
            // Dark overlay (optional but recommended for text readability)
            // div {
            //     class: "absolute inset-0 bg-black/50 z-10"
            // }
            
            // Content container - relatively positioned above video
            div {
                class: "relative z-20 min-h-screen flex items-center justify-center",
                HeroContent {}
            }
        }
    }
}

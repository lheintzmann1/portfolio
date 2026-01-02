//! # Skills Section Component
//!
//! Displays technical skills organized by category in a responsive grid.
//! Each category is presented as a glassmorphism card with hover effects.

use dioxus::prelude::*;

use crate::data::load_skills;
use crate::styles::{FONT_INTER, GLASS_CARD_LIFT};

// ============================================================================
// COMPONENTS
// ============================================================================

/// Skills section displaying technical competencies by category.
///
/// Renders a responsive grid of skill cards, each containing
/// a category title and list of related technologies.
#[component]
pub fn Skills() -> Element {
    let skills_data = load_skills();

    rsx! {
        section {
            id: "skills",
            class: "min-h-screen w-full py-20 px-8 sm:px-12 md:px-16 lg:px-20 xl:px-24 2xl:px-32",

            div { class: "max-w-7xl mx-auto",
                // Section header
                SectionTitle { title: "Skills & Technologies" }

                // Skills grid
                div { class: "grid md:grid-cols-2 lg:grid-cols-3 gap-8",
                    for category in skills_data {
                        SkillCard { title: category.title, skills: category.skills }
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

/// Glassmorphism card displaying a skill category.
///
/// # Arguments
/// * `title` - Category name (e.g., "Languages", "Tools & DevOps")
/// * `skills` - List of skills/technologies in this category
#[component]
fn SkillCard(title: String, skills: Vec<String>) -> Element {
    rsx! {
        div { class: "{GLASS_CARD_LIFT}",
            h3 { class: "text-2xl font-light text-white mb-6 tracking-tight", "{title}" }

            div { class: "space-y-3",
                for skill in skills {
                    SkillItem { name: skill }
                }
            }
        }
    }
}

/// Individual skill item with decorative bullet.
#[component]
fn SkillItem(name: String) -> Element {
    rsx! {
        div { class: "flex items-center space-x-4 transition-all duration-300 hover:translate-x-1",
            // Decorative gradient bullet
            div { class: "w-1.5 h-1.5 rounded-full bg-[linear-gradient(135deg,rgba(255,255,255,0.3)_0%,rgba(255,255,255,0.1)_100%)]" }
            span { class: "text-gray-400 hover:text-white font-light text-[15px]", "{name}" }
        }
    }
}

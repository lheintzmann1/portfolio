//! # Experience Section Component
//!
//! Displays professional experience and education as an interactive timeline.
//! Features alternating left/right alignment on desktop with animated dots.

use dioxus::prelude::*;

use crate::data::load_experience;
use crate::styles::{FONT_INTER, GLASS_CARD_LIFT};

// ============================================================================
// COMPONENTS
// ============================================================================

/// Experience section with interactive timeline.
///
/// Displays education and work experience in a vertical timeline format
/// with alternating left/right cards on desktop.
#[component]
pub fn Experience() -> Element {
    let experiences = load_experience();

    rsx! {
        section {
            id: "experience",
            class: "min-h-screen w-full py-20 px-8 sm:px-12 md:px-16 lg:px-20 xl:px-24 2xl:px-32",

            div { class: "max-w-5xl mx-auto",
                // Section header
                SectionTitle { title: "Experience" }

                // Timeline container
                div { class: "space-y-12 relative",
                    // Vertical timeline line
                    div { class: "absolute left-0 md:left-1/2 transform md:-translate-x-1/2 w-px h-full bg-white/20" }

                    // Timeline entries
                    for entry in experiences {
                        ExperienceItem {
                            position: entry.position,
                            company: entry.company,
                            period: entry.period,
                            description: entry.description,
                            achievements: entry.achievements,
                            align: entry.align,
                        }
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

/// Individual timeline entry with position, company, and achievements.
///
/// # Arguments
/// * `position` - Job title or degree name
/// * `company` - Company or institution name
/// * `period` - Date range (e.g., "2024 - 2025")
/// * `description` - Brief description of the role
/// * `achievements` - List of key accomplishments
/// * `align` - "left" or "right" side alignment
#[component]
fn ExperienceItem(
    position: String,
    company: String,
    period: String,
    description: String,
    achievements: Vec<String>,
    align: String,
) -> Element {
    // Compute alignment-specific classes
    let alignment_classes = if align == "right" {
        "md:ml-auto md:pl-12"
    } else {
        "md:mr-auto md:pr-12"
    };

    let dot_classes = if align == "right" {
        "absolute left-0 md:left-0 top-0 w-3 h-3 rounded-full \
            bg-gradient-to-br from-blue-400 to-blue-500 border-4 border-[#080808] \
            shadow-lg shadow-blue-400/30 transform md:-translate-x-1/2"
    } else {
        "absolute left-0 md:right-0 md:left-auto top-0 w-3 h-3 rounded-full \
            bg-gradient-to-br from-blue-400 to-blue-500 border-4 border-[#080808] \
            shadow-lg shadow-blue-400/30 transform md:translate-x-1/2"
    };

    rsx! {
        div { class: "relative pl-8 md:pl-0 md:w-1/2 {alignment_classes}",
            // Timeline dot indicator
            div { class: "{dot_classes}" }

            // Content card
            div { class: "{GLASS_CARD_LIFT}",
                // Header with position and period
                div { class: "flex flex-col md:flex-row md:items-center md:justify-between mb-3",
                    h3 { class: "text-xl font-light text-white tracking-tight", "{position}" }
                    span { class: "text-gray-500 text-xs mt-1 md:mt-0 tracking-wider uppercase",
                        "{period}"
                    }
                }

                // Company/institution name
                h4 { class: "text-lg text-gray-400 mb-4 font-light", "{company}" }

                // Description
                p { class: "text-gray-400 mb-5 leading-relaxed font-light text-[15px]",
                    "{description}"
                }

                // Achievement bullets
                div { class: "space-y-3",
                    for achievement in achievements {
                        AchievementItem { text: achievement }
                    }
                }
            }
        }
    }
}

/// Individual achievement bullet point.
#[component]
fn AchievementItem(text: String) -> Element {
    rsx! {
        div { class: "flex items-start space-x-3 transition-all duration-300 hover:translate-x-1",
            // Gradient bullet
            div { class: "w-1.5 h-1.5 rounded-full bg-gradient-to-br from-blue-400/50 to-blue-400/20 mt-2 shrink-0" }
            span { class: "text-gray-500 text-sm font-light hover:text-white", "{text}" }
        }
    }
}

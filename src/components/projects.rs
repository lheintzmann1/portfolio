//! # Projects Section Component
//!
//! Displays featured projects loaded from a JSON configuration file.
//! Each project is shown as a card with title, description, tags, and action buttons.

use dioxus::prelude::*;

use crate::data::{load_projects, ProjectButton};
use crate::styles::{BTN_PRIMARY, BTN_SECONDARY, FONT_INTER, GLASS_CARD_LIFT_STRONG};

// ============================================================================
// COMPONENTS
// ============================================================================

/// Projects section displaying featured work.
///
/// Loads project data from JSON and renders each as a card
/// in a responsive grid layout.
#[component]
pub fn Projects() -> Element {
    let projects = load_projects();

    rsx! {
        section {
            id: "projects",
            class: "min-h-screen w-full py-20 px-8 sm:px-12 md:px-16 lg:px-20 xl:px-24 2xl:px-32",

            div { class: "max-w-7xl mx-auto",
                // Section header
                SectionTitle { title: "Featured Projects" }

                // Projects grid
                div { class: "grid md:grid-cols-2 lg:grid-cols-3 gap-8",
                    for project in projects {
                        ProjectCard {
                            title: project.title,
                            description: project.description,
                            tags: project.tags,
                            buttons: project.buttons,
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

/// Glassmorphism project card with title, description, tags, and action buttons.
///
/// # Arguments
/// * `title` - Project name
/// * `description` - Short project description
/// * `tags` - Technology tags to display
/// * `buttons` - Action buttons (links to demo, source, etc.)
#[component]
fn ProjectCard(
    title: String,
    description: String,
    tags: Vec<String>,
    buttons: Vec<ProjectButton>,
) -> Element {
    rsx! {
        div { class: "{GLASS_CARD_LIFT_STRONG}",
            // Project title
            h3 { class: "text-2xl font-light text-white mb-4 tracking-tight", "{title}" }

            // Description
            p { class: "text-gray-400 mb-6 flex-grow leading-relaxed font-light text-[15px]",
                "{description}"
            }

            // Technology tags
            TagList { tags }

            // Action buttons
            if !buttons.is_empty() {
                ButtonGroup { buttons }
            }
        }
    }
}

/// Returns the color classes for a programming language tag.
/// Colors are inspired by GitHub's language colors.
fn get_language_color(tag: &str) -> Option<(&'static str, &'static str)> {
    // Returns (background_color, text_color)
    match tag {
        // Programming Languages
        "Rust" => Some(("bg-orange-500/20 hover:bg-orange-500/30 border-orange-500/40 hover:border-orange-500/50", "text-orange-300")),
        "Python" => Some(("bg-blue-500/20 hover:bg-blue-500/30 border-blue-500/40 hover:border-blue-500/50", "text-blue-300")),
        "JavaScript" => Some(("bg-yellow-500/20 hover:bg-yellow-500/30 border-yellow-500/40 hover:border-yellow-500/50", "text-yellow-300")),
        "TypeScript" => Some(("bg-blue-600/20 hover:bg-blue-600/30 border-blue-600/40 hover:border-blue-600/50", "text-blue-300")),
        "Kotlin" => Some(("bg-purple-500/20 hover:bg-purple-500/30 border-purple-500/40 hover:border-purple-500/50", "text-purple-300")),
        "Java" => Some(("bg-red-500/20 hover:bg-red-500/30 border-red-500/40 hover:border-red-500/50", "text-red-300")),
        "C" => Some(("bg-gray-500/20 hover:bg-gray-500/30 border-gray-500/40 hover:border-gray-500/50", "text-gray-300")),
        "C++" => Some(("bg-pink-500/20 hover:bg-pink-500/30 border-pink-500/40 hover:border-pink-500/50", "text-pink-300")),
        "C#" => Some(("bg-green-600/20 hover:bg-green-600/30 border-green-600/40 hover:border-green-600/50", "text-green-300")),
        "Go" => Some(("bg-cyan-500/20 hover:bg-cyan-500/30 border-cyan-500/40 hover:border-cyan-500/50", "text-cyan-300")),
        "Swift" => Some(("bg-orange-600/20 hover:bg-orange-600/30 border-orange-600/40 hover:border-orange-600/50", "text-orange-300")),
        "Ruby" => Some(("bg-red-600/20 hover:bg-red-600/30 border-red-600/40 hover:border-red-600/50", "text-red-300")),
        "PHP" => Some(("bg-indigo-500/20 hover:bg-indigo-500/30 border-indigo-500/40 hover:border-indigo-500/50", "text-indigo-300")),
        "Dart" => Some(("bg-teal-500/20 hover:bg-teal-500/30 border-teal-500/40 hover:border-teal-500/50", "text-teal-300")),
        "Lua" => Some(("bg-blue-800/20 hover:bg-blue-800/30 border-blue-800/40 hover:border-blue-800/50", "text-blue-300")),
        "Bash" => Some(("bg-green-700/20 hover:bg-green-700/30 border-green-700/40 hover:border-green-700/50", "text-green-300")),
        "OCaml" => Some(("bg-orange-400/20 hover:bg-orange-400/30 border-orange-400/40 hover:border-orange-400/50", "text-orange-300")),
        "HTML" | "HTML/CSS" => Some(("bg-orange-500/20 hover:bg-orange-500/30 border-orange-500/40 hover:border-orange-500/50", "text-orange-300")),
        "CSS" => Some(("bg-blue-500/20 hover:bg-blue-500/30 border-blue-500/40 hover:border-blue-500/50", "text-blue-300")),
        
        // Default: no special color
        _ => None,
    }
}

/// Horizontal list of technology tags with language-specific colors.
#[component]
fn TagList(tags: Vec<String>) -> Element {
    rsx! {
        div { class: "flex flex-wrap gap-2.5 mb-6",
            for tag in tags {
                Tag { name: tag }
            }
        }
    }
}

/// Individual tag with optional language-specific coloring.
#[component]
fn Tag(name: String) -> Element {
    // Check if this tag has a special language color
    let (bg_class, text_class) = get_language_color(&name)
        .unwrap_or(("bg-white/[0.06] hover:bg-white/[0.10] border-white/[0.08] hover:border-white/[0.12]", "text-gray-300"));
    
    rsx! {
        span { class: "px-4 py-1.5 text-xs font-light rounded-full {bg_class} {text_class} border backdrop-blur-sm tracking-wide transition-all duration-300 ease-in-out cursor-default",
            "{name}"
        }
    }
}

/// Group of action buttons for a project card.
#[component]
fn ButtonGroup(buttons: Vec<ProjectButton>) -> Element {
    rsx! {
        div { class: "flex gap-3 mt-auto",
            for button in buttons {
                ProjectButtonLink {
                    label: button.label,
                    url: button.url,
                    primary: button.primary,
                }
            }
        }
    }
}

/// Individual project action button/link.
#[component]
fn ProjectButtonLink(label: String, url: String, primary: bool) -> Element {
    let class = if primary { BTN_PRIMARY } else { BTN_SECONDARY };

    rsx! {
        a {
            href: "{url}",
            target: "_blank",
            rel: "noopener noreferrer",
            class: "{class}",
            "{label}"
        }
    }
}

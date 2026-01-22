// Projects section displaying featured work loaded from RON configuration.
// Each project is shown as a card with title, description, tags, and action buttons.

use dioxus::prelude::*;

use crate::data::{load_language_colors, load_projects, ProjectButton};
use crate::styles::{BTN_PRIMARY, BTN_SECONDARY, FONT_INTER, GLASS_CARD_LIFT_STRONG};

#[component]
pub fn Projects() -> Element {
    let projects = load_projects();

    rsx! {
        section {
            id: "projects",
            class: "min-h-screen w-full py-20 px-8 sm:px-12 md:px-16 lg:px-20 xl:px-24 2xl:px-32",

            div { class: "max-w-7xl mx-auto",
                SectionTitle { title: "Featured Projects" }

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

#[component]
fn ProjectCard(
    title: String,
    description: String,
    tags: Vec<String>,
    buttons: Vec<ProjectButton>,
) -> Element {
    rsx! {
        div { class: "{GLASS_CARD_LIFT_STRONG}",
            h3 { class: "text-2xl font-light text-white mb-4 tracking-tight", "{title}" }

            p { class: "text-gray-400 mb-6 flex-grow leading-relaxed font-light text-[15px]",
                "{description}"
            }

            TagList { tags }

            if !buttons.is_empty() {
                ButtonGroup { buttons }
            }
        }
    }
}

#[component]
fn TagList(tags: Vec<String>) -> Element {
    let language_colors = load_language_colors();

    rsx! {
        div { class: "flex flex-wrap gap-2.5 mb-6",
            for tag in tags {
                Tag { name: tag.clone(), language_colors: language_colors.clone() }
            }
        }
    }
}

#[component]
fn Tag(
    name: String,
    language_colors: std::collections::HashMap<String, crate::data::LanguageColor>,
) -> Element {
    let (bg_class, text_class) = language_colors
        .get(&name)
        .map(|color| (color.bg_class.as_str(), color.text_class.as_str()))
        .unwrap_or((
            "bg-white/[0.06] hover:bg-white/[0.10] border-white/[0.08] hover:border-white/[0.12]",
            "text-gray-300",
        ));

    rsx! {
        span { class: "px-4 py-1.5 text-xs font-light rounded-full {bg_class} {text_class} border backdrop-blur-sm tracking-wide transition-all duration-300 ease-in-out cursor-default",
            "{name}"
        }
    }
}

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

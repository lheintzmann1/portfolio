//! # Experience Section Component
//!
//! Displays professional experience and education as an interactive timeline.
//! Features alternating left/right alignment on desktop, dots for one-off
//! events and bars for periods, which span the entries nested inside them.

use dioxus::prelude::*;

use crate::data::{load_experience, ExperienceEntry};
use crate::styles::{FONT_INTER, GLASS_CARD_LIFT};

// ============================================================================
// HELPERS
// ============================================================================

/// Fill for a period bar.
///
/// A finished period is capped at both ends. One that is still running
/// dissolves toward the top of the timeline — the direction the future lies
/// in — so it never reads as terminated.
fn period_fill(status: &Option<String>) -> &'static str {
    if status.is_some() {
        "bg-linear-to-b from-blue-400/0 via-blue-400 via-[6rem] to-blue-500/50"
    } else {
        "bg-linear-to-b from-blue-400/70 to-blue-500/50"
    }
}

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
                        TimelineEntry { entry }
                    }
                }
            }
        }
    }
}

/// A timeline entry together with whatever happened inside its period.
///
/// With no nested entries the marker lives on the card itself. With nested
/// ones the period bar is hoisted onto a wrapper around the whole group, so it
/// runs the full height of the period and the events sit as dots along it.
/// The entries inside come first and the period's own card closes the group,
/// which puts it level with the date the period started.
#[component]
fn TimelineEntry(entry: ExperienceEntry) -> Element {
    if entry.nested.is_empty() {
        return rsx! { ExperienceItem { entry, marked: true } };
    }

    let fill = period_fill(&entry.status);
    let nested = entry.nested.clone();

    rsx! {
        div { class: "relative space-y-12",
            // Period bar spanning the group
            div { class: "absolute top-0 left-0 md:left-1/2 -translate-x-1/2 w-0.75 h-full rounded-full {fill}" }

            for child in nested {
                TimelineEntry { entry: child }
            }

            ExperienceItem { entry, marked: false }
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

/// A single card on the timeline, with position, company, and achievements.
///
/// # Arguments
/// * `entry` - The experience data to render
/// * `marked` - Whether this card draws its own marker. `false` when the card
///   heads a group whose period bar is already drawn around it.
#[component]
fn ExperienceItem(entry: ExperienceEntry, marked: bool) -> Element {
    let ExperienceEntry {
        position,
        company,
        period,
        marker,
        status,
        description,
        achievements,
        align,
        ..
    } = entry;

    // Compute alignment-specific classes
    let alignment_classes = if align == "right" {
        "md:ml-auto md:pl-12"
    } else {
        "md:mr-auto md:pr-12"
    };

    // Markers straddle the timeline line: it runs along the left edge of cards
    // aligned right, and along the right edge of cards aligned left.
    let anchor = if align == "right" {
        "absolute top-0 left-0 -translate-x-1/2"
    } else {
        "absolute top-0 left-0 -translate-x-1/2 md:left-auto md:right-0 md:translate-x-1/2"
    };

    let range_fill = period_fill(&status);
    let status_label = status.unwrap_or_default();
    // Anything still running or still ahead: the marker must not read as closed.
    let unfinished = !status_label.is_empty();

    // "none" when the surrounding group already draws this entry's period bar.
    let marker_kind = if !marked {
        "none"
    } else if marker == "range" {
        "range"
    } else if unfinished {
        "open"
    } else {
        "dot"
    };

    rsx! {
        div { class: "relative pl-8 md:pl-0 md:w-1/2 {alignment_classes}",
            // Timeline marker: a bar for a period, a dot for a single event
            if marker_kind == "range" {
                div { class: "{anchor} w-0.75 h-full rounded-full {range_fill}" }
            } else if marker_kind == "open" {
                div { class: "{anchor} w-3 h-3",
                    span { class: "absolute inset-0 rounded-full bg-blue-400/40 animate-ping" }
                    span { class: "absolute inset-0 rounded-full border-2 border-blue-400 bg-[#080808]" }
                }
            } else if marker_kind == "dot" {
                div { class: "{anchor} w-3 h-3 rounded-full bg-linear-to-br from-blue-400 to-blue-500 \
                    border-4 border-[#080808] shadow-lg shadow-blue-400/30" }
            }

            // Content card
            div { class: "{GLASS_CARD_LIFT}",
                // Header with position and period
                div { class: "flex flex-col md:flex-row md:items-center md:justify-between mb-3",
                    div { class: "flex items-center gap-3",
                        h3 { class: "text-xl font-light text-white tracking-tight", "{position}" }
                        if unfinished {
                            span { class: "shrink-0 px-2.5 py-0.5 rounded-full border border-blue-400/30 \
                                bg-blue-400/10 text-blue-300 text-[10px] tracking-wider uppercase",
                                "{status_label}"
                            }
                        }
                    }
                    span { class: "text-gray-500 text-xs mt-1 md:mt-0 tracking-wider uppercase shrink-0",
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
            div { class: "w-1.5 h-1.5 rounded-full bg-linear-to-br from-blue-400/50 to-blue-400/20 mt-2 shrink-0" }
            span { class: "text-gray-500 text-sm font-light hover:text-white", "{text}" }
        }
    }
}

//! # Contact Section Component
//!
//! Contact section with information, social links, and a functional form.
//! Uses FormSubmit for serverless form handling.

use dioxus::prelude::*;

use crate::data::{load_contact, load_profile};
use crate::styles::{BTN_SUBMIT, FONT_INTER, GLASS_FORM_BG, INPUT_FIELD};

// ============================================================================
// COMPONENTS
// ============================================================================

/// Contact section with information, social links, and form.
///
/// Layout:
/// - Left column: Contact details and social media links
/// - Right column: Contact form (powered by FormSubmit)
#[component]
pub fn Contact() -> Element {
    let contact = load_contact();
    let profile = load_profile();

    rsx! {
        section {
            id: "contact",
            class: "min-h-screen w-full py-20 px-8 sm:px-12 md:px-16 lg:px-20 xl:px-24 2xl:px-32 flex items-center",

            div { class: "max-w-6xl mx-auto w-full",
                // Section header with title and intro
                ContactHeader { intro: contact.intro.clone() }

                // Two-column layout
                div { class: "grid md:grid-cols-2 gap-12",
                    // Left: Contact info and social links
                    ContactInfoSection {}

                    // Right: Contact form
                    ContactForm { form_action: contact.form_action }
                }
            }
        }

        // Page footer
        Footer { copyright_year: profile.copyright_year, name: profile.name }
    }
}

/// Section header with title, underline, and introduction text.
#[component]
fn ContactHeader(intro: String) -> Element {
    rsx! {
        div { class: "text-center mb-16",
            h2 {
                class: "text-4xl md:text-5xl font-bold text-white mb-4",
                style: "{FONT_INTER}",
                "Get In Touch"
            }
            div { class: "h-0.5 w-20 bg-blue-400 mx-auto mb-6" }
            p { class: "text-gray-300 text-lg max-w-2xl mx-auto", "{intro}" }
        }
    }
}

/// Left column with contact information and social links.
#[component]
fn ContactInfoSection() -> Element {
    let contact = load_contact();

    rsx! {
        div { class: "space-y-6",
            h3 { class: "text-2xl font-bold text-white mb-6", "Contact Information" }

            // Contact info entries
            for info in contact.info {
                ContactInfoItem { label: info.label, value: info.value, href: info.href }
            }

            // Social links section
            div { class: "pt-8",
                h4 { class: "text-xl font-semibold text-white mb-4", "Social Links" }
                div { class: "flex gap-4",
                    for link in contact.social_links {
                        SocialLink { name: link.name, href: link.url }
                    }
                }
            }
        }
    }
}

/// Individual contact information item (email, phone, location).
#[component]
fn ContactInfoItem(label: String, value: String, href: String) -> Element {
    rsx! {
        div { class: "flex flex-col",
            span { class: "text-gray-500 text-sm mb-1", "{label}" }
            a {
                href: "{href}",
                class: "text-white text-lg hover:text-blue-400 transition-colors",
                "{value}"
            }
        }
    }
}

/// Social media link button.
#[component]
fn SocialLink(name: String, href: String) -> Element {
    rsx! {
        a {
            href: "{href}",
            target: "_blank",
            rel: "noopener noreferrer",
            class: "px-6 py-2.5 rounded-full bg-white/10 hover:bg-white/20 text-white border border-white/20 transition-all duration-200 text-sm font-medium",
            "{name}"
        }
    }
}

/// Contact form with FormSubmit integration.
#[component]
fn ContactForm(form_action: String) -> Element {
    rsx! {
        div { class: "{GLASS_FORM_BG} backdrop-blur-2xl border border-white/[0.06] rounded-[32px] p-10 shadow-2xl shadow-black/40",
            form { action: "{form_action}", method: "POST", class: "space-y-6",

                // FormSubmit hidden configuration fields
                input { r#type: "hidden", name: "_captcha", value: "false" }
                input {
                    r#type: "hidden",
                    name: "_subject",
                    value: "New Portfolio Contact Form Submission",
                }
                input { r#type: "hidden", name: "_template", value: "table" }

                // Name field
                FormField {
                    label: "Name",
                    input_type: "text",
                    name: "name",
                    placeholder: "Your name",
                }

                // Email field
                FormField {
                    label: "Email",
                    input_type: "email",
                    name: "email",
                    placeholder: "your.email@example.com",
                }

                // Message field (textarea)
                div {
                    label { class: "block text-white mb-3 font-light text-sm tracking-wide",
                        "Message"
                    }
                    textarea {
                        name: "message",
                        required: true,
                        class: "{INPUT_FIELD} resize-none",
                        rows: "5",
                        placeholder: "Your message...",
                    }
                }

                // Submit button
                button { r#type: "submit", class: "{BTN_SUBMIT}", "Send Message" }
            }
        }
    }
}

/// Reusable form field component.
#[component]
fn FormField(
    label: &'static str,
    input_type: &'static str,
    name: &'static str,
    placeholder: &'static str,
) -> Element {
    rsx! {
        div {
            label { class: "block text-white mb-3 font-light text-sm tracking-wide", "{label}" }
            input {
                r#type: "{input_type}",
                name: "{name}",
                required: true,
                class: "{INPUT_FIELD}",
                placeholder: "{placeholder}",
            }
        }
    }
}

/// Page footer with copyright notice.
#[component]
fn Footer(copyright_year: u16, name: String) -> Element {
    rsx! {
        footer { class: "w-full py-8 px-8 border-t border-white/10",
            div { class: "max-w-7xl mx-auto text-center text-gray-400",
                p { "© {copyright_year} {name}. All rights reserved." }
            }
        }
    }
}

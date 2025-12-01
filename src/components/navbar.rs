use crate::assets::LH_WHITE_SVG;
use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        nav {
            class: "fixed top-0 left-0 w-full z-50 backdrop-blur-md bg-black/20 border-b border-white/10",
            div {
                class: "max-w-8xl mx-auto px-4 sm:px-6 lg:px-8",
                div {
                    class: "flex items-center h-16",

                    // Logo/Brand - Left side
                    div {
                        class: "flex-1 flex items-center space-x-3",
                        // Logo
                        div {
                            class: "w-10 h-10",
                            img {
                                class: "w-full h-full hover:brightness-110 transition-all duration-300",
                                src: "{LH_WHITE_SVG}",
                                alt: "Lucas Heintzmann Logo"
                            }
                        }
                        // Name
                        a {
                            href: "#",
                            class: "hidden md:flex md:selffont-bold ml-2.5 text-white hover:text-blue-400 transition-colors duration-300", // TODO: text-white or text-gray-300
                            style: "font-family: 'Outfit', sans-serif; font-weight: 400;", // TODO: local font
                            "Lucas HEINTZMANN"
                        }
                    }

                    // Navigation Links - Center
                    div {
                        class: "flex-1 flex justify-center items-baseline space-x-8",
                        NavLink { href: "#about", text: "About" }
                        NavLink { href: "#skills", text: "Skills" }
                        NavLink { href: "#projects", text: "Projects" }
                        NavLink { href: "#experience", text: "Experience" }
                        NavLink { href: "#contact", text: "Contact" }
                    }

                    // Right side spacer to maintain balance
                    // TODO: Replace with social icons (Github, LinkedIn, Email) or EN/FR toggle
                    div {
                        class: "flex-1"
                    }
                }
            }
        }
    }
}

#[component]
fn NavLink(href: String, text: String) -> Element {
    rsx! {
        a {
            href: "{href}",
            class: "text-white/90 hover:text-white hover:bg-white/10 px-3 py-2 rounded-md text-sm font-medium transition-all duration-300 relative group",
            "{text}"
            // Animated underline effect
            span {
                class: "absolute bottom-0 left-0 w-0 h-0.5 bg-blue-400 group-hover:w-full transition-all duration-300"
            }
        }
    }
}

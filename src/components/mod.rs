//! # Components Module
//!
//! Contains all reusable UI components for the portfolio.
//!
//! ## Component Hierarchy
//!
//! - [`Navbar`]: Fixed navigation header with links to all sections
//! - [`Hero`]: Landing section with video background
//!   - [`HeroContent`]: Text content displayed over the hero video
//! - [`About`]: Personal introduction and statistics
//! - [`Skills`]: Technical skills organized by category
//! - [`Projects`]: Featured project showcase grid
//! - [`Experience`]: Professional timeline
//! - [`Contact`]: Contact form and social links

// Section components
mod about;
mod contact;
mod experience;
mod hero;
mod hero_content;
mod navbar;
mod projects;
mod skills;

// Public exports
pub use about::About;
pub use contact::Contact;
pub use experience::Experience;
pub use hero::Hero;
pub use hero_content::HeroContent;
pub use navbar::Navbar;
pub use projects::Projects;
pub use skills::Skills;

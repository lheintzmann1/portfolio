// Data loading and parsing module.
// All content is loaded from RON files in assets/data/ at compile time.

use serde::Deserialize;

// RON file contents embedded at compile time
const PROFILE_RON: &str = include_str!("../assets/data/profile.ron");
const CONTACT_RON: &str = include_str!("../assets/data/contact.ron");
const SKILLS_RON: &str = include_str!("../assets/data/skills.ron");
const EXPERIENCE_RON: &str = include_str!("../assets/data/experience.ron");
const NAVIGATION_RON: &str = include_str!("../assets/data/navigation.ron");
const PROJECTS_RON: &str = include_str!("../assets/data/projects.ron");
const LANGUAGE_COLORS_RON: &str = include_str!("../assets/data/language_colors.ron");

// Helper function to parse RON and provide better error messages
fn parse_ron<T: for<'de> Deserialize<'de>>(content: &str, filename: &str) -> T {
    ron::from_str(content).unwrap_or_else(|e| panic!("Failed to parse {}: {}", filename, e))
}

// Profile data structures

#[derive(Clone, PartialEq, Deserialize)]
pub struct Stat {
    pub value: String,
    pub label: String,
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct ProfileData {
    pub name: String,
    pub initials: String,
    pub role: String,
    pub headline: String,
    pub intro: String,
    pub bio: Vec<String>,
    pub stats: Vec<Stat>,
    pub copyright_year: u16,
}

pub fn load_profile() -> ProfileData {
    parse_ron(PROFILE_RON, "profile.ron")
}

// Contact data structures

#[derive(Clone, PartialEq, Deserialize)]
pub struct ContactInfo {
    pub label: String,
    pub value: String,
    pub href: String,
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct SocialLink {
    pub name: String,
    pub url: String,
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct ContactData {
    pub intro: String,
    pub form_action: String,
    pub info: Vec<ContactInfo>,
    pub social_links: Vec<SocialLink>,
}

pub fn load_contact() -> ContactData {
    parse_ron(CONTACT_RON, "contact.ron")
}

// Skills data structures

#[derive(Clone, PartialEq, Deserialize)]
pub struct SkillCategory {
    pub title: String,
    pub skills: Vec<String>,
}

pub fn load_skills() -> Vec<SkillCategory> {
    parse_ron(SKILLS_RON, "skills.ron")
}

// Experience data structures

#[derive(Clone, PartialEq, Deserialize)]
pub struct ExperienceEntry {
    pub position: String,
    pub company: String,
    pub period: String,
    pub description: String,
    pub achievements: Vec<String>,
    pub align: String,
}

pub fn load_experience() -> Vec<ExperienceEntry> {
    parse_ron(EXPERIENCE_RON, "experience.ron")
}

// Navigation data structures

#[derive(Clone, PartialEq, Deserialize)]
pub struct NavLink {
    pub href: String,
    pub text: String,
}

pub fn load_navigation() -> Vec<NavLink> {
    parse_ron(NAVIGATION_RON, "navigation.ron")
}

// Projects data structures

#[derive(Clone, PartialEq, Deserialize)]
pub struct ProjectButton {
    pub label: String,
    pub url: String,
    pub primary: bool,
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct ProjectData {
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
    pub buttons: Vec<ProjectButton>,
}

pub fn load_projects() -> Vec<ProjectData> {
    parse_ron(PROJECTS_RON, "projects.ron")
}

// Language colors data structures

#[derive(Clone, PartialEq, Deserialize)]
pub struct LanguageColor {
    pub bg_class: String,
    pub text_class: String,
}

pub fn load_language_colors() -> std::collections::HashMap<String, LanguageColor> {
    parse_ron(LANGUAGE_COLORS_RON, "language_colors.ron")
}

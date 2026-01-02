//! # Data Module
//!
//! Handles loading and parsing of JSON data files embedded at compile time.
//! All content data (profile, skills, experience, etc.) is loaded from
//! the `assets/data/` directory.

use serde::Deserialize;

// ============================================================================
// JSON FILE PATHS (embedded at compile time)
// ============================================================================

const PROFILE_JSON: &str = include_str!("../assets/data/profile.json");
const CONTACT_JSON: &str = include_str!("../assets/data/contact.json");
const SKILLS_JSON: &str = include_str!("../assets/data/skills.json");
const EXPERIENCE_JSON: &str = include_str!("../assets/data/experience.json");
const NAVIGATION_JSON: &str = include_str!("../assets/data/navigation.json");
const PROJECTS_JSON: &str = include_str!("../assets/data/projects.json");

// ============================================================================
// PROFILE DATA
// ============================================================================

/// Statistic item (e.g., "3+" years of experience).
#[derive(Clone, PartialEq, Deserialize)]
pub struct Stat {
    pub value: String,
    pub label: String,
}

/// Profile/personal information.
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

/// Loads profile data from embedded JSON.
pub fn load_profile() -> ProfileData {
    serde_json::from_str(PROFILE_JSON).expect("Failed to parse profile.json")
}

// ============================================================================
// CONTACT DATA
// ============================================================================

/// Contact information entry (email, phone, location).
#[derive(Clone, PartialEq, Deserialize)]
pub struct ContactInfo {
    pub label: String,
    pub value: String,
    pub href: String,
}

/// Social media link.
#[derive(Clone, PartialEq, Deserialize)]
pub struct SocialLink {
    pub name: String,
    pub url: String,
}

/// Contact section data.
#[derive(Clone, PartialEq, Deserialize)]
pub struct ContactData {
    pub intro: String,
    pub form_action: String,
    pub info: Vec<ContactInfo>,
    pub social_links: Vec<SocialLink>,
}

/// Loads contact data from embedded JSON.
pub fn load_contact() -> ContactData {
    serde_json::from_str(CONTACT_JSON).expect("Failed to parse contact.json")
}

// ============================================================================
// SKILLS DATA
// ============================================================================

/// Skill category with list of skills.
#[derive(Clone, PartialEq, Deserialize)]
pub struct SkillCategory {
    pub title: String,
    pub skills: Vec<String>,
}

/// Loads skills data from embedded JSON.
pub fn load_skills() -> Vec<SkillCategory> {
    serde_json::from_str(SKILLS_JSON).expect("Failed to parse skills.json")
}

// ============================================================================
// EXPERIENCE DATA
// ============================================================================

/// Experience/education timeline entry.
#[derive(Clone, PartialEq, Deserialize)]
pub struct ExperienceEntry {
    pub position: String,
    pub company: String,
    pub period: String,
    pub description: String,
    pub achievements: Vec<String>,
    pub align: String,
}

/// Loads experience data from embedded JSON.
pub fn load_experience() -> Vec<ExperienceEntry> {
    serde_json::from_str(EXPERIENCE_JSON).expect("Failed to parse experience.json")
}

// ============================================================================
// NAVIGATION DATA
// ============================================================================

/// Navigation link item.
#[derive(Clone, PartialEq, Deserialize)]
pub struct NavLink {
    pub href: String,
    pub text: String,
}

/// Loads navigation links from embedded JSON.
pub fn load_navigation() -> Vec<NavLink> {
    serde_json::from_str(NAVIGATION_JSON).expect("Failed to parse navigation.json")
}

// ============================================================================
// PROJECTS DATA
// ============================================================================

/// Action button for a project card.
#[derive(Clone, PartialEq, Deserialize)]
pub struct ProjectButton {
    pub label: String,
    pub url: String,
    pub primary: bool,
}

/// Project data structure.
#[derive(Clone, PartialEq, Deserialize)]
pub struct ProjectData {
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
    pub buttons: Vec<ProjectButton>,
}

/// Loads project data from embedded JSON.
pub fn load_projects() -> Vec<ProjectData> {
    serde_json::from_str(PROJECTS_JSON).expect("Failed to parse projects.json")
}

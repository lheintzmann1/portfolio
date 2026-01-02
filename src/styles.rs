//! # Shared Style Constants
//!
//! Centralizes reusable CSS classes and style definitions used across components.
//! This module ensures visual consistency and reduces duplication.

// ============================================================================
// TYPOGRAPHY
// ============================================================================

/// Inter font family declaration for headings and body text.
pub const FONT_INTER: &str = "font-family: 'Inter', sans-serif;";

/// Outfit font family declaration for branding/logo text.
pub const FONT_OUTFIT: &str = "font-family: 'Outfit', sans-serif;";

// ============================================================================
// GLASSMORPHISM EFFECTS
// ============================================================================

/// Standard glassmorphism card with subtle gradient and hover effects.
/// Used for: skill cards, project cards, experience timeline items.
pub const GLASS_CARD: &str = "group \
    bg-[linear-gradient(135deg,rgba(255,255,255,0.06)_0%,rgba(255,255,255,0.04)_50%,rgba(255,255,255,0.02)_100%)] \
    hover:bg-[linear-gradient(135deg,rgba(255,255,255,0.08)_0%,rgba(255,255,255,0.06)_50%,rgba(255,255,255,0.04)_100%)] \
    backdrop-blur-2xl border border-white/[0.06] rounded-[28px] p-8 \
    hover:border-white/[0.10] transition-all duration-500 ease-in-out \
    shadow-2xl shadow-black/40 hover:shadow-blue-500/5";

/// Glassmorphism card with lift effect on hover.
/// Used for: interactive cards that should appear to lift.
pub const GLASS_CARD_LIFT: &str = "group \
    bg-[linear-gradient(135deg,rgba(255,255,255,0.06)_0%,rgba(255,255,255,0.04)_50%,rgba(255,255,255,0.02)_100%)] \
    hover:bg-[linear-gradient(135deg,rgba(255,255,255,0.08)_0%,rgba(255,255,255,0.06)_50%,rgba(255,255,255,0.04)_100%)] \
    backdrop-blur-2xl border border-white/[0.06] rounded-[28px] p-8 \
    hover:border-white/[0.10] transition-all duration-500 ease-in-out \
    shadow-2xl shadow-black/40 hover:shadow-blue-500/5 hover:-translate-y-1";

/// Glassmorphism card with stronger lift effect.
/// Used for: project cards with more prominent hover interaction.
pub const GLASS_CARD_LIFT_STRONG: &str = "group \
    bg-[linear-gradient(135deg,rgba(255,255,255,0.06)_0%,rgba(255,255,255,0.04)_50%,rgba(255,255,255,0.02)_100%)] \
    hover:bg-[linear-gradient(135deg,rgba(255,255,255,0.08)_0%,rgba(255,255,255,0.06)_50%,rgba(255,255,255,0.04)_100%)] \
    backdrop-blur-2xl border border-white/[0.06] rounded-[28px] p-8 \
    hover:border-white/[0.10] transition-all duration-500 ease-in-out flex flex-col \
    shadow-2xl shadow-black/40 hover:shadow-blue-500/5 hover:-translate-y-2";

/// Glassmorphism background for larger containers (e.g., about avatar, contact form).
/// Has slightly different opacity values for a softer appearance.
pub const GLASS_CONTAINER_BG: &str = "bg-[linear-gradient(135deg,rgba(255,255,255,0.07)_0%,\
    rgba(255,255,255,0.05)_25%,rgba(255,255,255,0.03)_50%,\
    rgba(255,255,255,0.02)_75%,rgba(255,255,255,0.02)_100%)]";

/// Glassmorphism form container styling.
pub const GLASS_FORM_BG: &str = "bg-[linear-gradient(135deg,rgba(255,255,255,0.06)_0%,\
    rgba(255,255,255,0.04)_50%,rgba(255,255,255,0.02)_100%)]";

/// Avatar/profile circle gradient background.
pub const AVATAR_GRADIENT: &str = "bg-[linear-gradient(135deg,rgba(255,255,255,0.15)_0%,\
    rgba(255,255,255,0.10)_50%,rgba(255,255,255,0.05)_100%)]";

// ============================================================================
// BUTTONS
// ============================================================================

/// Primary action button (filled white, blue on hover).
pub const BTN_PRIMARY: &str = "flex-1 text-center px-5 py-3 rounded-full \
    bg-white hover:bg-blue-400 text-black hover:text-white \
    transition-all duration-300 ease-in-out text-sm font-medium tracking-wide \
    shadow-lg hover:shadow-xl hover:shadow-blue-400/30";

/// Secondary/ghost button (outline style).
pub const BTN_SECONDARY: &str = "flex-1 text-center px-5 py-3 rounded-full \
    bg-white/[0.06] hover:bg-white/[0.10] text-white \
    border border-white/[0.08] hover:border-white/[0.12] \
    transition-all duration-300 ease-in-out text-sm font-light tracking-wide";

/// Submit button for forms (full width, with scale effect).
pub const BTN_SUBMIT: &str = "w-full px-6 py-4 rounded-full bg-white hover:bg-blue-400 \
    text-black hover:text-white font-medium tracking-wide transition-all duration-300 \
    ease-in-out hover:shadow-2xl hover:shadow-blue-400/30 cursor-pointer hover:scale-[1.02]";

// ============================================================================
// FORM ELEMENTS
// ============================================================================

/// Standard input field styling with glassmorphism effect.
pub const INPUT_FIELD: &str = "w-full px-5 py-4 rounded-2xl bg-white/[0.04] backdrop-blur-sm \
    border border-white/[0.08] text-white placeholder-gray-500 \
    focus:border-white/[0.15] focus:bg-white/[0.06] focus:outline-none \
    transition-all duration-300 ease-in-out font-light";

// ============================================================================
// LAYOUT
// ============================================================================

/// Standard section padding for responsive layouts.
pub const SECTION_PADDING: &str = "py-20 px-8 sm:px-12 md:px-16 lg:px-20 xl:px-24 2xl:px-32";

/// Full-screen section base styling.
pub const SECTION_FULL: &str = "min-h-screen w-full py-20 px-8 sm:px-12 md:px-16 lg:px-20 xl:px-24 2xl:px-32";

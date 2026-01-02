//! # Static Assets Module
//!
//! Centralizes all static asset declarations for the portfolio.
//! Assets are embedded at compile time using Dioxus's `asset!` macro,
//! ensuring they are bundled with the application.

use dioxus::prelude::*;

// ============================================================================
// FAVICON
// ============================================================================

/// Browser tab icon (ICO format) for the portfolio.
pub const FAVICON_ICO: Asset = asset!("/assets/favicon/favicon.ico");

/// 16x16 PNG favicon.
pub const FAVICON_16: Asset = asset!("/assets/favicon/favicon-16x16.png");

/// 32x32 PNG favicon.
pub const FAVICON_32: Asset = asset!("/assets/favicon/favicon-32x32.png");

/// Apple touch icon for iOS devices.
pub const APPLE_TOUCH_ICON: Asset = asset!("/assets/favicon/apple-touch-icon.png");

/// Web manifest for PWA support.
pub const SITE_WEBMANIFEST: Asset = asset!("/assets/favicon/site.webmanifest");

// ============================================================================
// STYLESHEETS
// ============================================================================

/// Main custom styles for the portfolio.
pub const MAIN_CSS: Asset = asset!("/assets/main.css");

/// Tailwind CSS utility classes.
pub const TW_CSS: Asset = asset!("/assets/tailwind.css");

// ============================================================================
// GRAPHICS
// ============================================================================

/// White version of the "LH" logo used in the navbar.
pub const LH_WHITE_SVG: Asset = asset!("/assets/lh-white.svg");

/// Custom cursor image for enhanced UX.
pub const CURSOR_PNG: Asset = asset!("/assets/cursor.png");

// ============================================================================
// MEDIA
// ============================================================================

/// Background video for the hero section (black hole animation).
pub const BLACKHOLE_WEBM: Asset = asset!("/assets/blackhole.webm");

/// Profile photo displayed in the about section.
pub const ME_PNG: Asset = asset!("/assets/me.png");

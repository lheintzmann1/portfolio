// Static asset declarations for the portfolio.
// Assets are embedded at compile time using Dioxus's asset! macro.

use dioxus::prelude::*;

// Favicon files for browser tabs and mobile devices
pub const FAVICON_ICO: Asset = asset!("/assets/favicon/favicon.ico");
pub const FAVICON_16: Asset = asset!("/assets/favicon/favicon-16x16.png");
pub const FAVICON_32: Asset = asset!("/assets/favicon/favicon-32x32.png");
pub const APPLE_TOUCH_ICON: Asset = asset!("/assets/favicon/apple-touch-icon.png");
pub const SITE_WEBMANIFEST: Asset = asset!("/assets/favicon/site.webmanifest");

// Stylesheets
pub const MAIN_CSS: Asset = asset!("/assets/main.css");
pub const TW_CSS: Asset = asset!("/assets/tailwind.css");

// Graphics and icons
pub const LH_WHITE_SVG: Asset = asset!("/assets/lh-white.svg");
pub const CURSOR_PNG: Asset = asset!("/assets/cursor.png");

// Media files
pub const BLACKHOLE_WEBM: Asset = asset!("/assets/blackhole.webm");
pub const ME_PNG: Asset = asset!("/assets/me.png");

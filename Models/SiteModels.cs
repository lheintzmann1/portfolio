// Content models mirroring the JSON files in Data/.
// Property names map to snake_case JSON keys via SiteJsonContext's naming policy.

namespace Portfolio.Models;

// Profile

public sealed record Stat(string Value, string Label);

public sealed record ProfileData(
    string Name,
    string Initials,
    string Role,
    string Headline,
    string Intro,
    IReadOnlyList<string> Bio,
    IReadOnlyList<Stat> Stats,
    ushort CopyrightYear);

// Contact

public sealed record ContactInfo(string Label, string Value, string Href);

public sealed record SocialLink(string Name, string Url);

public sealed record ContactData(
    string Intro,
    string FormAction,
    IReadOnlyList<ContactInfo> Info,
    IReadOnlyList<SocialLink> SocialLinks);

// Skills

public sealed record SkillCategory(string Title, IReadOnlyList<string> Skills);

// Experience

public sealed record ExperienceEntry(
    string Position,
    string Company,
    string Period,
    /// "point" for a one-off event, "range" for a span of time.
    string Marker,
    /// Badge label for anything not finished yet ("In progress", "Seeking").
    /// null means the entry is over and done with.
    string? Status,
    string Description,
    IReadOnlyList<string> Achievements,
    string Align,
    /// Entries that fall inside this one's period. A range marker is drawn
    /// around the whole group, so a diploma visibly spans the internships
    /// taken during it instead of stopping at its own card.
    IReadOnlyList<ExperienceEntry> Nested);

// Navigation

public sealed record NavLink(string Href, string Text);

// Projects

public sealed record ProjectButton(string Label, string Url, bool Primary);

public sealed record ProjectData(
    string Title,
    string Description,
    IReadOnlyList<string> Tags,
    IReadOnlyList<ProjectButton> Buttons);

// Language colors

public sealed record LanguageColor(string BgClass, string TextClass);

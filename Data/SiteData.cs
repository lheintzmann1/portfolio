// Loads all site content from the JSON files embedded at build time.
//
// The files are embedded resources rather than fetched from wwwroot, so content is
// available on the very first render with no round trip. Each file is parsed once,
// on first access, and cached for the lifetime of the app.

using System.Reflection;
using System.Text.Json;
using System.Text.Json.Serialization;
using System.Text.Json.Serialization.Metadata;
using Portfolio.Models;

namespace Portfolio.Data;

[JsonSourceGenerationOptions(PropertyNamingPolicy = JsonKnownNamingPolicy.SnakeCaseLower)]
[JsonSerializable(typeof(ProfileData))]
[JsonSerializable(typeof(ContactData))]
[JsonSerializable(typeof(List<SkillCategory>))]
[JsonSerializable(typeof(List<ExperienceEntry>))]
[JsonSerializable(typeof(List<NavLink>))]
[JsonSerializable(typeof(List<ProjectData>))]
[JsonSerializable(typeof(Dictionary<string, LanguageColor>))]
internal sealed partial class SiteJsonContext : JsonSerializerContext;

public static class SiteData
{
    public static ProfileData Profile => _profile.Value;
    public static ContactData Contact => _contact.Value;
    public static IReadOnlyList<SkillCategory> Skills => _skills.Value;
    public static IReadOnlyList<ExperienceEntry> Experience => _experience.Value;
    public static IReadOnlyList<NavLink> Navigation => _navigation.Value;
    public static IReadOnlyList<ProjectData> Projects => _projects.Value;
    public static IReadOnlyDictionary<string, LanguageColor> LanguageColors => _languageColors.Value;

    private static readonly Lazy<ProfileData> _profile =
        new(() => Load("profile.json", SiteJsonContext.Default.ProfileData));

    private static readonly Lazy<ContactData> _contact =
        new(() => Load("contact.json", SiteJsonContext.Default.ContactData));

    private static readonly Lazy<List<SkillCategory>> _skills =
        new(() => Load("skills.json", SiteJsonContext.Default.ListSkillCategory));

    private static readonly Lazy<List<ExperienceEntry>> _experience =
        new(() => Load("experience.json", SiteJsonContext.Default.ListExperienceEntry));

    private static readonly Lazy<List<NavLink>> _navigation =
        new(() => Load("navigation.json", SiteJsonContext.Default.ListNavLink));

    private static readonly Lazy<List<ProjectData>> _projects =
        new(() => Load("projects.json", SiteJsonContext.Default.ListProjectData));

    private static readonly Lazy<Dictionary<string, LanguageColor>> _languageColors =
        new(() => Load("language_colors.json", SiteJsonContext.Default.DictionaryStringLanguageColor));

    private static T Load<T>(string fileName, JsonTypeInfo<T> typeInfo)
    {
        var resource = $"Portfolio.Data.{fileName}";
        using var stream = Assembly.GetExecutingAssembly().GetManifestResourceStream(resource)
            ?? throw new InvalidOperationException($"Embedded resource not found: {resource}");

        return JsonSerializer.Deserialize(stream, typeInfo)
            ?? throw new InvalidOperationException($"Failed to parse {fileName}");
    }
}

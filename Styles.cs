// Shared style constants for visual consistency across components.
//
// These strings are scanned by Tailwind (see the @source directives in tailwind.css),
// so they must stay as literal class names here.

namespace Portfolio;

public static class Styles
{
    // Typography

    public const string FontInter = "font-family: 'Inter', sans-serif;";
    public const string FontOutfit = "font-family: 'Outfit', sans-serif;";

    // Glassmorphism effects
    // These provide a frosted glass appearance with subtle gradients

    public const string GlassCard =
        "group " +
        "bg-[linear-gradient(135deg,rgba(255,255,255,0.06)_0%,rgba(255,255,255,0.04)_50%,rgba(255,255,255,0.02)_100%)] " +
        "hover:bg-[linear-gradient(135deg,rgba(255,255,255,0.08)_0%,rgba(255,255,255,0.06)_50%,rgba(255,255,255,0.04)_100%)] " +
        "backdrop-blur-2xl border border-white/[0.06] rounded-[28px] p-8 " +
        "hover:border-white/[0.10] transition-all duration-500 ease-in-out " +
        "shadow-2xl shadow-black/40 hover:shadow-blue-500/5";

    public const string GlassCardLift =
        "group " +
        "bg-[linear-gradient(135deg,rgba(255,255,255,0.06)_0%,rgba(255,255,255,0.04)_50%,rgba(255,255,255,0.02)_100%)] " +
        "hover:bg-[linear-gradient(135deg,rgba(255,255,255,0.08)_0%,rgba(255,255,255,0.06)_50%,rgba(255,255,255,0.04)_100%)] " +
        "backdrop-blur-2xl border border-white/[0.06] rounded-[28px] p-8 " +
        "hover:border-white/[0.10] transition-all duration-500 ease-in-out " +
        "shadow-2xl shadow-black/40 hover:shadow-blue-500/5 hover:-translate-y-1";

    public const string GlassCardLiftStrong =
        "group " +
        "bg-[linear-gradient(135deg,rgba(255,255,255,0.06)_0%,rgba(255,255,255,0.04)_50%,rgba(255,255,255,0.02)_100%)] " +
        "hover:bg-[linear-gradient(135deg,rgba(255,255,255,0.08)_0%,rgba(255,255,255,0.06)_50%,rgba(255,255,255,0.04)_100%)] " +
        "backdrop-blur-2xl border border-white/[0.06] rounded-[28px] p-8 " +
        "hover:border-white/[0.10] transition-all duration-500 ease-in-out flex flex-col " +
        "shadow-2xl shadow-black/40 hover:shadow-blue-500/5 hover:-translate-y-2";

    public const string GlassContainerBg =
        "bg-[linear-gradient(135deg,rgba(255,255,255,0.07)_0%," +
        "rgba(255,255,255,0.05)_25%,rgba(255,255,255,0.03)_50%," +
        "rgba(255,255,255,0.02)_75%,rgba(255,255,255,0.02)_100%)]";

    public const string GlassFormBg =
        "bg-[linear-gradient(135deg,rgba(255,255,255,0.06)_0%," +
        "rgba(255,255,255,0.04)_50%,rgba(255,255,255,0.02)_100%)]";

    public const string AvatarGradient =
        "bg-[linear-gradient(135deg,rgba(255,255,255,0.15)_0%," +
        "rgba(255,255,255,0.10)_50%,rgba(255,255,255,0.05)_100%)]";

    // Buttons

    public const string BtnPrimary =
        "flex-1 text-center px-5 py-3 rounded-full " +
        "bg-white hover:bg-blue-400 text-black hover:text-white " +
        "transition-all duration-300 ease-in-out text-sm font-medium tracking-wide " +
        "shadow-lg hover:shadow-xl hover:shadow-blue-400/30";

    public const string BtnSecondary =
        "flex-1 text-center px-5 py-3 rounded-full " +
        "bg-white/[0.06] hover:bg-white/[0.10] text-white " +
        "border border-white/[0.08] hover:border-white/[0.12] " +
        "transition-all duration-300 ease-in-out text-sm font-light tracking-wide";

    public const string BtnSubmit =
        "w-full px-6 py-4 rounded-full bg-white hover:bg-blue-400 " +
        "text-black hover:text-white font-medium tracking-wide transition-all duration-300 " +
        "ease-in-out hover:shadow-2xl hover:shadow-blue-400/30 cursor-pointer hover:scale-[1.02]";

    // Form elements

    public const string InputField =
        "w-full px-5 py-4 rounded-2xl bg-white/[0.04] backdrop-blur-sm " +
        "border border-white/[0.08] text-white placeholder-gray-500 " +
        "focus:border-white/[0.15] focus:bg-white/[0.06] focus:outline-none " +
        "transition-all duration-300 ease-in-out font-light";

    // Layout utilities

    public const string SectionPadding = "py-20 px-8 sm:px-12 md:px-16 lg:px-20 xl:px-24 2xl:px-32";

    public const string SectionFull =
        "min-h-screen w-full py-20 px-8 sm:px-12 md:px-16 lg:px-20 xl:px-24 2xl:px-32";
}

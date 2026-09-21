# Portfolio

Personal portfolio at [luhe.dev](https://luhe.dev). Blazor WebAssembly + Tailwind CSS,
deployed as static files to GitHub Pages.

```
portfolio/
├─ Components/   # Razor components, one per page section
├─ Pages/        # Index.razor - the whole single-page site
├─ Models/       # Records mirroring the JSON content files
├─ Data/         # Site content as JSON, embedded at build time, + the loader
├─ wwwroot/      # Static assets, index.html, starfield/video JS
├─ Styles.cs     # Shared Tailwind class strings
└─ tailwind.css  # Tailwind input file (@source globs live here)
```

Content lives in `Data/*.json` and is embedded as a resource, so editing it requires a
rebuild but never a network request at runtime.

## Prerequisites

- .NET 10 SDK
- ASP.NET Core 10 runtime (for `dotnet watch`; not needed to publish)
- Node.js (Tailwind CLI only)

```bash
npm install
```

## Developing

Runs the Tailwind watcher and the app together:

```bash
npm run dev
```

Or separately:

```bash
npm run dev:css     # tailwind --watch
dotnet watch
```

## Building

```bash
npm run build:css
dotnet publish -c Release -o publish
```

The result in `publish/wwwroot/` is a static site. To preview exactly what GitHub Pages
will serve:

```bash
python3 -m http.server --directory publish/wwwroot 5199
```

Pushing to `main` runs the same steps in `.github/workflows/deploy.yml` and deploys.

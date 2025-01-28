import { defineConfig } from "vitepress";

// https://vitepress.dev/reference/site-config
export default defineConfig({
    title: "Rsfrac",
    description:
        "The Terminal-Based Fractal Explorer. Rsfrac is your terminal gateway to Mandelbrot, Burning Ship, and Julia.",
    srcDir: "src",
    appearance: "force-dark",
    themeConfig: {
        siteTitle: "Rsfrac",
        // https://vitepress.dev/reference/default-theme-config
        nav: [
            { text: "Home", link: "/" },
            { text: "Getting Started", link: "/getting-started" },
        ],
        outline: "deep",
        footer: {
            message: "Released under the GNU General Public License (GPLv3).",
            copyright: "Copyright © 2021-present Léopold Koprivnik",
        },
        search: {
            provider: "local",
        },
        editLink: {
            pattern: "https://github.com/skwalexe/rsfrac/edit/main/docs/src/:path",
        },
        sidebar: [
            {
                text: "Introduction",
                items: [
                    { text: "📥 Getting Started", link: "/getting-started" },
                    { text: "❗  Color Issues", link: "/color-issues" },
                    { text: "🌟 Screenshots", link: "/screenshots" },
                ],
            },
            {
                text: "Guides",
                items: [
                    { text: "💎 General Presentation", link: "/presentation" },
                    { text: "📜 The Logs Panel", link: "/the-logs-panel" },
                    { text: "🛠️ The Command System", link: "/the-command-system" },
                    { text: "🧭 Navigation", link: "/navigation" },
                    { text: "📷 Taking Screenshots", link: "/taking-screenshots" },
                    { text: "⚡ GPU Mode", link: "/gpu-mode" },
                    { text: "🔢 Arbitrary Precision", link: "/arbitrary-precision" },
                    { text: "🎨 Color Palettes", link: "/color-palettes" },
                    { text: "💠 Render Settings", link: "/render-settings" },
                    { text: "📖 Fractal Logic", link: "/fractal-logic" },
                ],
            },
            {
                text: "Development",
                items: [
                    { text: "🫂 Contributing", link: "/contributing" },
                    { text: "✅ Recommended IDE setup", link: "/recommended-ide-setup" },
                    { text: "🏗️ Project setup", link: "/project-setup" },
                    {
                        text: "🩷 Creating a pull request",
                        link: "/creating-a-pull-request",
                    },
                ],
            },
        ],

        socialLinks: [
            { icon: "github", link: "https://github.com/skwalexe/rsfrac" },
        ],
    },
});

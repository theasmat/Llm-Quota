import daisyui from "daisyui";
import containerQueries from "@tailwindcss/container-queries";

/** @type {import('tailwindcss').Config} */
export default {
    content: [
        "./index.html",
        "./src/**/*.{js,ts,jsx,tsx}",
    ],
    darkMode: 'class',
    theme: {
        extend: {
            colors: {
                primary: "#3ecf8e",
                "primary-deep": "#24b47e",
                "primary-soft": "#4ade80",
                ink: "#171717",
                "ink-secondary": "#212121",
                "ink-mute": "#707070",
                "ink-mute-2": "#9a9a9a",
                "ink-faint": "#b2b2b2",
                "on-primary": "#171717",
                "on-dark": "#ffffff",
                canvas: "#ffffff",
                "canvas-soft": "#fafafa",
                "canvas-night": "#1c1c1c",
                "canvas-night-soft": "#202020",
                hairline: "#dfdfdf",
                "hairline-strong": "#c7c7c7",
                "hairline-cool": "#ededed",
                "hairline-cool-2": "#efefef",
                "hairline-cool-3": "#d4d4d4",
                "accent-purple": "#6b01c2",
                "accent-violet": "#644fc1",
                "accent-purple-soft": "#eddbf9",
                "accent-yellow": "#ffdb13",
                "accent-tomato": "#ff2201",
                "accent-pink": "#c7007e",
                "accent-indigo": "#054cff",
                "accent-crimson": "#e2005a",
                background: "hsl(var(--background))",
                foreground: "hsl(var(--foreground))",
            },
            fontFamily: {
                sans: ["Circular", "Inter", "Helvetica Neue", "Helvetica", "Arial", "sans-serif"],
                mono: ["ui-monospace", "Menlo", "Monaco", "Consolas", "Liberation Mono", "monospace"],
            },
            fontSize: {
                "display-xxl": ["64px", { lineHeight: "1.1", letterSpacing: "-1.92px", fontWeight: "500" }],
                "display-xl": ["48px", { lineHeight: "1.1", letterSpacing: "-1.44px", fontWeight: "500" }],
                "display-lg": ["36px", { lineHeight: "1.15", letterSpacing: "-0.72px", fontWeight: "500" }],
                "display-md": ["28px", { lineHeight: "1.2", letterSpacing: "-0.42px", fontWeight: "500" }],
                "heading-lg": ["22px", { lineHeight: "1.2", fontWeight: "500" }],
                "heading-md": ["18px", { lineHeight: "1.4", fontWeight: "500" }],
                "body-lg": ["18px", { lineHeight: "1.55", fontWeight: "400" }],
                "body-md": ["16px", { lineHeight: "1.5", fontWeight: "400" }],
                "button-md": ["14px", { lineHeight: "1.0", fontWeight: "500" }],
                "caption": ["13px", { lineHeight: "1.45", fontWeight: "400" }],
                "micro": ["12px", { lineHeight: "1.45", fontWeight: "400" }],
                "code": ["14px", { lineHeight: "1.5", fontWeight: "400" }],
            },
            borderRadius: {
                "xs": "4px",
                "sm": "6px",
                "md": "8px",
                "lg": "12px",
                "xl": "16px",
                "full": "9999px",
            },
            spacing: {
                "xxs": "2px",
                "xs": "4px",
                "sm": "8px",
                "md": "12px",
                "lg": "16px",
                "xl": "24px",
                "xxl": "32px",
                "huge": "64px",
            },
            animation: {
                'float': 'float 6s ease-in-out infinite',
                'pulse-slow': 'pulse 4s cubic-bezier(0.4, 0, 0.6, 1) infinite',
            },
            keyframes: {
                'float': {
                    '0%, 100%': { transform: 'translateY(0)' },
                    '50%': { transform: 'translateY(-10px)' },
                }
            }
        },
    },
    plugins: [daisyui, containerQueries],
    daisyui: {
        themes: [
            {
                light: {
                    "primary": "#3ecf8e",
                    "secondary": "#212121",
                    "accent": "#4ade80",
                    "neutral": "#171717",
                    "base-100": "#ffffff",
                    "base-200": "#fafafa",
                    "base-300": "#efefef",
                    "info": "#054cff",
                    "success": "#3ecf8e",
                    "warning": "#ffdb13",
                    "error": "#ff2201",
                },
            },
            {
                dark: {
                    "primary": "#3ecf8e",
                    "secondary": "#dfdfdf",
                    "accent": "#4ade80",
                    "neutral": "#fafafa",
                    "base-100": "#1c1c1c",
                    "base-200": "#202020",
                    "base-300": "#2a2a2a",
                    "info": "#054cff",
                    "success": "#3ecf8e",
                    "warning": "#ffdb13",
                    "error": "#ff2201",
                },
            },
        ],
        darkTheme: "dark",
        logs: false,
    },
}

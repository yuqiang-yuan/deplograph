/**
 * Theme preference for a flyonui-themed app.
 *
 * flyonui owns the colours: the theme is selected purely by the `data-theme`
 * attribute on `<html>` (see the `@plugin "flyonui" { themes: ... }` block in
 * src/app.css). So this module only decides *which* theme name goes there:
 *
 * - "light" / "dark" → `data-theme="light"` / `data-theme="dark"`
 * - "system"         → no attribute at all, which lets flyonui's
 *                      `--prefersdark` theme follow the OS
 *
 * The initial paint is handled by the inline script in src/app.html, which
 * reads the same localStorage key before any css has loaded.
 *
 * Client-only: it touches `window`/`localStorage` at module scope.
 */

export type Theme = "system" | "light" | "dark";

const STORAGE_KEY = "theme";
const DARK_QUERY = "(prefers-color-scheme: dark)";

function storedPreference(): Theme {
    try {
        const saved = localStorage.getItem(STORAGE_KEY);
        if (saved === "light" || saved === "dark" || saved === "system") {
            return saved;
        }
    } catch {
        // storage can be unavailable (private mode, disabled) — fall back to system
    }
    return "system";
}

const media = window.matchMedia(DARK_QUERY);

let preference = $state<Theme>(storedPreference());
let systemIsDark = $state(media.matches);

media.addEventListener("change", (event) => {
    systemIsDark = event.matches;
});

function apply(next: Theme) {
    if (next === "system") {
        delete document.documentElement.dataset.theme;
    } else {
        document.documentElement.dataset.theme = next;
    }
}

export const theme = {
    /** What the user picked, including "system". */
    get preference(): Theme {
        return preference;
    },
    /** The theme actually in effect — handy for icons/swatches in a switcher. */
    get resolved(): "light" | "dark" {
        return preference === "system" ? (systemIsDark ? "dark" : "light") : preference;
    },
    set(next: Theme) {
        preference = next;
        try {
            localStorage.setItem(STORAGE_KEY, next);
        } catch {
            // keep the in-memory value even when it cannot be persisted
        }
        apply(next);
    },
};

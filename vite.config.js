import { defineConfig } from "vite";
import { sveltekit } from "@sveltejs/kit/vite";
import adapter from "@sveltejs/adapter-static";
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";
import process from "node:process";
const host = process.env.TAURI_DEV_HOST;

import tailwindcss from "@tailwindcss/vite";

/**
 * Workaround for a Tailwind v4 + vite-plugin-svelte interaction bug.
 *
 * `@tailwindcss/vite` also transforms the virtual CSS modules that
 * vite-plugin-svelte creates for a component's `<style>` block
 * (`*.svelte?svelte&type=style&lang.css`). If such a module is requested
 * before its component has been compiled in that environment, the Svelte
 * plugin's load hook returns nothing, so Vite falls back to reading the raw
 * `.svelte` file from disk — which Tailwind then parses as CSS and fails with
 * `Invalid declaration: <something from the component script>`.
 *
 * Compiling the component first makes the real CSS available to both plugins.
 * See https://github.com/tailwindlabs/tailwindcss/issues/20000
 *
 * @returns {import("vite").Plugin}
 */
function svelteStyleCssGuard() {
    const virtualStyleCssRE = /\.svelte\?[^?]*&lang\.css/;

    return {
        name: "svelte-style-css-guard",
        enforce: "pre",
        async load(id) {
            if (!virtualStyleCssRE.test(id)) return null;

            const filename = id.split("?")[0];
            const compiledCss = () => this.getModuleInfo(filename)?.meta?.svelte?.css;
            if (compiledCss()) return null;

            // build compiles every component before its css is requested
            const env = /** @type {import("vite").DevEnvironment | undefined} */ (this.environment);
            if (env?.config?.command !== "serve") return null;

            if (typeof env.transformRequest === "function") {
                try {
                    await env.transformRequest(`/@fs/${filename.replace(/^\/+/, "")}`);
                } catch {
                    // fall through and let vite-plugin-svelte report the real problem
                }
            }
            if (compiledCss()) return null;

            // never let vite read the .svelte file as css (it has no styles, or
            // the compile failed — either way an empty module beats a crash)
            return "";
        },
    };
}

// https://vite.dev/config/
export default defineConfig(() => ({
    plugins: [
        svelteStyleCssGuard(),
        tailwindcss(),
        sveltekit({
            preprocess: vitePreprocess(),
            // Tauri doesn't have a Node.js server to do proper SSR
            // so we use adapter-static with a fallback to index.html to put the site in SPA mode
            // See: https://svelte.dev/docs/kit/single-page-apps
            // See: https://v2.tauri.app/start/frontend/sveltekit/ for more info
            adapter: adapter({ fallback: "index.html" }),
        }),
    ],

    // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
    //
    // 1. prevent Vite from obscuring rust errors
    clearScreen: false,
    // 2. tauri expects a fixed port, fail if that port is not available
    server: {
        port: 1420,
        strictPort: true,
        host: host || "127.0.0.1",
        hmr: host
            ? {
                  protocol: "ws",
                  host,
                  port: 1421,
              }
            : undefined,
        watch: {
            // 3. tell Vite to ignore watching `src-tauri`
            ignored: ["**/src-tauri/**"],
        },
    },
}));

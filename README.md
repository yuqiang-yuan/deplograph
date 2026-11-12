# deplograph

A desktop application built with [gpui-kit](https://gpui-kit.com) — Rust, GPUI, and
`gpui_kit::component` for the UI.

## Requirements

- A Rust toolchain with edition 2024 support.
- On Linux, the GPUI build and windowing dependencies:

  ```bash
  sudo apt install libwayland-dev libxkbcommon-dev libxkbcommon-x11-dev \
      libegl1-mesa-dev libssl-dev libclang-dev mesa-common-dev
  ```

The first build compiles GPUI and takes a few minutes; later builds are quick.

## Run

```bash
cargo run
```

Two environment notes for this machine (Wayland):

- `WGPU_BACKEND=vulkan cargo run` prefers the Vulkan backend.
- Native Wayland has an upstream frame-pacing bug that degrades long sessions;
  `WAYLAND_DISPLAY= cargo run` runs under XWayland when it shows up.

## Source layout

- `src/main.rs` — bootstrap: app identity, theme + settings, window options, the
  first window. Settings are re-saved on quit.
- `src/main_view.rs` — the window shell: a title bar (font size, theme toggle,
  settings) above the workspace. Product views belong in
  `MainView::render_workspace`.
- `src/appearance.rs` — how a setting change reaches the theme: swap light/dark,
  resize the base font, persist, repaint. The title bar and the dialog both go
  through it.
- `src/settings_dialog.rs` — the settings dialog: edits a draft copy, `Save`
  applies it (theme and `settings.json`), `Close` or Esc discards it.
- `src/settings.rs` — `AppSettings` (theme, font size, window size) and its
  `settings.json` round trip.
- `src/assets.rs` — the app's own `AssetSource`: everything under `assets/` is
  embedded in the binary, with `ExtraIcons` for icons outside gpui-kit's default
  set.
- `assets/themes/deplograph.json` — the app's light/dark theme, registered at
  startup and swapped by the title-bar toggle. Unlisted keys fall back to
  gpui-component's defaults, so a small file stays a full theme.
- `assets/icons/` — the app's own SVGs, resolved by path
  (`Icon::default().path("icons/deplograph.svg")`).

## Settings

Stored at `$XDG_CONFIG_HOME/<app id>/settings.json` (`~/.config/app.deplograph/`),
written whenever a setting changes and once more on quit:

```json
{
  "is_dark": true,
  "font_size": 16.0,
  "window_width": 1200.0,
  "window_height": 800.0,
  "window_maximized": false
}
```

Missing fields fall back to defaults, so an older file keeps working after a new
setting is added.

## Guides

The gpui-kit guides are requirements, not suggestions — read the relevant one
before designing a screen or writing component code:

- Coding guides: <https://gpui-kit.com/docs/coding-guides.md>
- Design guides: <https://gpui-kit.com/docs/design-guides.md>
- Per-component API: <https://gpui-kit.com/docs/components/button.md> and siblings

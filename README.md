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

- `src/main.rs` — bootstrap: app identity, window options, the first window.
- `src/main_view.rs` — the window shell (title bar plus workspace). Product
  views belong in `MainView::render_workspace`.

## Guides

The gpui-kit guides are requirements, not suggestions — read the relevant one
before designing a screen or writing component code:

- Coding guides: <https://gpui-kit.com/docs/coding-guides.md>
- Design guides: <https://gpui-kit.com/docs/design-guides.md>
- Per-component API: <https://gpui-kit.com/docs/components/button.md> and siblings

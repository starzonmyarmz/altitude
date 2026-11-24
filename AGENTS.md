# Altitude

Stacked octaves, from cave to peak: a VST3 and CLAP plugin

Read before working:
- `DESIGN.md` before touching anything people see.

## Commands

```sh
cargo test                                  # engine tests and golden renders, offline
BLESS=1 cargo test                          # rewrite golden renders after an intended change to the sound
cargo fmt --all && cargo clippy --workspace --all-targets -- -D warnings
cargo xtask bundle altitude --release     # target/bundled/Altitude.{vst3,clap}
scripts/install.sh                          # bundle, sign ad hoc and install on macOS as X.Y.Z-dev+<sha>
scripts/clap-validator.sh "target/bundled/Altitude.clap"
scripts/screenshot.sh                       # retake docs/altitude.png
scripts/release --dry-run                   # what the next release would be
```

Work on the editor in a browser against the mock: `cd ui && python3 -m http.server 8765`, then open http://127.0.0.1:8765 (add `?version=1.2.0` to show a version). To load the page live in the plugin, set `ALTITUDE_UI_URL` to that URL before starting the host.

## Layout

- `plugin/src/engine.rs`: the DSP, in f64, with no host types, so `cargo test` runs it offline. `Settings` holds the engine's raw values, in the units the controls show; `Controls` adds the footswitch, and `Controls::settings()` maps it onto them.
- `plugin/src/params.rs`: the parameters. Their defaults come from `Settings::default()`, `raw_settings()` turns them into `Settings`, and `settings()` runs that through `Controls`.
- `plugin/src/lib.rs`: host plumbing, plugin IDs and `PAGE` (title, window size, version).
- `plugin/src/editor.rs`, `bridge.rs`: the webview editor and its message bridge. Don't add another channel to `editor.rs`; use `WebEditor::with_feed`.
- `plugin/tests/`: engine tests, golden renders in `golden/` and real DI recordings in `di/`. `support/` has the WAV and golden helpers.
- `ui/index.html`: the whole editor, compiled into the plugin, with a hand-written mock bridge (`mockBridge()`) for running it in a browser.
- `xtask/`, `bundler.toml`: nice-plug's bundler.
- `scripts/`: install, clap-validator, screenshot and release.
- `.claude/skills/`: `design-kickoff` (settle the look before building the editor), `screenshot`, `port-jsfx` and `bless-goldens`. Use them rather than working from memory.

A new parameter goes in six places: `Settings` in `engine.rs` (with its `Default`, and `set()` if the engine needs it), or `Controls` for a front-panel-only control; `AltitudeParams` in `params.rs` and its `raw_settings()` mapping; the mock's `specs` in `ui/index.html`; a `data-knob` or `data-floor` element in the markup; `PRESETS` and `FULL_SCALE` in `ui/index.html` if presets should set it; and the controls table in `README.md`, plus the presets table if it's in them.

## Invariants and footguns

What was learned the hard way. Add to this list whenever something breaks in a way the code doesn't make obvious.

- `CLAP_ID` and `VST3_CLASS_ID` in `lib.rs` never change after the first release, and parameter IDs never change without a major release. Hosts use them to find the plugin and restore saved projects.
- After a change to `plugin/src` or `ui/`, run `scripts/install.sh` so the user can try it. It installs as `X.Y.Z-dev+<sha>`, so the panel shows which build the host loaded. REAPER keeps a plugin's dylib loaded: tell the user to restart it.
- Don't bump the version by hand. It only changes at release.

### Sound

- The golden renders are mono renders of `plugin/tests/di/riff.wav` as an organ-style stack (`riff-stacked`) and a slow, detuned swell (`riff-swell`). When blessing new ones, say in the commit what changed in the sound.
- `tick()` is the per-sample core in f64; `process()` wraps it for the host's f32 buffers. Tests that need exact values use `tick()`.
- The floors' state is order-sensitive: the delay lines are written before the shifters read, and the write position advances after.
- The level glides start from silence on a fresh instance, so a render's first 20 ms fades in.
- Everything recursive goes through `flush()`. Without it the 25 Hz high-pass sticks on the smallest subnormal forever after the input stops.

### Presets

- The eight presets live in `PRESETS` in `ui/index.html`, in plain units, not in Rust: nice-plug has no factory presets.
- Loading one sends a begin/set/end gesture per parameter, so hosts record and undo it like hand moves. It works only while the editor is open.
- The window works out which preset is showing from the current values (within 0.2% of each control's range, with every floor on), so nothing extra is saved with a project.
- Keep the README's presets table in step.

### Parameters and the engine

- The engine works in f64. Each `Settings` field is in the units the control shows. When a few knobs drive many engine values, add a `Controls` struct whose `settings()` does the mapping (Ghosted, Altitude), so the engine can still be tested on raw values.
- The engine glides to new settings itself, with one-pole smoothers built from `coef()`, so turning a knob doesn't click. `reset()` jumps to the current settings; the plugin calls it on activate and reset.
- Keep `stomp` on effects. Glide the wet amount toward 0 or 1, and `flush()` it: anything gliding toward 0 decays into subnormals otherwise.
- Random numbers come from a seeded RNG (Ghosted's xorshift64* `Rng`), so golden renders repeat exactly.
- Tempo sync: read the host transport in `process` and pass the engine plain values. Fall back to 120 BPM when the host gives no tempo.

### Things clap-validator catches

Run `scripts/clap-validator.sh` after any DSP change, not just in CI.

- Subnormal output. Recursive filters, envelopes and smoothers decay into subnormals in silence. Flush state and output below about 1e-30 to zero.
- nice-plug's `v2s_f32_hz_then_khz` already appends Hz/kHz, so adding `with_unit(" Hz")` doubles it. `v2s_f32_gain_to_db` and `v2s_f32_percentage` don't, so `with_unit` is right with those.
- NaN from filters whose corner frequency is above Nyquist at low sample rates. Clamp corners below Nyquist.

### Editor

- The window size is set in `PAGE.size` in `lib.rs` and by `.pedal`'s `inline-size` and `block-size` in `ui/index.html` (the window leaves 24 px around the pedal). Change them together.
- A fader's drag distance is its `--travel`, so the cap follows the pointer. Knobs take 200 px for the full range.
- The browser preview gets cached: add `?v=N` after each edit. Keep the mock's `specs` in `ui/index.html` in step with the plugin's parameters, including the skew of any skewed range. (This plugin predates the kit's generated `ui/mock.json`.)
- To show the page something that isn't a parameter (an LED, a scope, a meter), use `WebEditor::with_feed`. Share the state from the audio thread through atomics, and have the closure return a message only when something changed, since it runs every frame.
- SVG reused through `<use>` doesn't get page CSS rules. Stamp it from a `<template>` instead, or put attributes on the elements themselves.
- Fonts can't load from the network: embed them as base64 `@font-face` data URLs, and keep the file and its license in `ui/fonts`.
- The macOS webview is the system WKWebView, so check a newer CSS feature is in Safari for the macOS versions you support. Windows uses WebView2.
- After a visible UI change, retake the screenshot with the `screenshot` skill and keep the README `alt` text current. The Screenshot workflow retakes it on main too, but can't update the `alt` text.

### Testing in REAPER

- REAPER only monitors input on armed tracks.
- Multi-output plugins: declare a second layout without the aux port so hosts without multi-out still load them, and keep a scratch buffer for the aux signal in that case (see Halfwit).

### nice-plug fork

Plugins pin the `patina` branch of [starzonmyarmz/nice-plug](https://github.com/starzonmyarmz/nice-plug/tree/patina): upstream plus one commit that adds `ActivateContext::gui_context()`. Keep the fork to that one commit.

- Upgrading: rebase `patina` onto upstream, push, and bump the `rev` in `plugin/` and `xtask/` `Cargo.toml` in every plugin. Then run the tests and clap-validator in each.
- Don't send the patch upstream. It was written with AI, and nice-plug's [AI policy](https://rust.audio/community/ai/) doesn't accept AI-written code, PR descriptions or comments.

### Ported from a JSFX

If the plugin started from a JSFX, never mention the JSFX anywhere in this repo: not in the README, docs, code, comments, test names, parameter tables or commit messages.

## Conventions

- Conventional Commits with Title-Case scopes: `feat(Engine): …`. The body explains why.
- Tests ship in the same commit as the change. Pure logic is tested; the UI or host shell stays thin.
- A `feat`, `fix` or `perf` commit that someone using Altitude would notice adds a line under `## [Unreleased]` in `CHANGELOG.md`, in the same commit, written for them, not as a code summary.
- This repo is public. Keep real names, logins, paths and work data out of code, fixtures, docs, the changelog and commits.
- Breaking means a renamed parameter ID or a preset that no longer loads.

## Releasing

Releases happen only when asked. Never bump, tag or publish on your own.

`scripts/release` works out the bump from commits since the last tag, finalizes the changelog, commits `chore(Release): X.Y.Z`, tags `vX.Y.Z` and pushes. The tag triggers `.github/workflows/release.yml`.

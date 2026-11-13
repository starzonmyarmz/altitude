# Totem

Read `README.md` first for what the plugin does, its controls, and how to
build and run it. This file has the maintainer details it leaves out.

## Layout

- `plugin/src/engine.rs`: all DSP, with no plugin framework types, so tests
  drive it directly. `Settings` holds the engine's raw values, in the
  units the controls show; `Controls` adds the footswitch, and
  `Controls::settings()` maps it onto them.
- `plugin/src/params.rs`: host parameters, mapped to `Controls`.
- `plugin/src/editor.rs` and `plugin/src/bridge.rs`: nothing specific to
  Totem. The page's size and HTML come from `PAGE` in `lib.rs`, and the
  bridge works with any `#[derive(Params)]` struct.
- `ui/index.html`: the whole editor, compiled into the plugin, with a mock
  bridge for running it in a browser.

## Engine

- `tick()` is the per-sample core in f64; `process()` wraps it for the
  host's f32 buffers. Tests that need exact values use `tick()`.
- The floors' state is order-sensitive: the delay lines are written
  before the shifters read, and the write position advances after. The
  level glides start from silence on a fresh instance, so a render's
  first 20 ms fades in.
- Everything recursive goes through `flush()`. Without it the 25 Hz
  high-pass sticks on the smallest subnormal forever after the input
  stops.

## Adding a parameter

It has to go in five places:

1. `Settings` in `engine.rs` and its `Default`, and `set()` if the
   engine needs it. If it's a front-panel-only control, `Controls`
   instead.
2. `TotemParams` in `params.rs`, and its `raw_settings()` mapping.
3. The mock's `specs` in `ui/index.html`. The plugin's bridge picks new
   parameters up automatically, but the browser mock doesn't.
4. A `data-knob` (or `data-floor`, `data-stomp-switch`) element in the
   markup.
5. The controls table in `README.md`.

## Sound changes and golden renders

`plugin/tests/golden` holds mono renders of `plugin/tests/di/riff.wav`
as an organ-style stack and a slow, detuned swell. After an intended
change to the sound, rerun with `BLESS=1 cargo test` to rewrite them.
Never commit rewritten renders until the user has listened to them, and
say in the commit what changed in the sound.

## Design

Settled with the user; keep to it.

- Object: a tiki-pop carved totem pole, one critter per floor. Original
  characters only, not Northwest Coast formline art (no ovoids or
  U-forms).
- Names: Owl (+2), Fox (+1), You (dry), Bear (−1), Mole (−2), sky down
  to underground. Knobs Shade (tone), Wake (attack), Sway (detune), and
  Stomp.
- Palette: glossy tomato, mustard and mint on a teal night. Colour
  follows function: up floors mustard, You mint, down floors tomato.
  Knobs are all the same cream lacquer.
- Art: a critter's mouth opens with its level (`--level` on `.floor`);
  off closes its eyes and desaturates it. Stomp off does that to every
  critter but You.
- Font: Chango, letter-spacing −0.03em, for the wordmark and labels.
  The subset in `ui/fonts` has capitals only (labels are uppercased), so
  new label text needs those letters, or a new subset from Google Fonts'
  css2 API with `&text=`.

## UI

- The window size is set in two places that must change together:
  `PAGE.size` in `lib.rs`, and `.scene`'s `inline-size` and `block-size`
  in `ui/index.html`.
- Each floor is an inline SVG, so page CSS reaches its parts. The open
  and shut eyes are `.open` and `.shut` groups; `.mouth` scales with
  `--open`.
- The local preview server (`python3 -m http.server`) gets cached by the
  browser. Add a query string like `?v=2` after each edit.
- After a visible UI change, retake `docs/totem.jpg` with the
  `screenshot` skill.

## Releases

`release.yml` runs on `vX.Y.Z` tags, which must match `version` in the
root `Cargo.toml`. To release: bump `version`, commit, then
`git tag vX.Y.Z && git push origin main vX.Y.Z`.

## nice-plug fork

The plugin is pinned to the `patina` branch of a nice-plug fork, which
adds `ActivateContext::gui_context()`. To upgrade, rebase that branch onto
upstream, push it, and update the `rev` in both `plugin/Cargo.toml` and
`xtask/Cargo.toml`.

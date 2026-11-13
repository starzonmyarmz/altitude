# Altitude

Read `README.md` first for what the plugin does, its controls, and how to
build and run it. This file has the maintainer details it leaves out.

## Layout

- `plugin/src/engine.rs`: all DSP, with no plugin framework types, so tests
  drive it directly. `Settings` holds the engine's raw values, in the
  units the controls show; `Controls` adds the footswitch, and
  `Controls::settings()` maps it onto them.
- `plugin/src/params.rs`: host parameters, mapped to `Controls`.
- `plugin/src/editor.rs` and `plugin/src/bridge.rs`: nothing specific to
  Altitude. The page's size and HTML come from `PAGE` in `lib.rs`, and the
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

It has to go in six places:

1. `Settings` in `engine.rs` and its `Default`, and `set()` if the
   engine needs it. If it's a front-panel-only control, `Controls`
   instead.
2. `AltitudeParams` in `params.rs`, and its `raw_settings()` mapping.
3. The mock's `specs` in `ui/index.html`. The plugin's bridge picks new
   parameters up automatically, but the browser mock doesn't.
4. A `data-knob` (or `data-floor`) element in the markup.
5. `PRESETS` and `FULL_SCALE` in `ui/index.html`, if presets should set it.
6. The controls table in `README.md`, and the presets table if it's in
   them.

## Presets

The eight presets live in `PRESETS` in `ui/index.html`, in plain units,
not in Rust: nice-plug has no factory presets. Loading one sends a
begin/set/end gesture per parameter, so hosts record and undo it like
hand moves, and it works only while the editor is open. The window
works out which preset is showing from the current values (within 0.2%
of each control's range, with every floor on), so nothing extra is
saved with a project. Keep the README's presets table in step.

## Sound changes and golden renders

`plugin/tests/golden` holds mono renders of `plugin/tests/di/riff.wav`
as an organ-style stack and a slow, detuned swell. After an intended
change to the sound, rerun with `BLESS=1 cargo test` to rewrite them.
Never commit rewritten renders until the user has listened to them, and
say in the commit what changed in the sound.

## Design

Settled with the user; keep to it. The editor is a real stompbox, built
the way Halfwit's is (powder coat with noise, chrome screws, knurled
knobs with tick dots, chrome hex footswitch and LED). Look at Halfwit's
page before changing materials. No wood.

- Enclosure: powder blue, with everything printed on it in navy ink as
  if screen-printed (`.print` multiplies into the coat). No tags,
  badges or plates.
- Knobs: ivory knurled, navy pointer: Tone, Attack, Detune.
- Presets: an amber backlit window between chrome ◀ ▶ buttons.
- Floors: five slide faders printed straight onto the coat, low to
  high: Cave (−2), Valley (−1), You (dry), Ridge (+1), Peak (+2). Ivory
  caps with a navy index line, and a green switch light above each.
  Stomp off dims every light but You's.
- Wordmark: Rammetto One in navy under a printed mountain range with
  contour lines. The font in `ui/fonts` is subset to the letters of
  "Altitude"; labels are Jost.
- Version: printed at the end of the bottom line
  (`FOUR OCTAVES OF ELEVATION · V0.2.0`), from the root `Cargo.toml`.
- Colours by function: green for floor switches, amber for the effect
  (footswitch LED and preset window).

## UI

- The window size is set in two places that must change together:
  `PAGE.size` in `lib.rs`, and `.pedal`'s `inline-size` and `block-size`
  in `ui/index.html` (the window leaves 24 px around the pedal).
- A fader's drag distance is its `--travel`, so the cap follows the
  pointer. Knobs take 200 px for the full range.
- The local preview server (`python3 -m http.server`) gets cached by the
  browser. Add a query string like `?v=2` after each edit.
- After a visible UI change, retake `docs/altitude.jpg` with the
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

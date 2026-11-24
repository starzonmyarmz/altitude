# Design

How Altitude looks and feels. When this file and your instincts disagree, this file wins. Change it on purpose, in the same commit as the code.

## The look in one line

A powder-blue stompbox screen-printed in navy, with five floor faders climbing from cave to peak under a printed mountain range.

## Principles

Every plugin gets its own look, but they share an approach.

- **Skeuomorphic.** The editor is a physical object (Patina an OP-1-style synth, Halfwit a stompbox, Ghosted a vintage radio), with real materials like powder coat, bakelite, walnut, brass and grille cloth, built in CSS gradients and shadows rather than images.
- **One theme.** Pick the object from the plugin's vibe and name, and let it run through everything: the wordmark, the art and the control labels. Names should be playful and capture the vibe.
- **Restrained.** One knob style and colour family per plugin, and few controls. Fold many settings into a handful of knobs with evocative names rather than exposing every one.
- **Colour and art mean something.** If two waves or knobs differ in colour, it's because they differ in function; otherwise make them the same. Art can follow parameters (Ghosted's ghosts fade with Haunt, the dial needle follows Distance), but don't fake audio metering.
- **Plain labels.** Words a guitarist understands, not DSP terms or fieldset jargon. Numbers go in a tooltip readout.
- **Type with character.** Embed a display face for the wordmark (Futura-like Jost, pillowy Modak), never a system or "commercial" font, and centre it optically on its badge.
- **Drawn, not vector-perfect.** Vary repeated elements (size, tilt, expression), and give outlines weight that swells on the shadow side, like inked comic art.
- **A footswitch on effects.** Pedals and other effects get an on/off toggle (`stomp`, on by default). Off fades the effect out and passes the dry signal; a lamp or LED shows the state, and the art reacts (Ghosted's dial dims and its ghosts leave).
- **The version on the panel.** Print it small and quiet as part of the object, like a model number on a pedal's base plate or a serial plate on a radio's back (`v1.2.0`), so anyone can tell which build they're running. Local installs show `X.Y.Z-dev+<sha>`.
- **Physical details behave physically.** Shadows stay put while a knob turns, pressed keys sink, lit things glow.

## This plugin

Settled with the user; keep to it.

- **Object:** a real stompbox, built the way Halfwit's is: powder coat with noise, chrome screws, knurled knobs with tick dots, a chrome hex footswitch and an LED. Look at Halfwit's page before changing materials. No wood.
- **Palette:** powder blue, with everything printed on it in navy ink as if screen-printed (`.print` multiplies into the coat). No tags, badges or plates. Colours go by function: green for floor switches, amber for the effect (footswitch LED and preset window). See Tokens.
- **Wordmark font:** Rammetto One in navy, under a printed mountain range with contour lines. The font in `ui/fonts` is subset to the letters of "Altitude"; labels are Jost.
- **Controls:** ivory knurled knobs with a navy pointer (Tone, Attack, Detune); presets in an amber backlit window between chrome ◀ ▶ buttons; five slide faders printed straight onto the coat, low to high: Cave (−2), Valley (−1), You (dry), Ridge (+1), Peak (+2). They have ivory caps with a navy index line and a green switch light above each. Stomp off dims every light but You's.
- **Version placement:** it ends the bottom line, `FOUR OCTAVES OF ELEVATION · V0.2.0`.

## Tokens

Every color, size and space comes from a token. If nothing fits, add a token here before using a new value. They live as custom properties at the top of `ui/index.html`.

| Token | Value | Used for |
|---|---|---|
| `--board` | `#181716` | the page behind the pedal |
| `--coat`, `--coat-hi`, `--coat-lo` | `#9fc4de`, `#c6def0`, `#5f8db0` | the powder-blue coat and its light and shade |
| `--ink`, `--ink-soft` | `#1d2e4a`, 60% `--ink` | the navy print and quieter labels |
| `--cream` | `#f4ead2` | the value readout text |
| `--chrome-hi`, `--chrome`, `--chrome-lo` | `#fbfbf9`, `#c4c6c8`, `#6d7074` | screws, buttons and the footswitch |
| `--ivory`, `--ivory-hi`, `--ivory-lo` | `#efe7d4`, `#fffcf4`, `#b8ab8c` | knobs and fader caps |
| `--amber` | `#f2b230` | the footswitch LED and preset window |
| `--green` | `#5be37a` | the floor switch lights |
| `--pill` | `#120f0c` | the value readout under a knob |
| `--display`, `--label` | Rammetto One, Jost | the wordmark and labels |

## Components

## Patterns

## Adding something new

1. Look for an existing component or pattern that already does it.
2. Build it from tokens only.
3. Check Reduce Motion if it moves.
4. Retake the screenshot if it shows in one.
5. Record it above.

## Known drift

Existing debt to clean up, not to copy.

- About 15 raw colours in `ui/index.html` aren't tokens: the lit and unlit gradients of the green and amber lights, their `#8b8d90` bezels, the preset window's glass and text, the fader slots and the coat's edge shadow.

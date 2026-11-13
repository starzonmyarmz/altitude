use crate::engine::{Controls, Settings};
use nice_plug::prelude::*;

#[derive(Params)]
pub struct TotemParams {
    #[id = "mole"]
    pub down2: FloatParam,
    #[id = "bear"]
    pub down1: FloatParam,
    #[id = "you"]
    pub dry: FloatParam,
    #[id = "fox"]
    pub up1: FloatParam,
    #[id = "owl"]
    pub up2: FloatParam,
    #[id = "shade"]
    pub tone: FloatParam,
    #[id = "wake"]
    pub attack: FloatParam,
    #[id = "sway"]
    pub detune: FloatParam,
    #[id = "mole_on"]
    pub down2_on: BoolParam,
    #[id = "bear_on"]
    pub down1_on: BoolParam,
    #[id = "you_on"]
    pub dry_on: BoolParam,
    #[id = "fox_on"]
    pub up1_on: BoolParam,
    #[id = "owl_on"]
    pub up2_on: BoolParam,
    #[id = "stomp"]
    pub stomp: BoolParam,
}

fn percent(name: &str, default: f64) -> FloatParam {
    FloatParam::new(name, default as f32, FloatRange::Linear { min: 0.0, max: 100.0 })
        .with_unit("%")
        .with_value_to_string(formatters::v2s_f32_rounded(0))
}

impl Default for TotemParams {
    fn default() -> Self {
        let s = Settings::default();
        Self {
            down2: percent("Mole", s.down2),
            down1: percent("Bear", s.down1),
            dry: percent("You", s.dry),
            up1: percent("Fox", s.up1),
            up2: percent("Owl", s.up2),
            tone: percent("Shade", s.tone),
            attack: FloatParam::new("Wake", s.attack_ms as f32, FloatRange::Linear { min: 0.0, max: 1000.0 })
                .with_unit(" ms")
                .with_value_to_string(formatters::v2s_f32_rounded(0)),
            detune: FloatParam::new("Sway", s.detune_cents as f32, FloatRange::Linear { min: 0.0, max: 25.0 })
                .with_unit(" cents")
                .with_value_to_string(formatters::v2s_f32_rounded(1)),
            down2_on: BoolParam::new("Mole on", s.down2_on),
            down1_on: BoolParam::new("Bear on", s.down1_on),
            dry_on: BoolParam::new("You on", s.dry_on),
            up1_on: BoolParam::new("Fox on", s.up1_on),
            up2_on: BoolParam::new("Owl on", s.up2_on),
            stomp: BoolParam::new("Stomp", Controls::default().stomp),
        }
    }
}

impl TotemParams {
    pub fn settings(&self) -> Settings {
        Controls { settings: self.raw_settings(), stomp: self.stomp.value() }.settings()
    }

    fn raw_settings(&self) -> Settings {
        Settings {
            down2: self.down2.value() as f64,
            down1: self.down1.value() as f64,
            dry: self.dry.value() as f64,
            up1: self.up1.value() as f64,
            up2: self.up2.value() as f64,
            tone: self.tone.value() as f64,
            attack_ms: self.attack.value() as f64,
            detune_cents: self.detune.value() as f64,
            down2_on: self.down2_on.value(),
            down1_on: self.down1_on.value(),
            dry_on: self.dry_on.value(),
            up1_on: self.up1_on.value(),
            up2_on: self.up2_on.value(),
        }
    }
}

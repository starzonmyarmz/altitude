use crate::engine::Settings;
use nice_plug::prelude::*;

#[derive(Params)]
pub struct TotemParams {
    #[id = "down2"]
    pub down2: FloatParam,
    #[id = "down1"]
    pub down1: FloatParam,
    #[id = "dry"]
    pub dry: FloatParam,
    #[id = "up1"]
    pub up1: FloatParam,
    #[id = "up2"]
    pub up2: FloatParam,
    #[id = "tone"]
    pub tone: FloatParam,
    #[id = "attack"]
    pub attack: FloatParam,
    #[id = "detune"]
    pub detune: FloatParam,
    #[id = "down2_on"]
    pub down2_on: BoolParam,
    #[id = "down1_on"]
    pub down1_on: BoolParam,
    #[id = "dry_on"]
    pub dry_on: BoolParam,
    #[id = "up1_on"]
    pub up1_on: BoolParam,
    #[id = "up2_on"]
    pub up2_on: BoolParam,
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
            down2: percent("Oct -2", s.down2),
            down1: percent("Oct -1", s.down1),
            dry: percent("Dry", s.dry),
            up1: percent("Oct +1", s.up1),
            up2: percent("Oct +2", s.up2),
            tone: percent("Tone", s.tone),
            attack: FloatParam::new("Attack", s.attack_ms as f32, FloatRange::Linear { min: 0.0, max: 1000.0 })
                .with_unit(" ms")
                .with_value_to_string(formatters::v2s_f32_rounded(0)),
            detune: FloatParam::new("Detune", s.detune_cents as f32, FloatRange::Linear { min: 0.0, max: 25.0 })
                .with_unit(" cents")
                .with_value_to_string(formatters::v2s_f32_rounded(1)),
            down2_on: BoolParam::new("Oct -2 on", s.down2_on),
            down1_on: BoolParam::new("Oct -1 on", s.down1_on),
            dry_on: BoolParam::new("Dry on", s.dry_on),
            up1_on: BoolParam::new("Oct +1 on", s.up1_on),
            up2_on: BoolParam::new("Oct +2 on", s.up2_on),
        }
    }
}

impl TotemParams {
    pub fn settings(&self) -> Settings {
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

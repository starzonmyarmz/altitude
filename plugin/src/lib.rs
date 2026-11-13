mod bridge;
mod editor;
pub mod engine;
mod params;

use engine::Engine;
use params::TotemParams;
use nice_plug::prelude::*;
use std::sync::Arc;

static PAGE: editor::Page = editor::Page {
    title: "Totem",
    size: (600, 720),
    html: include_str!("../../ui/index.html"),
    background: (27, 26, 25, 255),
    dev_url_var: "TOTEM_UI_URL",
};

pub struct Totem {
    params: Arc<TotemParams>,
    engine: Engine,
}

impl Default for Totem {
    fn default() -> Self {
        Self { params: Arc::new(TotemParams::default()), engine: Engine::new(48_000.0) }
    }
}

impl Plugin for Totem {
    const NAME: &'static str = "Totem";
    const VENDOR: &'static str = "Dan Marino";
    const URL: &'static str = "";
    const EMAIL: &'static str = "";
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: NonZeroU32::new(2),
        main_output_channels: NonZeroU32::new(2),
        ..AudioIOLayout::const_default()
    }];

    type Editor = editor::WebEditor;
    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn editor(&mut self, _executor: AsyncExecutor<Self>) -> Option<Self::Editor> {
        Some(editor::WebEditor::new(&PAGE, self.params.clone()))
    }

    fn activate(
        &mut self,
        _layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        _context: &mut impl ActivateContext<Self>,
    ) -> bool {
        if self.engine.sample_rate() != buffer_config.sample_rate {
            self.engine = Engine::new(buffer_config.sample_rate);
        }
        self.engine.set(self.params.settings());
        true
    }

    fn reset(&mut self) {
        self.engine.reset();
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        self.engine.set(self.params.settings());
        if let [left, right] = buffer.as_slice() {
            self.engine.process(left, right);
        }
        ProcessStatus::Normal
    }
}

impl ClapPlugin for Totem {
    const CLAP_ID: &'static str = "com.danmarino.totem";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("Stacked octaves, two down to two up");
    const CLAP_MANUAL_URL: Option<&'static str> = None;
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::AudioEffect, ClapFeature::PitchShifter, ClapFeature::Stereo];
}

impl Vst3Plugin for Totem {
    const VST3_CLASS_ID: [u8; 16] = *b"TotemFxDanMarino";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[Vst3SubCategory::Fx, Vst3SubCategory::PitchShift];
}

nice_export_clap!(Totem);
nice_export_vst3!(Totem);

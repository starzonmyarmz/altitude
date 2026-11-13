mod support;

use altitude::engine::{Controls, Engine, Settings};

const SR: f32 = 48_000.0;

fn riff() -> Vec<f32> {
    let (mut input, sr) = support::read_wav(&support::path("di", "riff.wav"));
    assert_eq!(sr, SR as u32);
    input.resize(input.len() + SR as usize / 2, 0.0);
    input
}

/// Runs mono input through the engine in host-sized blocks, as a guitar DI
/// on both channels.
fn render(input: &[f32], settings: Settings) -> (Vec<f32>, Vec<f32>) {
    let mut engine = Engine::new(SR);
    engine.set(settings);
    let mut left = input.to_vec();
    let mut right = input.to_vec();
    for (l, r) in left.chunks_mut(512).zip(right.chunks_mut(512)) {
        engine.process(l, r);
    }
    (left, right)
}

fn floors(down2: f64, down1: f64, dry: f64, up1: f64, up2: f64) -> Settings {
    Settings { down2, down1, dry, up1, up2, ..Settings::default() }
}

fn rms(x: &[f32]) -> f64 {
    (x.iter().map(|&s| (s as f64) * (s as f64)).sum::<f64>() / x.len() as f64).sqrt()
}

#[test]
fn riff_stacked_matches_golden() {
    let settings = Settings { tone: 55.0, ..floors(30.0, 60.0, 70.0, 50.0, 25.0) };
    let (l, r) = render(&riff(), settings);
    assert_eq!(l, r);
    support::check_golden("riff-stacked", &l, SR as u32);
}

#[test]
fn riff_swell_matches_golden() {
    let settings = Settings { tone: 50.0, attack_ms: 450.0, detune_cents: 7.0, ..floors(0.0, 50.0, 40.0, 60.0, 30.0) };
    let (l, _) = render(&riff(), settings);
    support::check_golden("riff-swell", &l, SR as u32);
}

#[test]
fn dry_alone_passes_input_through_after_its_fade_in() {
    let input = riff();
    let (l, _) = render(&input, floors(0.0, 0.0, 100.0, 0.0, 0.0));
    let settled = SR as usize / 2;
    for (out, inp) in l[settled..].iter().zip(&input[settled..]) {
        assert!((out - inp).abs() < 1e-6);
    }
}

#[test]
fn a_switched_off_floor_is_silent() {
    let input = riff();
    let on = render(&input, floors(0.0, 100.0, 0.0, 0.0, 0.0)).0;
    let off = render(&input, Settings { down1_on: false, ..floors(0.0, 100.0, 0.0, 0.0, 0.0) }).0;
    assert!(rms(&on) > 0.01);
    assert_eq!(rms(&off), 0.0);
}

#[test]
fn octave_down_lands_an_octave_down() {
    let input = support::tone(220.0, 1.0, SR);
    let (l, _) = render(&input, floors(0.0, 100.0, 0.0, 0.0, 0.0));
    let tail = &l[SR as usize / 2..];
    let crossings = tail.windows(2).filter(|w| w[0] < 0.0 && w[1] >= 0.0).count();
    let freq = crossings as f64 / (tail.len() as f64 / SR as f64);
    assert!((freq - 110.0).abs() < 5.0, "{freq} Hz");
}

#[test]
fn silence_after_the_riff_settles_to_exact_zero() {
    let mut engine = Engine::new(SR);
    engine.set(Settings { attack_ms: 300.0, ..floors(100.0, 100.0, 100.0, 100.0, 100.0) });
    for s in riff() {
        engine.tick(s as f64, s as f64);
    }
    let silence: Vec<(f64, f64)> = (0..20 * SR as usize).map(|_| engine.tick(0.0, 0.0)).collect();
    assert!(silence[silence.len() - SR as usize..].iter().all(|&(l, r)| l == 0.0 && r == 0.0));
}

#[test]
fn stomp_off_passes_input_through_after_the_floors_fade() {
    let input = riff();
    let stacked = Settings { dry: 40.0, ..floors(100.0, 100.0, 40.0, 100.0, 100.0) };
    let (l, _) = render(&input, Controls { settings: stacked, stomp: false }.settings());
    let settled = SR as usize / 2;
    for (out, inp) in l[settled..].iter().zip(&input[settled..]) {
        assert!((out - inp).abs() < 1e-6);
    }
}

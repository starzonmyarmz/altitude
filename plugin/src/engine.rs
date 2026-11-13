use std::f64::consts::PI;

const N: usize = 65536;

/// The engine's raw values, in the units the controls show.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Settings {
    /// Two octaves down, 0 to 100%.
    pub down2: f64,
    /// One octave down, 0 to 100%.
    pub down1: f64,
    /// The input, 0 to 100%.
    pub dry: f64,
    /// One octave up, 0 to 100%.
    pub up1: f64,
    /// Two octaves up, 0 to 100%.
    pub up2: f64,
    /// Low-pass on the octave voices, 0 to 100% (about 400 Hz to 20 kHz).
    pub tone: f64,
    /// Fade-in of the octave voices after each pick attack, 0 to 1000 ms.
    pub attack_ms: f64,
    /// Spread between the octave voices, 0 to 25 cents.
    pub detune_cents: f64,
    pub down2_on: bool,
    pub down1_on: bool,
    pub dry_on: bool,
    pub up1_on: bool,
    pub up2_on: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            down2: 0.0,
            down1: 50.0,
            dry: 100.0,
            up1: 0.0,
            up2: 0.0,
            tone: 60.0,
            attack_ms: 0.0,
            detune_cents: 0.0,
            down2_on: true,
            down1_on: true,
            dry_on: true,
            up1_on: true,
            up2_on: true,
        }
    }
}

/// A granular pitch shifter: two taps sweep through a delay line at the
/// shift rate, crossfaded with sin² windows so the wrap is silent.
struct Shifter {
    buf: usize,
    w: f64,
    p: f64,
}

impl Shifter {
    fn new(buf: usize, win_ms: f64, sr: f64) -> Self {
        Self { buf, w: (win_ms * sr / 1000.0).floor(), p: 0.0 }
    }

    fn tick(&mut self, mem: &[f64], wpos: usize, ratio: f64) -> f64 {
        self.p += (1.0 - ratio) / self.w;
        if self.p >= 1.0 {
            self.p -= 1.0;
        }
        if self.p < 0.0 {
            self.p += 1.0;
        }
        let mut q = self.p + 0.5;
        if q >= 1.0 {
            q -= 1.0;
        }
        let s = (PI * self.p).sin();
        let ga = s * s;
        read(mem, self.buf, wpos, 2.0 + self.p * self.w) * ga + read(mem, self.buf, wpos, 2.0 + q * self.w) * (1.0 - ga)
    }
}

fn flush(x: f64) -> f64 {
    if x.abs() < 1e-30 {
        0.0
    } else {
        x
    }
}

fn read(mem: &[f64], base: usize, wpos: usize, d: f64) -> f64 {
    let mut pos = wpos as f64 - d;
    if pos < 0.0 {
        pos += N as f64;
    }
    let i = pos.floor();
    let f = pos - i;
    let i = i as usize;
    let j = if i + 1 >= N { 0 } else { i + 1 };
    mem[base + i] * (1.0 - f) + mem[base + j] * f
}

/// The plugin's DSP, apart from the host plumbing so tests can run it
/// offline. The plugin passes in [`Settings`] once per block.
pub struct Engine {
    sample_rate: f32,
    sr: f64,
    mem: Vec<f64>,
    wpos: usize,

    aa1: f64,
    aa2: f64,
    hp_a: f64,
    gsmooth: f64,
    env_att: f64,
    env_rel: f64,
    env_slow: f64,
    trighangmax: f64,

    f1a: f64,
    f1b: f64,
    f2a: f64,
    f2b: f64,
    t1: f64,
    t2: f64,
    hx: f64,
    hy: f64,
    envf: f64,
    envs: f64,
    trighang: f64,
    swell: f64,
    g1s: f64,
    g2s: f64,
    gds: f64,
    g4s: f64,
    g5s: f64,

    down2: Shifter,
    down1: Shifter,
    up1: Shifter,
    up2: Shifter,

    g1: f64,
    g2: f64,
    gd: f64,
    g4: f64,
    g5: f64,
    tone_a: f64,
    att_inc: f64,
    r_d2: f64,
    r_d1: f64,
    r_u1: f64,
    r_u2: f64,
}

impl Engine {
    pub fn new(sample_rate: f32) -> Self {
        let sr = sample_rate as f64;
        let raw = 0;
        let up1buf = N;
        let up2buf = N * 2;
        let mut engine = Self {
            sample_rate,
            sr,
            mem: vec![0.0; N * 3],
            wpos: 0,
            aa1: (-2.0 * PI * sr * 0.22 / sr).exp(),
            aa2: (-2.0 * PI * sr * 0.11 / sr).exp(),
            hp_a: (-2.0 * PI * 25.0 / sr).exp(),
            gsmooth: 1.0 - (-1.0 / (sr * 0.02)).exp(),
            env_att: 1.0 - (-1.0 / (sr * 0.0005)).exp(),
            env_rel: 1.0 - (-1.0 / (sr * 0.02)).exp(),
            env_slow: 1.0 - (-1.0 / (sr * 0.15)).exp(),
            trighangmax: (sr * 0.05).floor(),
            f1a: 0.0,
            f1b: 0.0,
            f2a: 0.0,
            f2b: 0.0,
            t1: 0.0,
            t2: 0.0,
            hx: 0.0,
            hy: 0.0,
            envf: 0.0,
            envs: 0.0,
            trighang: 0.0,
            swell: 1.0,
            g1s: 0.0,
            g2s: 0.0,
            gds: 0.0,
            g4s: 0.0,
            g5s: 0.0,
            down2: Shifter::new(raw, 70.0, sr),
            down1: Shifter::new(raw, 45.0, sr),
            up1: Shifter::new(up1buf, 40.0, sr),
            up2: Shifter::new(up2buf, 60.0, sr),
            g1: 0.0,
            g2: 0.0,
            gd: 0.0,
            g4: 0.0,
            g5: 0.0,
            tone_a: 0.0,
            att_inc: 1.0,
            r_d2: 0.25,
            r_d1: 0.5,
            r_u1: 2.0,
            r_u2: 4.0,
        };
        engine.set(Settings::default());
        engine
    }

    pub fn sample_rate(&self) -> f32 {
        self.sample_rate
    }

    /// Clears the delay lines and filters, and starts the floor levels from
    /// silence, as a fresh instance would.
    pub fn reset(&mut self) {
        self.mem.fill(0.0);
        self.wpos = 0;
        (self.f1a, self.f1b, self.f2a, self.f2b) = (0.0, 0.0, 0.0, 0.0);
        (self.t1, self.t2, self.hx, self.hy) = (0.0, 0.0, 0.0, 0.0);
        (self.envf, self.envs, self.trighang, self.swell) = (0.0, 0.0, 0.0, 1.0);
        (self.g1s, self.g2s, self.gds, self.g4s, self.g5s) = (0.0, 0.0, 0.0, 0.0, 0.0);
        for s in [&mut self.down2, &mut self.down1, &mut self.up1, &mut self.up2] {
            s.p = 0.0;
        }
    }

    pub fn set(&mut self, s: Settings) {
        let on = |b: bool| if b { 1.0 } else { 0.0 };
        self.g1 = s.down2 / 100.0 * on(s.down2_on);
        self.g2 = s.down1 / 100.0 * on(s.down1_on);
        self.gd = s.dry / 100.0 * on(s.dry_on);
        self.g4 = s.up1 / 100.0 * on(s.up1_on);
        self.g5 = s.up2 / 100.0 * on(s.up2_on);
        self.tone_a = (-2.0 * PI * (400.0 * 50f64.powf(s.tone / 100.0)).min(self.sr * 0.45) / self.sr).exp();
        self.att_inc = if s.attack_ms > 0.0 { 1000.0 / (s.attack_ms * self.sr) } else { 1.0 };
        let dt = 2f64.powf(s.detune_cents / 1200.0);
        self.r_d2 = 0.25 * dt;
        self.r_d1 = 0.5 / dt;
        self.r_u1 = 2.0 * dt;
        self.r_u2 = 4.0 / dt;
    }

    pub fn process(&mut self, left: &mut [f32], right: &mut [f32]) {
        for (l, r) in left.iter_mut().zip(right.iter_mut()) {
            let (ol, or) = self.tick(*l as f64, *r as f64);
            *l = ol as f32;
            *r = or as f32;
        }
    }

    /// One stereo sample, in f64 so renders can be compared exactly.
    pub fn tick(&mut self, spl0: f64, spl1: f64) -> (f64, f64) {
        let mono = (spl0 + spl1) * 0.5;

        // Pre-filter the up voices' input so shifted content stays below Nyquist.
        self.f1a = flush(self.f1a * self.aa1 + mono * (1.0 - self.aa1));
        self.f1b = flush(self.f1b * self.aa1 + self.f1a * (1.0 - self.aa1));
        self.f2a = flush(self.f2a * self.aa2 + mono * (1.0 - self.aa2));
        self.f2b = flush(self.f2b * self.aa2 + self.f2a * (1.0 - self.aa2));
        self.mem[self.wpos] = mono;
        self.mem[N + self.wpos] = self.f1b;
        self.mem[2 * N + self.wpos] = self.f2b;

        self.g1s = flush(self.g1s + (self.g1 - self.g1s) * self.gsmooth);
        self.g2s = flush(self.g2s + (self.g2 - self.g2s) * self.gsmooth);
        self.gds = flush(self.gds + (self.gd - self.gds) * self.gsmooth);
        self.g4s = flush(self.g4s + (self.g4 - self.g4s) * self.gsmooth);
        self.g5s = flush(self.g5s + (self.g5 - self.g5s) * self.gsmooth);

        let (mem, wpos) = (&self.mem, self.wpos);
        let mut wet = self.down2.tick(mem, wpos, self.r_d2) * self.g1s
            + self.down1.tick(mem, wpos, self.r_d1) * self.g2s
            + self.up1.tick(mem, wpos, self.r_u1) * self.g4s
            + self.up2.tick(mem, wpos, self.r_u2) * self.g5s;

        self.wpos += 1;
        if self.wpos >= N {
            self.wpos = 0;
        }

        self.t1 = flush(self.t1 * self.tone_a + wet * (1.0 - self.tone_a));
        self.t2 = flush(self.t2 * self.tone_a + self.t1 * (1.0 - self.tone_a));
        wet = flush(self.hp_a * (self.hy + self.t2 - self.hx));
        self.hx = self.t2;
        self.hy = wet;

        let env = mono.abs();
        self.envf = flush(self.envf + (env - self.envf) * if env > self.envf { self.env_att } else { self.env_rel });
        self.envs = flush(self.envs + (env - self.envs) * self.env_slow);
        if self.trighang > 0.0 {
            self.trighang -= 1.0;
        } else if self.envf > self.envs * 2.0 + 0.002 && self.envf > 0.008 {
            self.swell = 0.0;
            self.trighang = self.trighangmax;
        }
        self.swell = (self.swell + self.att_inc).min(1.0);
        wet *= self.swell * self.swell;

        (flush(spl0 * self.gds + wet), flush(spl1 * self.gds + wet))
    }
}

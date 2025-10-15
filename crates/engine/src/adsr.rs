#[derive(Clone, Debug)]
pub struct Adsr {
    attack_samples: u32,
    decay_samples: u32,
    sustain_level: f32,
    release_samples: u32,
    sample_rate: f32,
}

impl Adsr {
    /// Creates a new ADSR configuration.
    pub fn new(sample_rate: f32) -> Self {
        Self {
            attack_samples: 0,
            decay_samples: 0,
            sustain_level: 1.0,
            release_samples: 0,
            sample_rate,
        }
    }

    /// Sets the ADSR parameters from time values in seconds.
    pub fn set_parameters(
        &mut self,
        attack_s: f32,
        decay_s: f32,
        sustain_level: f32,
        release_s: f32,
    ) {
        self.attack_samples = (attack_s * self.sample_rate).max(0.0) as u32;
        self.decay_samples = (decay_s * self.sample_rate).max(0.0) as u32;
        self.sustain_level = sustain_level.clamp(0.0, 1.0);
        self.release_samples = (release_s * self.sample_rate).max(0.0) as u32;
    }
}

/// The current phase of the ADSR envelope.
#[derive(Debug)]
enum EnvelopePhase {
    Attack,
    Decay,
    Sustain,
    Release,
    Off,
}

/// An ADSR envelope generator for a single voice.
#[derive(Debug)]
pub(crate) struct Envelope {
    adsr: Adsr,
    phase: EnvelopePhase,
    value: f32,
    samples_in_phase: u32,
}

impl Envelope {
    pub(crate) fn new(adsr: Adsr) -> Self {
        Self {
            adsr,
            phase: EnvelopePhase::Attack,
            value: 0.0,
            samples_in_phase: 0,
        }
    }

    pub(crate) fn next_value(&mut self) -> f32 {
        if matches!(self.phase, EnvelopePhase::Off) {
            return 0.0;
        }

        self.samples_in_phase += 1;

        match self.phase {
            EnvelopePhase::Attack => {
                if self.adsr.attack_samples == 0 {
                    self.value = 1.0;
                    self.phase = EnvelopePhase::Decay;
                    self.samples_in_phase = 0;
                } else {
                    self.value = self.samples_in_phase as f32 / self.adsr.attack_samples as f32;
                    if self.samples_in_phase >= self.adsr.attack_samples {
                        self.value = 1.0;
                        self.phase = EnvelopePhase::Decay;
                        self.samples_in_phase = 0;
                    }
                }
            }
            EnvelopePhase::Decay => {
                if self.adsr.decay_samples == 0 {
                    self.value = self.adsr.sustain_level;
                    self.phase = EnvelopePhase::Sustain;
                    self.samples_in_phase = 0;
                } else {
                    let decay_progress =
                        self.samples_in_phase as f32 / self.adsr.decay_samples as f32;
                    self.value = 1.0 - decay_progress * (1.0 - self.adsr.sustain_level);
                    if self.samples_in_phase >= self.adsr.decay_samples {
                        self.value = self.adsr.sustain_level;
                        self.phase = EnvelopePhase::Sustain;
                        self.samples_in_phase = 0;
                    }
                }
            }
            EnvelopePhase::Sustain => {
                self.value = self.adsr.sustain_level;
            }
            EnvelopePhase::Release => {
                if self.adsr.release_samples == 0 {
                    self.value = 0.0;
                } else {
                    let release_progress =
                        self.samples_in_phase as f32 / self.adsr.release_samples as f32;
                    self.value = self.adsr.sustain_level * (1.0 - release_progress);
                }

                if self.value <= 0.0 {
                    self.value = 0.0;
                    self.phase = EnvelopePhase::Off;
                }
            }
            EnvelopePhase::Off => {
                self.value = 0.0;
            }
        }
        self.value
    }

    pub(crate) fn note_off(&mut self) {
        if !matches!(self.phase, EnvelopePhase::Release | EnvelopePhase::Off) {
            self.phase = EnvelopePhase::Release;
            self.samples_in_phase = 0;
        }
    }

    pub(crate) fn is_active(&self) -> bool {
        !matches!(self.phase, EnvelopePhase::Off)
    }
}

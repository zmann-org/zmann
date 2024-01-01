use crate::filters::Allpass;
use crate::filters::Comb;

/// Tuning for Moorer's Reverberator can be found here:
/// http://www.music.mcgill.ca/~gary/courses/papers/Moorer-Reverb-CMJ-1979.pdf

///
/// Computes number of samples in an old sampling rate to
/// the number of samples in a new sampling rate.
///
fn adjust_length(length: usize, old_sr: usize, new_sr: usize) -> usize {
    (length as f32 * new_sr as f32 / old_sr as f32) as usize
}

fn ms_to_samples(ms: f32, sr: usize) -> usize {
    let seconds = ms / 1000.;
    let samples = (sr as f32) * seconds;
    samples as usize
}

const MOORER_SAMPLING_RATE: usize = 50_000;

const SCALE_WET: f32 = 3.0;
const SCALE_DAMPING: f32 = 0.4;

const STEREO_SPREAD_MS: f32 = 0.52;
const FIXED_GAIN: f32 = 0.015;

const SCALE_ROOM: f32 = 0.28;
const OFFSET_ROOM: f32 = 0.7;

// Comb filter tunings (delay length, feedback)
const COMB_L1_DELAY_LENGTH_MS: f32 = 50.;
const COMB_R1_DELAY_LENGTH_MS: f32 = 50. + STEREO_SPREAD_MS;
const COMB_L2_DELAY_LENGTH_MS: f32 = 56.;
const COMB_R2_DELAY_LENGTH_MS: f32 = 56. + STEREO_SPREAD_MS;
const COMB_L3_DELAY_LENGTH_MS: f32 = 61.;
const COMB_R3_DELAY_LENGTH_MS: f32 = 61. + STEREO_SPREAD_MS;
const COMB_L4_DELAY_LENGTH_MS: f32 = 68.;
const COMB_R4_DELAY_LENGTH_MS: f32 = 68. + STEREO_SPREAD_MS;
const COMB_L5_DELAY_LENGTH_MS: f32 = 72.;
const COMB_R5_DELAY_LENGTH_MS: f32 = 72. + STEREO_SPREAD_MS;
const COMB_L6_DELAY_LENGTH_MS: f32 = 78.;
const COMB_R6_DELAY_LENGTH_MS: f32 = 78. + STEREO_SPREAD_MS;

const ALLPASS_L_DELAY_LENGTH: f32 = 6.;
const ALLPASS_R_DELAY_LENGTH: f32 = 6. + STEREO_SPREAD_MS;

#[derive(Debug)]
pub struct MoorerReverb {
    allpasses: (Allpass, Allpass),
    combs: [(Comb, Comb); 6],
    wet_gains: (f32, f32),
    wet: f32,
    width: f32,
    dry: f32,
    input_gain: f32,
    dampening: f32,
    room_size: f32,
    frozen: bool,
}

fn generate_comb_filters(sr: usize) -> [(Comb, Comb); 6] {
    [
        (
            Comb::new(adjust_length(
                ms_to_samples(COMB_L1_DELAY_LENGTH_MS, MOORER_SAMPLING_RATE),
                MOORER_SAMPLING_RATE,
                sr,
            )),
            Comb::new(adjust_length(
                ms_to_samples(COMB_R1_DELAY_LENGTH_MS, MOORER_SAMPLING_RATE),
                MOORER_SAMPLING_RATE,
                sr,
            )),
        ),
        (
            Comb::new(adjust_length(
                ms_to_samples(COMB_L2_DELAY_LENGTH_MS, MOORER_SAMPLING_RATE),
                MOORER_SAMPLING_RATE,
                sr,
            )),
            Comb::new(adjust_length(
                ms_to_samples(COMB_R2_DELAY_LENGTH_MS, MOORER_SAMPLING_RATE),
                MOORER_SAMPLING_RATE,
                sr,
            )),
        ),
        (
            Comb::new(adjust_length(
                ms_to_samples(COMB_L3_DELAY_LENGTH_MS, MOORER_SAMPLING_RATE),
                MOORER_SAMPLING_RATE,
                sr,
            )),
            Comb::new(adjust_length(
                ms_to_samples(COMB_R3_DELAY_LENGTH_MS, MOORER_SAMPLING_RATE),
                MOORER_SAMPLING_RATE,
                sr,
            )),
        ),
        (
            Comb::new(adjust_length(
                ms_to_samples(COMB_L4_DELAY_LENGTH_MS, MOORER_SAMPLING_RATE),
                MOORER_SAMPLING_RATE,
                sr,
            )),
            Comb::new(adjust_length(
                ms_to_samples(COMB_R4_DELAY_LENGTH_MS, MOORER_SAMPLING_RATE),
                MOORER_SAMPLING_RATE,
                sr,
            )),
        ),
        (
            Comb::new(adjust_length(
                ms_to_samples(COMB_L5_DELAY_LENGTH_MS, MOORER_SAMPLING_RATE),
                MOORER_SAMPLING_RATE,
                sr,
            )),
            Comb::new(adjust_length(
                ms_to_samples(COMB_R5_DELAY_LENGTH_MS, MOORER_SAMPLING_RATE),
                MOORER_SAMPLING_RATE,
                sr,
            )),
        ),
        (
            Comb::new(adjust_length(
                ms_to_samples(COMB_L6_DELAY_LENGTH_MS, MOORER_SAMPLING_RATE),
                MOORER_SAMPLING_RATE,
                sr,
            )),
            Comb::new(adjust_length(
                ms_to_samples(COMB_R6_DELAY_LENGTH_MS, MOORER_SAMPLING_RATE),
                MOORER_SAMPLING_RATE,
                sr,
            )),
        ),
    ]
}

fn generate_allpass_filters(sr: usize) -> (Allpass, Allpass) {
    (
        Allpass::new(adjust_length(
            ms_to_samples(ALLPASS_L_DELAY_LENGTH, MOORER_SAMPLING_RATE),
            MOORER_SAMPLING_RATE,
            sr,
        )),
        Allpass::new(adjust_length(
            ms_to_samples(ALLPASS_R_DELAY_LENGTH, MOORER_SAMPLING_RATE),
            MOORER_SAMPLING_RATE,
            sr,
        )),
    )
}

impl MoorerReverb {
    pub fn new(sr: usize) -> Self {
        let mut freeverb = MoorerReverb {
            combs: generate_comb_filters(sr),
            allpasses: generate_allpass_filters(sr),
            wet_gains: (0., 0.),
            wet: 0.,
            dry: 0.,
            input_gain: 0.,
            width: 0.,
            dampening: 0.,
            room_size: 0.,
            frozen: false,
        };

        freeverb.set_wet(1.0);
        freeverb.set_width(0.5);
        freeverb.set_damping(0.5);
        freeverb.set_room_size(0.5);
        freeverb.set_frozen(false);

        freeverb
    }

    pub fn generate_filters(&mut self, sr: usize) {
        self.combs = generate_comb_filters(sr);
        self.allpasses = generate_allpass_filters(sr);
    }

    pub fn set_wet(&mut self, value: f32) {
        self.wet = value * SCALE_WET;
        self.update_wet_gains();
    }

    pub fn set_width(&mut self, value: f32) {
        self.width = value;
        self.update_wet_gains();
    }

    pub fn set_damping(&mut self, value: f32) {
        self.dampening = value * SCALE_DAMPING;
        self.update_combs()
    }

    pub fn set_frozen(&mut self, frozen: bool) {
        self.frozen = frozen;
        self.input_gain = if frozen { 0.0 } else { 1.0 };
        self.update_combs();
    }

    pub fn set_room_size(&mut self, value: f32) {
        self.room_size = value * SCALE_ROOM + OFFSET_ROOM;
        self.update_combs();
    }

    fn update_combs(&mut self) {
        let (feedback, dampening) = if self.frozen {
            (1.0, 0.0)
        } else {
            (self.room_size, self.dampening)
        };

        for combs in self.combs.iter_mut() {
            combs.0.set_feedback(feedback);
            combs.1.set_feedback(feedback);

            combs.0.set_dampening(dampening);
            combs.1.set_dampening(dampening);
        }
    }

    fn update_wet_gains(&mut self) {
        self.wet_gains = (
            self.wet * (self.width / 2.0 + 0.5),
            self.wet * ((1.0 - self.width) / 2.0),
        )
    }

    pub fn tick(&mut self, input: (f32, f32)) -> (f32, f32) {
        let input_mixed = (input.0 + input.1) * FIXED_GAIN * self.input_gain;
        let mut out = (0.0, 0.0);

        let allpassed_l = self.allpasses.0.tick(input_mixed);
        let allpassed_r = self.allpasses.1.tick(input_mixed);

        for combs in self.combs.iter_mut() {
            out.0 += combs.0.tick(allpassed_l);
            out.1 += combs.1.tick(allpassed_r);
        }

        (
            out.0 * self.wet_gains.0 + out.1 * self.wet_gains.1 + input.0 * self.dry,
            out.1 * self.wet_gains.0 + out.0 * self.wet_gains.1 + input.1 * self.dry,
        )
    }
}

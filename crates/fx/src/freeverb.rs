// All code taken from Ian Hobson's Freeverb implementation talk: https://www.youtube.com/watch?v=Yom9E-67bdI
// Code here: https://github.com/irh/freeverb-rs/blob/main/src/freeverb/src/freeverb.rs
// Ian Hobson's `freeverb-rs` is licensed under MIT License.

use crate::filters::Allpass;
use crate::filters::Comb;

/// Tuning for Freeverb can be found here:
/// https://ccrma.stanford.edu/~jos/pasp/Freeverb.html

const SCALE_WET: f32 = 1.3;
const SCALE_DAMPING: f32 = 0.4;

const STEREO_SPREAD: usize = 23;
const FIXED_GAIN: f32 = 0.015;

const SCALE_ROOM: f32 = 0.28;
const OFFSET_ROOM: f32 = 0.7;

const COMB_TUNING_L1: usize = 1116;
const COMB_TUNING_R1: usize = 1116 + STEREO_SPREAD;
const COMB_TUNING_L2: usize = 1118;
const COMB_TUNING_R2: usize = 1118 + STEREO_SPREAD;
const COMB_TUNING_L3: usize = 1277;
const COMB_TUNING_R3: usize = 1277 + STEREO_SPREAD;
const COMB_TUNING_L4: usize = 1356;
const COMB_TUNING_R4: usize = 1356 + STEREO_SPREAD;
const COMB_TUNING_L5: usize = 1422;
const COMB_TUNING_R5: usize = 1422 + STEREO_SPREAD;
const COMB_TUNING_L6: usize = 1491;
const COMB_TUNING_R6: usize = 1491 + STEREO_SPREAD;
const COMB_TUNING_L7: usize = 1557;
const COMB_TUNING_R7: usize = 1557 + STEREO_SPREAD;
const COMB_TUNING_L8: usize = 1617;
const COMB_TUNING_R8: usize = 1617 + STEREO_SPREAD;

const ALLPASS_TUNING_L1: usize = 225;
const ALLPASS_TUNING_L2: usize = 556;
const ALLPASS_TUNING_L3: usize = 441;
const ALLPASS_TUNING_L4: usize = 341;
const ALLPASS_TUNING_R1: usize = 225 + STEREO_SPREAD;
const ALLPASS_TUNING_R2: usize = 556 + STEREO_SPREAD;
const ALLPASS_TUNING_R3: usize = 441 + STEREO_SPREAD;
const ALLPASS_TUNING_R4: usize = 341 + STEREO_SPREAD;

pub struct Freeverb {
    combs: [(Comb, Comb); 8],
    allpasses: [(Allpass, Allpass); 4],
    wet_gains: (f32, f32),
    wet: f32,
    width: f32,
    dry: f32,
    input_gain: f32,
    dampening: f32,
    room_size: f32,
    frozen: bool,
}

fn adjust_length(length: usize, sr: usize) -> usize {
    (length as f32 * sr as f32 / 44100.) as usize
}

fn generate_comb_filters(sr: usize) -> [(Comb, Comb); 8] {
    [
        (
            Comb::new(adjust_length(COMB_TUNING_L1, sr)),
            Comb::new(adjust_length(COMB_TUNING_R1, sr)),
        ),
        (
            Comb::new(adjust_length(COMB_TUNING_L2, sr)),
            Comb::new(adjust_length(COMB_TUNING_R2, sr)),
        ),
        (
            Comb::new(adjust_length(COMB_TUNING_L3, sr)),
            Comb::new(adjust_length(COMB_TUNING_R3, sr)),
        ),
        (
            Comb::new(adjust_length(COMB_TUNING_L4, sr)),
            Comb::new(adjust_length(COMB_TUNING_R4, sr)),
        ),
        (
            Comb::new(adjust_length(COMB_TUNING_L5, sr)),
            Comb::new(adjust_length(COMB_TUNING_R5, sr)),
        ),
        (
            Comb::new(adjust_length(COMB_TUNING_L6, sr)),
            Comb::new(adjust_length(COMB_TUNING_R6, sr)),
        ),
        (
            Comb::new(adjust_length(COMB_TUNING_L7, sr)),
            Comb::new(adjust_length(COMB_TUNING_R7, sr)),
        ),
        (
            Comb::new(adjust_length(COMB_TUNING_L8, sr)),
            Comb::new(adjust_length(COMB_TUNING_R8, sr)),
        ),
    ]
}

fn generate_allpass_filters(sr: usize) -> [(Allpass, Allpass); 4] {
    [
        (
            Allpass::new(adjust_length(ALLPASS_TUNING_L1, sr)),
            Allpass::new(adjust_length(ALLPASS_TUNING_R1, sr)),
        ),
        (
            Allpass::new(adjust_length(ALLPASS_TUNING_L2, sr)),
            Allpass::new(adjust_length(ALLPASS_TUNING_R2, sr)),
        ),
        (
            Allpass::new(adjust_length(ALLPASS_TUNING_L3, sr)),
            Allpass::new(adjust_length(ALLPASS_TUNING_R3, sr)),
        ),
        (
            Allpass::new(adjust_length(ALLPASS_TUNING_L4, sr)),
            Allpass::new(adjust_length(ALLPASS_TUNING_R4, sr)),
        ),
    ]
}

impl Freeverb {
    pub fn new(sr: usize) -> Self {
        let mut freeverb = Freeverb {
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

        for combs in self.combs.iter_mut() {
            out.0 += combs.0.tick(input_mixed);
            out.1 += combs.1.tick(input_mixed);
        }
        for allpasses in self.allpasses.iter_mut() {
            out.0 = allpasses.0.tick(out.0);
            out.1 = allpasses.1.tick(out.1);
        }

        (
            out.0 * self.wet_gains.0 + out.1 * self.wet_gains.1 + input.0 * self.dry,
            out.1 * self.wet_gains.0 + out.0 * self.wet_gains.1 + input.1 * self.dry,
        )
    }
}

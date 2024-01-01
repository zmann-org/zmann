// All code taken from Ian Hobson's Freeverb implementation talk: https://www.youtube.com/watch?v=Yom9E-67bdI
// Code here: https://github.com/irh/freeverb-rs/blob/main/src/freeverb/src/freeverb.rs
// Ian Hobson's `freeverb-rs` is licensed under MIT License.

/// A delay line with variable buffer size.
#[derive(Debug)]
pub struct DelayLine {
    buffer: Vec<f32>,
    index: usize,
}

impl DelayLine {
    pub fn new(length: usize) -> DelayLine {
        DelayLine {
            buffer: vec![0.; length],
            index: 0,
        }
    }

    pub fn read(&self) -> f32 {
        self.buffer[self.index]
    }

    pub fn write_and_advance(&mut self, value: f32) {
        self.buffer[self.index] = value;

        if self.index == self.buffer.len() - 1 {
            self.index = 0;
        } else {
            self.index += 1;
        }
    }
}

/// An allpass filter with a single delay line.
#[derive(Debug)]
pub struct Allpass {
    delay_line: DelayLine,
}

impl Allpass {
    pub fn new(delay_length: usize) -> Allpass {
        Allpass {
            delay_line: DelayLine::new(delay_length),
        }
    }

    ///
    /// Process an input value with output and feedback
    /// calculated in the style of Schroeder's allpass filter.
    /// See Fig. 2: https://hajim.rochester.edu/ece/sites/zduan/teaching/ece472/reading/Schroeder_1962.pdf
    ///
    pub fn tick(&mut self, input: f32) -> f32 {
        let delayed = self.delay_line.read();
        let output = -input + delayed;
        let feedback = 0.5;

        self.delay_line
            .write_and_advance(input + delayed * feedback);
        output
    }
}

///
/// A low pass feedback comb filter implemented with a single delay line.
///
#[derive(Debug)]
pub struct Comb {
    delay_line: DelayLine,
    feedback: f32,
    filter_state: f32,
    dampening: f32,
    dampening_inverse: f32,
}

impl Comb {
    pub fn new(delay_length: usize) -> Comb {
        Comb {
            delay_line: DelayLine::new(delay_length),
            feedback: 0.,
            filter_state: 0.,
            dampening: 0.,
            dampening_inverse: 0.,
        }
    }

    pub fn set_feedback(&mut self, value: f32) {
        self.feedback = value;
    }

    pub fn set_dampening(&mut self, value: f32) {
        self.dampening = value;
        self.dampening_inverse = 1.0 - value;
    }

    pub fn tick(&mut self, input: f32) -> f32 {
        let output = self.delay_line.read();
        self.filter_state = output * self.dampening_inverse + self.filter_state * self.dampening;

        self.delay_line
            .write_and_advance(input + self.filter_state * self.feedback);

        output
    }
}

#[cfg(test)]
mod tests {
    use super::DelayLine;

    #[test]
    fn length_10() {
        let length: usize = 10;
        let mut line = DelayLine::new(length);
        for i in 0..length {
            assert_eq!(line.read(), 0.);
            line.write_and_advance(i as f32);
        }

        for i in 0..length {
            assert_eq!(line.read(), i as f32);
            line.write_and_advance(0.);
        }
    }
}

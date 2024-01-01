use std::f32::consts::PI;

///
/// Performs cubic interpolation given four adjacent samples
/// https://www.musicdsp.org/en/latest/Other/49-cubic-interpollation.html?highlight=cubic
///
/// # Arguments
/// * `fpos` - fractional component of position
/// * `xm1` - value corresponding to `x[n-1]`
/// * `x0` - value corresponding to `x[n]`
/// * `x1` - value corresponding to `x[n+1]`
/// * `x2` - value corresponding to `x[n+2]`
///
fn get_cubic_interpolated_value(fpos: f32, xm1: f32, x0: f32, x1: f32, x2: f32) -> f32 {
    let a = (3. * (x0 - x1) - xm1 + x2) / 2.;
    let b = 2. * x1 + xm1 - (5. * x0 + x2) / 2.;
    let c = (x1 - xm1) / 2.;

    (((a * fpos) + b) * fpos + c) * fpos + x0
}

pub struct StereoDelay {
    buffer_l: Vec<f32>,
    buffer_r: Vec<f32>,
    write_pointer: usize,
    lfo_phase: f32,
    sample_rate: usize,
}

impl StereoDelay {
    ///
    /// # Arguments
    /// * `max_delay_time` - the maximum delay time in seconds
    /// * `sample_rate` - the sample rate in samples per seconds
    ///
    pub fn new(max_delay_time: f32, sample_rate: usize) -> StereoDelay {
        let buffer_size = (max_delay_time * sample_rate as f32) as usize;

        // Instantiate buffers
        let mut buffer_l = Vec::with_capacity(buffer_size);
        buffer_l.resize(buffer_size, 0.0);
        let mut buffer_r = Vec::with_capacity(buffer_size);
        buffer_r.resize(buffer_size, 0.0);

        // Create vibrato object
        StereoDelay {
            buffer_l,
            buffer_r,
            write_pointer: 0,
            lfo_phase: 0.0,
            sample_rate,
        }
    }

    ///
    /// Resize and clear the circular buffers.
    ///
    /// # Arguments
    /// - `max_delay_time`: the max delay time, in seconds
    /// - `sample_rate`: the new sample rate, in samples/second
    ///
    pub fn resize_buffers(&mut self, max_delay_time: f32, sample_rate: usize) {
        let new_size = (max_delay_time * sample_rate as f32) as usize;
        self.buffer_l.resize(new_size, 0.0);
        self.buffer_r.resize(new_size, 0.0);
    }

    ///
    /// Calculates value at time `t` using cubic interpolation.
    ///
    fn get_cubic_interpolated_value_from_buffer(&self, t: f32, buffer: &Vec<f32>) -> f32 {
        let time = t % buffer.len() as f32;
        let inpos = time.floor() as usize;
        let finpos = time.fract();

        // Get four surrounding samples from buffer
        let xm1 = buffer[if inpos == 0 { buffer.len() } else { inpos } - 1];
        let x0 = buffer[inpos];
        let x1 = buffer[(inpos + 1) % buffer.len()];
        let x2 = buffer[(inpos + 2) % buffer.len()];

        get_cubic_interpolated_value(finpos, xm1, x0, x1, x2)
    }

    ///
    /// Get fractional read time into buffer
    ///
    fn get_read_time(&self, lfo_phase: f32, lfo_width: f32) -> f32 {
        let phase_component = 2.0 * PI * lfo_phase;
        let current_delay = lfo_width * (0.5 + 0.5 * phase_component.sin());
        let buffer_len = self.buffer_l.len() as f32;

        self.write_pointer as f32 - (current_delay * self.sample_rate as f32) as f32 + buffer_len
            - 3.0
    }

    ///
    /// Calculate samples from buffer given LFO width in samples.
    /// Phase shift offsets right read pointer for stereo width.
    ///
    fn read_interpolated_samples(&self, lfo_width: f32, phase_shift: f32) -> (f32, f32) {
        // Recalculate read pointer with respect to write pointer
        let mut lfo_phase = self.lfo_phase;
        if lfo_phase >= 1.0 {
            lfo_phase -= 1.0;
        }

        // Offset right read pointer for stereo width
        let t_l = self.get_read_time(lfo_phase, lfo_width);
        let t_r = self.get_read_time(lfo_phase + phase_shift, lfo_width);

        let out_l = self.get_cubic_interpolated_value_from_buffer(t_l, &self.buffer_l);
        let out_r = self.get_cubic_interpolated_value_from_buffer(t_r, &self.buffer_r);

        (out_l, out_r)
    }

    pub fn process_with_chorus(
        &mut self,
        input: (f32, f32),
        lfo_frequency: f32,
        vibrato_width: f32,
        lfo_phase_right_offset: f32,
        depth: f32,
        feedback: f32,
    ) -> (f32, f32) {
        let interpolated_samples =
            self.read_interpolated_samples(vibrato_width, lfo_phase_right_offset);

        // Store information in buffers
        let (in_l, in_r) = input;
        let (interpolated_l, interpolated_r) = interpolated_samples;
        self.buffer_l[self.write_pointer] = in_l + interpolated_l * feedback;
        self.buffer_r[self.write_pointer] = in_r + interpolated_r * feedback;

        // Increment write pointer at constant rate
        self.write_pointer += 1;

        if self.write_pointer >= self.buffer_l.len() {
            self.write_pointer = 0;
        }

        // Update LFO phase
        let phase_increment = lfo_frequency * (self.sample_rate as f32).recip();
        self.lfo_phase += phase_increment;
        if self.lfo_phase >= 1.0 {
            self.lfo_phase -= 1.0;
        }

        let out_l = in_l + depth * interpolated_samples.0;
        let out_r = in_r + depth * interpolated_samples.1;
        (out_l, out_r)
    }

    pub fn process_with_vibrato(
        &mut self,
        input: (f32, f32),
        lfo_frequency: f32,
        vibrato_width: f32,
        lfo_phase_right_offset: f32,
    ) -> (f32, f32) {
        let interpolated_samples =
            self.read_interpolated_samples(vibrato_width, lfo_phase_right_offset);

        // Store information in buffers
        let (in_l, in_r) = input;
        self.buffer_l[self.write_pointer] = in_l;
        self.buffer_r[self.write_pointer] = in_r;

        // Increment write pointer at constant rate
        self.write_pointer += 1;

        if self.write_pointer >= self.buffer_l.len() {
            self.write_pointer = 0;
        }

        // Update LFO phase
        let phase_increment = lfo_frequency * (self.sample_rate as f32).recip();
        self.lfo_phase += phase_increment;
        if self.lfo_phase >= 1.0 {
            self.lfo_phase -= 1.0;
        }

        interpolated_samples
    }
}

pub struct DelayLine {
    circular_buffer: Vec<f32>,
    read_pointer: usize,
    write_pointer: usize,
    delay_time: usize,
    dry_mix: f32,
    wet_mix: f32,
    feedback: f32,
    sample_rate: usize,
    lfo_phase: f32,
}

impl DelayLine {
    pub fn new(buffer_length: usize, sample_rate: usize) -> DelayLine {
        let mut circular_buffer = Vec::with_capacity(buffer_length);
        circular_buffer.resize(buffer_length, 0.0);
        DelayLine {
            circular_buffer,
            read_pointer: 0,
            write_pointer: 0,
            dry_mix: 0.0,
            wet_mix: 1.0,
            feedback: 0.5,
            delay_time: 0,
            sample_rate,
            lfo_phase: 0.0,
        }
    }

    ///
    /// Changes the read pointer position based on a given delay time.
    ///
    /// # Arguments
    /// * `delay_time` - The desired delay time, in milliseconds
    /// * `sample_rate` - The sample rate of the system
    ///
    pub fn set_delay_time(&mut self, delay_time: f32, sample_rate: f32) {
        let wp = self.write_pointer as f32;
        let buffer_length = self.circular_buffer.len();
        let delay_in_samples = (delay_time / 1000.0) * sample_rate;
        self.delay_time = delay_in_samples as usize;
        self.read_pointer = (wp - delay_in_samples + buffer_length as f32) as usize % buffer_length;
    }

    pub fn set_feedback(&mut self, feedback: f32) {
        self.feedback = feedback;
    }

    pub fn set_dry_wet(&mut self, dry_mix: f32, wet_mix: f32) {
        self.dry_mix = dry_mix;
        self.wet_mix = wet_mix;
    }

    ///
    /// Resize and clear the circular buffer.
    ///
    /// # Arguments
    /// - `new_size`: the new size of the circular buffer, in samples
    ///
    pub fn resize_buffer(&mut self, new_size: usize) {
        self.circular_buffer.resize(new_size, 0.0);
    }

    ///
    /// Resize and clear the circular buffer when changing sample rate.
    ///
    /// # Arguments
    /// - `new_size`: the new size of the circular buffer, in samples
    /// - `sample_rate`: the new sample rate
    ///
    pub fn resize_buffer_with_sample_rate(&mut self, new_size: usize, sample_rate: usize) {
        self.sample_rate = sample_rate;
        self.circular_buffer.resize(new_size, 0.0);
    }

    ///
    /// Calculates value at time `t` using cubic interpolation.
    ///
    fn get_cubic_interpolated_value_from_buffer(&self, t: f32) -> f32 {
        let buffer = &self.circular_buffer;
        let time = t % buffer.len() as f32;
        let inpos = time.floor() as usize;
        let finpos = time.fract();

        // Get four surrounding samples from buffer
        let xm1 = buffer[if inpos == 0 { buffer.len() } else { inpos } - 1];
        let x0 = buffer[inpos];
        let x1 = buffer[(inpos + 1) % buffer.len()];
        let x2 = buffer[(inpos + 2) % buffer.len()];

        get_cubic_interpolated_value(finpos, xm1, x0, x1, x2)
    }

    fn get_interpolated_sample(&self, lfo_width: f32, sample_rate: f32, phase_shift: f32) -> f32 {
        // Recalculate read pointer with respect to write pointer
        let mut lfo_phase = self.lfo_phase + phase_shift;
        if lfo_phase >= 1.0 {
            lfo_phase -= 1.0;
        }
        let phase_component = 2.0 * PI * lfo_phase;
        let current_delay = lfo_width * (0.5 + 0.5 * phase_component.sin());
        let buffer_len = self.circular_buffer.len() as f32;
        let t = self.write_pointer as f32 - (current_delay * sample_rate) as f32 + buffer_len - 3.0;

        self.get_cubic_interpolated_value_from_buffer(t)
    }

    pub fn process_with_delay(&mut self, input: f32) -> f32 {
        let buffer_length = self.circular_buffer.len();
        let t = (self.write_pointer as f32 - self.delay_time as f32 + buffer_length as f32 - 3.0)
            % buffer_length as f32;
        let interpolated_sample = self.get_cubic_interpolated_value_from_buffer(t);
        let output = self.dry_mix * input + self.wet_mix * interpolated_sample;

        // Write input signal and feedback signal into buffer
        self.circular_buffer[self.write_pointer] =
            input + (self.circular_buffer[t as usize] * self.feedback);

        self.read_pointer += 1;
        self.write_pointer += 1;

        if self.read_pointer >= self.circular_buffer.len() {
            self.read_pointer = 0;
        }
        if self.write_pointer >= self.circular_buffer.len() {
            self.write_pointer = 0;
        }

        output
    }

    ///
    /// I don't know what happened here, but the effect was cool enough that I want to try using it.
    ///
    pub fn process_with_glitch(
        &mut self,
        input: f32,
        lfo_frequency: f32,
        vibrato_width: f32,
        sample_rate: f32,
        depth: f32,
    ) -> f32 {
        let interpolated_sample = self.get_interpolated_sample(vibrato_width, sample_rate, 0.0);

        // Store information in buffer
        self.circular_buffer[self.write_pointer] = input;

        // Increment write pointer at constant rate
        self.write_pointer += 1;

        if self.write_pointer >= self.circular_buffer.len() {
            self.write_pointer = 0;
        }

        // Update LFO phase
        self.lfo_phase += lfo_frequency * sample_rate;
        if self.lfo_phase >= 1.0 {
            self.lfo_phase -= 1.0;
        }

        input + depth * interpolated_sample
    }
}

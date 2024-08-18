pub struct Sample {
    root: Vec<f32>,
    velocity: f32,
    current_note: u8,
    current_sample_index: usize,
}

/// Represents a sample with associated data, note, velocity, and current sample
/// index.
impl Sample {
    /// Creates a new `Sample` instance.
    ///
    /// # Arguments
    ///
    /// * `data` - The data of the sample.
    /// * `note` - The note of the sample.
    /// * `velocity` - The velocity of the sample.
    ///
    /// # Returns
    ///
    /// A new `Sample` instance.
    pub fn new(data: Vec<f32>, note: u8, velocity: f32) -> Self {
        Self {
            root: data,
            velocity,
            current_note: note,
            current_sample_index: 0,
        }
    }

    /// Checks if the current note matches the given note.
    ///
    /// # Arguments
    ///
    /// * `note` - The note to compare with the current note.
    ///
    /// # Returns
    ///
    /// `true` if the current note matches the given note, `false` otherwise.
    pub fn get_note_bool(&self, note: u8) -> bool {
        self.current_note == note
    }

    /// Gets the next sample from the sample data.
    ///
    /// # Returns
    ///
    /// The next sample value.
    pub fn get_next_sample(&mut self) -> f32 {
        let sample = self.root[self.current_sample_index];
        self.current_sample_index += 1;
        sample
    }

    /// Gets the velocity of the sample.
    ///
    /// # Returns
    ///
    /// The velocity of the sample.
    pub fn get_velocity(&self) -> f32 {
        self.velocity
    }

    /// Checks if the sample should be removed based on the current sample
    /// index.
    ///
    /// # Returns
    ///
    /// `true` if the sample should be removed, `false` otherwise.
    pub fn should_be_removed(&self) -> bool {
        self.current_sample_index >= self.root.len()
    }
}

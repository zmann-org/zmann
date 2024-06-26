pub struct Sample {
    root: Vec<f32>,
    velocity: f32,
    current_note: u8,
    current_sample_index: usize,
}

impl Sample {
    pub fn new(data: Vec<f32>, note: u8, velocity: f32) -> Self {
        Self {
            root: data,
            velocity: velocity,
            current_note: note,
            current_sample_index: 0,
        }
    }

    pub fn get_note_bool(&self, note: u8) -> bool {
        self.current_note == note
    }

    pub fn get_next_sample(&mut self) -> f32 {
        let sample = self.root[self.current_sample_index];
        self.current_sample_index += 1;
        sample
    }

    pub fn get_velocity(&self) -> f32 {
        self.velocity
    }
    
    pub fn should_be_removed(&self) -> bool {
        self.current_sample_index >= self.root.len()
    }
}

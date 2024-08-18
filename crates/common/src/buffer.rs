pub struct Sample {
    pub root: Vec<f32>,
    pub current_note: u8,
    pub current_sample_index: usize,
}

impl Sample {
    pub fn new(data: Vec<f32>, note: u8) -> Self {
        Self {
            root: data,
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

    pub fn should_be_removed(&self) -> bool {
        self.current_sample_index >= self.root.len()
    }
}


use std::sync::Arc;

use crate::adsr::{Adsr, Envelope};

pub struct Voice {
    sample_data: Arc<Vec<f32>>,
    note: u8,
    position: usize,
    velocity: f32,
    envelope: Envelope,
    looping: bool,
}

impl Voice {
    /// Creates a new voice.
    pub fn new(
        sample_data: Arc<Vec<f32>>,
        note: u8,
        velocity: f32,
        adsr: Adsr,
        looping: bool,
    ) -> Self {
        Self {
            sample_data,
            note,
            position: 0,
            velocity,
            envelope: Envelope::new(adsr),
            looping,
        }
    }

    /// Checks if this voice is for a specific MIDI note.
    pub fn matches_note(&self, note: u8) -> bool {
        self.note == note
    }

    /// Triggers the release phase of the envelope.
    pub fn note_off(&mut self) {
        self.envelope.note_off();
    }

    /// Returns `true` if the voice is still active.
    pub fn is_active(&self) -> bool {
        if self.looping {
            self.envelope.is_active()
        } else {
            self.envelope.is_active() && self.position < self.sample_data.len()
        }
    }

    /// Generates the next sample for this voice.
    pub fn next_sample(&mut self) -> f32 {
        if !self.is_active() {
            return 0.0;
        }

        let envelope_value = self.envelope.next_value();

        if self.looping && !self.sample_data.is_empty() {
            self.position %= self.sample_data.len();
        }

        let sample_value = self.sample_data.get(self.position).copied().unwrap_or(0.0);
        self.position += 1;

        sample_value * self.velocity * envelope_value
    }
}

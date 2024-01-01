/// A basic DC filter for correcting DC offset.
/// 
/// Adapted for non-SIMD from Fredemus in va-filter, which is licensed under GPL 3.0:
/// https://github.com/Fredemus/va-filter
/// 
/// Originally copied from Understanding Digital Signal Processing by Richard Lyons
pub struct DcFilter {
    y0: f32,
    x0: f32,
    // Higher alpha moves the cutoff lower, but also makes it settle slower. See if it's reasonable to make it higher
    alpha: f32,
}

impl Default for DcFilter {
    fn default() -> Self {
        Self {
            y0: 0.,
            x0: 0.,
            alpha: 0.999,
        }
    }
}

impl DcFilter {
    pub fn process(&mut self, input: f32) -> f32 {
        let y_new = input - self.x0 + self.alpha * self.y0;
        self.x0 = input;
        self.y0 = y_new;

        y_new
    }
}

/// A second-order allpass filter.
///
/// Adapted for non-SIMD from Fredemus in va-filter, which is licensed under GPL
/// 3.0: https://github.com/Fredemus/va-filter
#[derive(Clone, Copy)]
pub struct AllpassFilter {
    pub a: f32,

    pub x0: f32,
    pub x1: f32,
    pub x2: f32,

    pub y0: f32,
    pub y1: f32,
    pub y2: f32,
}

impl Default for AllpassFilter {
    fn default() -> Self {
        Self {
            a: 0.0,
            x0: 0.0,
            x1: 0.0,
            x2: 0.0,
            y0: 0.0,
            y1: 0.0,
            y2: 0.0,
        }
    }
}

impl AllpassFilter {
    fn process(&mut self, input_sample: f32) -> f32 {
        // Shuffle inputs
        self.x2 = self.x1;
        self.x1 = self.x0;
        self.x0 = input_sample;

        // Shuffle outputs
        self.y2 = self.y1;
        self.y1 = self.y0;

        let output = self.x2 + ((input_sample - self.y2) * self.a);
        self.y0 = output;

        output
    }
}

/// A cascading allpass filter chain.
///
/// Adapted for non-SIMD from Fredemus in va-filter, which is licensed under GPL
/// 3.0: https://github.com/Fredemus/va-filter
#[derive(Clone, Copy)]
struct AllpassFilterCascade {
    allpass_filters: [AllpassFilter; 6],
    filter_count: usize,
}

impl AllpassFilterCascade {
    fn process(&mut self, input_sample: f32) -> f32 {
        let mut output = input_sample;
        for i in 0..self.filter_count {
            output = self.allpass_filters[i].process(output);
        }
        output
    }
}

/// A half band filter.
///
/// Adapted for non-SIMD from Fredemus in va-filter, which is licensed under GPL
/// 3.0: https://github.com/Fredemus/va-filter
#[derive(Clone, Copy)]
pub struct HalfbandFilter {
    filter_a: AllpassFilterCascade,
    filter_b: AllpassFilterCascade,
    old_out: f32,
}

impl HalfbandFilter {
    pub fn new(order: usize, steep: bool) -> HalfbandFilter {
        let a_coefficients: Vec<f32>;
        let b_coefficients: Vec<f32>;

        if steep {
            // rejection=104dB, transition band=0.01
            if order == 12 {
                a_coefficients = vec![
                    0.036_681_503,
                    0.274_631_77,
                    0.561_099,
                    0.769_741_83,
                    0.892_260_8,
                    0.962_094_55,
                ];

                b_coefficients = vec![
                    0.136_547_63,
                    0.423_138_62,
                    0.677_540_06,
                    0.839_889_65,
                    0.931_542,
                    0.987_816_4,
                ];
            }
            // rejection=86dB, transition band=0.01
            else if order == 10 {
                a_coefficients = vec![
                    0.051_457_617,
                    0.359_786_57,
                    0.672_547_6,
                    0.859_088_5,
                    0.954_021,
                ];

                b_coefficients = vec![
                    0.186_219_07,
                    0.529_951_4,
                    0.781_025_77,
                    0.914_181_6,
                    0.985_475,
                ];
            }
            // rejection=69dB, transition band=0.01
            else if order == 8 {
                a_coefficients = vec![
                    0.077_115_08,
                    0.482_070_62,
                    0.796_820_46,
                    0.941_251_46,
                ];

                b_coefficients = vec![
                    0.265_968_53,
                    0.665_104_15,
                    0.884_101_5,
                    0.982_005_4,
                ];
            }
            // rejection=51dB, transition band=0.01
            else if order == 6 {
                a_coefficients = vec![0.127_141_42, 0.652_824_6, 0.917_694_3];

                b_coefficients = vec![0.400_567_9, 0.820_416_4, 0.976_311_45];
            }
            // rejection=53dB,transition band=0.05
            else if order == 4 {
                a_coefficients = vec![0.120_732_12, 0.663_202_05];

                b_coefficients = vec![0.390_362_17, 0.890_786_8];
            }
            // order=2, rejection=36dB, transition band=0.1
            else {
                a_coefficients = vec![0.236_471_03];
                b_coefficients = vec![0.714_542_15];
            }
        }
        // softer slopes, more attenuation and less stopband ripple
        else {
            // rejection=150dB, transition band=0.05
            if order == 12 {
                a_coefficients = vec![
                    0.016_774_667,
                    0.139_021_49,
                    0.332_501_1,
                    0.537_661_1,
                    0.721_418_4,
                    0.882_185_8,
                ];
                b_coefficients = vec![
                    0.065_013_19,
                    0.230_941_3,
                    0.436_494_23,
                    0.632_961, //0.06329609551399348
                    0.803_780_85,
                    0.959_968_75,
                ];
            }
            // rejection=133dB, transition band=0.05
            else if order == 10 {
                a_coefficients = vec![
                    0.023_668_313,
                    0.189_894_77,
                    0.431_573_18,
                    0.663_202_05,
                    0.860_015_6,
                ];
                b_coefficients = vec![
                    0.090_565_56,
                    0.307_857_57,
                    0.551_678_24,
                    0.765_214_7,
                    0.952_477_3,
                ];
            }
            // rejection=106dB, transition band=0.05
            else if order == 8 {
                a_coefficients = vec![
                    0.035_832_79,
                    0.272_040_13,
                    0.572_057_2,
                    0.827_124_8,
                ];

                b_coefficients = vec![
                    0.134_090_14,
                    0.424_324_87,
                    0.706_292_15,
                    0.941_503_1,
                ];
            }
            // rejection=80dB, transition band=0.05
            else if order == 6 {
                a_coefficients = vec![0.060_297_392, 0.412_590_7, 0.772_715_6];

                b_coefficients = vec![0.215_971_44, 0.604_358_6, 0.923_886_1];
            }
            // rejection=70dB,transition band=0.1
            else if order == 4 {
                a_coefficients = vec![0.079_866_424, 0.545_353_65];

                b_coefficients = vec![0.283_829_33, 0.834_411_9];
            }
            // order=2, rejection=36dB, transition band=0.1
            else {
                a_coefficients = vec![0.236_471_03];
                b_coefficients = vec![0.714_542_15];
            }
        }
        let mut allpasses_a = [AllpassFilter::default(); 6];
        for i in 0..order / 2 {
            allpasses_a[i].a = a_coefficients[i];
        }
        let filter_a = AllpassFilterCascade {
            allpass_filters: allpasses_a,
            filter_count: order / 2,
        };
        let mut allpasses_b = [AllpassFilter::default(); 6];
        for i in 0..order / 2 {
            allpasses_b[i].a = b_coefficients[i];
        }
        let filter_b = AllpassFilterCascade {
            allpass_filters: allpasses_b,
            filter_count: order / 2,
        };
        HalfbandFilter {
            filter_a,
            filter_b,
            old_out: 0.,
        }
    }

    pub fn process(&mut self, input: f32) -> f32 {
        let output = (self.filter_a.process(input) + self.old_out) * 0.5;
        self.old_out = self.filter_b.process(input);
        output
    }
}

impl Default for HalfbandFilter {
    fn default() -> HalfbandFilter {
        let a_coefficients = [0.016_774_667,
            0.139_021_49,
            0.332_501_1,
            0.537_661_1,
            0.721_418_4,
            0.882_185_8];

        let b_coefficients = [0.065_013_19,
            0.230_941_3,
            0.436_494_23,
            0.632_961, //0.06329609551399348
            0.803_780_85,
            0.959_968_75];
        let mut allpasses_a = [AllpassFilter::default(); 6];
        let default_order = 12;
        for i in 0..default_order / 2 {
            allpasses_a[i].a = a_coefficients[i];
        }
        let filter_a = AllpassFilterCascade {
            filter_count: default_order / 2,
            allpass_filters: allpasses_a,
        };
        let mut allpasses_b = [AllpassFilter::default(); 6];
        for i in 0..default_order / 2 {
            allpasses_b[i].a = b_coefficients[i];
        }
        let filter_b = AllpassFilterCascade {
            filter_count: default_order / 2,
            allpass_filters: allpasses_b,
        };
        HalfbandFilter {
            filter_a,
            filter_b,
            old_out: 0.,
        }
    }
}

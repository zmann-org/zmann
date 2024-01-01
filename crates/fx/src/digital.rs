pub fn round_to_multiple(number: f32, multiple: f32) -> f32 {
    multiple * (number / multiple).round()
}

pub fn bitcrush_sample(input: f32, bits: f32) -> f32 {
    round_to_multiple(input, 2_f32.powf(-bits))
}

pub fn floating_point_quantize(input: f32, constant: f32) -> f32 {
    input + constant - constant
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bitcrush_is_correct_4bits() {
        let inputs = vec![0., 0.1, 0.2, 0.5, 0.87, 1.0];
        let bits = 4.;
        let outputs: Vec<f32> = inputs.iter().map(|x| bitcrush_sample(*x, bits)).collect();
        let expected: Vec<f32> = vec![0.0, 0.125, 0.1875, 0.5, 0.875, 1.0];
        assert_eq!(expected, outputs);
    }

    #[test]
    fn bitcrush_is_correct_2bits() {
        let inputs = vec![0., 0.1, 0.2, 0.5, 0.87, 1.0];
        let bits = 2.;
        let outputs: Vec<f32> = inputs.iter().map(|x| bitcrush_sample(*x, bits)).collect();
        let expected: Vec<f32> = vec![0.0, 0.0, 0.25, 0.5, 0.75, 1.0];
        assert_eq!(expected, outputs);
    }

    #[test]
    fn bitcrush_is_correct_7bits() {
        let inputs = vec![0., 0.1, 0.2, 0.5, 0.87, 1.0];
        let bits = 7.;
        let outputs: Vec<f32> = inputs.iter().map(|x| bitcrush_sample(*x, bits)).collect();
        let expected: Vec<f32> = vec![0.0, 0.1015625, 0.203125, 0.5, 0.8671875, 1.0];
        assert_eq!(expected, outputs);
    }

    #[test]
    fn test_floating_point_quantize() {
        let inputs = vec![0., 0.1, 0.2, 0.5, 0.87, 1.0];
        let constant: f32 = 128.0;
        let outputs: Vec<f32> = inputs
            .iter()
            .map(|x| floating_point_quantize(*x, constant))
            .collect();
        assert_ne!(inputs, outputs);
        println!("{:?}", outputs);
    }

    #[test]
    fn test_floating_point_quantize_large_constant() {
        let inputs = vec![0., 0.1, 0.2, 0.5, 0.87, 1.0];
        let constant: f32 = 10000.0;
        let outputs: Vec<f32> = inputs
            .iter()
            .map(|x| floating_point_quantize(*x, constant))
            .collect();
        assert_ne!(inputs, outputs);
        println!("{:?}", outputs);
    }
}

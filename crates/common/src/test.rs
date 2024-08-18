#[cfg(test)]
use buffer::Sample;

#[cfg(test)]
use super::*;

#[test]
fn parameter_minimum() {
    assert_eq!(PARAMETER_MINIMUM, 0.00);
}

#[test]
fn new_sample() {
    let data = vec![1.0, 2.0, 3.0];
    let note = 1;
    let sample = Sample::new(data.clone(), note);

    assert_eq!(sample.root, data);
    assert_eq!(sample.current_note, note);
    assert_eq!(sample.current_sample_index, 0);
}

#[test]
fn get_note_bool() {
    let data = vec![1.0, 2.0, 3.0];
    let note = 1;
    let sample = Sample::new(data.clone(), note);

    assert!(sample.get_note_bool(note));
    assert!(!sample.get_note_bool(2));
}

#[test]
fn get_next_sample() {
    let data = vec![1.0, 2.0, 3.0];
    let note = 1;
    let mut sample = Sample::new(data.clone(), note);

    assert_eq!(sample.get_next_sample(), 1.0);
    assert_eq!(sample.get_next_sample(), 2.0);
    assert_eq!(sample.get_next_sample(), 3.0);
}

#[test]
fn should_be_removed() {
    let data = vec![1.0, 2.0, 3.0];
    let note = 1;
    let mut sample = Sample::new(data.clone(), note);

    assert!(!sample.should_be_removed());
    sample.get_next_sample();
    assert!(!sample.should_be_removed());
    sample.get_next_sample();
    assert!(!sample.should_be_removed());
    sample.get_next_sample();
    assert!(sample.should_be_removed());
}

use std::{thread, time::Duration};

use autopilot::{key, mouse};
use rand::prelude::SliceRandom;

pub fn run(string: &str) {
    if string.is_empty() {
        return;
    }
    let mut rng = rand::thread_rng();
    let secs = [1, 3, 5, 10, 15, 20];
    string.split_whitespace().cycle().for_each(|word| {
        let second = secs.choose(&mut rng).map_or(1, |v| *v);
        key::type_string(word, &[], 200., 0.);
        thread::sleep(Duration::from_secs(second));
        let second = secs.choose(&mut rng).map_or(1, |v| *v);
        thread::sleep(Duration::from_secs(second));
        mouse::click(mouse::Button::Left, None);
        mouse::click(mouse::Button::Left, None);
    });
}

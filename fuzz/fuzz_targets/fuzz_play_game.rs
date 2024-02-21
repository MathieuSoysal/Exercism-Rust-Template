#![no_main]

use libfuzzer_sys::fuzz_target;
#[macro_use] extern crate libfuzzer_sys;

use template_exercisme::play_game;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let i = s.parse::<u32>().unwrap_or(0);
        play_game(i);
    }
});

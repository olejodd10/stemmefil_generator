use crate::{save_isolated_midi_tracks_from_file, mix_mp3s, mix_mp3s_panned};

#[test]
fn test_isolated() {
    save_isolated_midi_tracks_from_file("./hyvan.mid", "./output");
}

#[test]
fn test_mix_simple() {
    mix_mp3s(&["1.mp3", "2.mp3"], "mix.mp3");
}

#[test]
fn test_mix_panned() {
    mix_mp3s_panned("1.mp3", "2.mp3", "panned.mp3");
}

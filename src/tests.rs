use crate::*;

#[test]
fn test_isolated() {
    save_isolated_midi_tracks_from_file("./hyvan.mid", "./output");
}

#[test]
fn test_mix_simple() {
    mix_audio(&["1.mp3", "2.mp3"], "mix.mp3");
}

#[test]
fn test_mix_panned() {
    mix_audio_panned("1.mp3", "2.mp3", "panned.mp3");
}

#[test]
fn test_generate() {
    generate_stemmefiler("FluidR3_GM.sf2", "sit.mid", "output", 5.0)
}
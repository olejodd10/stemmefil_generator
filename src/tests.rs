use crate::*;

// #[test]
// fn test_isolated() {
//     save_isolated_midi_tracks_from_file("./hyvan.mid", "./output");
// }

// #[test]
// fn test_mix_simple() {
//     mix_audio(&["1.mp3", "2.mp3"], "mix.mp3");
// }

// #[test]
// fn test_mix_panned() {
//     mix_audio_panned("1.mp3", "2.mp3", "panned.mp3");
// }

#[test]
fn test_generate() {
    let temp_dir = format!("{}/stemmefil/sit", std::env::temp_dir().to_str().unwrap());
    std::fs::create_dir_all(&temp_dir).unwrap();
    generate_stemmefiler_from_midi("OJs stemme.sf2", "sit.mid", "output", &temp_dir, 5.0, true);
    std::fs::remove_dir_all(temp_dir).unwrap();
}

#[test]
fn test_custom() {
    let temp_dir = format!("{}/stemmefil/sit", std::env::temp_dir().to_str().unwrap());
    std::fs::create_dir_all(&temp_dir).unwrap();
    custom_stemmefil_from_midi("OJs stemme.sf2", "sit.mid", "output", &temp_dir);
    std::fs::remove_dir_all(temp_dir).unwrap();
}

// Metode 1: splitte midi-fil i alle tracks
// Konvertere dem til mp3
// Justere volum på dem
// Merge en eller flere filer

// Metode 2: endre velocity på alle tracks utenom 1.



use crate::{save_isolated_midi_tracks_from_file, overlay_mp3s};

#[test]
fn test_isolated() {
    save_isolated_midi_tracks_from_file("./hyvan.mid", "./output");
}

#[test]
fn test_overlay() {
    println!("{:?}", std::env::current_dir().unwrap());
    overlay_mp3s(&["1.mp3", "2.mp3"], "out.mp3");
}
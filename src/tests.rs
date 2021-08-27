
// Metode 1: splitte midi-fil i alle tracks
// Konvertere dem til mp3
// Justere volum på dem
// Merge en eller flere filer

// Metode 2: endre velocity på alle tracks utenom 1.



use crate::{save_weighted_midi_tracks_from_file, save_separated_midi_tracks_from_file};

#[test]
fn test_weighted() {
    save_weighted_midi_tracks_from_file("./tous.mid", "./output", None, Some(0.5));
}

#[test]
fn test_separated() {
    save_separated_midi_tracks_from_file("./hyvan.mid", "./output");
}
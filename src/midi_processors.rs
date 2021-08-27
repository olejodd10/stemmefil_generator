use std::path::Path;

mod track_utils;
use track_utils::{extract_track_name, relative_track_velocity_change};

// Splits midi_path into its tracks and saves them separately
// WARNING: Some MIDI files store meta information relevant so subsequent tracks in the first track.
// This might lead to time signature differences between tracks, for example
#[allow(dead_code)]
pub fn save_separated_midi_tracks_from_file<P: AsRef<Path>>(midi_path: P, out_dir: P) {
    let data = std::fs::read(midi_path.as_ref()).unwrap();
    let smf = midly::Smf::parse(&data).unwrap();
    save_separated_midi_tracks(smf, out_dir);
}

//Saves the tracks of an smf
fn save_separated_midi_tracks<P: AsRef<Path>>(smf: midly::Smf, out_dir: P) {
    for (i, track) in smf.tracks.into_iter().enumerate() {
        let mut separated_track_smf = midly::Smf::new(smf.header); // midly::Header implements Clone
        let track_name = extract_track_name(&track).unwrap_or(format!("track_{}", i));
        let track_midi_filename = format!("{}.mid", track_name);
        separated_track_smf.tracks = vec![track];
        separated_track_smf.save(out_dir.as_ref().join(&track_midi_filename)).expect(&format!("Error saving {}", track_midi_filename));
    }
}

#[allow(dead_code)]
pub fn save_weighted_midi_tracks_from_file<P: AsRef<Path>>(midi_path: P, out_dir: P, focused_factor: f64, dampened_factor: f64) {
    let data = std::fs::read(midi_path.as_ref()).unwrap();
    let smf = midly::Smf::parse(&data).unwrap();
    save_weighted_midi_tracks(smf, out_dir, focused_factor, dampened_factor);
}

//Saves the tracks of an smf
fn save_weighted_midi_tracks<P: AsRef<Path>>(smf: midly::Smf, out_dir: P, focused_factor: f64, dampened_factor: f64) {
    for i in 0..smf.tracks.len() {
        let mut weighted_smf = smf.clone();
        let track_name = extract_track_name(&weighted_smf.tracks[i]).unwrap_or(format!("track_{}", i));
        let track_midi_filename = format!("{}.mid", track_name);
        for (j, ref mut track) in weighted_smf.tracks.iter_mut().enumerate() {
            if j == i {
                relative_track_velocity_change(track, focused_factor);
            } else {
                relative_track_velocity_change(track, dampened_factor);
            }
        }
        weighted_smf.save(out_dir.as_ref().join(&track_midi_filename)).expect(&format!("Error saving {}", track_midi_filename));
    }
}
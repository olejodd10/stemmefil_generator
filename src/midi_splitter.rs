use std::path::{Path, PathBuf};

mod track_utils;
use track_utils::{extract_track_name, dampen_track};

// Splits midi_path into its tracks and saves them separately
#[allow(dead_code)]
pub fn save_isolated_midi_tracks_from_file<P: AsRef<Path>>(midi_path: P, out_dir: P) -> Vec<PathBuf> {
    let data = std::fs::read(midi_path.as_ref()).unwrap();
    let smf = midly::Smf::parse(&data).unwrap();
    save_isolated_midi_tracks(smf, out_dir)
}

//Saves the tracks of an smf as isolated midi files
// Some tracks may contain meta information that is important to other tracks - that's why tracks are dampened and not removed
fn save_isolated_midi_tracks<P: AsRef<Path>>(smf: midly::Smf, out_dir: P) -> Vec<PathBuf> {
    let mut isolated_midi_paths = Vec::new();
    for i in 0..smf.tracks.len() {
        let mut isolated_smf = smf.clone();
        let track_name = extract_track_name(&isolated_smf.tracks[i]).unwrap_or(format!("track_{}", i));
        let track_midi_filename = format!("{}.mid", track_name);
        let out_file_path = out_dir.as_ref().join(&track_midi_filename); 

        for (j, track) in isolated_smf.tracks.iter_mut().enumerate() {
            if j != i {
                dampen_track(track);
            }
        }
        
        if out_file_path.exists() {
            println!("Overwriting existing file {}", track_midi_filename);
        } else {
            isolated_smf.save(&out_file_path).expect(&format!("Error saving {}", track_midi_filename));
        }
        isolated_midi_paths.push(out_file_path);
    }
    isolated_midi_paths
}

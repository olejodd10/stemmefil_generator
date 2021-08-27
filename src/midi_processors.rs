use std::path::Path;

mod track_utils;
use track_utils::{extract_track_name, relative_track_velocity_change, dampen_track};

// Splits midi_path into its tracks and saves them separately
// WARNING: Some MIDI files store meta information relevant so subsequent tracks in the first track.
// This might lead to time signature differences between tracks, for example
#[allow(dead_code)]
pub fn save_separated_midi_tracks_from_file<P: AsRef<Path>>(midi_path: P, out_dir: P) {
    let data = std::fs::read(midi_path.as_ref()).unwrap();
    let smf = midly::Smf::parse(&data).unwrap();
    save_separated_midi_tracks(smf, out_dir);
}

//Saves the tracks of an smf as separated midi files
fn save_separated_midi_tracks<P: AsRef<Path>>(smf: midly::Smf, out_dir: P) {

    // for (i, track) in smf.tracks.into_iter().enumerate() {
    //     let mut separated_track_smf = midly::Smf::new(smf.header); // midly::Header implements Clone
    //     let track_name = extract_track_name(&track).unwrap_or(format!("track_{}", i));
    //     let track_midi_filename = format!("{}.mid", track_name);
    //     separated_track_smf.tracks = vec![track];
    //     separated_track_smf.save(out_dir.as_ref().join(&track_midi_filename)).expect(&format!("Error saving {}", track_midi_filename));
    // }

    // This implementation yields a more precise output than the commented one above, since some tracks may contain meta information that is important to other tracks
    for i in 0..smf.tracks.len() {
        let mut separated_smf = smf.clone();
        let track_name = extract_track_name(&separated_smf.tracks[i]).unwrap_or(format!("track_{}", i));
        let track_midi_filename = format!("{}.mid", track_name);
        let out_file_path = out_dir.as_ref().join(&track_midi_filename); 

        for (j, track) in separated_smf.tracks.iter_mut().enumerate() {
            if j != i {
                dampen_track(track);
            }
        }
        
        if out_file_path.exists() {
            println!("Overwriting existing file {}", track_midi_filename);
        } else {
            separated_smf.save(&out_file_path).expect(&format!("Error saving {}", track_midi_filename));
        }
    }
}

#[allow(dead_code)]
pub fn save_weighted_midi_tracks_from_file<P: AsRef<Path>>(midi_path: P, out_dir: P, focused_change: Option<f64>, dampened_change: Option<f64>) {
    let data = std::fs::read(midi_path.as_ref()).unwrap();
    let smf = midly::Smf::parse(&data).unwrap();
    save_weighted_midi_tracks(smf, out_dir, focused_change, dampened_change);
}

//Saves the tracks of an smf
fn save_weighted_midi_tracks<P: AsRef<Path>>(smf: midly::Smf, out_dir: P, focused_change: Option<f64>, dampened_change: Option<f64>) {
    for i in 0..smf.tracks.len() {
        let mut weighted_smf = smf.clone();
        let track_name = extract_track_name(&weighted_smf.tracks[i]).unwrap_or(format!("track_{}", i));
        let track_midi_filename = format!("{}.mid", track_name);
        let out_file_path = out_dir.as_ref().join(&track_midi_filename); 

        if let Some(focused_change) = focused_change {
            relative_track_velocity_change(&mut weighted_smf.tracks[i], focused_change);
        }

        if let Some(dampened_change) = dampened_change {
            for (j, track) in weighted_smf.tracks.iter_mut().enumerate() {
                if i != j {
                    relative_track_velocity_change(track, dampened_change);
                }
            }
        }
        
        if out_file_path.exists() {
            println!("Overwriting existing file {}", track_midi_filename);
        } else {
            weighted_smf.save(&out_file_path).expect(&format!("Error saving {}", track_midi_filename));
        }
    }
}
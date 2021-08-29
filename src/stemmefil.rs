use std::path::{Path, PathBuf};

mod midi_splitter;
mod midi_player;
mod audio_mixer;

pub use midi_splitter::save_isolated_midi_tracks_from_file;
pub use midi_player::play_to_raw;
pub use audio_mixer::{mix_audio, mix_audio_panned};

fn mix_stemmefil<P: AsRef<Path>, S: AsRef<str>>(name: S, left_sources: &[P], right_sources: &[P], out_dir: P, temp_dir: P) {
    let out_path = out_dir.as_ref().join(format!("{}.mp3", name.as_ref()));
    let left_mix_path = temp_dir.as_ref().join(format!("{}_left.mp3", name.as_ref()));
    mix_audio(&left_sources.iter().map(|p| p.as_ref()).collect::<Vec<&Path>>(), left_mix_path.as_ref()); //Ugly workaround here
    let right_mix_path = temp_dir.as_ref().join(format!("{}_right.mp3", name.as_ref()));
    mix_audio(&right_sources.iter().map(|p| p.as_ref()).collect::<Vec<&Path>>(), right_mix_path.as_ref()); //Ugly workaround here
    mix_audio_panned(left_mix_path, right_mix_path, out_path);
}

pub fn generate_stemmefiler<P: AsRef<Path>>(sound_font_path: P, midi_path: P, out_dir: P, gain: f64) {
    let temp_dir = std::env::temp_dir().join("stemmefil_generator").join(midi_path.as_ref().file_stem().unwrap());
    std::fs::create_dir_all(&temp_dir).unwrap();

    let isolated_midi_paths = save_isolated_midi_tracks_from_file(midi_path.as_ref(), &temp_dir);
    
    let source_paths: Vec<PathBuf> = isolated_midi_paths.into_iter().map(|isolated_midi_path| {
        let source_path = isolated_midi_path.with_extension("pcm");
        play_to_raw(sound_font_path.as_ref(), isolated_midi_path.as_path(), &source_path, gain);
        source_path
    }).collect();

    for (i, left_source) in source_paths.iter().enumerate() {
        let stemmefil_name = left_source.file_stem().unwrap().to_str().unwrap();
        eprintln!("Creating stemmefil {}", stemmefil_name);

        let right_sources: Vec<&Path> = source_paths.iter().enumerate().filter_map(|(j,p)| {
            if j != i {
                Some(p.as_path())
            } else {
                None
            }
        }).collect();

        mix_stemmefil(stemmefil_name, &[left_source.as_path()], &right_sources, out_dir.as_ref(), temp_dir.as_path())
    }

    std::fs::remove_dir_all(temp_dir).unwrap();
}


// fn tailor_stemmefil(...)
use std::path::{Path, PathBuf};

mod midi_splitter;
mod midi_player;
mod audio_mixer;

use midi_splitter::save_isolated_midi_tracks_from_file;
use midi_player::play_to_raw;
use audio_mixer::{mix_audio, mix_audio_panned};

fn mix_stemmefil<P: AsRef<Path>, S: AsRef<str>>(name: S, left_sources: &[P], right_sources: &[P], out_dir: P, temp_dir: P) {
    let out_path = out_dir.as_ref().join(format!("{}.mp3", name.as_ref()));
    let left_mix_path = temp_dir.as_ref().join(format!("{}_left.mp3", name.as_ref()));
    mix_audio(&left_sources.iter().map(|p| p.as_ref()).collect::<Vec<&Path>>(), left_mix_path.as_ref()); //Ugly workaround here
    let right_mix_path = temp_dir.as_ref().join(format!("{}_right.mp3", name.as_ref()));
    mix_audio(&right_sources.iter().map(|p| p.as_ref()).collect::<Vec<&Path>>(), right_mix_path.as_ref()); //Ugly workaround here
    mix_audio_panned(left_mix_path, right_mix_path, out_path);
}

pub fn generate_stemmefiler_from_midi<P: AsRef<Path>>(soundfont_path: P, midi_path: P, out_dir: P, temp_dir: P, gain: f64, left: bool) {
    let isolated_midi_paths = save_isolated_midi_tracks_from_file(midi_path.as_ref(), temp_dir.as_ref());
    // Ugly workaround again:
    generate_stemmefiler_from_isolated_midis(soundfont_path.as_ref(), &isolated_midi_paths.iter().map(|p| p.as_ref()).collect::<Vec<&Path>>(), out_dir.as_ref(), temp_dir.as_ref(), gain, left)
}

pub fn generate_stemmefiler_from_isolated_midis<P: AsRef<Path>>(soundfont_path: P, isolated_midi_paths: &[P], out_dir: P, temp_dir: P, gain: f64, left: bool) {
    let source_paths: Vec<PathBuf> = isolated_midi_paths.into_iter().map(|isolated_midi_path| {
        let source_path = isolated_midi_path.as_ref().with_extension("pcm");
        play_to_raw(soundfont_path.as_ref(), isolated_midi_path.as_ref(), &source_path, gain);
        source_path
    }).collect();
    generate_stemmefiler_from_sources(&source_paths.iter().map(|p| p.as_ref()).collect::<Vec<&Path>>(), out_dir.as_ref(), temp_dir.as_ref(), left)
}

pub fn generate_stemmefiler_from_sources<P: AsRef<Path>>(source_paths: &[P], out_dir: P, temp_dir: P, left: bool) {
    for (i, main_source) in source_paths.iter().enumerate() {
        let stemmefil_name = main_source.as_ref().file_stem().unwrap().to_str().unwrap();
        eprintln!("Creating stemmefil {}", stemmefil_name);

        let accompanying_sources: Vec<&Path> = source_paths.iter().enumerate().filter_map(|(j,p)| {
            if j != i {
                Some(p.as_ref())
            } else {
                None
            }
        }).collect();

        if left {
            mix_stemmefil(stemmefil_name, &[main_source.as_ref()], &accompanying_sources, out_dir.as_ref(), temp_dir.as_ref())
        } else {
            mix_stemmefil(stemmefil_name, &accompanying_sources, &[main_source.as_ref()], out_dir.as_ref(), temp_dir.as_ref())
        }
    }
}

pub fn custom_stemmefil_from_midi<P: AsRef<Path>>(soundfont_path: P, midi_path: P, out_dir: P, temp_dir: P, gain: f64) {
    let isolated_midi_paths = save_isolated_midi_tracks_from_file(midi_path.as_ref(), temp_dir.as_ref());
    // Ugly workaround again:
    custom_stemmefil_from_isolated_midis(soundfont_path.as_ref(), &isolated_midi_paths.iter().map(|p| p.as_ref()).collect::<Vec<&Path>>(), out_dir.as_ref(), temp_dir.as_ref(), gain)
}

pub fn custom_stemmefil_from_isolated_midis<P: AsRef<Path>>(soundfont_path: P, isolated_midi_paths: &[P], out_dir: P, temp_dir: P, gain: f64) {
    let source_paths: Vec<PathBuf> = isolated_midi_paths.into_iter().map(|isolated_midi_path| {
        let source_path = isolated_midi_path.as_ref().with_extension("pcm");
        play_to_raw(soundfont_path.as_ref(), isolated_midi_path.as_ref(), &source_path, gain);
        source_path
    }).collect();

    custom_stemmefil_from_sources(&source_paths.iter().map(|p| p.as_ref()).collect::<Vec<&Path>>(), out_dir.as_ref(), temp_dir.as_ref())
}

pub fn custom_stemmefil_from_sources<P: AsRef<Path>>(source_paths: &[P], out_dir: P, temp_dir: P) {    
    println!("Disse sporene er detektert:");
    for (i, source_path) in source_paths.iter().enumerate() {
        println!("{}: {}", i, source_path.as_ref().file_stem().unwrap().to_str().unwrap());
    }
    
    println!("Velg spor til venstre øre som en space-separert string av indekser:");
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let left_sources: Vec<&Path> = buffer.trim().split_whitespace().filter_map(|s| s.parse().ok()).map(|i: usize| source_paths[i].as_ref()).collect();

    println!("Velg spor til høyre øre som en space-separert string av indekser:");
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let right_sources: Vec<&Path> = buffer.trim().split_whitespace().filter_map(|s| s.parse().ok()).map(|i: usize| source_paths[i].as_ref()).collect();

    println!("Gi et navn til stemmefilen:");
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let name = buffer.trim();

    mix_stemmefil(name, &left_sources, &right_sources, out_dir.as_ref(), temp_dir.as_ref());
}
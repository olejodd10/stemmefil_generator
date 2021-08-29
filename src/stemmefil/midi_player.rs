use std::path::Path;

pub fn play_to_raw<P: AsRef<Path>>(sound_font_path: P, midi_path: P, out_path: P, gain: f64) {
    // TODO: Use https://crates.io/crates/fluidlite
    let _output = std::process::Command::new("fluidsynth")
        .arg("-F")
        .arg(out_path.as_ref())
        .arg("-g")
        .arg(format!("{:.1}", gain)) // :.1?
        .arg(sound_font_path.as_ref())
        .arg(midi_path.as_ref())
        .output()
        .expect("Failed to play midi file");
}

// pub fn play_to_mp3<P: AsRef<Path>>(sound_font_path: P, midi_path: P, out_path: P, gain: f64) {
//     // TODO: Use https://crates.io/crates/fluidlite
//     let _output = std::process::Command::new("fluidsynth")
//         .arg("-g")
//         .arg(format!("{:.1}", gain)) // :.1?
//         .arg(sound_font_path.as_ref())
//         .arg(midi_path.as_ref())
//         .output()
//         .expect("Failed to play midi file");
// }


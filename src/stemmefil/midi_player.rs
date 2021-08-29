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

#[allow(dead_code)]
pub fn play_to_mp3<P: AsRef<Path>>(sound_font_path: P, midi_path: P, out_path: P, gain: f64) {
    // TODO: Use https://crates.io/crates/fluidlite
    // fluidsynth -T raw -F - FluidR3_GM.sf2 sit.mid | lame -B 100 -r - test.mp3
    // https://wiki.archlinux.org/title/FluidSynth
    let play_process = std::process::Command::new("fluidsynth")
        .arg("-g")
        .arg(format!("{:.1}", gain)) // :.1?
        .arg("-T")
        .arg("raw")
        .arg("-F")
        .arg("-")
        .arg(sound_font_path.as_ref())
        .arg(midi_path.as_ref())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to start play process");
    
    let _encoding_process_output = std::process::Command::new("lame")
        .arg("-b")
        .arg("256")
        .arg("-r")
        .arg("-")
        .arg(out_path.as_ref())
        .stdin(play_process.stdout.expect("Piping failed"))
        .output()
        .expect("Failed to play midi file");
}


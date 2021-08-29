use std::path::Path;

pub fn convert_to_mp3<P: AsRef<Path>>(pcm_path: P, out_path: P) {
    // TODO: use https://crates.io/crates/lame
    // Might be possible with ffmpeg as well, if in-built??
    let _output = std::process::Command::new("lame")
        .arg(pcm_path.as_ref())
        .arg(out_path.as_ref())
        .output()
        .expect("Failed to convert pcm file");
}
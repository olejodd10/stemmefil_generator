use std::path::Path;

// https://trac.ffmpeg.org/wiki/AudioChannelManipulation

// ffmpeg -i input0.mp3 -i input1.mp3 -filter_complex amix=inputs=2:duration=longest output.mp3
// NOTE: ffmpeg handles extensions automatically, so an out path with .mp3 will be converted automatically with lame
// Beware of audio channel errors
pub fn mix_audio<P: AsRef<Path>>(in_paths: &[P], out_path: P) {
    let mut command = std::process::Command::new("ffmpeg");
    for in_path in in_paths {
        command.arg("-i").arg(in_path.as_ref());
    }
    command.arg("-filter_complex")
        .arg(format!("amix=inputs={}:duration=longest", in_paths.len()))
        .arg(out_path.as_ref())
        .output().expect("Failed to overlay audio files");
}

// ffmpeg -i input0.mp3 -i input1.mp3 -filter_complex "amerge=inputs=2,pan=stereo|c0<c0+c1|c1<c2+c3" output.mp3
// WARNING: Shortest duration is kept! Amix filter doesnt work well with panning (?)
pub fn mix_audio_panned<P: AsRef<Path>>(left_path: P, right_path: P, out_path: P) {
    std::process::Command::new("ffmpeg")
        .arg("-i")
        .arg(left_path.as_ref())
        .arg("-i")
        .arg(right_path.as_ref())
        .arg("-filter_complex")
        .arg("amerge=inputs=2,pan=stereo|c0<c0+c1|c1<c2+c3")
        .arg(out_path.as_ref())
        .output()
        .expect("Failed to overlay audio files");
}
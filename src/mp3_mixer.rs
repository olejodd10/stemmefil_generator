use std::path::Path;

// Veldig bra: https://stackoverflow.com/questions/14498539/how-to-overlay-downmix-two-audio-files-using-ffmpeg/14528482#14528482
// Funker også: https://softwarerecs.stackexchange.com/questions/25773/audio-editing-cli-application-that-can-fuse-multiple-wav-files-into-a-new-one

// ffmpeg -i input0.mp3 -i input1.mp3 -filter_complex amix=inputs=2:duration=longest output.mp3
pub fn mix_mp3s<P: AsRef<Path>>(in_paths: &[P], out_path: P) {

    let mut command = std::process::Command::new("ffmpeg");

    for in_path in in_paths {
        command.arg("-i").arg(in_path.as_ref());
    }
    command.arg("-filter_complex")
        .arg(format!("amix=inputs={}:duration=longest", in_paths.len()))
        .arg(out_path.as_ref());

    let _output = command.output().expect("Failed to overlay mp3s");
}

// ffmpeg -i input0.mp3 -i input1.mp3 -filter_complex "amerge=inputs=2,pan=stereo|c0<c0+c1|c1<c2+c3" output.mp3
// WARNING: Shortest duration is kept! Amix filter doesnt work well with panning (?)
pub fn mix_mp3s_panned<P: AsRef<Path>>(left_path: P, right_path: P, out_path: P) {
    let _output = std::process::Command::new("ffmpeg")
        .arg("-i")
        .arg(left_path.as_ref())
        .arg("-i")
        .arg(right_path.as_ref())
        .arg("-filter_complex")
        .arg("amerge=inputs=2,pan=stereo|c0<c0+c1|c1<c2+c3")
        .arg(out_path.as_ref())
        .output()
        .expect("Failed to overlay mp3s");
}
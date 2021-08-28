use std::path::Path;

// https://wiki.videolan.org/VLC_HowTo/Merge_videos_together/

// https://wiki.videolan.org/LibVLC
// https://wiki.videolan.org/Command-line_interface/
// https://wiki.videolan.org/VLC_command-line_help/

// ALT OVER ER FEIL

//DETTE ER RETT: 
// https://softwarerecs.stackexchange.com/questions/25773/audio-editing-cli-application-that-can-fuse-multiple-wav-files-into-a-new-one

// ffmpeg -i 1.mp3 -i 2.mp3 -filter_complex amerge -c:a libmp3lame -q:a 4 output.mp3

pub fn overlay_mp3s<P: AsRef<Path>>(in_paths: &[P], out_path: P) {    
    let mut command = std::process::Command::new("ffmpeg");

    for in_path in in_paths {
        command.arg("-i").arg(in_path.as_ref());
    }
    command.arg("-filter_complex")
        .arg("amerge")
        .arg("-c:a")
        .arg("libmp3lame")
        .arg("-q:a")
        .arg("4")
        .arg(out_path.as_ref());

    let _output = command.output().expect("Failed to overlay mp3s");
    
    // eprintln!("{:?}", output);
    // format!("ffmpeg {} -filter_complex amerge -c:a libmp3lame -q:a 4 {}", in_paths.join(" "), out_path.as_ref().to_str().unwrap());
    
}
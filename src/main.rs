use clap::{Arg, App};

use std::path::PathBuf;

use stemmefil_generator::{
    custom_stemmefil_from_midi, 
    generate_stemmefiler_from_midi
};

const DEFAULT_GAIN: f64 = 4.0;


fn main() {
    // Temp dir creation
    let temp_dir = std::env::temp_dir().join("stemmefil_generator");
    if !temp_dir.exists() {
        std::fs::create_dir(&temp_dir).unwrap();
    }

    // CLI definition
    let matches = App::new("Stemmefil Generator")
        .version("0.1.0")
        .about("Generate stemmefiler from MIDI file")
        .arg(Arg::with_name("out-dir")
            .short("o")
            .long("out-dir")
            .help("Directory to store stemmefiler in. Default: midi directory.")
            .takes_value(true)
            .global(true))
        .arg(Arg::with_name("soundfont")
            .short("s")
            .long("soundfont")
            .help("Path to soundfont to use. Default: $STEMMEFIL_SOUNDFONT_PATH")
            .takes_value(true)
            .global(true))
        .arg(Arg::with_name("gain")
            .short("g")
            .long("gain")
            .help("Master gain for stemmefil (0.0 to 10.0). Default: 4.0")
            .takes_value(true)
            .global(true))
        .subcommand(App::new("single")
            .about("Create stemmefil for single MIDI file")
            .arg(Arg::with_name("midi")
                .help("Path to MIDI file to create stemmefiler from")
                .required(true))
            .arg(Arg::with_name("left")
                .short("l")
                .long("left")
                .help("Whether to place focused track in left channel instead of right channel. Default: false")))
        .subcommand(App::new("bulk")
            .about("Create stemmefil for all MIDI files in directory")
            .arg(Arg::with_name("midi-dir")
                .help("Path to directory with MIDI files to create stemmefiler from")
                .required(true))
            .arg(Arg::with_name("left")
                .short("l")
                .long("left")
                .help("Whether to place focused track in left channel instead of right channel. Default: false")))
        .subcommand(App::new("custom")
            .about("Create custom stemmefil for single MIDI file")
            .arg(Arg::with_name("midi")
            .help("Path to MIDI file to create stemmefil from")
            .required(true)))
        .get_matches();
    
    // Global option handling
    let gain = matches.value_of("gain").map(|v| v.parse().unwrap()).unwrap_or(DEFAULT_GAIN);
    let soundfont_path = matches.value_of("soundfont")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(
            std::env::var("STEMMEFIL_SOUNDFONT_PATH").expect("Error: STEMMEFIL_SOUNDFONT_PATH environment variable is not set.")
        ));

    // Subcommand handling
    if let Some(matches) = matches.subcommand_matches("single") {
        let left = matches.is_present("left");
        let midi_path = PathBuf::from(matches.value_of("midi").unwrap());
        let out_dir = PathBuf::from(matches.value_of("out-dir").unwrap_or_else(|| midi_path.parent().unwrap().to_str().unwrap()));
        generate_stemmefiler_from_midi(soundfont_path.as_path(), midi_path.as_path(), out_dir.as_path(), temp_dir.as_path(), gain, left)
    } else if let Some(matches) = matches.subcommand_matches("bulk") {
        let left = matches.is_present("left");
        let midi_dir = PathBuf::from(matches.value_of("midi-dir").unwrap());
        let out_dir = PathBuf::from(matches.value_of("out-dir").unwrap_or_else(|| midi_dir.to_str().unwrap()));
        for entry in std::fs::read_dir(&midi_dir).unwrap() {
            let entry_path = entry.unwrap().path();
            if entry_path.extension().unwrap() == "mid" {
                generate_stemmefiler_from_midi(soundfont_path.as_path(), entry_path.as_path(), out_dir.as_path(), temp_dir.as_path(), gain, left)
            }
        }
    } else if let Some(matches) = matches.subcommand_matches("custom") {
        let midi_path = PathBuf::from(matches.value_of("midi").unwrap());
        let out_dir = PathBuf::from(matches.value_of("out-dir").unwrap_or_else(|| midi_path.parent().unwrap().to_str().unwrap()));
        custom_stemmefil_from_midi(soundfont_path.as_path(), midi_path.as_path(), out_dir.as_path(), temp_dir.as_path(), gain)
    } else {
        panic!("Subcommand must be specified");
    }

    // Cleanup
    std::fs::remove_dir_all(temp_dir).unwrap();
}

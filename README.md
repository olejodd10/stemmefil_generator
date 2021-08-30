# Stemmefil Generator

A library and command line tool to assist with creating stemmefiler.

## Setup

This tool requires that `ffmpeg` and `fluidsynth` are installed. Downloads can be found [here](https://www.ffmpeg.org/download.html#build-linux) and [here](https://github.com/FluidSynth/fluidsynth/wiki/Download).

The `STEMMEFIL_SOUNDFONT_PATH` variable can be set to simplify usage. Otherwise, soundfont path must be specified with `-s` option.

In Ubuntu, the whole setup can be executed like so:

    $ sudo apt install ffmpeg
    $ sudo apt install fluidsynth
    $ export STEMMEFIL_SOUNDFONT_PATH=/path/to/sf2/file

## Example usage

    $ stemmefil single --gain 5.0 --left ./darude_sandstorm.mid 
    $ stemmefil bulk ./directories_with_several_midi_files/concert_fall_2021
    $ stemmefil custom ./darude_sandstorm.mid
    $ stemmefil custom --soundfont ./another_soundfont.sf2 --out-dir ./another/directory ./darude_sandstorm.mid
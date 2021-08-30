//Trims null character from end of string
fn trim_null<S: AsRef<str>>(s: S) -> String {
    String::from(s.as_ref().trim_end_matches('\0'))
}


// For debugging
#[allow(dead_code)]
pub fn print_track_meta(track: &midly::Track) {
    println!("Examining track {:?}", extract_track_name(track));
    for track_event in track {
        if let midly::TrackEventKind::Meta(meta_message) = track_event.kind {
            println!("{:?}", meta_message);
        }
    }
}


// Extracts track name
pub fn extract_track_name(track: &midly::Track) -> Option<String> {
    for track_event in track.iter() {
        if let midly::TrackEventKind::Meta(midly::MetaMessage::TrackName(data)) = track_event.kind {
            return String::from_utf8(data.to_vec()).ok().map(trim_null);
        }
    }
    None
}

// Extracts instrument name
#[allow(dead_code)]
pub fn extract_instrument_name(track: &midly::Track) -> Option<String> {
    for track_event in track.iter() {
        if let midly::TrackEventKind::Meta(midly::MetaMessage::InstrumentName(data)) = track_event.kind {
            return Some(trim_null(String::from_utf8(data.to_vec()).unwrap()));
        }
    }
    None
}


//Changes all NoteOn event velocities to initial_value*change
#[allow(dead_code)]
pub fn relative_track_velocity_change(track: &mut midly::Track, change: f64) {
    for track_event in track.iter_mut() {
        if let midly::TrackEventKind::Midi{channel: _, message: midly::MidiMessage::NoteOn{key:_, ref mut vel}} = track_event.kind {
            let new_velocity = midly::num::u7::new(((vel.as_int() as f64) * change) as u8);
            *vel = new_velocity;
        }
    }
}

pub fn dampen_track(track: &mut midly::Track) {
    for track_event in track.iter_mut() {
        if let midly::TrackEventKind::Midi{channel: _, message: midly::MidiMessage::NoteOn{key:_, ref mut vel}} = track_event.kind {
            *vel = midly::num::u7::new(0); // "Note that by convention a NoteOn message with a velocity of 0 is equivalent to a NoteOff."
        }
    }
}
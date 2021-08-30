#[cfg(test)]
mod tests;

mod stemmefil;
pub use stemmefil::{
    generate_stemmefiler_from_midi,
    generate_stemmefiler_from_isolated_midis,
    generate_stemmefiler_from_sources,
    custom_stemmefil_from_midi,
    custom_stemmefil_from_isolated_midis,
    custom_stemmefil_from_sources
};


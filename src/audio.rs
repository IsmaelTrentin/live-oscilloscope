use portaudio::{stream::Stream, PaError};
use portaudio_rs as portaudio;

pub const FRAMES: u32 = 1024;
pub type Sample = f32;

pub fn get_input_stream() -> Result<Stream<'static, f32, f32>, PaError> {
    let stream: Result<Stream<'_, Sample, _>, PaError> = Stream::open_default(
        1,       // input channels
        0,       // output channels
        44100.0, // sample rate
        FRAMES as u64,
        None, // no callback
    );

    stream
}
pub fn get_output_stream() -> Result<Stream<'static, f32, f32>, PaError> {
    let stream: Result<Stream<'_, Sample, _>, PaError> = Stream::open_default(
        0,      // input channels
        2,      // output channels
        48000., // sample rate
        FRAMES as u64,
        None, // no callback
    );

    stream
}

use portaudio::{stream::Stream, PaError};
use portaudio_rs as portaudio;

pub type Sample = f32;

pub fn get_input_stream() -> Result<Stream<'static, f32, f32>, PaError> {
    let stream: Result<Stream<'_, Sample, _>, PaError> = Stream::open_default(
        1,       // input channels
        0,       // output channels
        44100.0, // sample rate
        portaudio::stream::FRAMES_PER_BUFFER_UNSPECIFIED,
        None, // no callback
    );

    stream
}

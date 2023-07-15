use crate::audio::FRAMES;
use crate::audio::*;
use crate::State;
use byteorder::{LittleEndian, ReadBytesExt};
use spectrum_analyzer::scaling::divide_by_N_sqrt;
use spectrum_analyzer::windows::hann_window;
use spectrum_analyzer::{samples_fft_to_spectrum, FrequencyLimit};
use std::sync::{Arc, Mutex};

pub fn update_ui_samples(state_clone: Arc<Mutex<State>>, input_file: Option<String>) {
    // let input_file = Option::Some("testaudio/horn.v..raw");
    // out thread
    if input_file.is_some() {
        // let ts = SystemTime::now();
        std::thread::spawn(move || {
            println!("reading file...");
            let file_bytes = match input_file {
                Some(path) => std::fs::read(path).unwrap(),
                None => Vec::with_capacity(0),
            };
            println!("reading file done");
            let mut sliced = file_bytes.as_slice();
            let out_stream = get_output_stream().unwrap();

            out_stream.start().unwrap();

            loop {
                let mut data = Vec::with_capacity(FRAMES as usize);

                for _ in 0..FRAMES as usize {
                    data.push(sliced.read_f32::<LittleEndian>().unwrap());
                }

                out_stream.write(&data).unwrap();

                let hann_window = hann_window(&data);
                // fft
                // calc spectrum
                let spectrum_hann_window = samples_fft_to_spectrum(
                    // (windowed) samples
                    &hann_window,
                    // sampling rate
                    44100,
                    // optional frequency limit: e.g. only interested in frequencies 50 <= f <= 150?
                    FrequencyLimit::All,
                    // optional scale
                    Some(&divide_by_N_sqrt),
                )
                .unwrap();

                // for (fr, fr_val) in spectrum_hann_window.data().iter() {
                //     println!("{}Hz => {}", fr, fr_val)
                // }
                state_clone.lock().unwrap().fft_samples = spectrum_hann_window
                    .data()
                    .iter()
                    .map(|v| v.1.val())
                    .collect();

                state_clone.lock().unwrap().current_samples = data;
            }
        });
    } else {
        let stream = get_input_stream().expect("could not open audio stream");
        stream.start().expect("failed to start audio stream");

        loop {
            let data = match stream.read(FRAMES as u32) {
                Ok(stream_data) => stream_data,
                Err(err) => {
                    eprintln!("{err}");
                    Vec::with_capacity(FRAMES as usize)
                }
            };
            // let mut data = Vec::with_capacity(FRAMES as usize);
            // let freq = ts.elapsed().unwrap().as_secs_f32() * 10.;

            // println!("freq: {}", freq);

            // for n in 0..FRAMES {
            //     data.push((freq * std::f32::consts::PI * n as f32 / FRAMES as f32).sin())
            // }

            let hann_window = hann_window(&data);

            state_clone.lock().unwrap().current_samples = data;

            // fft
            // calc spectrum
            let spectrum_hann_window = samples_fft_to_spectrum(
                // (windowed) samples
                &hann_window,
                // sampling rate
                44100,
                // optional frequency limit: e.g. only interested in frequencies 50 <= f <= 150?
                FrequencyLimit::All,
                // optional scale
                Some(&divide_by_N_sqrt),
            )
            .unwrap();

            // for (fr, fr_val) in spectrum_hann_window.data().iter() {
            //     println!("{}Hz => {}", fr, fr_val)
            // }
            state_clone.lock().unwrap().fft_samples = spectrum_hann_window
                .data()
                .iter()
                .map(|v| v.1.val())
                .collect();
        }
        // println!("done ui thread in: {}ms", t1.elapsed().unwrap().as_millis());
    }
}

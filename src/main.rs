use clap::*;
use components::App::*;
use eframe::egui;
use portaudio_rs as portaudio;

mod audio;
mod components;
mod ui;
mod utils;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short = 'D', long)]
    print_devices: bool,

    #[arg(short = 'f', long)]
    input_file: Option<String>,
}

fn main() -> Result<(), eframe::Error> {
    let args = Args::parse();

    if let Some(input_file) = args.input_file.clone() {
        match std::fs::metadata(&input_file) {
            Ok(_) => (),
            Err(err) => {
                eprintln!("{}", err);
                panic!();
            }
        };
    }

    portaudio::initialize().unwrap();
    println!("initialized portaudio");

    let num_of_devices = portaudio::device::get_count().unwrap();
    if args.print_devices {
        println!("available devices: {}", num_of_devices);
        for i in 0..num_of_devices {
            let device_info = portaudio::device::get_info(i);

            match device_info {
                Some(info) => println!("{}: {}", i, info.name),
                None => println!("could not get info for {}", i),
            }
        }

        return Ok(());
    }

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0 * 3., 240.0 * 3.)),
        centered: true,
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| Box::new(MyApp::new(cc, args))),
    )
}

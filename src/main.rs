use clap::*;
use eframe::egui::{containers::*, *};
use eframe::{egui, emath};
use portaudio_rs as portaudio;
use std::sync::{Arc, Mutex};
use std::thread;
use ui::*;
use utils::*;

mod audio;
mod ui;
mod utils;

pub struct State {
    current_samples: Vec<f32>,
}

impl State {
    pub fn new() -> Self {
        Self {
            current_samples: Vec::with_capacity(44100),
        }
    }
}

struct MyApp {
    state: Arc<Mutex<State>>,
}

impl MyApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let state = Arc::new(Mutex::new(State::new()));
        let state_clone = state.clone();

        thread::spawn(move || {
            // blocking
            update_ui_samples(state_clone);
        });

        Self { state }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            Frame::canvas(ui.style()).show(ui, |ui| {
                ctx.request_repaint();

                let samples = &self.state.lock().unwrap().current_samples;

                if samples.len() == 0 {
                    ui.label("no data");
                    return;
                }

                let color = if ui.visuals().dark_mode {
                    Color32::from_additive_luminance(196)
                } else {
                    Color32::from_black_alpha(240)
                };

                let desired_size = ui.available_width() * vec2(1.0, 0.35);

                let (_id, rect) = ui.allocate_space(desired_size);

                let to_screen = emath::RectTransform::from_to(
                    Rect::from_x_y_ranges(0.0..=1.0, -1.0..=1.0),
                    rect,
                );

                let mut shapes = vec![];
                let points_amnt = ui.available_width() as usize;
                let points: Vec<Pos2> = (0..=points_amnt)
                    .map(|i| {
                        let mapped_i = map_range(
                            (0., points_amnt as f64),
                            (0., samples.len() as f64 - 1.),
                            i as f64,
                        ) as usize;
                        let amp = samples[mapped_i].to_owned();
                        let x = i as f32 / points_amnt as f32;
                        let y = map_range((-1., 1.), (1., 0.), amp as f64);
                        to_screen * pos2(x as f32, y as f32 - 0.5)
                    })
                    .collect();

                let thickness = 1.;
                shapes.push(epaint::Shape::line(points, Stroke::new(thickness, color)));

                ui.painter().extend(shapes);
            });
        });
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short = 'D', long)]
    print_devices: bool,
}

fn main() -> Result<(), eframe::Error> {
    let args = Args::parse();

    portaudio::initialize().unwrap();

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
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        centered: true,
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| Box::new(MyApp::new(cc))),
    )
}

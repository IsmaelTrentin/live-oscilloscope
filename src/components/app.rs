use crate::audio::FRAMES;
use crate::ui::*;
use crate::utils::*;
use crate::Args;
use eframe::egui::{containers::*, *};
use eframe::{egui, emath};
use std::sync::{Arc, Mutex};
use std::thread;

pub struct State {
    pub current_samples: Vec<f32>,
    pub fft_samples: Vec<f32>,
    pub input_file: Option<String>,
}

impl State {
    pub fn new(input_file: Option<String>) -> Self {
        Self {
            current_samples: Vec::with_capacity(44100),
            fft_samples: Vec::with_capacity(FRAMES as usize),
            input_file,
        }
    }
}

pub struct MyApp {
    state: Arc<Mutex<State>>,
}

impl MyApp {
    pub fn new(_cc: &eframe::CreationContext<'_>, args: Args) -> Self {
        let args_clone = args.clone();
        let state = Arc::new(Mutex::new(State::new(args_clone.input_file)));
        let state_clone = state.clone();

        thread::spawn(move || {
            // blocking
            update_ui_samples(state_clone, args.input_file);
        });

        Self { state }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // oscilloscope
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
                        let y = amp.clamp(-1., 1.); //map_range((-1., 1.), (0., 1.), amp as f64);
                        to_screen * pos2(x as f32, y as f32)
                    })
                    .collect();

                let thickness = 1.;
                shapes.push(epaint::Shape::line(points, Stroke::new(thickness, color)));

                ui.painter().extend(shapes);
            });

            // spectrum
            Frame::canvas(ui.style()).show(ui, |ui| {
                ctx.request_repaint();

                let samples = &self.state.lock().unwrap().fft_samples;

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
                        let x = map_range((0., points_amnt as f64), (0., 1.), i as f64);
                        let y = amp * -1. + 1.; //map_range((-1., 1.), (0., 1.), amp as f64);
                        to_screen * pos2(x as f32, y)
                    })
                    .collect();
                // let points: Vec<Pos2> = vec![to_screen * pos2(0., 1.), to_screen * pos2(1., 1.)];

                let thickness = 1.;
                shapes.push(epaint::Shape::line(points, Stroke::new(thickness, color)));

                ui.painter().extend(shapes);
            });
        });
    }
}

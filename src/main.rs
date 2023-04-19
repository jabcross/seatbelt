#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}

struct MyApp {
    name: String,
    age: u32,
    text: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
            text: "default_text".to_owned(),
        }
    }
}

impl MyApp {
    fn panels(&mut self, ctx: &egui::Context) {
        let code = &mut self.text;
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.columns(2, |columns| {
                columns[0].heading("Column 1");
                columns[0].add(egui::TextEdit::multiline(code).desired_width(f32::INFINITY));
                columns[1].heading("Column 2");
            })
        });
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.panels(ctx);
        // egui::CentralPanel::default().show(ctx, |ui| {
        //     ui.heading("My egui Application");
        //     ui.horizontal(|ui| {
        //         let name_label = ui.label("Your name: ");
        //         ui.text_edit_singleline(&mut self.name)
        //             .labelled_by(name_label.id);
        //     });
        //     ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
        //     if ui.button("Click each year").clicked() {
        //         self.age += 1;
        //     }
        //     ui.label(format!("Hello '{}', age {}", self.name, self.age));
        // });
    }
}

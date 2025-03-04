use eframe::{egui, NativeOptions};
use rand::Rng;

const SYMBOLS: [&str; 5] = ["a", "b", "c", "d", "e"];

struct SlotsMachine {
    spin_result: Vec<String>,
    message: String,
}

impl Default for SlotsMachine {
    fn default() -> Self {
        Self {
            spin_result: vec!["?".to_string(), "?".to_string(), "?".to_string()],
            message: "Press Spin to play".to_string(),
        }
    }
}

impl SlotsMachine {
    fn spin(&mut self) {
        let mut rng = rand::thread_rng();
        self.spin_result = (0..3)
            .map(|_| SYMBOLS[rng.gen_range(0..SYMBOLS.len())].to_string())
            .collect();
        if self.spin_result[0] == self.spin_result[1] && self.spin_result[1] == self.spin_result[2] {
            self.message = "Winner".to_string();
        } else {
            self.message = "Try again".to_string ()
        }
    }
}

impl eframe::App for SlotsMachine {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Slots machine");

            ui.horizontal(|ui| {
                for symbol in &self.spin_result {
                    ui.label(format!("| {} |", symbol));
                }
            });

            if ui.button("Spin").clicked() {
                self.spin();
            }

            ui.label(&self.message)
        });
    }
}

fn main() {
    let options = NativeOptions::default();
    eframe::run_native(
        "Slots machine",
        options,
        Box::new(|_cc| Box::new(SlotsMachine::default())),
    );
}
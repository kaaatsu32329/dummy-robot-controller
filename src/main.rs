use eframe::egui;

fn main() {
    tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Dummy Robot Controller",
        options,
        Box::new(|_cc| Box::<MoveBaseGUI>::default()),
    )
}

struct MoveBaseGUI {
    x: f64,
    y: f64,
    theta: f64,
}

impl Default for MoveBaseGUI {
    fn default() -> Self {
        Self {
            x: 0f64,
            y: 0f64,
            theta: 0f64,
        }
    }
}

impl eframe::App for MoveBaseGUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Dummy Robot Controller");

            ui.horizontal(|ui| {
                if ui.button("Down").clicked() {
                    self.x -= 0.1;
                    dummy_send_velocity_task()
                }
                if ui.button("Up").clicked() {
                    self.x += 0.1;
                    dummy_send_velocity_task()
                }
            });
            ui.add(egui::Slider::new(&mut self.x, -1f64..=1f64).text("X"));

            ui.horizontal(|ui| {
                if ui.button("Left").clicked() {
                    self.y -= 0.1;
                    dummy_send_velocity_task()
                }
                if ui.button("Right").clicked() {
                    self.y += 0.1;
                    dummy_send_velocity_task()
                }
            });
            ui.add(egui::Slider::new(&mut self.y, -1f64..=1f64).text("Y"));

            ui.horizontal(|ui| {
                if ui.button("CCW").clicked() {
                    self.theta -= 0.1;
                    dummy_send_velocity_task()
                }
                if ui.button("CW").clicked() {
                    self.theta += 0.1;
                    dummy_send_velocity_task()
                }
            });
            ui.add(egui::Slider::new(&mut self.theta, -1f64..=1f64).text("THETA"));
        });
    }
}

fn dummy_send_velocity_task() {}

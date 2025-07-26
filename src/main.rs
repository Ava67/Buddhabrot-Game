fn main() -> eframe::Result {
    println!("Hello, world!");

    // Our application state:
    let mut name = "Ava".to_owned();
    let mut age = 42;
    let inner_width = 320;
    let inner_height = 240;
    let inner_dims = [inner_width as f32, inner_height as f32];
    let num_pixels: usize = inner_width * inner_height;
    let mut data = std::sync::Arc::new(vec![0_u8; num_pixels]);

    let pixels: Vec<egui::Color32> = vec![egui::Color32::BROWN; num_pixels];

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size(inner_dims),
        ..Default::default()
    };

    eframe::run_simple_native("My eGUI app", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My application");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut age, 0..=120).text("age"));
            if ui.button("Increment").clicked() {
                age += 1;
            }
            ui.label(format!("Hello {name}, age {age}"));

            // ui.add(egui::Image::new(data))
            // 	.corner_radius(5)
            // 	.tint(egui::Color32::LIGHT_BLUE)
            // 	.paint_at(ui, [0.0, 0.0]);
        });
    })
}

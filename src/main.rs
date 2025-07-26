
fn main() -> eframe::Result {
    println!("Hello, world!");

	// Our application state:
	let mut name = "Ava".to_owned();
	let mut age = 42;
	let inner_width = 320.0;
	let inner_height = 240.0;

	let options = eframe::NativeOptions{
		viewport: egui::ViewportBuilder::default()
    		.with_inner_size([inner_width, inner_height]),
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
			ui.add(
    			egui::Slider::new(&mut age, 0..=120)
        			.text("age"));
			if ui.button("Increment").clicked() {
    			age += 1;
			}
			ui.label(format!("Hello {name}, age {age}"));
		});
	})
}

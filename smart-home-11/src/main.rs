mod common;
mod smart_outlet;

use common::{Report, SwitchStatusEnum};
use smart_outlet::SmartOutlet;
use std::sync::Arc;

fn main() {
    let outlet = Arc::new(SmartOutlet::new("Fridge", "127.0.0.1:20001"));
    outlet.set_switch(SwitchStatusEnum::On);
    let outlet_ui = Arc::clone(&outlet);

    outlet.runner();
    eframe::run_native(
        "Smart Home UI",
        eframe::NativeOptions::default(),
        Box::new(move |_| Box::new(App::new(outlet_ui))),
    );
}

struct App {
    outlet: Arc<SmartOutlet>,
}

impl App {
    fn new(outlet: Arc<SmartOutlet>) -> Self {
        Self { outlet }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut outlet_name: String = self.outlet.get_name();
            let mut outlet_switch: bool = self.outlet.get_switch().into();

            ui.heading("Smart Outlet");
            ui.horizontal(|ui| {
                ui.label("Outlet name: ");
                ui.text_edit_singleline(&mut outlet_name);
            });
            ui.horizontal(|ui| {
                let outlet_switch_enum: SwitchStatusEnum = outlet_switch.into();
                ui.checkbox(&mut outlet_switch, format!(" {}", outlet_switch_enum));
            });
            ui.label(self.outlet.report_to_string());

            if self.outlet.get_name() != outlet_name {
                self.outlet.set_name(&outlet_name);
            }
            let outlet_switch_enum: SwitchStatusEnum = outlet_switch.into();
            if self.outlet.get_switch() != outlet_switch_enum {
                self.outlet.set_switch(outlet_switch_enum);
            }
        });
    }
}

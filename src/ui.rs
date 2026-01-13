use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPlugin, EguiPrimaryContextPass, egui};

use super::GlobalState;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin::default());
        app.add_systems(EguiPrimaryContextPass, ui_example_system);
    }
}

fn ui_example_system(mut contexts: EguiContexts, mut global_state: ResMut<GlobalState>) -> Result {
    egui::Window::new("Settings").show(contexts.ctx_mut()?, |ui| {
        ui.label("world");
        ui.add(egui::Slider::new(&mut global_state.month, 1..=12).text("month"));
        ui.checkbox(&mut global_state.cycle_month, "cycle months");
        ui.checkbox(&mut global_state.wireframe, "wireframe");
    });
    Ok(())
}

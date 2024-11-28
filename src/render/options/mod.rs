mod general_tab;
use crate::addon::Addon;
use nexus::imgui::Ui;

const ERROR_COLOR: [f32; 4] = [0.4, 0.4, 0.4, 1.0];
const SUCCESS_COLOR: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

impl Addon {
    pub fn render_options(&mut self, ui: &Ui) {
        self.render_general_tab(ui);
    }
}

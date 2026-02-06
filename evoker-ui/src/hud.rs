//! In-game HUD

/// Heads-up display
pub struct Hud {
    show_debug: bool,
}

impl Hud {
    /// Create a new HUD
    pub fn new() -> Self {
        Self {
            show_debug: false,
        }
    }
    
    /// Render HUD
    pub fn render(&mut self, ctx: &egui::Context) {
        // Render crosshair
        self.render_crosshair(ctx);
        
        // Render hotbar
        self.render_hotbar(ctx);
        
        // Render debug info if enabled
        if self.show_debug {
            self.render_debug(ctx);
        }
    }
    
    fn render_crosshair(&self, ctx: &egui::Context) {
        egui::Area::new("crosshair".into())
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.label("+");
            });
    }
    
    fn render_hotbar(&self, ctx: &egui::Context) {
        egui::Area::new("hotbar".into())
            .anchor(egui::Align2::CENTER_BOTTOM, [0.0, -10.0])
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    for i in 1..=9 {
                        ui.label(format!("[{}]", i));
                    }
                });
            });
    }
    
    fn render_debug(&self, ctx: &egui::Context) {
        egui::Window::new("Debug Info")
            .default_pos([10.0, 10.0])
            .show(ctx, |ui| {
                ui.label("EvokerMC Debug");
                ui.separator();
                ui.label("FPS: 60");
                ui.label("Chunks loaded: 0");
            });
    }
    
    /// Toggle debug info
    pub fn toggle_debug(&mut self) {
        self.show_debug = !self.show_debug;
    }
}

impl Default for Hud {
    fn default() -> Self {
        Self::new()
    }
}

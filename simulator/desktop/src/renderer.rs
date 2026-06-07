use tracing::info;

/// Stub renderer for CodeSim v0.1 — receives compositor frames and drives the device screen.
pub struct SimulatorRenderer {
    device_width: u32,
    device_height: u32,
}

impl SimulatorRenderer {
    pub fn new(device_width: u32, device_height: u32) -> Self {
        Self {
            device_width,
            device_height,
        }
    }

    /// Present a decoded surface frame (base64 pixel buffer stub).
    pub fn present_frame(&self, surface_id: &str, pixels_base64: &str) {
        info!(
            surface_id,
            frame_bytes = pixels_base64.len(),
            "SimulatorRenderer::present_frame (stub v0.1)"
        );
    }

    /// Placeholder for full device-screen redraw (status bar + app surface).
    pub fn render_device_screen(&self) {
        info!(
            width = self.device_width,
            height = self.device_height,
            "SimulatorRenderer::render_device_screen (placeholder)"
        );
    }
}

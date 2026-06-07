/// First-class CodeOS application lifecycle hooks.
///
/// Maps to hooks declared in `codeos_manifest.toml` and invoked by codesvc.app.
pub trait CodeApp {
    fn on_create(&mut self);
    fn on_start(&mut self);
    fn on_resume(&mut self);
    fn on_pause(&mut self);
    fn on_stop(&mut self);
    fn on_destroy(&mut self);
}

/// Blanket helper for apps that only need start/stop in v0.1.
pub struct NoopApp;

impl CodeApp for NoopApp {
    fn on_create(&mut self) {}
    fn on_start(&mut self) {}
    fn on_resume(&mut self) {}
    fn on_pause(&mut self) {}
    fn on_stop(&mut self) {}
    fn on_destroy(&mut self) {}
}

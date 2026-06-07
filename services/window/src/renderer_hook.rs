use std::sync::{Arc, Mutex};

pub trait FrameSink: Send + Sync {
    fn submit_frame(
        &self,
        surface_id: &str,
        app_id: &str,
        width: u32,
        height: u32,
        frame_data: &str,
    );
}

pub struct LogFrameSink;

impl FrameSink for LogFrameSink {
    fn submit_frame(
        &self,
        surface_id: &str,
        app_id: &str,
        width: u32,
        height: u32,
        frame_data: &str,
    ) {
        println!(
            "[codesvc.window] SubmitFrame surface_id={} app_id={} size={}x{} (frame size={} bytes base64)",
            surface_id,
            app_id,
            width,
            height,
            frame_data.len()
        );
    }
}

/// Forwards frames to a user-supplied callback (simulator / test harness).
pub struct CallbackFrameSink {
    callback: Box<dyn Fn(&str, &str, u32, u32, &str) + Send + Sync>,
}

impl CallbackFrameSink {
    pub fn new<F>(callback: F) -> Self
    where
        F: Fn(&str, &str, u32, u32, &str) + Send + Sync + 'static,
    {
        Self {
            callback: Box::new(callback),
        }
    }
}

impl FrameSink for CallbackFrameSink {
    fn submit_frame(
        &self,
        surface_id: &str,
        app_id: &str,
        width: u32,
        height: u32,
        frame_data: &str,
    ) {
        (self.callback)(surface_id, app_id, width, height, frame_data);
    }
}

static FRAME_SINK: Mutex<Option<Arc<dyn FrameSink>>> = Mutex::new(None);

pub fn set_frame_sink(sink: Arc<dyn FrameSink>) {
    *FRAME_SINK
        .lock()
        .expect("Failed to lock frame sink") = Some(sink);
}

pub(crate) fn get_frame_sink() -> Arc<dyn FrameSink> {
    FRAME_SINK
        .lock()
        .expect("Failed to lock frame sink")
        .clone()
        .unwrap_or_else(|| Arc::new(LogFrameSink))
}

#[cfg(test)]
pub fn reset_frame_sink() {
    *FRAME_SINK
        .lock()
        .expect("Failed to lock frame sink") = None;
}

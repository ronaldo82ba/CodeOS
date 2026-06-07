use std::collections::HashMap;

use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Surface {
    pub app_id: String,
    pub surface_id: String,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Default)]
pub struct WindowState {
    surfaces: HashMap<String, Surface>, // key: surface_id
}

impl WindowState {
    pub fn create_surface(&mut self, app_id: String, width: u32, height: u32) -> String {
        let surface_id = format!("surface-{}", Uuid::new_v4());
        let surface = Surface {
            app_id,
            surface_id: surface_id.clone(),
            width,
            height,
        };
        self.surfaces.insert(surface_id.clone(), surface);
        surface_id
    }

    pub fn destroy_surface(&mut self, surface_id: &str) {
        self.surfaces.remove(surface_id);
    }

    pub fn get_surface(&self, surface_id: &str) -> Option<&Surface> {
        self.surfaces.get(surface_id)
    }
}

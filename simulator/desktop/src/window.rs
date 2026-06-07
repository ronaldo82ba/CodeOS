/// Simulated device display — represents the phone screen hosted by CodeSim.
pub struct DeviceWindow {
    width: u32,
    height: u32,
}

impl DeviceWindow {
    pub const DEFAULT_WIDTH: u32 = 1080;
    pub const DEFAULT_HEIGHT: u32 = 1920;

    pub fn phone_default() -> Self {
        Self {
            width: Self::DEFAULT_WIDTH,
            height: Self::DEFAULT_HEIGHT,
        }
    }

    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}

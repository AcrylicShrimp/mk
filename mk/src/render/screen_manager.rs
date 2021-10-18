use glutin::dpi::PhysicalSize;

#[derive(Debug)]
pub struct ScreenManager {
    width: f64,
    height: f64,
    scale_factor: f64,
}

impl ScreenManager {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width: width as _,
            height: height as _,
            scale_factor: 1f64,
        }
    }

    pub fn width(&self) -> f64 {
        self.width
    }

    pub fn height(&self) -> f64 {
        self.height
    }

    pub fn scale_factor(&self) -> f64 {
        self.scale_factor
    }

    pub fn physical_width(&self) -> f64 {
        self.width * self.scale_factor
    }

    pub fn physical_height(&self) -> f64 {
        self.height * self.scale_factor
    }

    pub fn update_size(&mut self, inner_size: &PhysicalSize<u32>) {
        let size = inner_size.to_logical(self.scale_factor);
        self.width = size.width;
        self.height = size.height;
    }

    pub fn update_scale_factor(&mut self, scale_factor: f64, inner_size: &PhysicalSize<u32>) {
        let size = inner_size.to_logical(scale_factor);
        self.width = size.width;
        self.height = size.height;
        self.scale_factor = scale_factor;
    }
}

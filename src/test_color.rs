pub trait CursorDependColor {
    fn update(&mut self, x: f64, y: f64, width: u32, height: u32);
}

impl CursorDependColor for wgpu::Color {
    fn update(&mut self, x: f64, y: f64, width: u32, height: u32) {
        (self.r, self.g, self.b, self.a) = (
            x / width as f64, 
            y / height as f64, 
            (x + y) / (width + height) as f64, 
            1.0
        );
    }
}
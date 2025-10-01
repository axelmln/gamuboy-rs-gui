use gamuboy::lcd::{self, FrameBuffer, LCD};
use sdl2::{
    render::{Canvas, Texture},
    video::Window,
};

pub struct Gui {
    canvas: Canvas<Window>,
    texture: Texture<'static>,
    pixel_buffer: Vec<u8>,
}

impl Gui {
    pub fn new(canvas: Canvas<Window>, texture: Texture<'static>) -> Self {
        Self {
            canvas,
            texture,
            pixel_buffer: vec![0u8; lcd::PIXELS_WIDTH * lcd::PIXELS_HEIGHT * 3],
        }
    }

    fn fill_pixel_buffer(&mut self, matrix: &FrameBuffer) {
        for (y, line) in matrix.iter().enumerate() {
            for (x, pixel) in line.iter().enumerate() {
                let i = (y * lcd::PIXELS_WIDTH + x) * 3;
                self.pixel_buffer[i] = pixel.0;
                self.pixel_buffer[i + 1] = pixel.1;
                self.pixel_buffer[i + 2] = pixel.2;
            }
        }
    }
}

impl LCD for Gui {
    fn draw_buffer(&mut self, matrix: &FrameBuffer) {
        self.fill_pixel_buffer(matrix);

        self.texture
            .update(None, &self.pixel_buffer, lcd::PIXELS_WIDTH * 3)
            .unwrap();

        self.canvas.clear();
        self.canvas.copy(&self.texture, None, None).unwrap();
        self.canvas.present();
    }
}

use crate::database as db;
use qrcode::{QrCode, render::{Pixel, Canvas, Renderer}, types::Color, EcLevel};

#[derive(Clone, Copy)]
struct ImagePixel(u8);

impl Pixel for ImagePixel {
    type Image = <ImageCanvas as Canvas>::Image;
    type Canvas = ImageCanvas;

    fn default_unit_size() -> (u32, u32) {
        (1, 1)
    }

    fn default_color(color: Color) -> Self {
        match color {
            Color::Light => Self(255),
            Color::Dark => Self(0)
        }
    }
}

struct ImageCanvas {
    image_buffer: Vec<u8>,
    pitch: usize,
    width: u32,
    height: u32,
    dark_pixel: ImagePixel,
    light_pixel: ImagePixel,
}

impl Canvas for ImageCanvas {
    type Pixel = ImagePixel;
    type Image = Result<Vec<u8>, png::EncodingError>;

    fn new(width: u32, height: u32, dark_pixel: Self::Pixel, light_pixel: Self::Pixel) -> Self {
        let pitch = ((width + 7) / 8) as usize;
        Self {
            image_buffer: vec![0; pitch * height as usize],
            pitch,
            width,
            height,
            dark_pixel,
            light_pixel,
        }
    }

    fn draw_dark_pixel(&mut self, x: u32, y: u32) {
        let x = x as usize;
        let y = y as usize;
        let pixel_index = y * self.pitch + x / 8;
        let bit_index = 7 - x % 8;
        self.image_buffer[pixel_index] |= 1 << bit_index;
    }

    fn into_image(self) -> Self::Image {
        let mut bytes = Vec::new();
        {
            let mut encoder = png::Encoder::new(&mut bytes, self.width, self.height);
            encoder.set_color(png::ColorType::Indexed);
            encoder.set_depth(png::BitDepth::One);
            encoder.set_palette(vec![
                self.light_pixel.0, self.light_pixel.0, self.light_pixel.0,
                self.dark_pixel.0, self.dark_pixel.0, self.dark_pixel.0,
            ]);
            let mut writer = encoder.write_header()?;
            writer.write_image_data(&self.image_buffer)?;
        }
        Ok(bytes)
    }
}

pub async fn get_qr(kind: char, poll_id: db::PollID) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    if (kind != 'n' && kind != 'c') || poll_id.len() != db::POLL_ID_LENGTH {
        return Ok(Box::new(warp::http::StatusCode::NOT_FOUND));
    }

    let data = format!("http://indi-mac.local/respond/{}/{}", kind, poll_id);
    let code = try_500!(QrCode::with_error_correction_level(data, EcLevel::L));

    let width = code.width();
    let colors = code.into_colors();
    let renderer = Renderer::<ImagePixel>::new(&colors, width, 1);
    let image = try_500!(renderer.build());

    Ok(Box::new(warp::reply::with_header(
        image,
        "Content-Type",
        "image/png"
    )))
}

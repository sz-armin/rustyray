use super::*;

#[derive(Builder, Debug, Clone)]
#[builder(build_fn(skip))]
pub struct Canvas {
    pub width: u32,
    pub height: u32,

    #[builder(setter(skip))]
    pub aspect_ratio: f64,
    #[builder(setter(skip))]
    pub buffer: Array3<f64>,
}

impl Canvas {
    pub fn save(&self, path: &str) -> Result<(), ImageError> {
        let tmapped_raw = self
            .buffer
            .as_standard_layout()
            .mapv(|x| (x * 255.0) as u8)
            .into_raw_vec();
        let file = std::fs::File::create(path).unwrap();
        let encoder = PngEncoder::new(file);
        encoder.encode(
            tmapped_raw.as_bytes(),
            self.width,
            self.height,
            image::ColorType::Rgb8,
        )?;
        Ok(())
    }
}

impl CanvasBuilder {
    pub fn build(&self) -> Result<Canvas, CanvasBuilderError> {
        let width = match self.width {
            Some(value) => value,
            None => 960,
        };
        let height = match self.height {
            Some(value) => value,
            None => 540,
        };

        let aspect_ratio = width as f64 / height as f64;
        let buffer = Array3::zeros((height as usize, width as usize, 3));

        Ok(Canvas {
            width,
            height,
            aspect_ratio,
            buffer,
        })
    }
}

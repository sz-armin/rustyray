use super::*;

mod camera;
pub use camera::*;

mod canvas;
pub use canvas::*;

mod ray;
pub use ray::*;

#[derive(Builder, Debug)]
#[builder(build_fn(skip))]
// TODO lifetimes
pub struct Renderer<'a> {
    samples: u32,
    max_depth: u32,
    thread_count: u32,

    gamma: f64,

    pub camera: Camera,
    canvas: Canvas,

    scene_objects: &'a [Object<'a>],

    #[builder(setter(skip))]
    progress_bar: ProgressBar,
}

impl Renderer<'_> {
    pub fn render(&mut self) {
        Zip::indexed(self.canvas.buffer.lanes_mut(Axis(2))).par_for_each(|(j, i), mut pixel| {
            let mut accum_color = vector![0.0, 0.0, 0.0];
            let mut rng = thread_rng();
            for _ in 0..self.samples {
                let u = (i as f64 + rng.gen::<f64>()) / (self.canvas.width - 1) as f64;
                let v = (j as f64 + rng.gen::<f64>()) / (self.canvas.height - 1) as f64;
                // TODO move to camera
                let ray = self.camera.get_ray(u, v);
                accum_color += &ray.get_color(&self.scene_objects, self.max_depth);
            }
            // TODO allow manual gamma correction
            let arr = Array1::from_iter((accum_color / self.samples as f64).iter().cloned());
            pixel.assign(&arr);
            self.progress_bar.inc(1);
        });
    }
    pub fn save_render(&mut self, path: &str) {
        // TODO error
        self.canvas
            .buffer
            .mapv_inplace(|x| x.powf(1.0 / self.gamma));
        self.canvas.save(path).expect("Failed to save the render.");
    }
}

impl<'a> RendererBuilder<'a> {
    pub fn build(&self) -> Result<Renderer<'a>, RendererBuilderError> {
        let samples = match self.samples {
            Some(value) => value,
            None => 50,
        };
        let max_depth = match self.max_depth {
            Some(value) => value,
            None => 25,
        };
        let thread_count = match self.thread_count {
            Some(value) => value,
            None => 8,
        };
        let canvas = match self.canvas {
            Some(ref value) => (*value).clone(),
            None => CanvasBuilder::default().build().unwrap(),
        };
        let gamma = match self.gamma {
            Some(value) => value,
            None => 2.0,
        };
        let camera = match self.camera {
            Some(ref value) => {
                let mut camera = (*value).clone();
                // TODO cleanup
                camera.aspect_ratio = canvas.aspect_ratio;
                camera.view_width = camera.view_height * camera.aspect_ratio;
                camera.vertical = camera.focus_dist * camera.view_height * camera.v;
                camera.horizontal = camera.focus_dist * camera.view_width * camera.u;
                camera.top_left_corner = camera.origin - (camera.horizontal / 2.0)
                    + (camera.vertical / 2.0)
                    - camera.focus_dist * camera.w;
                camera
            }
            None => CameraBuilder::default().build().unwrap(),
        };
        let scene_objects = match self.scene_objects {
            Some(value) => value,
            None => return Result::Err(Into::into(UninitializedFieldError::from("scene_objects"))),
        };

        let pixel_count = (canvas.width * canvas.height) as u64;
        let progress_bar = ProgressBar::new(pixel_count);

        rayon::ThreadPoolBuilder::new()
            .num_threads(thread_count as usize)
            .build_global()
            .unwrap();

        Ok(Renderer {
            samples,
            max_depth,
            thread_count,
            camera,
            canvas,
            scene_objects,
            progress_bar,
            gamma,
        })
    }
}

use super::*;

#[derive(Builder, Debug)]
#[builder(build_fn(skip))]
pub struct Camera {
    origin: Vector3<f64>,
    look_at: Vector3<f64>,
    vup: Vector3<f64>,

    vfov: f64, // Vertical field of view (in degrees)
    aspect_ratio: f64,
    focal_length: f64,
    #[builder(setter(skip))]
    view_height: f64,
    #[builder(setter(skip))]
    view_width: f64,

    aperture: f64,
    focus_dist: f64,
    #[builder(setter(skip))]
    lens_radius: f64,

    #[builder(setter(skip))]
    w: Vector3<f64>,
    #[builder(setter(skip))]
    u: Vector3<f64>,
    #[builder(setter(skip))]
    v: Vector3<f64>,

    #[builder(setter(skip))]
    vertical: Vector3<f64>,
    #[builder(setter(skip))]
    horizontal: Vector3<f64>,
    #[builder(setter(skip))]
    top_left_corner: Vector3<f64>,
}

impl Camera {
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd[0] + self.v * rd[1];

        Ray {
            origin: self.origin + offset,
            direction: (self.top_left_corner + s * self.horizontal
                - t * self.vertical
                - self.origin
                - offset),
        }
    }
}

impl Default for Camera {
    fn default() -> Self {
        CameraBuilder::default().build().unwrap()
    }
}

impl CameraBuilder {
    pub fn build(&self) -> Result<Camera, CameraBuilderError> {
        let origin = match self.origin {
            Some(ref value) => value.clone(),
            None => vector![0.0, 0.0, 0.0],
        };
        let look_at = match self.look_at {
            Some(ref value) => value.clone(),
            None => vector![0.0, 0.0, -1.0],
        };
        let vup = match self.vup {
            Some(ref value) => value.clone(),
            None => vector![0.0, 1.0, 0.0],
        };
        let vfov = match self.vfov {
            Some(ref value) => value.clone(),
            None => 90.0,
        };
        let aspect_ratio = match self.aspect_ratio {
            Some(ref value) => value.clone(),
            None => 16.0 / 9.0,
        };
        let focal_length = match self.focal_length {
            Some(ref value) => value.clone(),
            None => 1.0,
        };
        let aperture = match self.aperture {
            Some(ref value) => value.clone(),
            None => 2.0,
        };
        let focus_dist = match self.focus_dist {
            Some(ref value) => value.clone(),
            None => return Result::Err(Into::into(UninitializedFieldError::from("focus_dist"))),
        };
        let lens_radius = aperture / 2.0;
        let w = (origin - look_at).normalize();
        let u = vup.cross(&w).normalize();
        let v = w.cross(&u);
        let view_height = 2.0 * (vfov.to_radians() / 2.0).tan(); // 2*h
        let view_width = view_height * aspect_ratio;
        let vertical = focus_dist * view_height * v;
        let horizontal = focus_dist * view_width * u;
        let top_left_corner = origin - (horizontal / 2.0) + (vertical / 2.0) - focus_dist * w;

        let result = Ok(Camera {
            origin,
            look_at,
            vup,
            vfov,
            aspect_ratio,
            focal_length,
            aperture,
            focus_dist,
            lens_radius,
            w,
            u,
            v,
            vertical,
            horizontal,
            top_left_corner,
            view_height,
            view_width,
        });
        result
    }
}

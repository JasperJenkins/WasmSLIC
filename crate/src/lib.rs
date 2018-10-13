#[macro_use]
extern crate cfg_if;

extern crate wasm_bindgen;
extern crate web_sys;
use wasm_bindgen::prelude::{*};
use web_sys::{ImageData};
use std::ops::{AddAssign};
use std::{f32, i16};
mod color;

cfg_if! {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function to get better error messages if we ever panic.
    if #[cfg(feature = "console_error_panic_hook")] {
        extern crate console_error_panic_hook;
        use console_error_panic_hook::set_once as set_panic_hook;
    }
}

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
pub fn segment_image(
    segment_num: JsValue, m_num: JsValue, image_data: ImageData
) -> ImageData {
    let (width, height) = (image_data.width(), image_data.height());
    let segment_num = segment_num.as_f64().unwrap() as usize;
    let m_num = m_num.as_f64().unwrap() as f32;
    let mut pixels_rgb = image_data.data().clone();
    let mut pixels_slic = init_pixels_slic(
        &pixels_rgb, width as usize, height as usize
    );
    let (mut centroids, spacing) = init_centroids_and_get_spacing(
        segment_num,
        width as usize,
        height as usize,
        &pixels_slic,
    );
    let segments = create_segments(
        spacing,
        m_num,
        width as usize,
        height as usize,
        &mut pixels_slic,
        &mut centroids,
    );
    for i in (0..pixels_rgb.len()).step_by(4) {
        let segment = segments.vec[i / 4];
        if segment == -1 {
            pixels_rgb[i] = 255;
            pixels_rgb[i + 1] = 255;
            pixels_rgb[i + 2] = 255;
            pixels_rgb[i + 3] = 255;
        } else {
            pixels_rgb[i] = (segment * 90 % 255) as u8;
            pixels_rgb[i + 1] = (segment * 47 % 255) as u8;
            pixels_rgb[i + 2] = (segment * 173 % 255) as u8;
            pixels_rgb[i + 3] = 90;
        }
    }
    ImageData::new_with_u8_clamped_array_and_sh(
        wasm_bindgen::Clamped(&mut pixels_rgb), width, height
    ).unwrap()
}

#[derive(Clone)]
struct Point {
    l: f32,
    a: f32,
    b: f32,
    x: f32,
    y: f32,
}

impl Point {
    pub fn from_rgb(
        r: u8, g: u8, b: u8, x: usize, y: usize, rgb2xyz_table: &[f32]
    ) -> Self {
        let (l, a, b) = color::rgb2lab(r, g, b, rgb2xyz_table);
        Point { l, a, b, x: x as f32, y: y as f32 }
    }

    pub fn distance(a: &Self, b: &Self, s: f32, m: f32) -> f32 {
        let d_lab = (
              (a.l - b.l).powi(2)
            + (a.a - b.a).powi(2)
            + (a.b - b.b).powi(2)
        ).sqrt();
        let d_xy = (
              (a.x - b.x).powi(2)
            + (a.y - b.y).powi(2)
        ).sqrt();
        d_lab + (m / s) * d_xy
    }

    pub fn zero(&mut self) {
        self.l = 0.0;
        self.a = 0.0;
        self.b = 0.0;
        self.x = 0.0;
        self.y = 0.0;
    }

    fn add(&mut self, other: &Point) {
        self.l += other.l;
        self.a += other.a;
        self.b += other.b;
        self.x += other.x;
        self.y += other.y;
    }

    fn div(&mut self, other: f32) {
        self.l /= other;
        self.a /= other;
        self.b /= other;
        self.x /= other;
        self.y /= other;
    }
}

struct Vec2d<T> {
    vec: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Vec2d<T> {
    pub fn from_vec(vec: Vec<T>, width: usize, height: usize) -> Vec2d<T> {
        Vec2d { vec, width, height }
    }

    pub fn i(&self, x: usize, y: usize) -> &T {
        &self.vec[self.width * y + x]
    }

    pub fn i_mut(&mut self, x: usize, y: usize) -> &mut T {
        &mut self.vec[self.width * y + x]
    }

    pub fn i_assign(&mut self, x: usize, y: usize, item: T) {
        self.vec[self.width * y + x] = item;
    }
}

fn init_pixels_slic(
    pixels: &Vec<u8>, width: usize, height: usize
) -> Vec2d<Point> {
    let mut table: [f32; 256] = [0.0; 256];
    for i in 0..table.len() {
        table[i] = color::rgb2xyz_coord(i as u8);
    }
    let mut vec_slic = Vec::with_capacity(pixels.len() / 4);
    for i in (0..pixels.len()).step_by(4) {
        vec_slic.push(Point::from_rgb(
            pixels[i],
            pixels[i + 1],
            pixels[i + 2],
            (i / 4) % width,
            (i / 4) / width,
            &table,
        ));
    }
    Vec2d::from_vec(vec_slic, width, height)
}

fn init_centroids_and_get_spacing(
    segment_num: usize, width: usize, height: usize, pixels_slic: &Vec2d<Point>
) -> (Vec<Point>, f32) {
    let spacing = centroid_spacing(width, height, segment_num);
    let mut centroids = Vec::with_capacity(segment_num);
    let mut i = 0;
    while i < segment_num {
        let px = i as f32 * spacing;
        let x = px % width as f32;
        let y = (px / width as f32).floor() * spacing;
        centroids.push(pixels_slic.i(x as usize, y as usize).clone());
        i += 1;
    }
    (centroids, spacing)
}

fn centroid_spacing(w: usize, h: usize, n: usize) -> f32 {
    let (h, w, n) = (h as f32, w as f32, n as f32);
    let (mut lower, mut upper) = (1.0, f32::max(h, w));
    loop {
        let mid = (lower + upper) / 2.0;
        let n_estimate = (h / mid) * (w / mid);
        let diff = n - n_estimate;
        if diff.abs() < 0.0001 {
            return mid;
        } else if diff > 0.0 {
            upper = mid;
        } else {
            lower = mid;
        }
    }
}

fn create_segments(
    spacing: f32,
    m_num: f32,
    width: usize,
    height: usize,
    pixels_slic: &mut Vec2d<Point>,
    centroids: &mut Vec<Point>,
) -> Vec2d<i16> {
    let mut segments: Vec2d<i16> = Vec2d::from_vec(
        vec![-1; width * height], width, height
    );
    let search_space = spacing * 1.75;
    let mut cluster_counts: Vec<u32> = vec![0; centroids.len()];
    for _ in 0..10 {
        for (k, centroid) in centroids.iter().enumerate() {
            let (x_min, x_max) = (
                usize::max((centroid.x - search_space) as usize, 0),
                usize::min((centroid.x + search_space) as usize, width),
            );
            let (y_min, y_max) = (
                usize::max((centroid.y - search_space) as usize, 0),
                usize::min((centroid.y + search_space) as usize, height),
            );
            for x_i in x_min..x_max {
                for y_i in y_min..y_max {
                    if *segments.i(x_i, y_i) != -1 {
                        let distance_k = Point::distance(
                            pixels_slic.i(x_i, y_i),
                            centroid,
                            spacing,
                            m_num
                        );
                        let distance_now = Point::distance(
                            pixels_slic.i(x_i, y_i),
                            &centroids[*segments.i(x_i, y_i) as usize],
                            spacing,
                            m_num
                        );
                        if distance_k < distance_now {
                            segments.i_assign(x_i, y_i, k as i16);
                        }
                    } else {
                        segments.i_assign(x_i, y_i, k as i16);
                    }
                }
            }
        }
        for i in 0..centroids.len() {
            centroids[i].zero();
            cluster_counts[i] = 0;
        }
        for (k, point) in segments.vec.iter().zip(pixels_slic.vec.iter()) {
            centroids[*k as usize].add(point);
            cluster_counts[*k as usize] += 1;
        }
        for i in 0..centroids.len() {
            centroids[i].div(cluster_counts[i] as f32);
        }
    }
    segments
}
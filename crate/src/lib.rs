#[macro_use]
extern crate cfg_if;
extern crate packed_simd;
extern crate wasm_bindgen;
extern crate web_sys;
use packed_simd::f32x4;
use std::collections::HashMap;
use std::{f32, i16};
use wasm_bindgen::prelude::*;
use web_sys::ImageData;
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
pub fn segment_image(segment_num: JsValue, m_num: JsValue, image_data: ImageData) -> ImageData {
    let (width, height) = (image_data.width(), image_data.height());
    let segment_num = segment_num.as_f64().unwrap() as usize;
    let m_num = m_num.as_f64().unwrap() as f32;
    let mut pixels_rgb = image_data.data().clone();
    let mut pixels_slic = init_pixels_slic(&pixels_rgb, width as usize, height as usize);
    let (mut centroids, spacing) = init_centroids_and_get_spacing(segment_num, &pixels_slic);
    let mut segments = create_segments(spacing, m_num, &mut pixels_slic, &mut centroids);
    segments = enforce_connectivity(segments, &centroids);
    mark_boundaries(&mut pixels_rgb, &segments);
    ImageData::new_with_u8_clamped_array_and_sh(
        wasm_bindgen::Clamped(&mut pixels_rgb),
        width,
        height,
    )
    .unwrap()
}

#[derive(Clone)]
struct Point {
    /*
    l: f32,
    a: f32,
    b: f32,
    x: f32,
    y: f32,
    */
    lab: f32x4,
    x: f32,
    y: f32,
}

impl Point {
    #[inline(always)]
    pub fn from_rgb(r: u8, g: u8, b: u8, x: usize, y: usize, rgb2xyz_table: &[f32]) -> Self {
        let (l, a, b) = color::rgb2lab(r, g, b, rgb2xyz_table);
        Point {
            lab: f32x4::new(l, a, b, 0.0),
            x: x as f32,
            y: y as f32,
        }
    }

    #[inline(always)]
    pub fn distance(a: &Self, b: &Self, xy_coeff: f32) -> f32 {
        /*
        (
              (a.l - b.l).powi(2)
            + (a.a - b.a).powi(2)
            + (a.b - b.b).powi(2)
        ).sqrt() + xy_coeff * (
              (a.x - b.x).powi(2)
            + (a.y - b.y).powi(2)
        ).sqrt()
        */
        /*
        (
              (a.l - b.l).abs()
            + (a.a - b.a).abs()
            + (a.b - b.b).abs()
        ) + xy_coeff * (
              (a.x - b.x).abs()
            + (a.y - b.y).abs()
        )
        */
        (a.lab - b.lab).abs().sum() + xy_coeff * ((a.x - b.x).abs() + (a.y - b.y).abs())
    }

    #[inline(always)]
    pub fn zero(&mut self) {
        /*
        self.l = 0.0;
        self.a = 0.0;
        self.b = 0.0;
        self.x = 0.0;
        self.y = 0.0;
        */
        self.lab = f32x4::new(0.0, 0.0, 0.0, 0.0);
        self.x = 0.0;
        self.y = 0.0;
    }

    #[inline(always)]
    fn add(&mut self, other: &Point) {
        /*
        self.l += other.l;
        self.a += other.a;
        self.b += other.b;
        self.x += other.x;
        self.y += other.y;
        */
        self.lab += other.lab;
        self.x += other.x;
        self.y += other.y;
    }

    #[inline(always)]
    fn div(&mut self, other: f32) {
        /*
        self.l /= other;
        self.a /= other;
        self.b /= other;
        self.x /= other;
        self.y /= other;
        */
        self.lab /= other;
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

    #[inline(always)]
    pub fn i(&self, x: usize, y: usize) -> &T {
        &self.vec[self.width * y + x]
    }

    #[inline(always)]
    pub fn i_mut(&mut self, x: usize, y: usize) -> &mut T {
        &mut self.vec[self.width * y + x]
    }

    #[inline(always)]
    pub fn i_assign(&mut self, x: usize, y: usize, item: T) {
        self.vec[self.width * y + x] = item;
    }
}

fn init_pixels_slic(pixels: &Vec<u8>, width: usize, height: usize) -> Vec2d<Point> {
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
    segment_num: usize,
    pixels_slic: &Vec2d<Point>,
) -> (Vec<Point>, f32) {
    let spacing = centroid_spacing(pixels_slic.width, pixels_slic.height, segment_num);
    let mut centroids = Vec::with_capacity(segment_num);
    let mut i = 0;
    while i < segment_num {
        let px = i as f32 * spacing;
        let x = px % pixels_slic.width as f32 + (spacing / 2.0);
        let y = (px / pixels_slic.width as f32).floor() * spacing;
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
        if diff.abs() < 0.001 {
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
    pixels_slic: &mut Vec2d<Point>,
    centroids: &mut Vec<Point>,
) -> Vec2d<i16> {
    let max_size = 0;
    let (width, height) = (pixels_slic.width, pixels_slic.height);
    let mut segments: Vec2d<i16> = Vec2d::from_vec(vec![-1; width * height], width, height);
    let xy_coeff = m_num / spacing;
    let search_space = spacing;
    let mut cluster_counts: Vec<u32> = vec![0; centroids.len()];
    for _ in 0..6 {
        for (k, centroid) in centroids.iter().enumerate() {
            let (x_min, x_max) = (
                usize::max((centroid.x - search_space * 1.2) as usize, 0),
                usize::min((centroid.x + search_space * 1.25) as usize, width),
            );
            let (y_min, y_max) = (
                usize::max((centroid.y - search_space * 1.2) as usize, 0),
                usize::min((centroid.y + search_space * 2.0) as usize, height),
            );
            for x_i in x_min..x_max {
                for y_i in y_min..y_max {
                    if *segments.i(x_i, y_i) != -1 {
                        if Point::distance(pixels_slic.i(x_i, y_i), centroid, xy_coeff)
                            < Point::distance(
                                pixels_slic.i(x_i, y_i),
                                &centroids[*segments.i(x_i, y_i) as usize],
                                xy_coeff,
                            ) {
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

fn enforce_connectivity(segments: Vec2d<i16>, centroids: &Vec<Point>) -> Vec2d<i16> {
    let (width, height) = (segments.width, segments.height);
    let optimal_size = (width * height / centroids.len()) as f32;
    let (min_size, max_size) = (
        (optimal_size * 0.25) as usize,
        (optimal_size * 5.0) as usize,
    );
    let mut connected_segments = Vec2d::from_vec(vec![-1 as i16; width * height], width, height);
    let mut bfs_visited = Vec::with_capacity(max_size);
    let mut adjacent_labels = HashMap::new();
    let bfs_neighbors = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    let (mut cur_label, mut new_label, mut cur_size) = (0, 0, 0);
    let (mut x_j, mut y_j, mut bfs_i) = (0, 0, 0);
    for x_i in 0..width {
        for y_i in 0..height {
            if *connected_segments.i(x_i, y_i) >= 0 {
                continue;
            }
            cur_label = *segments.i(x_i, y_i);
            connected_segments.i_assign(x_i, y_i, new_label);
            cur_size = 1;
            bfs_i = 0;
            bfs_visited.clear();
            bfs_visited.push((x_i, y_i));
            adjacent_labels.clear();
            while bfs_i < cur_size && bfs_i < max_size {
                for (x_offset, y_offset) in bfs_neighbors.iter() {
                    x_j = bfs_visited[bfs_i].0 as i16 + x_offset;
                    y_j = bfs_visited[bfs_i].1 as i16 + y_offset;
                    if (x_j >= 0 && x_j < width as i16 && y_j >= 0 && y_j < height as i16) {
                        let (x_j, y_j) = (x_j as usize, y_j as usize);
                        if (*segments.i(x_j, y_j) == cur_label
                            && *connected_segments.i(x_j, y_j) == -1)
                        {
                            connected_segments.i_assign(x_j, y_j, new_label);
                            bfs_visited.push((x_j, y_j));
                            cur_size += 1;
                            if cur_size >= max_size {
                                break;
                            }
                        } else if (*connected_segments.i(x_j, y_j) >= 0
                            && *connected_segments.i(x_j, y_j) != new_label)
                        {
                            *adjacent_labels
                                .entry(*connected_segments.i(x_j, y_j))
                                .or_insert(0) += 1;
                        }
                    }
                }
                bfs_i += 1;
            }
            if cur_size < min_size {
                let mut largest_adjacent = 0;
                let mut largest_adjacent_size = 0;
                for (adjacent, adjacent_size) in adjacent_labels.iter() {
                    if *adjacent_size > largest_adjacent_size {
                        largest_adjacent = *adjacent;
                        largest_adjacent_size = *adjacent_size;
                    }
                }
                for (x_k, y_k) in bfs_visited.iter() {
                    connected_segments.i_assign(*x_k, *y_k, largest_adjacent);
                }
            } else {
                new_label += 1;
            }
        }
    }
    connected_segments
}

fn mark_boundaries(pixels_rgb: &mut Vec<u8>, segments: &Vec2d<i16>) {
    let neighbors = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    let (mut x_j, mut y_j) = (0, 0);
    let mut edge = false;
    let mut label = 0;
    for x_i in 0..segments.width {
        for y_i in 0..segments.height {
            label = *segments.i(x_i, y_i);
            edge = false;
            for (x_offset, y_offset) in neighbors.iter() {
                x_j = x_i as i16 + *x_offset;
                y_j = y_i as i16 + *y_offset;
                if (x_j < 0
                    || x_j >= segments.width as i16
                    || y_j < 0
                    || y_j >= segments.height as i16
                    || *segments.i(x_j as usize, y_j as usize) != label)
                {
                    edge = true;
                    break;
                }
            }
            let rgb_i = (y_i * segments.width + x_i) * 4;
            if edge {
                pixels_rgb[rgb_i] = 92;
                pixels_rgb[rgb_i + 1] = 66;
                pixels_rgb[rgb_i + 2] = 244;
                pixels_rgb[rgb_i + 3] = 255;
            } else {
                pixels_rgb[rgb_i + 3] = 0;
            }
        }
    }
}

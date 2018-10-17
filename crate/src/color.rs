extern crate packed_simd;
use packed_simd::f32x4;

// https://www2.eecs.berkeley.edu/Research/Projects/CS/vision/bsds/code/Util/RGB2Lab.m
#[inline(always)]
pub fn rgb2lab(r: u8, g: u8, b: u8, rgb2xyz_table: &[f32]) -> (f32, f32, f32) {
    let rgb = f32x4::new(
        rgb2xyz_table[r as usize],
        rgb2xyz_table[g as usize],
        rgb2xyz_table[b as usize],
        0.0,
    );
    let (x, y, z) = (
        (rgb * f32x4::new(0.412453, 0.357580, 0.180423, 0.0)).sum() / 0.950456,
        (rgb * f32x4::new(0.212671, 0.715160, 0.072169, 0.0)).sum(),
        (rgb * f32x4::new(0.019334, 0.119193, 0.950227, 0.0)).sum() / 1.088754,
    );
    let threshold = 0.008856;
    let f_x = if x > threshold {
        x.cbrt()
    } else {
        7.787 * x + (16.0 / 116.0)
    };
    let y3 = y.cbrt();
    let (f_y, l) = if y > threshold {
        (y3, 116.0 * y3 - 16.0)
    } else {
        (7.787 * y + (16.0 / 116.0), 903.3 * y)
    };
    let f_z = if z > threshold {
        z.cbrt()
    } else {
        7.787 * z + (16.0 / 116.0)
    };
    (l, 500.0 * (f_x - f_y), 200.0 * (f_y - f_z))
}

#[inline(always)]
pub fn rgb2xyz_coord(c: u8) -> f32 {
    let c = c as f32 / 255.0;
    if c > 0.04045 {
        ((c + 0.055) / 1.055).powf(2.4)
    //powf_so_slow(((c + 0.055) / 1.055) as f64, 2.4) as f32
    } else {
        c / 12.92
    }
}

#[cfg(test)]
mod tests {
    use color;

    // https://github.com/ashleygwilliams/assert_approx_eq/blob/master/src/lib.rs
    fn assert_almost_eq(a: f32, b: f32, eps: f32) {
        let diff = (a - b).abs();
        assert!(
            diff < eps,
            "assertion failed: `(left != right)` \
             (left: `{:?}`, right: `{:?}`, expect diff: `{:?}`, real diff: `{:?}`)",
            a,
            b,
            eps,
            diff
        );
    }

    #[test]
    fn test_rgb2lab() {
        let mut table: [f32; 256] = [0.0; 256];
        for i in 0..table.len() {
            table[i] = color::rgb2xyz_coord(i as u8);
        }
        let eps = 0.05;
        let (r, g, b): (u8, u8, u8) = (0, 127, 255);
        let (l, a, b): (f32, f32, f32) = color::rgb2lab(r, g, b, &table);
        assert_almost_eq(l, 54.444, eps);
        assert_almost_eq(a, 19.402, eps);
        assert_almost_eq(b, -71.357, eps);
    }
}

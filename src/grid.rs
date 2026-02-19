use bevy::prelude::*;
use noise::{NoiseFn, Perlin};

#[derive(Resource)]
pub struct Grid {
    x: f32,
    y: f32,
    width: usize,
    height: usize,
    spacing: f32,
    data: Vec<f32>,
}

impl Grid {
    pub fn new(x: f32, y: f32, width: usize, height: usize, spacing: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
            spacing,
            data: vec![0.; width * height],
        }
    }

    fn get(&self, x: usize, y: usize) -> Option<f32> {
        if x < self.width && y < self.height {
            Some(self.data[y * self.width + x])
        } else {
            None
        }
    }

    fn gets(&self, x: f32, y: f32) -> Option<f32> {
        let x0 = x.floor() as usize;
        let y0 = y.floor() as usize;
        let x1 = x0 + 1;
        let y1 = y0 + 1;

        let tx = x.fract();
        let ty = y.fract();

        let v00 = self.get(x0, y0)?;
        let v10 = self.get(x1, y0)?;
        let v01 = self.get(x0, y1)?;
        let v11 = self.get(x1, y1)?;

        let v = (v00 * (1. - tx) + v10 * tx) * (1. - ty) + (v01 * (1. - tx) + v11 * tx) * ty;

        return Some(v);
    }

    pub fn get_world(&self, x: f32, y: f32) -> Option<f32> {
        let gx = (x - self.x) / self.spacing;
        let gy = (y - self.y) / self.spacing;
        if gx < 1. || gy < 1. {
            return None;
        }
        self.gets(gx, gy)
    }

    fn set(&mut self, x: usize, y: usize, v: f32) {
        if x < self.width && y < self.height {
            self.data[y * self.width + x] = v.clamp(0., 1.)
        }
    }

    fn get_normal(&self, x: f32, y: f32) -> Option<(f32, f32)> {
        let x0 = x.floor() as usize;
        let y0 = y.floor() as usize;
        let x1 = x0 + 1;
        let y1 = y0 + 1;

        let tx = x.fract();
        let ty = y.fract();

        let grad = |gx: usize, gy: usize| -> Option<(f32, f32)> {
            let dx = self.get(gx + 1, gy)? - self.get(gx - 1, gy)?;
            let dy = self.get(gx, gy + 1)? - self.get(gx, gy - 1)?;
            Some((dx, dy))
        };

        let (g00x, g00y) = grad(x0, y0)?;
        let (g10x, g10y) = grad(x1, y0)?;
        let (g01x, g01y) = grad(x0, y1)?;
        let (g11x, g11y) = grad(x1, y1)?;

        let mut nx =
            (g00x * (1. - tx) + g10x * tx) * (1. - ty) + (g01x * (1. - tx) + g11x * tx) * ty;
        let mut ny =
            (g00y * (1. - tx) + g10y * tx) * (1. - ty) + (g01y * (1. - tx) + g11y * tx) * ty;

        let nl = (nx.powi(2) + ny.powi(2)).sqrt();
        if nl > 0. {
            nx /= nl;
            ny /= nl;
        }

        return Some((nx, ny));
    }

    pub fn get_normal_world(&self, x: f32, y: f32) -> Option<(f32, f32)> {
        let gx = (x - self.x) / self.spacing;
        let gy = (y - self.y) / self.spacing;
        if gx < 1. || gy < 1. {
            return None;
        }
        self.get_normal(gx, gy)
    }

    pub fn draw_dots(&self, gizmos: &mut Gizmos) {
        for x in 0..self.width {
            for y in 0..self.height {
                if let Some(v) = self.get(x, y) {
                    let color = Color::linear_rgba(1., 1., 1., v.powf(3.));
                    gizmos.circle_2d(
                        Vec2::new(
                            self.x + x as f32 * self.spacing,
                            self.y + y as f32 * self.spacing,
                        ),
                        self.spacing / 5.,
                        color,
                    );
                }
            }
        }
    }

    pub fn draw_segments(&self, threshold: f32, smooth: bool, gizmos: &mut Gizmos) {
        let segments = self.gen_segments(threshold, smooth);

        for segment in segments {
            gizmos.line_2d(
                Vec2::new(
                    segment.0.0 * self.spacing + self.x,
                    segment.0.1 * self.spacing + self.y,
                ),
                Vec2::new(
                    segment.1.0 * self.spacing + self.x,
                    segment.1.1 * self.spacing + self.y,
                ),
                Color::linear_rgb(1., 1., 1.),
            );
        }
    }

    pub fn generate(&mut self, seed: u32, scale: f64, offset: (f64, f64)) {
        let perlin = Perlin::new(seed);

        for x in 0..self.width {
            for y in 0..self.height {
                let v = perlin.get([x as f64 * scale + offset.0, y as f64 * scale + offset.1]);
                let nv = (v + 1.) / 2.;
                self.set(x, y, nv.powf(2.) as f32);
            }
        }
    }

    pub fn gen_segments(&self, threshold: f32, smooth: bool) -> Vec<((f32, f32), (f32, f32))> {
        let mut segments = Vec::new();
        for x in 0..self.width - 1 {
            for y in 0..self.height - 1 {
                let vs = [
                    self.get(x, y),
                    self.get(x + 1, y),
                    self.get(x, y + 1),
                    self.get(x + 1, y + 1),
                ];
                let mut vsf = [0., 0., 0., 0.];
                let mut vi = 0u8;
                for (i, v) in vs.iter().enumerate() {
                    let b = v.is_some_and(|v| v > threshold);
                    vsf[i] = v.unwrap_or(0.);
                    vi |= (b as u8) << (i as u8);
                }

                let x = x as f32;
                let y = y as f32;

                let rs = match smooth {
                    true => (
                        (threshold - vsf[0]) / (vsf[1] - vsf[0]),
                        (threshold - vsf[1]) / (vsf[3] - vsf[1]),
                        (threshold - vsf[2]) / (vsf[3] - vsf[2]),
                        (threshold - vsf[0]) / (vsf[2] - vsf[0]),
                    ),
                    false => (0.5, 0.5, 0.5, 0.5),
                };

                let e = (
                    (x + rs.0, y),
                    (x + 1., y + rs.1),
                    (x + rs.2, y + 1.),
                    (x, y + rs.3),
                );

                if let Some(segment) = match vi {
                    // corners
                    0b0001 | 0b1110 => Some((e.0, e.3)),
                    0b0010 | 0b1101 => Some((e.0, e.1)),
                    0b0100 | 0b1011 => Some((e.2, e.3)),
                    0b1000 | 0b0111 => Some((e.1, e.2)),

                    // edges
                    0b0011 | 0b1100 => Some((e.1, e.3)),
                    0b0101 | 0b1010 => Some((e.0, e.2)),

                    // diagonals
                    0b1001 => Some(((x + 1., y), (x, y + 1.))),
                    0b0110 => Some(((x, y), (x + 1., y + 1.))),

                    _ => None,
                } {
                    segments.push(segment);
                }
            }
        }
        segments
    }
}

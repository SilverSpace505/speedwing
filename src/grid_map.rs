use bevy::{platform::collections::HashMap, prelude::*};

use crate::{
    common::div_floor,
    grid::{Grid, GridMaterial},
};

#[derive(Resource)]
pub struct GridMap {
    scale: f32,
    grid_size: u32,
    grids: HashMap<(i32, i32), Grid>,
    threshold: f32,
    smooth: bool,
}

impl GridMap {
    pub fn new(scale: f32, grid_size: u32, threshold: f32, smooth: bool) -> Self {
        Self {
            scale,
            grid_size,
            grids: HashMap::new(),
            threshold,
            smooth,
        }
    }
    fn create_grid(&self, x: i32, y: i32) -> Grid {
        Grid::new(
            x as f32 * self.scale * self.grid_size as f32,
            y as f32 * self.scale * self.grid_size as f32,
            self.grid_size,
            self.grid_size,
            self.scale,
        )
    }
    // pub fn set(&mut self, x: i32, y: i32, v: f32) {
    //     let gx = x / self.grid_size as i32;
    //     let gy = y / self.grid_size as i32;

    //     let x = (x - gx * self.grid_size as i32) as u32;
    //     let y = (y - gy * self.grid_size as i32) as u32;

    //     let grid = match self.grids.get_mut(&(gx, gy)) {
    //         Some(grid) => Some(grid),
    //         None => {
    //             let grid = self.create_grid(gx, gy);
    //             self.grids.insert((gx, gy), grid);
    //             self.grids.get_mut(&(gx, gy))
    //         }
    //     };
    //     let Some(grid) = grid else {
    //         return;
    //     };

    //     grid.set(x, y, v);
    // }
    //
    pub fn get(&self, x: i32, y: i32) -> Option<f32> {
        let gx = div_floor(x, self.grid_size as i32);
        let gy = div_floor(y, self.grid_size as i32);

        let x = (x - gx * self.grid_size as i32) as u32;
        let y = (y - gy * self.grid_size as i32) as u32;

        match self.grids.get(&(gx, gy)) {
            Some(grid) => grid.get(x, y),
            None => None,
        }
    }
    fn gets(&self, x: f32, y: f32) -> Option<f32> {
        let x0 = x.floor() as i32;
        let y0 = y.floor() as i32;
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
        let gx = x / self.scale;
        let gy = y / self.scale;
        self.gets(gx, gy)
    }
    //
    fn get_normal(&self, x: f32, y: f32) -> Option<(f32, f32)> {
        let x0 = x.floor() as i32;
        let y0 = y.floor() as i32;
        let x1 = x0 + 1;
        let y1 = y0 + 1;

        let tx = x - x0 as f32;
        let ty = y - y0 as f32;

        let grad = |gx: i32, gy: i32| -> Option<(f32, f32)> {
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
        let gx = x / self.scale;
        let gy = y / self.scale;
        self.get_normal(gx, gy)
    }
    //
    pub fn manage_meshes(
        &mut self,
        commands: &mut Commands,
        mut meshes: &mut ResMut<Assets<Mesh>>,
        mut materials: &mut ResMut<Assets<GridMaterial>>,
    ) {
        let mut to_mesh = Vec::new();
        for (coords, grid) in self.grids.iter() {
            if grid.mesh.is_none() {
                let bridges = Some([
                    self.grids.get(&(coords.0 + 1, coords.1 + 1)),
                    self.grids.get(&(coords.0 + 1, coords.1)),
                    self.grids.get(&(coords.0, coords.1 + 1)),
                ]);

                to_mesh.push((
                    coords.clone(),
                    grid.gen_attributes(self.threshold, self.smooth, &bridges),
                ));
            }
        }
        for (coords, attributes) in to_mesh.into_iter() {
            if let Some(grid) = self.grids.get_mut(&coords) {
                commands.spawn(grid.bundle_attributes(&mut meshes, &mut materials, attributes));
            }
        }
    }

    pub fn draw_dots(&self, gizmos: &mut Gizmos) {
        for (_coords, grid) in self.grids.iter() {
            grid.draw_dots(gizmos);
        }
    }

    pub fn draw_segments(&self, gizmos: &mut Gizmos) {
        for (_coords, grid) in self.grids.iter() {
            grid.draw_segments(self.threshold, self.smooth, gizmos);
        }
    }
    pub fn draw_borders(&self, gizmos: &mut Gizmos) {
        for (coords, grid) in self.grids.iter() {
            gizmos.rect_2d(
                Isometry2d::from_translation(
                    (Vec2::new(coords.0 as f32, coords.1 as f32) + 0.5)
                        * self.grid_size as f32
                        * self.scale,
                ),
                Vec2::splat(self.grid_size as f32 * self.scale),
                Color::linear_rgb(0., 1., 0.),
            );
            grid.draw_segments(self.threshold, self.smooth, gizmos);
        }
    }
    pub fn generate(&mut self, gx: i32, gy: i32, seed: u32, scale: f64, offset: (f64, f64)) {
        let grid = match self.grids.get_mut(&(gx, gy)) {
            Some(grid) => Some(grid),
            None => {
                let grid = self.create_grid(gx, gy);
                self.grids.insert((gx, gy), grid);
                self.grids.get_mut(&(gx, gy))
            }
        };
        let Some(grid) = grid else {
            return;
        };

        grid.generate(
            seed,
            scale,
            (
                offset.0 * self.grid_size as f64 * scale,
                offset.1 * self.grid_size as f64 * scale,
            ),
        );
    }
}

pub fn manage_meshes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<GridMaterial>>,
    mut map: ResMut<GridMap>,
) {
    map.manage_meshes(&mut commands, &mut meshes, &mut materials);
}

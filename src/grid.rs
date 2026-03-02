use bevy::{
    asset::RenderAssetUsages,
    mesh::{Indices, MeshVertexAttribute, VertexFormat},
    prelude::*,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
    sprite_render::Material2d,
};
use noise::{NoiseFn, Perlin};

use crate::common::{GameEntity, in_viewport};

const SHADER_ASSET_PATH: &str = "shaders/grid.wgsl";

#[derive(Resource)]
pub struct Grid {
    x: f32,
    y: f32,
    pub width: u32,
    pub height: u32,
    spacing: f32,
    data: Vec<f32>,
    pub mesh: Option<Handle<Mesh>>,
    entity: Option<Entity>,
    pub changed: bool,
}

const ATTRIBUTE_V: MeshVertexAttribute =
    MeshVertexAttribute::new("V", 988540917, VertexFormat::Float32);

pub struct MeshAttributes {
    positions: Vec<[f32; 3]>,
    colours: Vec<[f32; 4]>,
    indices: Indices,
    vs: Vec<f32>,
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct GridMaterial {
    #[uniform(0)]
    color: LinearRgba,
}

impl Material2d for GridMaterial {
    fn vertex_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }
    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }

    fn specialize(
        descriptor: &mut bevy::render::render_resource::RenderPipelineDescriptor,
        layout: &bevy::mesh::MeshVertexBufferLayoutRef,
        _key: bevy::sprite_render::Material2dKey<Self>,
    ) -> Result<(), bevy::render::render_resource::SpecializedMeshPipelineError> {
        let vertex_layout = layout.0.get_layout(&[
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
            ATTRIBUTE_V.at_shader_location(1),
        ])?;
        descriptor.vertex.buffers = vec![vertex_layout];
        Ok(())
    }
}

impl Grid {
    pub fn new(x: f32, y: f32, width: u32, height: u32, spacing: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
            spacing,
            data: vec![0.; (width * height) as usize],
            mesh: None,
            entity: None,
            changed: false,
        }
    }

    pub fn get(&self, x: u32, y: u32) -> Option<f32> {
        if x < self.width && y < self.height {
            Some(self.data[(y * self.width + x) as usize])
        } else {
            None
        }
    }

    // fn gets(&self, x: f32, y: f32) -> Option<f32> {
    //     let x0 = x.floor() as u32;
    //     let y0 = y.floor() as u32;
    //     let x1 = x0 + 1;
    //     let y1 = y0 + 1;

    //     let tx = x.fract();
    //     let ty = y.fract();

    //     let v00 = self.get(x0, y0)?;
    //     let v10 = self.get(x1, y0)?;
    //     let v01 = self.get(x0, y1)?;
    //     let v11 = self.get(x1, y1)?;

    //     let v = (v00 * (1. - tx) + v10 * tx) * (1. - ty) + (v01 * (1. - tx) + v11 * tx) * ty;

    //     return Some(v);
    // }

    // pub fn get_world(&self, x: f32, y: f32) -> Option<f32> {
    //     let gx = (x - self.x) / self.spacing;
    //     let gy = (y - self.y) / self.spacing;
    //     if gx < 1. || gy < 1. {
    //         return None;
    //     }
    //     self.gets(gx, gy)
    // }

    pub fn set(&mut self, x: u32, y: u32, v: f32) {
        if x < self.width && y < self.height {
            self.data[(y * self.width + x) as usize] = v.clamp(0., 1.);
            self.changed = true;
        }
    }

    // fn get_normal(&self, x: f32, y: f32) -> Option<(f32, f32)> {
    //     let x0 = x.floor() as u32;
    //     let y0 = y.floor() as u32;
    //     let x1 = x0 + 1;
    //     let y1 = y0 + 1;

    //     let tx = x.fract();
    //     let ty = y.fract();

    //     let grad = |gx: u32, gy: u32| -> Option<(f32, f32)> {
    //         let dx = self.get(gx + 1, gy)? - self.get(gx - 1, gy)?;
    //         let dy = self.get(gx, gy + 1)? - self.get(gx, gy - 1)?;
    //         Some((dx, dy))
    //     };

    //     let (g00x, g00y) = grad(x0, y0)?;
    //     let (g10x, g10y) = grad(x1, y0)?;
    //     let (g01x, g01y) = grad(x0, y1)?;
    //     let (g11x, g11y) = grad(x1, y1)?;

    //     let mut nx =
    //         (g00x * (1. - tx) + g10x * tx) * (1. - ty) + (g01x * (1. - tx) + g11x * tx) * ty;
    //     let mut ny =
    //         (g00y * (1. - tx) + g10y * tx) * (1. - ty) + (g01y * (1. - tx) + g11y * tx) * ty;

    //     let nl = (nx.powi(2) + ny.powi(2)).sqrt();
    //     if nl > 0. {
    //         nx /= nl;
    //         ny /= nl;
    //     }

    //     return Some((nx, ny));
    // }

    // pub fn get_normal_world(&self, x: f32, y: f32) -> Option<(f32, f32)> {
    //     let gx = (x - self.x) / self.spacing;
    //     let gy = (y - self.y) / self.spacing;
    //     if gx < 1. || gy < 1. {
    //         return None;
    //     }
    //     self.get_normal(gx, gy)
    // }

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
    pub fn get_bridge(&self, x: u32, y: u32, bridges: &Option<[Option<&Grid>; 3]>) -> Option<f32> {
        match self.get(x, y) {
            Some(v) => Some(v),
            None => {
                let Some(bridges) = bridges else { return None };
                if x >= self.width
                    && y >= self.height
                    && let Some(grid) = &bridges[0]
                {
                    grid.get(x - self.width, y - self.height)
                } else if x >= self.width
                    && let Some(grid) = &bridges[1]
                {
                    grid.get(x - self.width, y)
                } else if y >= self.height
                    && let Some(grid) = &bridges[2]
                {
                    grid.get(x, y - self.height)
                } else {
                    None
                }
            }
        }
    }
    fn gets_bridge(&self, x: f32, y: f32, bridges: &Option<[Option<&Grid>; 3]>) -> Option<f32> {
        let x0 = x.floor() as u32;
        let y0 = y.floor() as u32;
        let x1 = x0 + 1;
        let y1 = y0 + 1;

        let tx = x.fract();
        let ty = y.fract();

        let v00 = self.get_bridge(x0, y0, bridges)?;
        let v10 = self.get_bridge(x1, y0, bridges)?;
        let v01 = self.get_bridge(x0, y1, bridges)?;
        let v11 = self.get_bridge(x1, y1, bridges)?;

        let v = (v00 * (1. - tx) + v10 * tx) * (1. - ty) + (v01 * (1. - tx) + v11 * tx) * ty;

        return Some(v);
    }
    pub fn gen_triangles(
        &self,
        threshold: f32,
        smooth: bool,
        bridges: &Option<[Option<&Grid>; 3]>,
    ) -> Vec<((f32, f32), (f32, f32), (f32, f32))> {
        let mut triangles = Vec::new();

        for x in 0..self.width {
            for y in 0..self.height {
                let vs = [
                    self.get_bridge(x, y, &bridges),
                    self.get_bridge(x + 1, y, &bridges),
                    self.get_bridge(x, y + 1, &bridges),
                    self.get_bridge(x + 1, y + 1, &bridges),
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
                        ((threshold - vsf[0]) / (vsf[1] - vsf[0])).clamp(0., 1.),
                        ((threshold - vsf[1]) / (vsf[3] - vsf[1])).clamp(0., 1.),
                        ((threshold - vsf[2]) / (vsf[3] - vsf[2])).clamp(0., 1.),
                        ((threshold - vsf[0]) / (vsf[2] - vsf[0])).clamp(0., 1.),
                    ),
                    false => (0.5, 0.5, 0.5, 0.5),
                };

                let c = ((x, y), (x + 1., y), (x + 1., y + 1.), (x, y + 1.));

                let e = (
                    (x + rs.0, y),
                    (x + 1., y + rs.1),
                    (x + rs.2, y + 1.),
                    (x, y + rs.3),
                );

                if let Some(mut triangle) = match vi {
                    // corners
                    0b0001 => Some(vec![(e.0, e.3, c.0)]),
                    0b0010 => Some(vec![(e.0, c.1, e.1)]),
                    0b0100 => Some(vec![(e.3, e.2, c.3)]),
                    0b1000 => Some(vec![(e.1, c.2, e.2)]),

                    // big corners
                    0b0111 => Some(vec![(c.0, c.1, e.1), (c.0, e.1, e.2), (c.0, e.2, c.3)]),
                    0b1011 => Some(vec![(c.1, c.2, e.2), (c.1, e.2, e.3), (c.1, e.3, c.0)]),
                    0b1101 => Some(vec![(c.3, c.0, e.0), (c.3, e.0, e.1), (c.3, e.1, c.2)]),
                    0b1110 => Some(vec![(c.2, c.3, e.3), (c.2, e.3, e.0), (c.2, e.0, c.1)]),

                    // edges
                    0b0011 => Some(vec![(c.0, c.1, e.1), (c.0, e.1, e.3)]),
                    0b1100 => Some(vec![(e.1, c.2, c.3), (e.1, c.3, e.3)]),
                    0b0101 => Some(vec![(c.0, e.0, e.2), (c.0, e.2, c.3)]),
                    0b1010 => Some(vec![(c.1, c.2, e.2), (c.1, e.2, e.0)]),

                    // diagonals
                    0b1001 => Some(vec![(c.0, e.0, e.3), (e.1, c.2, e.2)]),
                    0b0110 => Some(vec![(c.1, e.1, e.0), (c.3, e.3, e.2)]),

                    0b1111 => Some(vec![(c.0, c.1, c.2), (c.0, c.2, c.3)]),

                    _ => None,
                } {
                    triangles.append(&mut triangle);
                }
            }
        }

        triangles
    }

    pub fn gen_attributes(
        &self,
        threshold: f32,
        smooth: bool,
        bridges: &Option<[Option<&Grid>; 3]>,
    ) -> MeshAttributes {
        let mut positions = Vec::new();
        let mut colours = Vec::new();
        let mut indices = Vec::new();
        let mut vs = Vec::new();

        let triangles = self.gen_triangles(threshold, smooth, bridges);

        for (i, triangle) in triangles.iter().enumerate() {
            positions.push([triangle.0.0, triangle.0.1, 0.]);
            positions.push([triangle.1.0, triangle.1.1, 0.]);
            positions.push([triangle.2.0, triangle.2.1, 0.]);

            let c = [0.5, 0.5, 0.5, 1.];
            colours.push(c);
            colours.push(c);
            colours.push(c);

            let i = i as u32;
            indices.push(i * 3);
            indices.push(i * 3 + 1);
            indices.push(i * 3 + 2);

            vs.push(
                match self.gets_bridge(triangle.0.0, triangle.0.1, bridges) {
                    Some(v) => (v - threshold) / (1. - threshold),
                    None => 0.,
                },
            );
            vs.push(
                match self.gets_bridge(triangle.1.0, triangle.1.1, bridges) {
                    Some(v) => (v - threshold) / (1. - threshold),
                    None => 0.,
                },
            );
            vs.push(
                match self.gets_bridge(triangle.2.0, triangle.2.1, bridges) {
                    Some(v) => (v - threshold) / (1. - threshold),
                    None => 0.,
                },
            );
        }

        let indices = Indices::U32(indices);

        MeshAttributes {
            positions,
            colours,
            indices,
            vs,
        }
    }
    // pub fn bundle(
    //     &mut self,
    //     meshes: &mut Assets<Mesh>,
    //     materials: &mut Assets<GridMaterial>,
    //     threshold: f32,
    //     smooth: bool,
    //     bridges: &Option<[Option<&Grid>; 3]>,
    // ) -> impl Bundle {
    //     let attributes = self.gen_attributes(threshold, smooth, bridges);
    //     let mut mesh = Mesh::new(
    //         bevy::mesh::PrimitiveTopology::TriangleList,
    //         RenderAssetUsages::default(),
    //     );

    //     mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, attributes.positions);
    //     mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, attributes.colours);
    //     mesh.insert_attribute(ATTRIBUTE_V, attributes.vs);
    //     mesh.insert_indices(attributes.indices);

    //     let mesh = meshes.add(mesh);

    //     self.mesh = Some(mesh.clone());

    //     (
    //         Mesh2d(mesh),
    //         MeshMaterial2d(materials.add(GridMaterial {
    //             color: LinearRgba::WHITE,
    //         })),
    //         Transform::from_translation(Vec3::new(self.x, self.y, 0.))
    //             .with_scale(Vec3::splat(self.spacing)),
    //     )
    // }
    pub fn spawn_attributes(
        &mut self,
        commands: &mut Commands,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<GridMaterial>,
        attributes: MeshAttributes,
    ) {
        let mut mesh = Mesh::new(
            bevy::mesh::PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        );

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, attributes.positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, attributes.colours);
        mesh.insert_attribute(ATTRIBUTE_V, attributes.vs);
        mesh.insert_indices(attributes.indices);

        let mesh = meshes.add(mesh);

        self.mesh = Some(mesh.clone());

        self.entity = Some(
            commands
                .spawn((
                    Mesh2d(mesh),
                    MeshMaterial2d(materials.add(GridMaterial {
                        color: LinearRgba::WHITE,
                    })),
                    Transform::from_translation(Vec3::new(self.x, self.y, 0.))
                        .with_scale(Vec3::splat(self.spacing)),
                    GameEntity,
                ))
                .id(),
        );
    }
    // pub fn despawn(&self, commands: &mut Commands) {
    //     if let Some(entity) = self.entity {
    //         commands.entity(entity).despawn();
    //     }
    // }
    pub fn set_mesh(&self, meshes: &mut Assets<Mesh>, attributes: MeshAttributes) {
        let Some(mesh) = &self.mesh else {
            return;
        };

        let Some(mesh) = meshes.get_mut(mesh) else {
            return;
        };

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, attributes.positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, attributes.colours);
        mesh.insert_attribute(ATTRIBUTE_V, attributes.vs);
        mesh.insert_indices(attributes.indices);
    }
    pub fn in_viewport(&self, camera: &Camera, camera_transform: &GlobalTransform) -> bool {
        in_viewport(Vec2::new(self.x, self.y), camera, camera_transform)
            || in_viewport(
                Vec2::new(
                    self.x + self.width as f32 * self.spacing,
                    self.y + self.height as f32 * self.spacing,
                ),
                camera,
                camera_transform,
            )
            || in_viewport(
                Vec2::new(self.x + self.width as f32 * self.spacing, self.y),
                camera,
                camera_transform,
            )
            || in_viewport(
                Vec2::new(self.x, self.y + self.height as f32 * self.spacing),
                camera,
                camera_transform,
            )
    }
}

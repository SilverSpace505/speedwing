use bevy::{
    asset::RenderAssetUsages,
    mesh::{MeshVertexAttribute, PrimitiveTopology, VertexFormat},
    prelude::*,
    render::render_resource::{
        AsBindGroup, BlendComponent, BlendFactor, BlendOperation, BlendState,
    },
    shader::ShaderRef,
    sprite_render::Material2d,
};

use crate::{common::GameEntity, grid_map::GridMap};

const ATTRIBUTE_COLOUR: MeshVertexAttribute =
    MeshVertexAttribute::new("Colour", 188540917, VertexFormat::Float32x4);

const ATTRIBUTE_SIZE: MeshVertexAttribute =
    MeshVertexAttribute::new("Size", 288540917, VertexFormat::Float32);

struct MeshAttributes {
    positions: Vec<[f32; 3]>,
    colours: Vec<[f32; 4]>,
    sizes: Vec<f32>,
}

#[derive(Asset, TypePath, Clone, AsBindGroup)]
pub struct ParticlesMaterial {}

impl Material2d for ParticlesMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/particles.wgsl".into()
    }
    fn fragment_shader() -> ShaderRef {
        "shaders/particles.wgsl".into()
    }
    fn specialize(
        descriptor: &mut bevy::render::render_resource::RenderPipelineDescriptor,
        layout: &bevy::mesh::MeshVertexBufferLayoutRef,
        _key: bevy::sprite_render::Material2dKey<Self>,
    ) -> Result<(), bevy::render::render_resource::SpecializedMeshPipelineError> {
        let vertex_layout = layout.0.get_layout(&[
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
            ATTRIBUTE_COLOUR.at_shader_location(1),
            ATTRIBUTE_SIZE.at_shader_location(2),
        ])?;
        descriptor.vertex.buffers = vec![vertex_layout];

        if let Some(target) = descriptor
            .fragment
            .as_mut()
            .and_then(|f| f.targets.get_mut(0))
            .and_then(|t| t.as_mut())
        {
            target.blend = Some(BlendState {
                color: BlendComponent {
                    src_factor: BlendFactor::SrcAlpha,
                    dst_factor: BlendFactor::One,
                    operation: BlendOperation::Add,
                },
                alpha: BlendComponent {
                    src_factor: BlendFactor::One,
                    dst_factor: BlendFactor::One,
                    operation: BlendOperation::Min,
                },
            })
        }

        Ok(())
    }
}

#[derive(Resource)]
pub struct Particles {
    particles: Vec<Particle>,
    mesh: Handle<Mesh>,
}

impl Particles {
    pub fn spawn_cmd(
        commands: &mut Commands,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<ParticlesMaterial>,
    ) -> Self {
        let mut mesh = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        );
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, Vec::<[f32; 3]>::new());
        mesh.insert_attribute(ATTRIBUTE_COLOUR, Vec::<[f32; 4]>::new());
        mesh.insert_attribute(ATTRIBUTE_SIZE, Vec::<f32>::new());

        let mesh = meshes.add(mesh);
        commands.spawn((
            Mesh2d(mesh.clone()),
            MeshMaterial2d(materials.add(ParticlesMaterial {})),
            Transform::default(),
            GameEntity,
        ));

        Self {
            particles: Vec::new(),
            mesh,
        }
    }
    pub fn spawn(
        &mut self,
        position: Vec2,
        velocity: Vec2,
        colour: Color,
        size: f32,
        lifetime: f32,
    ) {
        self.particles.push(Particle {
            position,
            velocity,
            colour,
            size,
            lifetime,
            time: 0.,
        })
    }
    fn move_particles(&mut self, delta: f32, grid_map: &GridMap) {
        for particle in self.particles.iter_mut() {
            particle.time += delta;

            particle
                .colour
                .set_alpha((1. - particle.time / particle.lifetime).powi(2));
        }
        self.particles.retain(|p| p.time <= p.lifetime);
        for particle in self.particles.iter_mut() {
            particle.position += particle.velocity * delta;

            if grid_map
                .get_world(particle.position.x, particle.position.y)
                .is_some_and(|v| v > 0.5)
                && let Some(normal) =
                    grid_map.get_normal_world(particle.position.x, particle.position.y)
            {
                particle.position -= particle.velocity * delta;
                let normal = Vec2::new(normal.0, normal.1);
                particle.velocity -= normal * 100.;
            }
        }
    }
    fn construct_attributes(&self) -> MeshAttributes {
        let mut positions = Vec::new();
        let mut colours = Vec::new();
        let mut sizes = Vec::new();

        for particle in self.particles.iter() {
            for _ in 0..6 {
                positions.push([particle.position.x, particle.position.y, 0.]);
                let c = particle.colour.to_linear();
                colours.push([c.red, c.green, c.blue, c.alpha]);
                sizes.push(particle.size);
            }
        }

        MeshAttributes {
            positions,
            colours,
            sizes,
        }
    }
    pub fn update(
        time: Res<Time>,
        grid_map: Res<GridMap>,
        mut particles: ResMut<Particles>,
        mut meshes: ResMut<Assets<Mesh>>,
    ) {
        particles.move_particles(time.delta_secs(), &grid_map);

        let attributes = particles.construct_attributes();

        let Some(mesh) = meshes.get_mut(&particles.mesh) else {
            return;
        };

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, attributes.positions);
        mesh.insert_attribute(ATTRIBUTE_COLOUR, attributes.colours);
        mesh.insert_attribute(ATTRIBUTE_SIZE, attributes.sizes);
    }
}

struct Particle {
    position: Vec2,
    velocity: Vec2,
    colour: Color,
    size: f32,
    lifetime: f32,
    time: f32,
}

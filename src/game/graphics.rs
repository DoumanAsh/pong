/// Converts a vector of vertices into a mesh.
pub fn create_mesh(world: &amethyst::prelude::World, vertices: Vec<amethyst::renderer::PosTex>) -> amethyst::renderer::MeshHandle {
    let loader = world.read_resource::<amethyst::assets::Loader>();
    loader.load_from_data(vertices.into(), (), &world.read_resource())
}

/// Creates a solid material of the specified colour.
pub fn create_colour_material(world: &amethyst::prelude::World, colour: [f32; 4]) -> amethyst::renderer::Material {
    let mat_defaults = world.read_resource::<amethyst::renderer::MaterialDefaults>();
    let loader = world.read_resource::<amethyst::assets::Loader>();

    let albedo = loader.load_from_data(colour.into(), (), &world.read_resource());

    amethyst::renderer::Material {
        albedo,
        ..mat_defaults.0.clone()
    }
}

//TODO: find math behind it
/// Generates vertices for a circle. The circle will be made of `resolution`
/// triangles.
pub fn generate_circle_vertices(radius: f32, resolution: usize) -> Vec<amethyst::renderer::PosTex> {
    use std::f32::consts::PI;

    let mut vertices = Vec::with_capacity(resolution * 3);
    let angle_offset = 2.0 * PI / resolution as f32;

    // Helper function to generate the vertex at the specified angle.
    let generate_vertex = |angle: f32| {
        let x = angle.cos();
        let y = angle.sin();
        amethyst::renderer::PosTex {
            position: [x * radius, y * radius, 0.0],
            tex_coord: [x, y],
        }
    };

    for index in 0..resolution {
        vertices.push(amethyst::renderer::PosTex {
            position: [0.0, 0.0, 0.0],
            tex_coord: [0.0, 0.0],
        });

        vertices.push(generate_vertex(angle_offset * index as f32));
        vertices.push(generate_vertex(angle_offset * (index + 1) as f32));
    }

    vertices
}

//TODO: find math behind it
/// Generates six vertices forming a rectangle.
pub fn generate_rectangle_vertices(left: f32, bottom: f32, right: f32, top: f32) -> Vec<amethyst::renderer::PosTex> {
    vec![
        amethyst::renderer::PosTex {
            position: [left, bottom, 0.0],
            tex_coord: [0.0, 0.0],
        },
        amethyst::renderer::PosTex {
            position: [right, bottom, 0.0],
            tex_coord: [1.0, 0.0],
        },
        amethyst::renderer::PosTex {
            position: [left, top, 0.0],
            tex_coord: [1.0, 1.0],
        },
        amethyst::renderer::PosTex {
            position: [right, top, 0.0],
            tex_coord: [1.0, 1.0],
        },
        amethyst::renderer::PosTex {
            position: [left, top, 0.],
            tex_coord: [0.0, 1.0],
        },
        amethyst::renderer::PosTex {
            position: [right, bottom, 0.0],
            tex_coord: [0.0, 0.0],
        },
    ]
}

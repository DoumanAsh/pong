use amethyst::prelude::{Builder, World};
use amethyst::core::transform::Transform;
//TODO: Projection will be merged with Camera
use amethyst::renderer::{Camera, Projection};

pub fn initialize(world: &mut World) {
    let mut transform = Transform::default();
    transform.translation.z = 1.0;

    let camera = Projection::orthographic(0.0, super::ARENA_WIDTH, super::ARENA_HEIGHT, 0.0);
    let camera = Camera::from(camera);

    world.create_entity_unchecked()
         .with(camera)
         .with(transform)
         .build();
}

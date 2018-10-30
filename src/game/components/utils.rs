use amethyst::assets::SimpleFormat;
use amethyst::ecs::{WriteStorage, world::EntitiesRes};
use amethyst::ui::{UiImage, UiTransform};
use amethyst::utils::time_destroy::DestroyInTime;

use super::gameplay;

const NEPU: &'static [u8] = include_bytes!("../../../resources/images/Nepu.png");

pub struct Images {
    pub nepu: amethyst::renderer::TextureHandle,
}

impl Images {
    pub fn spawn_nepu<'s>(&self, entities: &EntitiesRes, side: gameplay::Side, ui_transform: &mut WriteStorage<'s, UiTransform>, ui_image: &mut WriteStorage<'s, UiImage>, time_destroy: &mut WriteStorage<'s, DestroyInTime>) {
        let timer = DestroyInTime { timer: 0.5 };
        let image = amethyst::ui::UiImage { texture: self.nepu.clone() };

        let (anchor, transform_x) = match side {
            gameplay::Side::Right => (amethyst::ui::Anchor::TopRight, -60.0),
            gameplay::Side::Left => (amethyst::ui::Anchor::TopLeft, 60.0),
        };

        let transform = amethyst::ui::UiTransform::new("NEPU".to_owned(),
                                                       anchor,
                                                       transform_x, -50.0, 1.0,
                                                       200.0, 134.0,
                                                       0);

        entities.build_entity()
                .with(transform, ui_transform)
                .with(image, ui_image)
                .with(timer, time_destroy)
                .build();
    }
}

fn load_texture(data: &'static [u8], world: &mut amethyst::prelude::World) -> amethyst::renderer::TextureHandle {
    let img = amethyst::renderer::PngFormat.import(data.to_owned(), amethyst::renderer::TextureMetadata::srgb()).expect("To import builtin image");
    world.read_resource::<amethyst::assets::Loader>().load_from_data(img, (), &world.read_resource())
}

pub fn init_pop_up(world: &mut amethyst::prelude::World) {
    let nepu = load_texture(NEPU, world);

    let images = Images {
        nepu,
    };

    world.add_resource(images);
}

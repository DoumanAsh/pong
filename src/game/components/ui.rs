use amethyst::prelude::{Builder, World};
use amethyst::assets::SimpleFormat;

const FONT: &'static [u8] = include_bytes!("../../../resources/fonts/georgia.ttf");
const SCORE_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0]; //white
const INITIAL_SCORE: &'static str = "0";
const SCORE_WIDTH: f32 = 200.0;
const SCORE_HEIGHT: f32 = 50.0;

pub struct Images {
    overlay: amethyst::renderer::TextureHandle,
}
pub struct Fonts {
    pub default: amethyst::ui::FontHandle
}
pub struct ScoreText {
    pub p1: amethyst::ecs::Entity,
    pub p2: amethyst::ecs::Entity,
}

pub fn create_images(world: &mut World) {
    const OVERLAY_DATA: [f32; 4] = [144.0, 144.0, 144.0, 0.5];
    let overlay = world.read_resource::<amethyst::assets::Loader>().load_from_data(OVERLAY_DATA.into(), (), &world.read_resource());

    world.add_resource(Images {
        overlay
    });
}

pub fn get_default_font(world: &mut World) -> amethyst::ui::FontHandle {
    let font = amethyst::ui::TtfFormat.import(FONT.to_owned(), ()).expect("To import builtin font");
    let font = world.read_resource::<amethyst::assets::Loader>().load_from_data(font, (), &world.read_resource());

    world.add_resource(Fonts {
        default: font.clone()
    });

    font
}

pub fn init_score_board(world: &mut World, font: amethyst::ui::FontHandle) {
    let first_score = amethyst::ui::UiText::new(font.clone(), INITIAL_SCORE.to_string(), SCORE_COLOR, 50.0);
    let p1 = world.create_entity_unchecked()
                  .with(amethyst::ui::UiTransform::new(
                          "P1".to_string(),
                          amethyst::ui::Anchor::TopMiddle,
                          -50.0, -50.0, 1.0, //position
                          SCORE_WIDTH, SCORE_HEIGHT,
                          0
                  ))
                  .with(first_score)
                  .build();

    let second_score = amethyst::ui::UiText::new(font.clone(), INITIAL_SCORE.to_string(), SCORE_COLOR, 50.0);
    let p2 = world.create_entity_unchecked()
                  .with(amethyst::ui::UiTransform::new(
                          "P2".to_string(),
                          amethyst::ui::Anchor::TopMiddle,
                          50.0, -50.0, 1.0, //position
                          SCORE_WIDTH, SCORE_HEIGHT,
                          0
                  ))
                  .with(second_score)
                  .build();

    world.add_resource(ScoreText { p1, p2 });
}

pub struct Paused {
    overlay: amethyst::ecs::prelude::Entity,
    message: amethyst::ecs::prelude::Entity,
}

impl Paused {
    pub fn destroy(self, world: &mut World) {
        let _ = world.delete_entity(self.overlay);
        let _ = world.delete_entity(self.message);
    }
}

pub fn create_paused_ui(world: &mut World) -> Paused {
    let texture = world.read_resource::<Images>().overlay.clone();
    let image = amethyst::ui::UiImage { texture };
    let overlay = world.create_entity()
                       .with(amethyst::ui::UiTransform::new(
                               "Overlay".to_string(),
                               amethyst::ui::Anchor::Middle,
                               0.0, 0.0, 1.5,
                               1000.0, 1000.0,
                               0
                       ).as_percent())
                       .with(image)
                       .build();

    let text = amethyst::ui::UiText::new(world.read_resource::<Fonts>().default.clone(), "PAUSE".to_string(), SCORE_COLOR, 70.0);
    let message = world.create_entity()
                       .with(amethyst::ui::UiTransform::new(
                               "Pause".to_string(),
                               amethyst::ui::Anchor::Middle,
                               0.0, 0.0, 2.0, //position
                               200.0, 200.0,
                               0
                       ))
                       .with(text)
                       .build();

    Paused {
        overlay,
        message,
    }
}

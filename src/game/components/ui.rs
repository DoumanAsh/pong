use amethyst::prelude::{Builder, World};
use amethyst::assets::SimpleFormat;

const FONT: &'static [u8] = include_bytes!("../../../resources/fonts/georgia.ttf");
const SCORE_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0]; //white
const INITIAL_SCORE: &'static str = "0";
const SCORE_WIDTH: f32 = 200.0;
const SCORE_HEIGHT: f32 = 50.0;

pub struct Fonts {
    pub default: amethyst::ui::FontHandle
}
pub struct ScoreText {
    pub p1: amethyst::ecs::Entity,
    pub p2: amethyst::ecs::Entity,
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

pub fn create_paused_ui(world: &mut World) -> amethyst::ecs::prelude::Entity {
    let fonts = world.read_resource::<Fonts>();
    let text = amethyst::ui::UiText::new(fonts.default.clone(), "PAUSE".to_string(), SCORE_COLOR, 70.0);
    world.create_entity_unchecked()
         .with(amethyst::ui::UiTransform::new(
                 "Pause".to_string(),
                 amethyst::ui::Anchor::Middle,
                 0.0, 0.0, 1.0, //position
                 200.0, 200.0,
                 0
         ))
         .with(text)
         .build()
}

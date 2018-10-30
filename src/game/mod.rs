use amethyst::{StateData, GameData, Application, GameDataBuilder, SimpleState};
use amethyst::input::InputBundle;
use amethyst::renderer::{DisplayConfig, Pipeline, RenderBundle, Stage};
use amethyst::core::transform::bundle::TransformBundle;

const ARENA_HEIGHT: f32 = 100.0;
const ARENA_WIDTH: f32 = 100.0;
const ARENA_HEIGHT_MIDDLE: f32 = ARENA_HEIGHT / 2.0;
const ARENA_WIDTH_MIDDLE: f32 = ARENA_WIDTH / 2.0;

mod audio;
mod camera;
mod graphics;
mod components;
mod systems;

pub struct Game;

macro_rules! init_world {
    ($data:ident: [$($component:ident, )+]) => {
        $(
            $component::initialize($data.world);
         )+
    }
}

impl<'a, 'b> SimpleState<'a, 'b> for Game {
    fn on_start(&mut self, data: StateData<GameData>) {
        init_world!(data: [camera, components, audio,]);
    }
}

fn get_resource_dir() -> std::path::PathBuf {
    let mut path = std::env::current_exe().unwrap();
    path.pop();

    path
}

fn get_display_config() -> DisplayConfig {
    DisplayConfig {
        title: "Nepu Pong".to_owned(),
        dimensions: None,
        max_dimensions: None,
        min_dimensions: None,
        fullscreen: false,
        multisampling: 1,
        visibility: true,
        vsync: true,
    }
}

fn get_input_config() -> InputBundle<String, String> {
    use std::io::Write;

    const DEFAULT_BINDINGS: &'static [u8] = include_bytes!("../../resources/bindings_config.ron");
    let mut path = get_resource_dir();
    path.push("bindings_config.ron");

    if !path.is_file() {
        let mut file = std::fs::File::create(&path).expect("To create default bindings file");
        let _ = file.write_all(DEFAULT_BINDINGS);
    }

    InputBundle::<String, String>::new().with_bindings_from_file(path).expect("To load input config")
}

pub fn run() -> amethyst::Result<()> {
    //Clear screen with black
    //clear_target takes RGB colour
    let pipe = Stage::with_backbuffer().clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
                                       .with_pass(amethyst::renderer::DrawFlat::<amethyst::renderer::PosTex>::new())
                                       .with_pass(amethyst::ui::DrawUi::new());

    let pipe = Pipeline::build().with_stage(pipe);
    let pipe = RenderBundle::new(pipe, Some(get_display_config()));

    let game_data = GameDataBuilder::default().with_bundle(pipe).expect("To add bundle")
                                              .with_bundle(TransformBundle::new()).expect("To add bundle")
                                              .with_bundle(get_input_config()).expect("To add bundle")
                                              .with_bundle(amethyst::audio::AudioBundle::new(|_: &mut audio::Sounds| None)).expect("To add bundle")
                                              .with_bundle(amethyst::ui::UiBundle::<String, String>::new()).expect("To add bundle")
                                              .with(amethyst::utils::time_destroy::TimedDestroySystem, "TimedDestroySystem", &[])
                                              .with(systems::PaddleSystem, systems::paddle::NAME, &["input_system"])
                                              .with(systems::BallMove, systems::ball::MOVE, &[])
                                              .with(systems::BallCollision, systems::ball::COLLISION, &[systems::ball::MOVE, systems::paddle::NAME]);

    Application::build("./", Game).expect("Create application builder")
                                  .build(game_data)
                                  .expect("Build application")
                                  .run();

    Ok(())
}

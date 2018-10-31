use amethyst::{Application};
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

struct GameData<'a, 'b> {
    basic: amethyst::ecs::prelude::Dispatcher<'a, 'b>,
    running: amethyst::ecs::prelude::Dispatcher<'a, 'b>,
}

trait DispatcherSelector {
    fn select<'a, 'b, 'c>(builder: &'c mut GameDataBuilder<'a, 'b>) -> &'c mut amethyst::ecs::prelude::DispatcherBuilder<'a, 'b>;
}

struct Base;
impl DispatcherSelector for Base {
    fn select<'a, 'b, 'c>(builder: &'c mut GameDataBuilder<'a, 'b>) -> &'c mut amethyst::ecs::prelude::DispatcherBuilder<'a, 'b> {
        &mut builder.basic
    }
}
struct Running;
impl DispatcherSelector for Running {
    fn select<'a, 'b, 'c>(builder: &'c mut GameDataBuilder<'a, 'b>) -> &'c mut amethyst::ecs::prelude::DispatcherBuilder<'a, 'b> {
        &mut builder.running
    }
}

#[derive(Default)]
struct GameDataBuilder<'a, 'b> {
    basic: amethyst::ecs::prelude::DispatcherBuilder<'a, 'b>,
    running: amethyst::ecs::prelude::DispatcherBuilder<'a, 'b>,
}

impl<'a, 'b> GameDataBuilder<'a, 'b> {
    fn with_bundle<D: DispatcherSelector, B: amethyst::core::SystemBundle<'a, 'b>>(mut self, bundle: B) -> amethyst::core::bundle::Result<Self> {
        bundle.build(D::select(&mut self))?;
        Ok(self)
    }

    fn with<D: DispatcherSelector, S>(mut self, system: S, name: &str, deps: &[&str]) -> Self where for<'c> S: amethyst::ecs::prelude::System<'c> + Send + 'a{
        D::select(&mut self).add(system, name, deps);
        self
    }
}

impl<'a, 'b> amethyst::DataInit<GameData<'a, 'b>> for GameDataBuilder<'a, 'b> {
    fn build(self, world: &mut amethyst::prelude::World) -> GameData<'a, 'b> {
        let pool = world.read_resource::<amethyst::core::ArcThreadPool>().clone();

        let mut basic = self.basic.with_pool(pool.clone()).build();
        let mut running = self.running.with_pool(pool.clone()).build();
        basic.setup(&mut world.res);
        running.setup(&mut world.res);

        GameData {
            basic,
            running,
        }
    }
}

macro_rules! init_world {
    ($data:ident: [$($component:ident, )+]) => {
        $(
            $component::initialize($data.world);
         )+
    }
}

#[derive(Default)]
struct PausedGame {
    ui: Option<components::ui::Paused>,
}

impl<'a, 'b> amethyst::State<GameData<'a, 'b>, amethyst::StateEvent> for PausedGame {
    fn on_start(&mut self, mut data: amethyst::StateData<GameData>) {
        self.ui = Some(components::ui::create_paused_ui(&mut data.world));
    }

    fn update(&mut self, data: amethyst::StateData<GameData>) -> amethyst::Trans<GameData<'a, 'b>, amethyst::StateEvent> {
        data.data.basic.dispatch(&mut data.world.res);
        amethyst::Trans::None
    }

    fn handle_event(&mut self, mut data: amethyst::StateData<GameData>, event: amethyst::StateEvent) -> amethyst::Trans<GameData<'a, 'b>, amethyst::StateEvent> {
        match event {
            amethyst::StateEvent::Window(event) => if amethyst::input::is_close_requested(&event) {
                amethyst::Trans::Quit
            } else if amethyst::input::is_key_down(&event, amethyst::renderer::VirtualKeyCode::Escape) {
                self.ui.take().unwrap().destroy(&mut data.world);
                amethyst::Trans::Pop
            } else {
                amethyst::Trans::None
            },
            _ => amethyst::Trans::None
        }
    }

}

struct Game;

impl<'a, 'b> amethyst::State<GameData<'a, 'b>, amethyst::StateEvent> for Game {
    fn on_start(&mut self, data: amethyst::StateData<GameData>) {
        init_world!(data: [camera, components, audio,]);
    }

    fn handle_event(&mut self, _: amethyst::StateData<GameData>, event: amethyst::StateEvent) -> amethyst::Trans<GameData<'a, 'b>, amethyst::StateEvent> {
        match event {
            amethyst::StateEvent::Window(event) => if amethyst::input::is_close_requested(&event) {
                amethyst::Trans::Quit
            } else if amethyst::input::is_key_down(&event, amethyst::renderer::VirtualKeyCode::Escape) {
                amethyst::Trans::Push(Box::new(PausedGame::default()))
            } else {
                amethyst::Trans::None
            },
            _ => amethyst::Trans::None

        }
    }

    fn update(&mut self, data: amethyst::StateData<GameData>) -> amethyst::Trans<GameData<'a, 'b>, amethyst::StateEvent> {
        data.data.basic.dispatch(&mut data.world.res);
        data.data.running.dispatch(&mut data.world.res);
        amethyst::Trans::None
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

    let game_data = GameDataBuilder::default().with_bundle::<Base, _>(pipe).expect("To add bundle")
                                              .with_bundle::<Base, _>(TransformBundle::new()).expect("To add bundle")
                                              .with_bundle::<Running, _>(get_input_config()).expect("To add bundle")
                                              .with_bundle::<Base, _>(amethyst::ui::UiBundle::<String, String>::new()).expect("To add bundle")
                                              .with_bundle::<Base, _>(amethyst::audio::AudioBundle::new(|_: &mut audio::Sounds| None)).expect("To add bundle")
                                              .with::<Running, _>(amethyst::utils::time_destroy::TimedDestroySystem, "TimedDestroySystem", &[])
                                              .with::<Running, _>(systems::PaddleSystem, systems::paddle::NAME, &["input_system"])
                                              .with::<Running, _>(systems::BallMove, systems::ball::MOVE, &[])
                                              .with::<Running, _>(systems::BallCollision, systems::ball::COLLISION, &[systems::ball::MOVE, systems::paddle::NAME]);

    Application::build("./", Game).expect("Create application builder")
                                  .build(game_data)
                                  .expect("Build application")
                                  .run();

    Ok(())
}

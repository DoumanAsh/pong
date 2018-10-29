use amethyst::assets::SimpleFormat;

pub struct Music {
}

pub struct Sounds {
    pub nepu: amethyst::audio::SourceHandle,
}

impl Sounds {
    fn play_sound(sound: &amethyst::audio::SourceHandle, storage: &amethyst::assets::AssetStorage<amethyst::audio::Source>, output: Option<&amethyst::audio::output::Output>) {
        if let Some(ref output) = output.as_ref() {
            if let Some(sound) = storage.get(sound) {
                output.play_once(sound, 1.0);
            }
        }
    }

    pub fn play_nepu(&self, storage: &amethyst::assets::AssetStorage<amethyst::audio::Source>, output: Option<&amethyst::audio::output::Output>) {
        Self::play_sound(&self.nepu, storage, output)
    }
}

const NEPU: &'static [u8] = include_bytes!("../../resources/music/NEPU.ogg");

fn load_static_music(world: &amethyst::prelude::World, data: &'static [u8]) -> amethyst::audio::SourceHandle {
    let source = amethyst::audio::OggFormat.import(data.to_owned(), ()).expect("To import builtin sound");
    world.read_resource::<amethyst::assets::Loader>().load_from_data(source, (), &world.read_resource())
}

pub fn initialize(world: &mut amethyst::prelude::World) {
    world.write_resource::<amethyst::audio::AudioSink>().set_volume(0.25);
    let nepu = load_static_music(world, NEPU);

    let sounds = Sounds {
        nepu
    };

    let music = Music {
    };

    world.add_resource(sounds);
    world.add_resource(music);
}

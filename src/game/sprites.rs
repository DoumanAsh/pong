use amethyst::prelude::{World};
use amethyst::renderer::{SpriteSheet, SpriteSheetFormat, SpriteSheetHandle, Texture, PngFormat, TextureMetadata, MaterialTextureSet};
use amethyst::assets::{Loader, AssetStorage};

const PADDLE_TEXTURE_ID: u64 = 0;
const PADDLE_TEXTURE_PATH: &'static str = "resources/texture/pong_spritesheet.png";
const PADDLE_SPRITE_PATH: &'static str = "resources/texture/pong_spritesheet.ron";

pub fn load_paddle(world: &mut World) -> SpriteSheetHandle {
    //Creates handler to loading resource.
    //It is not necessary loaded immediately, but already can be used
    let handle = {
        //This is builtin resource to load assets
        let loader = world.read_resource::<Loader>();
        let storage = world.read_resource::<AssetStorage<Texture>>();

        loader.load(PADDLE_TEXTURE_PATH, PngFormat, TextureMetadata::srgb_scale(), (), &storage)
    };

    //Write new sprite into texture store.
    //Note: write_resource returns mutable writer for resource, and will panic if already borrowed
    //The id is global and used to refer to the texture.
    let mut texture_set = world.write_resource::<MaterialTextureSet>();
    texture_set.insert(PADDLE_TEXTURE_ID, handle);

    //Load RON file that describes sprites
    let loader = world.read_resource::<Loader>();
    let storage = world.read_resource::<AssetStorage<SpriteSheet>>();

    loader.load(PADDLE_SPRITE_PATH, SpriteSheetFormat, PADDLE_TEXTURE_ID, (), &storage)
}

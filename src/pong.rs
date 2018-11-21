use amethyst::{
    assets::{AssetStorage, Loader},
    core::transform::Transform,
    ecs::prelude::World,
    prelude::*,
    renderer::{
        Camera, PngFormat, Projection, SpriteRender, SpriteSheet, SpriteSheetFormat,
        SpriteSheetHandle, Texture, TextureMetadata,
    },
};
use {Paddle};
use {ARENA_HEIGHT, ARENA_WIDTH};

pub struct Pong;

impl<'a, 'b> SimpleState<'a, 'b> for Pong {
    fn on_start(&mut self, data: StateData<GameData>) {
        let StateData { world, .. } = data;
        use audio::initialise_audio;

        let sprite_sheet_handle = load_sprite_sheet(world);

        initialise_paddles(world, sprite_sheet_handle.clone());
        initialise_camera(world);
        initialise_audio(world);
    }
}

fn load_sprite_sheet(world: &mut World) -> SpriteSheetHandle {
    // Load the sprite sheet necessary to render the graphics.
    // The texture is the pixel data
    // `sprite_sheet` is the layout of the sprites on the image
    // `texture_handle` is a cloneable reference to the texture
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "./texture/pong_spritesheet.png",
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "./texture/pong_spritesheet.ron", // Here we load the associated ron file
        SpriteSheetFormat,
        texture_handle, // We pass it the texture we want it to use
        (),
        &sprite_sheet_store,
    )
}

/// Initialise the camera.
fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_z(1.0);
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0,
            ARENA_WIDTH,
            0.0,
            ARENA_HEIGHT,
        ))).with(transform)
        .build();
}

fn initialise_paddles(world: &mut World, sprite_sheet_handle: SpriteSheetHandle) {
    use {PADDLE_HEIGHT, PADDLE_VELOCITY, PADDLE_WIDTH};

    let mut transform = Transform::default();

    transform.set_xyz(
        (ARENA_WIDTH - PADDLE_WIDTH) / 2.0,
        (ARENA_HEIGHT - PADDLE_HEIGHT) / 2.0,
        0.0);

    let sprite = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 0, // paddle is the first sprite in the sprite_sheet
        flip_horizontal: false,
        flip_vertical: false,
    };

    world
        .create_entity()
        .with(sprite)
        .with(Paddle {
            width: PADDLE_WIDTH,
            height: PADDLE_HEIGHT,
            velocity: PADDLE_VELOCITY,
        })
        .with(transform)
        .build();
}
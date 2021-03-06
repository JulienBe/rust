extern crate amethyst;

mod audio;
mod bundle;
mod pong;
mod systems;

use amethyst::{
    audio::AudioBundle,
    core::{frame_limiter::FrameRateLimitStrategy, transform::TransformBundle},
    ecs::prelude::{Component, DenseVecStorage},
    input::InputBundle,
    prelude::*,
    renderer::{DisplayConfig, DrawSprite, Pipeline, RenderBundle, Stage},
    ui::{DrawUi, UiBundle},
    utils::application_root_dir,
};

use audio::Music;
use bundle::PongBundle;
use std::time::Duration;

const ARENA_HEIGHT: f32 = 100.0;
const ARENA_WIDTH: f32 = 100.0;
const PADDLE_HEIGHT: f32 = 1.0;
const PADDLE_WIDTH: f32 = 1.0;
const PADDLE_VELOCITY: f32 = 75.0;

const AUDIO_MUSIC: &'static [&'static str] = &[
    "audio/Computer_Music_All-Stars_-_Wheres_My_Jetpack.ogg",
    "audio/Computer_Music_All-Stars_-_Albatross_v2.ogg",
];
const AUDIO_BOUNCE: &'static str = "audio/bounce.ogg";
const AUDIO_SCORE: &'static str = "audio/score.ogg";

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    use pong::Pong;

    let app_root = application_root_dir();

    let display_config_path = format!("{}/resources/display.ron", app_root);
    let config = DisplayConfig::load(&display_config_path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawSprite::new())
            .with_pass(DrawUi::new()),
    );

    let key_bindings_path = {
        if cfg!(feature = "sdl_controller") {
            format!("{}/resources/input_controller.ron", app_root)
        } else {
            format!("{}/resources/input.ron", app_root)
        }
    };

    let assets_dir = format!("{}/", app_root);

    let game_data = GameDataBuilder::default()
        .with_bundle(
            InputBundle::<String, String>::new().with_bindings_from_file(&key_bindings_path)?,
        )?.with_bundle(PongBundle)?
        .with_bundle(RenderBundle::new(pipe, Some(config)).with_sprite_sheet_processor())?
        .with_bundle(TransformBundle::new().with_dep(&["paddle_system"]))?
        .with_bundle(AudioBundle::new(|music: &mut Music| music.music.next()))?
        .with_bundle(UiBundle::<String, String>::new())?;
    let mut game = Application::build(assets_dir, Pong)?
        .with_frame_limit(
            FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(2)),
            144,
        ).build(game_data)?;
    game.run();
    Ok(())
}

pub struct Paddle {
    pub velocity: f32,
    pub width: f32,
    pub height: f32,
}

impl Paddle {
    pub fn new() -> Paddle {
        Paddle {
            velocity: 1.0,
            width: 1.0,
            height: 1.0,
        }
    }
}

impl Component for Paddle {
    type Storage = DenseVecStorage<Self>;
}
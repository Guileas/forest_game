extern crate amethyst;

use amethyst::{
    core::TransformBundle,
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

mod systems;
mod components;
mod entities;
mod forest;

use crate::{forest::Forest};

//use crate::forest::Forest;

fn main() -> amethyst::Result<()> {
    //log error and warning
    amethyst::start_logger(Default::default());

    //set basic files
    let app_root = application_root_dir()?;
    let asset_dir = app_root.join("assets");

    //get game param from config .ron file
    let display_config_path = app_root.join("config").join("display.ron");

    let game_data = GameDataBuilder::default()
        // Handles tracking entity positions
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                //plugin to open window
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        //define window background color
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                //plugin to render sprite
                .with_plugin(RenderFlat2D::default()),
        )?
        // call the animation system for hero animation
        .with(systems::AnimationSystem, "animation_system", &[]);

    let mut game = Application::new(asset_dir, Forest::default(), game_data)?;
    game.run();
    Ok(())
}

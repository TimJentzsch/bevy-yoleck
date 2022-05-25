use std::path::Path;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_yoleck::tools_3d::{
    transform_edit_adapter, OrbitCameraBundle, OrbitCameraController, Tools3DCameraBundle,
    Transform3dProjection,
};
use bevy_yoleck::{
    YoleckEditorLevelsDirectoryPath, YoleckExtForApp, YoleckLoadingCommand,
    YoleckPluginForEditor, YoleckPluginForGame, YoleckPopulate, YoleckTypeHandlerFor,
};
use serde::{Deserialize, Serialize};

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    let level = std::env::args().nth(1);
    if let Some(level) = level {
        app.add_plugin(YoleckPluginForGame);
        app.add_startup_system(
            move |asset_server: Res<AssetServer>,
                  mut yoleck_loading_command: ResMut<YoleckLoadingCommand>| {
                *yoleck_loading_command = YoleckLoadingCommand::FromAsset(
                    asset_server.load(Path::new("levels3d").join(&level)),
                );
            },
        );
    } else {
        app.add_plugin(EguiPlugin);
        app.add_plugin(YoleckPluginForEditor);
        app.insert_resource(YoleckEditorLevelsDirectoryPath(
            Path::new(".").join("assets").join("levels3d"),
        ));
        app.add_plugin(bevy_yoleck::tools_3d::YoleckTools3dPlugin);
    }
    app.add_yoleck_handler({
        YoleckTypeHandlerFor::<Spaceship>::new("Spaceship")
            .populate_with(populate_spaceship)
            .with(transform_edit_adapter(|data: &mut Spaceship| {
                Transform3dProjection {
                    translation: &mut data.position,
                    rotation: Some(&mut data.rotation),
                }
            }))
    });
    app.init_resource::<GameAssets>();
    app.add_startup_system(setup_camera);
    app.run();
}

struct GameAssets {
    spaceship_model: Handle<Scene>,
}

impl FromWorld for GameAssets {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        Self {
            spaceship_model: asset_server.load("models/spaceship.glb#Scene0"),
        }
    }
}

fn setup_camera(mut commands: Commands) {
    let camera = Tools3DCameraBundle::new(OrbitCameraBundle::new(
        {
            let mut controller = OrbitCameraController::default();
            controller.mouse_translate_sensitivity *= 10.0;
            controller
        },
        PerspectiveCameraBundle::new_3d(),
        Vec3::new(0.0, 100.0, 0.0),
        Vec3::ZERO,
    ));
    commands.spawn_bundle(camera);
}

#[derive(Component)]
struct IsSpaceship;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct Spaceship {
    #[serde(default)]
    position: Vec3,
    #[serde(default)]
    rotation: Quat,
}

fn populate_spaceship(mut populate: YoleckPopulate<Spaceship>, assets: Res<GameAssets>) {
    populate.populate(|_ctx, data, mut cmd| {
        cmd.despawn_descendants();
        cmd.insert_bundle(TransformBundle {
            local: Transform::from_translation(data.position).with_rotation(data.rotation),
            ..Default::default()
        }).with_children(|commands| {
            commands.spawn_scene(assets.spaceship_model.clone());
        });
        cmd.insert(IsSpaceship);
    });
}
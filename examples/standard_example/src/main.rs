use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bevy_ui_proxy::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            BevyUiProxyPlugin,
            WorldInspectorPlugin::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
    let proxy_target_entity = commands
        .spawn(MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle { radius: 50.0 })),
            material: materials.add(Color::BLUE),
            ..default()
        })
        .id();
    commands
        .spawn(SpatialBundle {
            transform: Transform::from_xyz(100., 100., 0.),
            ..default()
        })
        .add_child(proxy_target_entity);
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                ..default()
            },
            ..default()
        })
        .with_children(|children| {
            children
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(50.),
                        height: Val::Percent(100.),
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    background_color: Color::MIDNIGHT_BLUE.into(),
                    ..default()
                })
                .with_children(|children| {
                    children.spawn(TextBundle {
                        text: Text::from_section(
                            "this is ui text",
                            TextStyle {
                                color: Color::WHITE,
                                font_size: 100.0,
                                ..default()
                            },
                        ),
                        style: Style {
                            align_self: AlignSelf::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
            children
                .spawn(ProxyUi::proxy(proxy_target_entity))
                .insert(NodeBundle {
                    style: Style {
                        width: Val::Percent(50.),
                        height: Val::Percent(100.),
                        ..default()
                    },
                    ..default()
                });
        });
}

#[allow(clippy::type_complexity)]
fn update(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    query: Query<(Entity, &ProxyUiNodeState), (With<ProxyUiProxied>, Changed<ProxyUiNodeState>)>,
) {
    for (entity, node_state) in query.iter() {
        let scale = *node_state.get_ui_scale();
        let rect = node_state.get_ui_rect();
        let rect = Rect {
            min: rect.min * scale,
            max: rect.max * scale,
        };
        let radius = {
            let mut radius_val = rect.height().min(rect.width());
            if radius_val == 0.0 {
                radius_val = rect.height().max(rect.width());
            }
            // radius has to be non zero
            (radius_val * scale / 2.0).max(0.001)
        };
        commands
            .entity(entity)
            .insert(Mesh2dHandle(meshes.add(Circle { radius })))
            // if we want to maintain the relative transform to our parent
            //.insert(*node_state.get_transform())
            // if we want to ignore the relative transform to our parent
            .insert(GlobalTransform::from(
                node_state.get_transform().compute_affine(),
            ))
            .insert(node_state.get_computed_visibility());
    }
}

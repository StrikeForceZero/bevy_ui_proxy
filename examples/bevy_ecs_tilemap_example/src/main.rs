use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bevy_ui_proxy::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            BevyUiProxyPlugin,
            WorldInspectorPlugin::default(),
        ))
        .add_plugins(TilemapPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run()
}

fn create_tilemap(commands: &mut Commands, asset_server: Res<AssetServer>) -> Entity {
    let map_size = TilemapSize { x: 32, y: 32 };
    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(map_size);
    let texture_handle: Handle<Image> = asset_server.load("tiles.png");

    let tilemap_id = TilemapId(tilemap_entity);
    commands.entity(tilemap_entity).with_children(|tiles| {
        for x in 0..map_size.x {
            for y in 0..map_size.y {
                let tile_pos = TilePos { x, y };
                // Normalize x and y to range between 0.0 and 1.0
                let norm_x = x as f32 / map_size.x as f32;
                let norm_y = y as f32 / map_size.y as f32;
                let color: TileColor =
                    Color::rgba(norm_x, norm_y, (norm_x + norm_y) / 2.0, 1.0).into();
                let tile_entity = tiles
                    .spawn(TileBundle {
                        position: tile_pos,
                        tilemap_id,
                        color,
                        ..Default::default()
                    })
                    .id();
                tile_storage.set(&tile_pos, tile_entity);
            }
        }
    });

    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };
    let grid_size: TilemapGridSize = tile_size.into();
    let map_type = TilemapType::default();

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        tile_size,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        texture: TilemapTexture::Single(texture_handle),
        ..Default::default()
    });

    tilemap_entity
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    let proxy_target_entity = create_tilemap(&mut commands, asset_server);

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
    query: Query<
        (
            Entity,
            &ProxyUiNodeState,
            &TilemapSize,
            &TilemapGridSize,
            &TilemapType,
        ),
        (With<ProxyUiProxied>, Changed<ProxyUiNodeState>),
    >,
) {
    for (entity, node_state, tilemap_size, tilemap_grid_size, tilemap_type) in query.iter() {
        let ui_scale = *node_state.get_ui_scale();

        let world_rect = node_state.get_world_rect();
        let rect = Rect {
            min: world_rect.min * ui_scale,
            max: world_rect.max * ui_scale,
        };

        // blindly casting u32 to f32 because it's unlikely for a tilemap's tile size or the result to be over 2^24
        let tilemap_width = tilemap_grid_size.x * tilemap_size.x as f32;
        let tilemap_height = tilemap_grid_size.y * tilemap_size.y as f32;

        // Calculate center position of the UI rect
        let rect_center = rect.center();
        let rect_center_x = rect_center.x;
        let rect_center_y = rect_center.y;

        // create the transform that gives us the center point for the tilemap
        let tilemap_transform =
            get_tilemap_center_transform(tilemap_size, tilemap_grid_size, tilemap_type, 0.0);

        // create the transform that will scale the tilemap to match the ui rect
        let scale_x = rect.width() / tilemap_width;
        let scale_y = rect.height() / tilemap_height;
        let tilemap_scale = Vec3::new(scale_x, scale_y, 1.0);
        let transform_scale = Transform {
            scale: tilemap_scale,
            ..default()
        };

        // flip y coordinates to match world
        let transform = Transform::from_xyz(rect_center_x, -rect_center_y, 0.0);

        let corrected_global_transform = GlobalTransform::from(transform.compute_affine())
            // scale tilemap to fit full rect
            .mul_transform(transform_scale)
            // apply tilemap transform to get center point
            .mul_transform(tilemap_transform);

        commands
            .entity(entity)
            .insert(corrected_global_transform)
            .insert(node_state.get_computed_visibility());
    }
}

use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::prelude::*;
use crate::util::ui::ui_to_world;

#[allow(clippy::type_complexity)]
#[derive(SystemParam)]
pub(crate) struct ProxyUiUpdateQueries<'w, 's> {
    proxy_ui_query: Query<
        'w,
        's,
        (
            Entity,
            &'static ProxyUiTarget,
            Option<&'static ProxyUiNodeState>,
            &'static Node,
            &'static Style,
            &'static Transform,
            &'static GlobalTransform,
        ),
        (With<ProxyUiTarget>, Without<ProxyUiProxied>),
    >,
    proxy_target_query: Query<
        'w,
        's,
        (Entity, Option<&'static Node>),
        (With<ProxyUiProxied>, Without<ProxyUiTarget>),
    >,
    primary_window_query: Query<'w, 's, &'static Window, With<PrimaryWindow>>,
    camera_query: Query<
        'w,
        's,
        (
            &'static Camera,
            &'static Transform,
            &'static OrthographicProjection,
        ),
        With<Camera>,
    >,
}

pub(crate) fn proxy_ui_update(
    mut commands: Commands,
    ui_scale: Option<Res<UiScale>>,
    queries: ProxyUiUpdateQueries,
) {
    for (
        proxy_ui_entity,
        ui_proxy_target,
        option_proxy_ui_node_state,
        node,
        style,
        transform,
        global_transform,
    ) in queries.proxy_ui_query.iter()
    {
        let ui_rect = node.logical_rect(global_transform);
        // TODO: this should be deferred until a change is detected
        let world_rect = {
            let window_query_result = match queries.primary_window_query.get_single() {
                Ok(window) => Some(window),
                Err(err) => {
                    warn!("failed to get primary window {err:?}");
                    None
                }
            };
            if let Some(window) = window_query_result {
                if let Some((_camera, camera_transform, camera_projection)) =
                    queries.camera_query.iter().find(|(c, ..)| c.is_active)
                {
                    Rect {
                        min: ui_to_world(ui_rect.min, window, camera_transform, camera_projection),
                        max: ui_to_world(ui_rect.max, window, camera_transform, camera_projection),
                    }
                } else {
                    Rect::default()
                }
            } else {
                Rect::default()
            }
        };

        // Res does not implement Default so we can't just do unwrap_or_default()
        let ui_scale = if let Some(ref ui_scale) = ui_scale {
            ui_scale.0
        } else {
            UiScale::default().0
        };

        let new_node_state = ProxyUiNodeState {
            ui_rect,
            world_rect,
            global_transform: *global_transform,
            transform: *transform,
            style: style.clone(),
            ui_scale,
        };

        let proxy_ui_node_state_has_changed = match option_proxy_ui_node_state {
            Some(proxy_ui_node_state) => proxy_ui_node_state.is_changed(&new_node_state),
            _ => true,
        };

        let (proxied_entity, option_bad_node) = queries
            .proxy_target_query
            .get(ui_proxy_target.target_entity())
            .unwrap_or_else(|err| {
                panic!(
                    "failed to find proxy target: {:?} - {err:?}",
                    ui_proxy_target.target_entity()
                )
            });

        if option_bad_node.is_some() {
            warn!("unexpected Ui Node found on proxy target! removing proxy");
            commands.entity(proxied_entity).remove::<ProxyUiProxied>();
            commands.entity(proxy_ui_entity).remove::<ProxyUiTarget>();
            continue;
        }
        if proxy_ui_node_state_has_changed {
            debug!("proxy ui node state updated: {new_node_state:?}");
            commands
                .entity(proxy_ui_entity)
                .insert(new_node_state.clone());
            commands.entity(proxied_entity).insert(new_node_state);
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;
    use bevy::prelude::*;

    use crate::system::proxy_ui_added::proxy_ui_added;
    use crate::test::lib::*;

    use super::*;

    #[test]
    fn test_proxy_ui_update() {
        let mut app = setup_test_app();
        app.world.run_system_once(set_scene);
        app.update();
        app.world.run_system_once(proxy_ui_added);
        app.update();
        app.world.run_system_once(proxy_ui_update);
        app.update();
        let mut query_proxy_ui_proxied = app
            .world
            .query_filtered::<(Entity, &ProxyUiNodeState), With<ProxyUiProxied>>();
        let mut query_proxy_ui_proxy_target = app
            .world
            .query_filtered::<(Entity, &ProxyUiNodeState), With<ProxyUiTarget>>();
        let query_proxy_ui_proxied = query_proxy_ui_proxied.iter(&app.world);
        let query_proxy_ui_proxy_target = query_proxy_ui_proxy_target.iter(&app.world);
        assert_eq!(query_proxy_ui_proxied.len(), 1);
        assert_eq!(query_proxy_ui_proxy_target.len(), 1);

        for ((proxied_entity, proxied_node_state), (proxy_target_entity, ui_node_state)) in
            query_proxy_ui_proxied.zip(query_proxy_ui_proxy_target)
        {
            assert_eq!(proxied_node_state, ui_node_state);
        }
    }
}

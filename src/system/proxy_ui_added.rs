use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

use crate::error::ProxyUiStateError;
use crate::prelude::*;
use crate::resource::proxy_ui_entity_map::ProxyUiEntityMap;

#[derive(SystemParam)]
pub(crate) struct ProxyUiAddedQueries<'w, 's> {
    proxy_ui_added_query:
        Query<'w, 's, (Entity, &'static ProxyUi, Option<&'static Node>), Added<ProxyUi>>,
    proxy_target_node_check_query: Query<'w, 's, Option<&'static Node>>,
}

pub(crate) fn proxy_ui_added(
    mut commands: Commands,
    mut proxy_ui_entity_map: ResMut<ProxyUiEntityMap>,
    queries: ProxyUiAddedQueries,
) {
    for (proxy_ui_entity, proxy_ui, option_node) in queries.proxy_ui_added_query.iter() {
        commands.entity(proxy_ui_entity).remove::<ProxyUi>();

        if let Err(err) = validate_state(&proxy_ui_entity_map, &proxy_ui_entity, proxy_ui) {
            warn!("{err}");
            continue;
        }

        debug!("associating proxy target for {proxy_ui_entity:?}... ");
        let proxy_target_entity = commands
            .entity(proxy_ui.target_entity)
            .insert(ProxyUiProxied(proxy_ui_entity))
            .id();
        commands
            .entity(proxy_ui_entity)
            .insert(ProxyUiTarget(proxy_target_entity));
        debug!("associated proxy target for {proxy_ui_entity:?} -> {proxy_target_entity:?}");

        // check to see if there is an ui node already, if not, create one
        if option_node.is_none() {
            commands.entity(proxy_ui_entity).insert(NodeBundle {
                ..Default::default()
            });
        }

        let option_bad_node = queries
            .proxy_target_node_check_query
            .get(proxy_target_entity)
            .unwrap_or_else(|err| {
                panic!("failed to find proxy: {proxy_target_entity:?} - {err:?}")
            });
        if option_bad_node.is_some() {
            warn!("proxied entities can not contain ui nodes, entity: {proxy_target_entity:?}, removing proxy.");
            commands.entity(proxy_ui_entity).remove::<ProxyUi>();
            commands.entity(proxy_ui_entity).remove::<ProxyUiTarget>();
            commands
                .entity(proxy_target_entity)
                .remove::<ProxyUiProxied>();
            continue;
        }

        proxy_ui_entity_map.insert(proxy_target_entity, proxy_ui_entity);
    }
}

fn validate_state(
    proxy_ui_entity_map: &ProxyUiEntityMap,
    proxy_ui_entity: &Entity,
    proxy_ui: &ProxyUi,
) -> Result<(), ProxyUiStateError> {
    let proxy_target_entity = proxy_ui.target_entity;
    if proxy_ui_entity_map.contains_proxy(proxy_ui_entity) {
        if Some(&proxy_target_entity)
            != proxy_ui_entity_map.get_proxied_target_entity(proxy_ui_entity)
        {
            Err(ProxyUiStateError::MultipleProxyUiPerEntity)
        } else {
            Err(ProxyUiStateError::DuplicateProxyUi)
        }
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;
    use bevy::prelude::*;

    use crate::test::lib::*;

    use super::*;

    #[test]
    fn test_proxy_ui_added() {
        let mut app = setup_test_app();
        app.world.run_system_once(set_scene);
        app.update();
        app.world.run_system_once(proxy_ui_added);
        app.update();
        // should have been removed
        assert_eq!(
            app.world
                .query_filtered::<Entity, With<ProxyUi>>()
                .iter(&app.world)
                .len(),
            0
        );

        let mut query_proxy_ui_proxied = app.world.query::<(Entity, &ProxyUiProxied)>();
        let mut query_proxy_ui_proxy_target = app.world.query::<(Entity, &ProxyUiTarget)>();
        let query_proxy_ui_proxied = query_proxy_ui_proxied.iter(&app.world);
        let query_proxy_ui_proxy_target = query_proxy_ui_proxy_target.iter(&app.world);
        assert_eq!(query_proxy_ui_proxied.len(), 1);
        assert_eq!(query_proxy_ui_proxy_target.len(), 1);

        for ((proxied_entity, proxied), (proxy_target_entity, proxy_target)) in
            query_proxy_ui_proxied.zip(query_proxy_ui_proxy_target)
        {
            assert_eq!(proxied_entity, proxy_target.target_entity());
            assert_eq!(proxy_target_entity, proxied.get_proxy_ui_entity());
        }
    }
}

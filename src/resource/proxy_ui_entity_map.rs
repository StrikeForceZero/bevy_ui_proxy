use bevy::prelude::*;
use bimap::{BiMap, Overwritten};

/// BiMap<ProxyTargetEntity, ProxyUiEntity>
#[derive(Debug, Default, Resource)]
pub(crate) struct ProxyUiEntityMap(BiMap<Entity, Entity>);

impl ProxyUiEntityMap {
    pub(crate) fn is_proxied(&self, proxy_target_entity: &Entity) -> bool {
        self.0.contains_left(proxy_target_entity)
    }
    pub(crate) fn contains_proxy(&self, proxy_ui_entity: &Entity) -> bool {
        self.0.contains_right(proxy_ui_entity)
    }
    pub(crate) fn get_proxied_target_entity(&self, proxy_ui_entity: &Entity) -> Option<&Entity> {
        self.0.get_by_right(proxy_ui_entity)
    }
    pub(crate) fn get_proxy_entity(&self, proxied_target_entity: &Entity) -> Option<&Entity> {
        self.0.get_by_left(proxied_target_entity)
    }
    pub(crate) fn remove_by_proxied_target_entity(
        &mut self,
        proxied_target_entity: &Entity,
    ) -> Option<(Entity, Entity)> {
        self.0.remove_by_left(proxied_target_entity)
    }
    pub(crate) fn remove_by_proxy_entity(
        &mut self,
        proxy_ui_entity: &Entity,
    ) -> Option<(Entity, Entity)> {
        self.0.remove_by_left(proxy_ui_entity)
    }
    pub(crate) fn insert(
        &mut self,
        proxied_target_entity: Entity,
        proxy_ui_entity: Entity,
    ) -> Overwritten<Entity, Entity> {
        self.0.insert(proxied_target_entity, proxy_ui_entity)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proxy_ui_entity_map() {
        let mut proxy_ui_entity_map = ProxyUiEntityMap::default();
        let proxy_target_entity = Entity::from_raw(1);
        let proxy_ui_entity = Entity::from_raw(2);
        assert_eq!(proxy_ui_entity_map.insert(proxy_target_entity, proxy_ui_entity), Overwritten::Neither);
        assert!(proxy_ui_entity_map.is_proxied(&proxy_target_entity));
        assert!(proxy_ui_entity_map.contains_proxy(&proxy_ui_entity));
        assert_eq!(proxy_ui_entity_map.get_proxied_target_entity(&proxy_ui_entity), Some(&proxy_target_entity));
        assert_eq!(proxy_ui_entity_map.get_proxy_entity(&proxy_target_entity), Some(&proxy_ui_entity));
        assert_eq!(proxy_ui_entity_map.remove_by_proxied_target_entity(&proxy_target_entity), Some((proxy_target_entity, proxy_ui_entity)));
        assert_eq!(proxy_ui_entity_map.remove_by_proxy_entity(&proxy_ui_entity), None);
    }
}

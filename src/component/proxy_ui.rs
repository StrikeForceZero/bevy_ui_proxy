use bevy::prelude::*;

#[derive(Debug, Component, Reflect)]
pub struct ProxyUi {
    pub(crate) target_entity: Entity,
}

impl ProxyUi {
    pub fn proxy(target_entity: Entity) -> Self {
        Self { target_entity }
    }
}

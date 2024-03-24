use bevy::prelude::*;

#[derive(Debug, Component, Reflect)]
pub struct ProxyUiTarget(pub(crate) Entity);

impl ProxyUiTarget {
    pub fn target_entity(&self) -> Entity {
        self.0
    }
}

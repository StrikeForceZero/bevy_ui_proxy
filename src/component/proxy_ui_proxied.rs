use bevy::prelude::*;

#[derive(Debug, Component, Reflect)]
pub struct ProxyUiProxied(pub(crate) Entity);

impl ProxyUiProxied {
    pub fn get_proxy_ui_entity(&self) -> Entity {
        self.0
    }
}

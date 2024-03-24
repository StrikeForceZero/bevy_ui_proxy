use bevy::prelude::*;

#[derive(Debug, Clone, PartialEq, Component, Reflect)]
pub struct ProxyUiNodeState {
    pub(crate) ui_rect: Rect,
    pub(crate) world_rect: Rect,
    pub(crate) global_transform: GlobalTransform,
    pub(crate) transform: Transform,
    pub(crate) style: Style,
    pub(crate) ui_scale: f32,
}

impl ProxyUiNodeState {
    pub fn get_ui_rect(&self) -> &Rect {
        &self.ui_rect
    }
    pub fn get_world_rect(&self) -> &Rect {
        &self.world_rect
    }
    pub fn get_global_transform(&self) -> &GlobalTransform {
        &self.global_transform
    }
    pub fn get_transform(&self) -> &Transform {
        &self.transform
    }
    pub fn get_style(&self) -> &Style {
        &self.style
    }
    pub fn get_ui_scale(&self) -> &f32 {
        &self.ui_scale
    }
    pub fn is_changed(&self, new_state: &ProxyUiNodeState) -> bool {
        self != new_state
    }
}

use bevy::prelude::*;

use crate::prelude::*;
use crate::resource::proxy_ui_entity_map::ProxyUiEntityMap;
use crate::system::proxy_ui_added::proxy_ui_added;
use crate::system::proxy_ui_update::proxy_ui_update;

pub struct BevyProxyUiPlugin;

impl Plugin for BevyProxyUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ProxyUiEntityMap>()
            .register_type::<ProxyUiProxied>()
            .register_type::<ProxyUiTarget>()
            .register_type::<ProxyUiNodeState>()
            .register_type::<ProxyUi>()
            .add_systems(Update, (proxy_ui_added, proxy_ui_update).chain());
    }
}

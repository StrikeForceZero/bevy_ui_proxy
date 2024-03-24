
#[cfg(test)]
pub(crate) mod lib {
    use bevy::prelude::*;

    use crate::prelude::*;
    use crate::resource::proxy_ui_entity_map::ProxyUiEntityMap;

    pub(crate) fn setup_test_app() -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .init_resource::<ProxyUiEntityMap>();
        app
    }

    #[derive(Debug, Component)]
    pub(crate) struct DummyComponent;

    pub(crate) fn set_scene(mut commands: Commands) {
        let dummy_proxied_entity = commands.spawn(DummyComponent).id();
        let dummy_proxy_ui_entity = commands.spawn(DummyComponent).id();

        commands
            .entity(dummy_proxy_ui_entity)
            .insert(ProxyUi::proxy(dummy_proxied_entity))
            .insert(ProxyUiTarget(dummy_proxied_entity));

        commands
            .entity(dummy_proxied_entity)
            .insert(ProxyUiProxied(dummy_proxy_ui_entity));
    }
}

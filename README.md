# Bevy UI Proxy 

⚠️ **NOTE: This project is currently experimental and under active development. Use at your own risk.**

## Description
The goal of this project is to provide various helper methods to proxy any type of bundle/component as if it was a UI node and help them react to UI layout changes.

## Usage
```rust
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BevyUiProxyPlugin)
        .add_systems(Startup, setup_proxy_system)
        .add_systems(Update, on_proxy_node_state_update)
        .run()
}
```
```rust
fn setup_proxy_system(mut commands: Commands) {
    let proxy_target = commands.spawn(MyComponent).id();
    commands.spawn(ProxyUi::proxy(proxy_target));
}
```
```rust
fn on_proxy_node_state_update(
    mut commands: Commands,
    query: Query<(Entity, &ProxyUiNodeState, &MyComponent), Changed<ProxyUiNodeState>>
) {
    for (entity, node_state, ..) in query.iter() {
        commands.entity(entity)
            .insert((
                MyComponentWidth(node_state.get_world_rect().width()),
                MyComponentHeight(node_state.get_world_rect().height()),
                GlobalTransform::from(node_state.get_transform().affine()),
            ));
    }
}
```

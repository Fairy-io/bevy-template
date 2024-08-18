# bevy-template

## Purpose

The purpose of this template is to generate code which an be used for developing games.

## Stack

- Rust
- Bevy

## Required tools:

- `cargo install watch`

## How it works?

- `src` directory contains base game setup like camera or dev tools (e.g. hot reloading)
- `lib` directory contains game specific components and systems

### Hot reloading

Lib is compiled separately and the changes are detected by main application. `hot_reload_system` is watching lib changes and despawns any entity with `Despawnable` component and then runs `startup` system.

#### game_plugin.rs
In order to make it work, we have to keep track on any file from `lib` separately. It is done by using `hot_functions_from_file!` macro. It should be used for each file, e.g.:
```rust
hot_functions_from_file!("lib/src/lib.rs");

hot_functions_from_file!("lib/src/components/despawnable.rs");

hot_functions_from_file!("lib/src/systems/startup_system.rs");
```

#### GamePlugin
Game plugin cannot be declared in lib, because it makes hot reloading not working. So that's why it should be defined in `game_plugin.rs`.

#### Attention
For development purposes `hot_lib` module is used and it contains more public methods than `lib`, so there can be situation when everything works in dev mode, but after running `cargo run`, it breaks, because it cannot find imports.

### Scripts

- `./run.sh app` - start app in watch mode
- `./run.sh lib` - compile lib and run tests in watch mode

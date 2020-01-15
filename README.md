[![Build Status](https://github.com/dakom/todo-shipyard-lit-dominator/workflows/Test%2C%20Build%2C%20and%20Deploy/badge.svg)](https://github.com/dakom/todo-shipyard-lit-dominator/actions)

## [LIVE DEMO](https://dakom.github.io/todo-shipyard-lit-dominator)

## [Medium Article](https://medium.com/@david.komer/shipyard-dominator-litelement-b4bcdc7ec42d)

# Architecture

* [Rust] Logic and State are handled via the [Shipyard Entity Component System](https://github.com/leudz/shipyard)
* [Rust] DOM is rendered via [Dominator](https://github.com/pauan/rust-dominator)
* [Rust] This happens declaratively due to the ECS holding things that generate [Signals](https://github.com/Pauan/rust-signals)
* [Typescript] Aesthetics are driven by WebComponents in [lit-element](https://lit-element.polymer-project.org/)

# Communication and Flow

It's essentially like this:

Shipyard Components -> Signals -> Dominator -> Web Components -> Events -> Update Shipyard (back to beginning)

This implies a couple things:

1. Events flow unidirectionally from the dom to the ecs.
2. The dom is rendered declaratively based on app state.

This is basically the React, Elm, FRP, pattern. For dom rendering itself, it is merely standing on the shoulders of Dominator. 

Yet - systems and ad-hoc world updates can happen outside the above flow too and modify the same state. For those coming from frameworks like Redux it may sound familiar. In this demo, the save mechanism runs a system on a timer, completely independently of the dom.

By holding state in a proper ECS there are some interesting advantages: 

1. It is idiomatic to update state without any connection to the dom tree whatsoever and the ECS provides ways to make this very efficient
2. Thinking in a data-oriented way (and getting the hardcore performance advantages where possible)
3. Everything is extremely decoupled. For example, the same `World` could be used to power an imperative webgl renderer. _Some_ components hold Mutables that turn into Signals that update the dom - but that's a choice, not a requirement.

# Events

In order to communicate from the Typescript web components to the Rust event handlers, events need to be converted.

It's pretty straightforward - on the typescript side extend CustomEvent and on the Rust side use the helper macros.

Ultimately, by making sure all events are covered in the [unit test](typescript/tests/events.spec.ts) you can't go wrong.

### Breakdown

* [typescript/components](typescript/components): web components (including styles and typescript helpers)
* [typescript/events/events.ts](typescript/events/events.ts): `CustomEvent`s that are emitted from the web components (and intercepted by Dominator)
* [rust/src/actions.rs](rust/src/actions.rs): Shipyard world updaters 
* [rust/src/components.rs](rust/src/components.rs): ecs components that hold data 
* [rust/src/dom.rs](rust/src/dom.rs): Dominator tree 
* [rust/src/events.rs](rust/src/events.rs): mirror of the typescript `CustomEvents` but in Rust (and with helpers to do the conversion, run tests, etc.)
* [rust/src/lib.rs](rust/src/lib.rs): main entry point 
* [rust/src/router.rs](rust/src/router.rs): url routing
* [rust/src/signals.rs](rust/src/signals.rs): Signals that are generated from Shipyard components (which hold `Mutable`/`MutableVec`s) and fed to Dominator
* [rust/src/storage.rs](rust/src/storage.rs): accessors for LocalStorage 
* [rust/src/systems.rs](rust/src/systems.rs): Shipyard systems 
* [rust/src/timers.rs](rust/src/timers.rs): timer loops 
* [rust/src/world.rs](rust/src/world.rs): Shipyard world registration 
* [typescript/tests/events.spec.ts](typescript/tests/events.spec.ts): unit tests to check event conversion

# Development

1. Install prerequisites like [cargo-make](https://github.com/sagiegurari/cargo-make), and `npm install`
2. `npm start` (will open a browser and rebuild/reload on source change - either typescript, rust, or static files)

More commands are available via `cargo make`:

* `cargo make test` (runs all the tests)
* `cargo make build --profile production` (used in ci/cd - but can check out release builds in `dist/` this way)
* `cargo make build --profile development` (same but used for seeing how non-optmized builds look)

# Prior Art

This picks up where [wasm-app-boilerplate](https://github.com/dakom/wasm-app-boilerplate) left off
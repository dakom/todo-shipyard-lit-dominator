[![Build Status](https://github.com/dakom/todo-shipyard-lit/workflows/Test%2C%20Build%2C%20and%20Deploy/badge.svg)](https://github.com/dakom/todo-shipyard-lit/actions)

## [LIVE DEMO](https://dakom.github.io/todo-shipyard-lit)

# Architecture

* Logic and State are handled in Rust via [shipyard ECS](https://github.com/leudz/shipyard)
* DOM is populated by WebComponents which use [lit-element](https://lit-element.polymer-project.org/)
* CSS for this demo is from [TodoMVC](http://todomvc.com/)

# Development

The task runner uses [cargo-make](https://github.com/sagiegurari/cargo-make)

After installing that (and other dependencies for the project itself - e.g. npm install, rust nightly, etc.), these commands are available:

* `cargo make serve` (opens a browser and rebuilds/reloads on file change)
* `cargo make test` (runs all the tests)
* `cargo make build --profile production` (used in ci/cd - but can check out release builds in `dist/` this way)
* `cargo make build --profile development` (same but used for seeing how non-optmized builds look)
[![Build Status](https://github.com/dakom/todo-shipyard-lit-dominator/workflows/Test%2C%20Build%2C%20and%20Deploy/badge.svg)](https://github.com/dakom/todo-shipyard-lit-dominator/actions)

## [LIVE DEMO](https://dakom.github.io/todo-shipyard-lit-dominator)

# Architecture

* Logic and State are handled in Rust via [shipyard ECS](https://github.com/leudz/shipyard)
* DOM is populated by WebComponents which use [lit-element](https://lit-element.polymer-project.org/)
* CSS for this demo is from [TodoMVC](http://todomvc.com/)

# Development

1. Install prerequisites like [cargo-make](https://github.com/sagiegurari/cargo-make), and `npm install`
2. `npm start` (will open a browser and rebuild/reload on source change - either typescript, rust, or static files)

More commands are available via `cargo make`:

* `cargo make test` (runs all the tests)
* `cargo make build --profile production` (used in ci/cd - but can check out release builds in `dist/` this way)
* `cargo make build --profile development` (same but used for seeing how non-optmized builds look)
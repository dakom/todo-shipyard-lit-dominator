[![Build Status](https://github.com/dakom/todo-shipyard-lit/workflows/Test%2C%20Build%2C%20and%20Deploy/badge.svg)](https://github.com/dakom/todo-shipyard-lit/actions)

## [LIVE DEMO](https://dakom.github.io/todo-shipyard-lit)

# Architecture

* Logic and State are handled in Rust via [shipyard ECS](https://github.com/leudz/shipyard)
* DOM is populated by WebComponents which use [lit-element](https://lit-element.polymer-project.org/)
* CSS for this demo is from [TodoMVC](http://todomvc.com/)

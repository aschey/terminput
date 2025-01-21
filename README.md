# terminput

An library to handle events from different backends that provide input sources
such as key and mouse events. This was mainly created to provide a common
interface to the terminal backends that
[Ratatui](https://crates.io/crates/ratatui) supports.

Many TUI libraries want to support input from multiple backends, but mapping all
of these input types into a common interface is a very tedious process. This
library aims to provide a uniform interface to these input types.

## Backends

The following backends are currently supported:

- [`crossterm`](https://crates.io/crates/crossterm)
- [`termion`](https://crates.io/crates/termion)
- [`termwiz`](https://crates.io/crates/termwiz)
- [`egui`](https://crates.io/crates/egui)

The `Event` struct provided in this library is an attempt to provide a superset
of the input functionality that TUI apps may be interested in. Not all backends
support all features provided.

The following table shows the matrix of supported features:

|                        | crossterm | termion | termwiz | egui |
| ---------------------- | --------- | ------- | ------- | ---- |
| **keypress**           | ✓         | ✓       | ✓       | ✓    |
| **key release/repeat** | ✓         |         |         | ✓    |
| **mouse down**         | ✓         | ✓       | ✓       | ✓    |
| **mouse up**           | ✓         | ✓       |         | ✓    |
| **mouse move**         | ✓         |         | ✓       | ✓    |
| **mouse drag**         | ✓         | ✓       |         |      |
| **focus**              | ✓         |         |         | ✓    |
| **paste**              | ✓         |         | ✓       | ✓    |
| **resize**             | ✓         |         | ✓       |      |

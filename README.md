Iron mounting router
====

Routing handler for the [Iron](https://github.com/iron/iron) web framework.
First version based on [Router](https://github.com/iron/router/) project trying
to keep all of its features and add [Mount](https://github.com/iron/mount)
functionality.

## Why not just used Router + Mount, they work together perfectly?
Mount only works with predefined strings and cannot be used like django's
router to strip matched part of query for further handlers.

## Router does not accept http method, why is that?
Usually routers restrict developer to specifiy method. Mountrouter enforces REST
ideology: you first identify resource using path and then perform action
based on method. MethodPicker struct allows developer to specify handler for
http methods.

## Installation
If you're using cargo, just add router to your `Cargo.toml`.

```toml
[dependencies]

iron-mountrouter = { git = https://github.com/SlNPacifist/iron-mountrouter, version = "*" }
```

Otherwise, `cargo build`, and the rlib will be in your `target` directory.

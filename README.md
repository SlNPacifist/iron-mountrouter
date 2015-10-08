Iron mounting router
====

Routing handler for the [Iron](https://github.com/iron/iron) web framework.
First version based on [Router](https://github.com/iron/router/) project trying
to keep all of its features and add [Mount](https://github.com/iron/mount)
functionality.

## Why not just used Router + Mount, they work together perfectly?
Mount only works with predefined strings and cannot be used like django's
router to strip matched part of query for further handlers.

Router is a fast, convenient, and flexible routing middleware for Iron. It
allows complex glob patterns and named url parameters and also allows handlers
to be any Handler, including all Chains.

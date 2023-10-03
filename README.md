# Actix SPA boilerplate

A boilerplate for an SPA (Singe Page Application) web app using [Actix](https://actix.rs/).
It uses Actix for serving a REST API as well as the [Vue](https://vuejs.org/)
client. (This may not be the optimal setup, but it keeps everything
nicely in place (and in *one* repository :grinning:)).

The boilerplate is not very optioned, so many choices are hardcoded.
Some steps will be available via [branches]. Further modularization
isn't planned.

[Bootstrap](https://getbootstrap.com/) is used as a CSS framework.
Component testing is done via [Reqwest](https://docs.rs/reqwest/latest/reqwest/) for the API
and where needed via Vitest for the client. The whole app is E2E tested
via [Cypress](https://www.cypress.io/).
 
Building is done via [Cargo-Make](https://docs.rs/crate/cargo-make/latest)
(and of course cargo and npm).

CI will be done via [Docker](https://www.docker.com/)
and [GitHubActions](https://github.com/features/actions). 

Next steps will be adding internationalisation (via [vue-i18n](https://vue-i18n.intlify.dev/))
and implementing a login API and page using a
[Postgresql](https://www.postgresql.org/) DB as the backend. For further
authentication we'll be using [JWT](https://jwt.io/).

## TODO
- Add more tests for exposed routes
- Add CI (probably using `cargo-make`. We'll want testing,
building and preparing commit in one place)
- Add Login and restrict side and API access
- Improve health check for DB connection and FS access.

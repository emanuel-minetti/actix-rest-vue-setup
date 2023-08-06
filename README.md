# Actix SPA boilerplate

We'll build a web app using [aktix](https://actix.rs/)
for serving a REST API as well as the [Vue](https://vuejs.org/)
client. (This may not be the optimal setup, but it keeps everything
nicely in place (and in *one* repository :grinning:)). We'll add E2E
testing for the API as well as the client.

We'll implement a login API and page using a
[Postgresql](https://www.postgresql.org/) backend. For further
authentication we'll be using [JWT](https://jwt.io/).

Some steps will be available via [releases]. Further modularization
is not planned.

## TODO
- Add more tests for exposed routes
- Add Bootstrap
- Add CI (maybe using `cargo-make`. We'll want testing,
building and preparing commit in one place)
- Add *real* E2E tests (client browser tests)
- Add Login and restrict side and API access
- Implement dummy service

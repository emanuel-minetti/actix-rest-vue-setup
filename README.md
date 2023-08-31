# Actix SPA boilerplate

We'll build a web app using [aktix](https://actix.rs/)
for serving a REST API as well as the [Vue](https://vuejs.org/)
client. (This may not be the optimal setup, but it keeps everything
Typpnicely in place (and in *one* repository :grinning: )).
We'll add component
testing for the API (where needed) and E2E
(via [Cypress](https://www.cypress.io/))
testing the client. Next we'll add CI using [Docker](https://www.docker.com/)
and [GitHubActions](https://github.com/features/actions). 

Next we'll add internationalisation (via [vue-i18n](https://vue-i18n.intlify.dev/)).
Further on we'll implement a login API and page using a
[Postgresql](https://www.postgresql.org/) backend. For further
authentication we'll be using [JWT](https://jwt.io/).

Some steps will be available via [branches]. Further modularization
isn't planned.

## TODO
- Add more tests for exposed routes
- Add CI (probably using `cargo-make`. We'll want testing,
building and preparing commit in one place)
- Add *real* E2E tests (client browser tests)
- Add Login and restrict side and API access
- Improve health check for DB connection and FS access.

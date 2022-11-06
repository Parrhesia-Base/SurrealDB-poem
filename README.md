# surreal-poem
Library to help you integrate SurrealDB into your Poem-powered Rust server

The SurrealDB library supports being embedded into other executables, but there aren't any bindings or convenience features to aid in exposing the embedded database to any web clients. In practical large-scale applications it is preferrable to use the standalone SurrealDB application, but for smaller projects it is appropriate and much simpler to run the database directly in the backend.

The purpose of surreal-poem is to make your poem server compatible with the SurrealDB Javascript client library.
use std::sync::Arc;
use poem::{EndpointExt, IntoEndpoint};
use surrealdb::{ Datastore, Error, Session };

// Re-expose so projects don't need imports from surrealdb itself
// pub use surrealdb::Session;

/// Public Types
pub type SurrealConnection = Arc<Datastore>;
pub type SurrealSession = Session;

/// Initiates a SurrealDB database connection from the given URI. URI can
/// have the following forms:
/// 
/// > `"memory"` initiates the database in RAM
/// 
/// > `"file://path/to/db"` initiates the database within a directory.
/// ***Keep in mind this will generate a host of files***, so it's probably appropriate
/// to keep them in their own database directory.
/// 
/// > `"tikv://127.0.0.1:2397"` connects to a tikv-backed distributed store.
pub async fn get_connection( uri: &str ) -> Result<SurrealConnection, Error> {
    Ok( Arc::new( Datastore::new( &uri ).await? ) )
}

/// The handler for the SurrealDB connection. Works based on websocket technology
#[poem::handler]
pub fn surreal_socket( ) -> String {
    "Hello, there!".into()
}

/// This function creates the websocket endpoint that SurrealDB's
/// client libraries connect to. It works through a constant websocket
/// connection.
/// 
/// #### This should be used as one of Poem's routes:
/// 
/// `EXAMPLE HERE`
pub fn create_surreal_socket_endpoint( db: SurrealConnection, ses: SurrealSession ) -> impl IntoEndpoint {
    poem::get( surreal_socket ).post( surreal_socket ).data( db ).data( ses )
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

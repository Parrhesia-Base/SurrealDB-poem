use futures_util::{SinkExt, StreamExt};
use std::{sync::Arc};
use poem::{EndpointExt, IntoEndpoint, web::{Data, websocket::{WebSocket, Message}}, IntoResponse};
use surrealdb::{ Datastore, Error, Session, sql::{Part, Value} };

/// Public Types
pub type SurrealDB = Arc<Datastore>;
pub type SurrealSession = Session;

/// Initiates a SurrealDB database connection from the given URI.
///  URI can
/// have the following forms:
/// 
/// > `"memory"` initiates the database in RAM
/// 
/// > `"file://path/to/db"` initiates the database within a directory.
/// ***Keep in mind this will generate a host of files***, so it's probably appropriate
/// to keep them in their own database directory.
/// 
/// > `"tikv://127.0.0.1:2397"` connects to a tikv-backed distributed store.
pub async fn get_connection( uri: &str, db: &str, ns: &str ) -> Result<SurrealDB, Error> {
    Ok( Arc::new( Datastore::new( &uri ).await? ) )
}

/// This function initializes an empty database with a ready-to-use user table,
/// which is compatible with the user authentication functions built into the
/// javascript client libraries for SurrealDB. If the database is already
/// configured with a user table, this function won't make any changes.
pub async fn initiate_auth_db( 
    db: SurrealDB,
    db_name: &str, 
    ns_name: &str,
) {

    let check_query = "
        
    ";

    let ses = Session::for_db( ns_name, db_name );
    // db.execute(txt, sess, vars, strict)
}

/// The handler for the SurrealDB connection. Works based on websocket technology
/// This is handler is designed to work exactly like the /rpc endpoint of the
/// actual SurrealDB application, but in the Poem web framework, which is a
/// pleasure to work with.
#[poem::handler]
fn surreal_socket_handler(
    ws: WebSocket,
    db: Data<&SurrealDB>, 
    ses: Data<&SurrealSession>
    ) -> impl IntoResponse
{
    ws.on_upgrade(move |socket| async move {
        let ( mut sink, mut stream ) = socket.split();
        
        tokio::spawn( async move {
            while let Some( Ok( msg ) ) = stream.next().await {

                // When a message is received,
                if let Message::Text( text ) = &msg {

                    // Echo it:
                    println!( "Websocket client sent: '{}'", text );

                    // Send it right back!
                    if sink.send( Message::Text( format!("{}", text ) ) ).await.is_err() {
                        break;
                    }
                }

                call( &msg ).await;
            }
        })
    })
}

async fn call( msg: &Message ) {
    println!( "Starting message parser!" );

    let msg = msg.clone();

    let req = match msg {

        // Binary message
        m if m.is_binary() => {
            todo!();
        }

        // Text message
        Message::Text( text ) => {
            // Guard
            let Ok( v ) = surrealdb::sql::json( &text ) else {
                todo!();
            };

            v
        }

        // Unsupported message type
        _ => {
            todo!();
        }
    };

    // Get the 'params' argument
    let id = match req.pick( &[Part::from( "id" )] ) {
        v if v.is_none() => None,
        v if v.is_null() => Some(v),
        v if v.is_uuid() => Some(v),
        v if v.is_strand() => Some(v),
        _ => todo!()
    };

    // Get the 'method' argument
    let method = match req.pick( &[Part::from( "method" )]) {
        Value::Strand( v ) => v.to_raw(),
        _ => todo!()
    };

    // Get the 'params' argument
    let params = match req.pick( &[Part::from( "params" ) ] ) {
        Value::Array( v ) => v,
        _ => todo!()
    };

    println!( "id: {:?}", id );
    println!( "method: {:?}", method );
    println!( "params: {:?}", params );

    let res = match &method[..] {
        "ping" => Value::None,
        "signin" => {
            Value::None
        }
        _ => todo!()
    };
}

/// This function creates the websocket endpoint that SurrealDB's
/// client libraries connect to. It works through a constant websocket
/// connection.
/// 
/// #### This should be used as one of Poem's routes:
/// 
/// `EXAMPLE HERE`
pub fn surreal_socket( db: SurrealDB, ses: SurrealSession ) -> impl IntoEndpoint {
    poem::get( surreal_socket_handler )
         .post( surreal_socket_handler )
         .data( db ).data( ses )
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

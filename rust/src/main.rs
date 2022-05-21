pub mod models;
pub mod schema;

use diesel::SqliteConnection;

use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router, response::{Html, IntoResponse}, Extension
};

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate log;

use diesel::prelude::*;
use parking_lot::Mutex;
use std::{env, net::SocketAddr};
use std::sync::Arc;

struct State {
    pub conn: SqliteConnection,
}

pub fn establish_connection() -> SqliteConnection {
    let db_url = env::var("DATABASE_URL").expect("No DATABASE_URL found");
    SqliteConnection::establish(&db_url)
        .expect(&format!("Error connecting to {}", db_url))
}

async fn get_root(
    Extension(state): Extension<Arc<Mutex<State>>>
) -> Html<String> {
    let conn = &state.lock().conn;

    let res = {
        use schema::comments::dsl::*;

        comments.load::<models::Comment>(conn)
    };
    if let Err(x) = res {
        return Html(format!("<h1>Fuck</h1><br><code>{:#?}</code>", x))
    }
    let res = res.ok().unwrap();

    let mut o = format!("There are <strong>{}</strong> comments.<br>", res.len());
    for i in res {
        o += &format!("<strong>{}</strong>: {}<br>", i.author, i.body);
    }

    Html(o.into())
}

#[derive(serde::Deserialize)]
struct NewCommentPayload {
    author: String,
    text: String,
}

async fn new_comment(
    Json(payload): Json<NewCommentPayload>,
    Extension(state): Extension<Arc<Mutex<State>>>
) -> impl IntoResponse {
    use schema::comments;
    use models::NewComment;

    let new_comment = NewComment {
        author: &payload.author,
        body: &payload.text,
        created_at: chrono::Utc::now().naive_utc(),
    };

    let conn = &state.lock().conn;

    let er = diesel::insert_into(comments::table)
        .values(&new_comment)
        .execute(conn);
    if let Err(x) = er {
        return (StatusCode::INTERNAL_SERVER_ERROR, format!("{:#?}", x));
    }

    (StatusCode::OK, "".into())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    pretty_env_logger::init_timed();

    info!("commenter starting now!");

    let conn = establish_connection();
    let shared_state = Arc::new(Mutex::new(State {
        conn
    }));

    let app = Router::new()
        .route("/", get(get_root))
        .route("/submit", post(new_comment))
        .layer(Extension(shared_state));

    let port = match env::var("LISTEN_PORT") {
        Ok(x) => x.parse::<u16>().unwrap(),
        Err(_) => 3000,
    };
    
    let addr = SocketAddr::from(([127,0,0,1], port));
    info!("Listening on address {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

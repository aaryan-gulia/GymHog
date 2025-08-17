#[cfg(feature = "server")]
use {
    serde::{Deserialize, Serialize},
    surrealdb::{
        engine::remote::ws::{Client, Ws},
        opt::auth::Root,
        RecordId, Surreal, Value,
    },
    tokio::sync::OnceCell,
};

#[cfg(feature = "server")]
static DB: OnceCell<Surreal<Client>> = OnceCell::const_new();

#[cfg(feature = "server")]
async fn db() -> Surreal<Client> {
    let db = Surreal::new::<Ws>("127.0.0.1:8000")
        .await
        .expect("Can't connect to surrealdb");

    db.signin(Root {
        username: "root",
        password: "secret",
    })
    .await
    .expect("Can't signin to surrealdb");

    db.use_ns("test")
        .use_db("test")
        .await
        .expect("Can't use_ns and use_db");
    db
}

#[cfg(feature = "server")]
pub async fn get_db() -> &'static Surreal<Client> {
    DB.get_or_init(db).await
}

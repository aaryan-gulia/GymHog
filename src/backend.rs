#[cfg(feature = "server")]
use {
    lazy_static::lazy_static,
    serde::{Deserialize, Serialize},
    surrealdb::{engine::remote::ws::Ws, opt::auth::Root, opt::Resource, RecordId, Surreal, Value},
};

use std::{collections::BTreeSet, sync::Arc};
use tokio::sync::RwLock;

// NOTE: it is possible to repace usage of std::sync::Arc with the actix_web::web::Data, but need better investigation
pub type ContextLock = Arc<RwLock<Context>>;

#[derive(Default)]
pub struct Context {
    pub tickets_collection: BTreeSet<String>,
}

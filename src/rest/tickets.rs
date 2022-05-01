use actix_web::{web, Responder, Scope};

async fn create_collection() -> impl Responder {
    "create_collection"
}

async fn get_collection() -> impl Responder {
    "get_collection"
}

pub fn collection() -> Scope {
    web::scope("/collection")
        .route("", web::post().to(create_collection))
        .route("/{id}", web::get().to(get_collection))
}

pub fn tickets() -> Scope {
    web::scope("/tickets").service(collection())
}

#[cfg(test)]
mod test {}

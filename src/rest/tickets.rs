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
mod tests {
    use crate::rest::config_app;
    use actix_web::{test, App};

    #[tokio::test]
    async fn create_collection_test() {
        let app = test::init_service(App::new().configure(config_app)).await;

        let req = test::TestRequest::post()
            .uri("/tickets/collection")
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());
    }

    #[tokio::test]
    async fn get_collection_test() {
        let app = test::init_service(App::new().configure(config_app)).await;

        let req = test::TestRequest::get()
            .uri(format!("/tickets/collection/{}", "id1").as_str())
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());
    }
}

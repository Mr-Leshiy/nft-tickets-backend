use crate::context::ContextLock;
use actix_web::{web, Responder, Scope};

async fn create_collection(context: web::Data<ContextLock>) -> impl Responder {
    let mut context = context.write().await;
    context.tickets_collection.insert("new_value".to_string());
    "create_collection"
}

async fn get_collection() -> impl Responder {
    "get_collection"
}

pub fn collection(context: ContextLock) -> Scope {
    web::scope("/collection")
        .app_data(web::Data::new(context))
        .route("", web::post().to(create_collection))
        .route("/{id}", web::get().to(get_collection))
}

pub fn tickets(context: ContextLock) -> Scope {
    web::scope("/tickets").service(collection(context))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rest::config_app;
    use actix_web::{test, App};

    #[tokio::test]
    async fn create_collection_test() {
        let context: ContextLock = Default::default();

        let app =
            test::init_service(App::new().configure(|cfg| config_app(context.clone(), cfg))).await;

        let req = test::TestRequest::post()
            .uri("/tickets/collection")
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());

        assert!(context
            .read()
            .await
            .tickets_collection
            .contains("new_value"));

        assert!(!context
            .read()
            .await
            .tickets_collection
            .contains("another_new_value"));
    }

    #[tokio::test]
    async fn get_collection_test() {
        let context = Default::default();

        let app = test::init_service(App::new().configure(|cfg| config_app(context, cfg))).await;

        let req = test::TestRequest::get()
            .uri(format!("/tickets/collection/{}", "id1").as_str())
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());
    }
}

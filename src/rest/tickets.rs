use crate::context::ContextLock;
use actix_web::{web, HttpResponse, Responder, Scope};
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Deserialize, Serialize)]
struct Collection {
    id: String,
    name: String,
}

async fn create_collection(
    context: web::Data<ContextLock>,
    collection: web::Json<Collection>,
) -> impl Responder {
    let mut context = context.write().await;
    let collection = collection.into_inner();
    context
        .tickets_collection
        .insert(collection.id, collection.name);
    HttpResponse::Ok()
}

async fn get_collection(context: web::Data<ContextLock>, id: web::Path<String>) -> impl Responder {
    let context = context.read().await;
    let id = id.into_inner();
    context.tickets_collection.get(&id).cloned()
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

        let collection = Collection {
            id: "id".to_string(),
            name: "name".to_string(),
        };

        let req = test::TestRequest::post()
            .set_json(collection.clone())
            .uri("/tickets/collection")
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());

        assert!(context
            .read()
            .await
            .tickets_collection
            .contains_key(&collection.id));

        assert!(!context
            .read()
            .await
            .tickets_collection
            .contains_key("another_id"));
    }

    #[tokio::test]
    async fn get_collection_test() {
        let context: ContextLock = Default::default();

        context
            .write()
            .await
            .tickets_collection
            .insert("id".to_string(), "value".to_string());

        let app = test::init_service(App::new().configure(|cfg| config_app(context, cfg))).await;

        let req = test::TestRequest::get()
            .uri(format!("/tickets/collection/{}", "id").as_str())
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());

        let req = test::TestRequest::get()
            .uri(format!("/tickets/collection/{}", "id1").as_str())
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert!(!resp.status().is_success());
    }
}

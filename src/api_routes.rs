use actix_session::CookieSession;
use actix_web::{guard, web, *};
use api::actions::ManagementTrait;
// use api::actions::monitoring::AggregationManagementTrait;

use api::common::user::*;
use api::common::Health;
use serde::Deserialize;

use utils::rate_limit::RateLimit;

pub fn config(cfg: &mut web::ServiceConfig, rate_limit: &RateLimit) {
    cfg.service(
        web::scope("/auth/v1")
            // Health Check
            .service(web::scope("/health").route("", web::get().to(route_responder::<Health>)))
            // Building Management
            .service(
                web::scope("/users")
                    .route("", web::post().to(UserApi::create_item))
                    .route("/{user_id}", web::get().to(UserApi::get_item))
                    .route("/{user_id}", web::delete().to(UserApi::delete_item))
                    .route("", web::get().to(UserApi::get_collection))
                    .route("/{user_id}", web::put().to(UserApi::update_item)),
            ),
    );
}

async fn route_responder<'a, Data>(data: Data) -> impl Responder
where
    Data: Send + 'static + Responder,
{
    data
}

async fn route_responder_with_payload<'a, Data>(
    data: web::Either<web::Form<Data>, web::Json<Data>>,
) -> impl Responder
where
    Data: Deserialize<'a> + Send + 'static + Responder,
{
    data.into_inner()
}

use actix_web::{web, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use safe_user::db::DbPool;
use safe_user::handlers::{create_user, create_jwt_for_user, get_all_users, protected_route};
use safe_user::auth::jwt_validator;
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let db_pool = DbPool::new().await.expect("No se pudo crear la conexión a la base de datos.");
    let pool_data = web::Data::new(db_pool.pool);

    HttpServer::new(move || {
        let auth = HttpAuthentication::bearer(jwt_validator);

        App::new()
            .app_data(pool_data.clone())
            .route("/create_user", web::post().to(create_user))
            .route("/get_jwt", web::post().to(create_jwt_for_user))
            .service(
                web::scope("/protected")
                    .wrap(auth)
                    .route("/users", web::get().to(get_all_users))
                    .route("/route", web::get().to(protected_route))
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

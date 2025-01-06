use actix_web::{web, HttpResponse, Responder};
use sqlx::Pool;
use sqlx::mssql::Mssql;
use uuid::Uuid;
use crate::auth::generate_jwt;
use crate::models::User;

/// It includes functions for creating users, generating JWTs, and retrieving users.
///
/// # Examples
///
/// ```
/// use actix_web::{web, App, HttpServer};
/// use safe_user::handlers::create_user;
/// use safe_user::models::User;
///
/// #[actix_web::main]
/// async fn main() -> std::io::Result<()> {
///     HttpServer::new(|| {
///         App::new()
///             .route("/create_user", web::post().to(create_user))
///     })
///     .bind("127.0.0.1:8080")?
///     .run()
///     .await
/// }
/// ```
pub async fn create_user(pool: web::Data<sqlx::Pool<sqlx::Mssql>>,new_user: web::Json<User>) -> impl Responder {
    let user = new_user.into_inner();

    let query_result = sqlx::query!(
        r#"
        INSERT INTO [users] (
            id,
            UserId,
            Name,
            LastName,
            Email,
            Age,
            Phone,
            Address,
            BirthDate,
            PlaceBirth
        )
        VALUES (
            @p1, @p2, @p3, @p4, @p5,
            @p6, @p7, @p8, @p9, @p10
        )
        "#,
        Uuid::new_v4().to_string(),
        user.user_id,
        user.name,
        user.last_name,
        user.email,
        user.age,
        user.phone,
        user.address,
        user.birthdate,
        user.place_birth
    )
    .execute(pool.get_ref())
    .await;

    match query_result {
        Ok(_) => HttpResponse::Ok().json("User created successfully."),
        Err(e) => {
            eprintln!("Error creating user: {:?}", e);
            HttpResponse::InternalServerError().json("Error creating user.")
        }
    }
}

/// Generates a JWT for a given user.
///
/// # Arguments
///
/// * `info` - A JSON payload containing user information.
///
/// # Returns
///
/// * `HttpResponse` - A JSON response containing the JWT or an error message.
///
/// # Examples
///
/// ```
/// use actix_web::{web, App, HttpServer};
/// use safe_user::handlers::create_jwt_for_user;
/// use safe_user::models::User;
///
/// #[actix_web::main]
/// async fn main() -> std::io::Result<()> {
///     HttpServer::new(|| {
///         App::new()
///             .route("/get_jwt", web::post().to(create_jwt_for_user))
///     })
///     .bind("127.0.0.1:8080")?
///     .run()
///     .await
/// }
///```
pub async fn create_jwt_for_user(info: web::Json<User>) -> impl Responder {
    let response = match generate_jwt(Some(&info.id.clone().expect("REASON").to_string()).unwrap()) {
        Ok(token) => format!("{}", token),
        Err(_) => "Failed to generate JWT".to_string(),
    };

    if response.starts_with("JWT:") {
        HttpResponse::Ok().json(response)
    } else {
        HttpResponse::InternalServerError().json(response)
    }
}

/// Retrieves all users from the database.
///
/// # Arguments
///
/// * `pool` - A connection pool to the database.
///
/// # Returns
///
/// * `HttpResponse` - A JSON response containing the list of users or an error message.
///
/// # Examples
///
/// ```
/// use actix_web::{web, App, HttpServer};
/// use safe_user::handlers::get_all_users;
/// use safe_user::db::DbPool;
///
/// #[actix_web::main]
/// async fn main() -> std::io::Result<()> {
///     let pool = DbPool::new().await.unwrap().pool;
///     HttpServer::new(move || {
///         App::new()
///             .app_data(web::Data::new(pool.clone()))
///             .route("/get_users", web::get().to(get_all_users))
///     })
///     .bind("127.0.0.1:8080")?
///     .run()
///     .await
/// }
///```
pub async fn get_all_users(pool: web::Data<Pool<Mssql>>) -> impl Responder {
    let query_result = sqlx::query_as!(
        User,
        r#"
        SELECT
            CAST(id AS VARCHAR(36))         AS "id?", -- Cast UUID to String
            UserId                          AS "user_id!",
            Name                            AS "name!",
            LastName                        AS "last_name!",
            Email                           AS "email!",
            Age                             AS "age?",
            Phone                           AS "phone?",
            Address                         AS "address?",
            CONVERT(VARCHAR, BirthDate, 23) AS "birthdate!",
            PlaceBirth                      AS "place_birth?"
        FROM [users]
        "#
    )
    .fetch_all(pool.get_ref())
    .await;

    match query_result {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => {
            eprintln!("Error getting users: {:?}", e);
            HttpResponse::InternalServerError().json("Error getting users")
        }
    }
}

/// A protected route that requires a valid token to access.
///
/// # Returns
///
/// * `HttpResponse` - A JSON response indicating that the route is protected.
///
/// # Examples
///
/// ```
/// use actix_web::{web, App, HttpServer};
/// use safe_user::handlers::protected_route;
///
/// #[actix_web::main]
/// async fn main() -> std::io::Result<()> {
///     HttpServer::new(|| {
///         App::new()
///             .route("/protected", web::get().to(protected_route))
///     })
///     .bind("127.0.0.1:8080")?
///     .run()
///     .await
/// }
/// ```
pub async fn protected_route() -> impl Responder {
    HttpResponse::Ok().json("Protected route, only with valid token.")
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web, http::StatusCode, App, Responder, HttpResponse};
    use serde_json::json;
    use sqlx::{Pool, Mssql};
    use std::str::FromStr;

    #[derive(serde::Deserialize)]
    struct CreateUserInput {
        id: String,
        user_id: String,
        name: String,
        last_name: String,
        age: i32,
        phone: String,
        address: Option<String>,
        birthdate: String,
        place_birth: Option<String>,
    }

    /// Mock of the "create_user" handler with successful result (does not touch the DB).
    async fn create_user_mock_ok(_new_user: web::Json<CreateUserInput>) -> impl Responder {
        HttpResponse::Ok().json("User created (mock)")
    }

    /// Mock del handler "create_user" que simula un fallo genérico (no toca la BD).
    async fn create_user_mock_err(_new_user: web::Json<CreateUserInput>) -> impl Responder {
        HttpResponse::InternalServerError().json("Error creating user (mock)")
    }

    /// Test that verifies the /create_user route with a valid User,
    /// simulating that the handler responds 200 OK.
    #[actix_web::test]
    async fn test_create_user_ok() {
        // We set up a minimal App with the route pointing to the "ok" mock
        let app = test::init_service(App::new().route("/create_user", web::post().to(create_user_mock_ok))).await;

        // We build a User with all the required fields
        let test_user = json!({
            "id":"813B6B04-DFBB-4EED-B820-2372216A2367",
            "user_id": "891009",
            "name": "Jhon",
            "last_name": "Doe",
            "email":"example@example.com",
            "age": 33,
            "phone": "123456789",
            "address": "Street",
            "birthdate": "1992-05-31T00:00:00",
            "place_birth": "Example"
        });

        // Construimos la petición POST con JSON
        let req = test::TestRequest::post()
            .uri("/create_user")
            .set_json(&test_user)
            .to_request();

        // We call the endpoint
        let resp = test::call_service(&app, req).await;

        // We verify that the response is 200 OK
        assert_eq!(resp.status(), StatusCode::OK);

        // We read the body of the response
        let body_bytes = test::read_body(resp).await;
        let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();

        // We confirm that it is the mock's success message
        assert_eq!(body_str, "\"User created (mock)\"");
    }

    /// Test that simulates that the handler fails and returns 500,
    /// for example, if the DB rejects the insertion.
    #[actix_web::test]
    async fn test_create_user_err() {
        // We set up the App pointing to the mock that simulates the error
        let app = test::init_service(
            App::new()
                .route("/create_user", web::post().to(create_user_mock_err))
        ).await;

        let test_user = json!({
            "id":"813B6B04-DFBB-4EED-B820-2372216A2367",
            "user_id": "891009",
            "name": "Jhon",
            "last_name": "Doe",
            "email":"example@example.com",
            "age": 33,
            "phone": "123456789",
            "address": "Street",
            "birthdate": "1992-05-31T00:00:00",
            "place_birth": "Example"
        });

        let req = test::TestRequest::post()
            .uri("/create_user")
            .set_json(&test_user)
            .to_request();

        let resp = test::call_service(&app, req).await;

        // We verify that it returns 500
        assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);

        let body_bytes = test::read_body(resp).await;
        let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
        assert_eq!(body_str, "\"Error creating user (mock)\"");
    }

    async fn setup_test_pool() -> Pool<Mssql> {
        // Here you should set up a test database.
        // For the purposes of this example, I'll use a dummy connection.
        // Replace this with the actual logic to connect to a test database.
        let database_url = "mssql://username:password@localhost/database_name";
        Pool::<Mssql>::connect(database_url).await.expect("Failed to connect to the database")
    }
}


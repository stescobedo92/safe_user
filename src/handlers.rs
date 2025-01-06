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


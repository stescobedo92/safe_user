use actix_web::{test, App, web, Responder, HttpResponse};
use safe_user::models::User;

/// Test handler that does NOT use the database
pub async fn create_user_mock(new_user: web::Json<User>) -> impl Responder {
    // We simulate the user processing and return a success message
    HttpResponse::Ok().json(format!("Mock user created: {}", new_user.name))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web, App};
    use actix_web::http::StatusCode;
    use uuid::Uuid;
    use chrono::NaiveDateTime;
    use sqlx::{Mssql, Pool};

    /// Implement this function to set up a test pool, possibly using a Docker container or an in-memory database
    async fn setup_test_pool() -> Pool<Mssql> {
        // Here you should set up a test database. For example, with SQLx and Docker
        let database_url = "mssql://sa:Tester*31@localhost:1433/master";
        Pool::<Mssql>::connect(database_url).await.expect("Failed to connect to the test database")
    }

    /// Test that verifies the creation of a valid user using the mock
    #[actix_web::test]
    async fn test_create_user_handler() {
        // We set up a minimal App with the route pointing to the mock "create_user_mock"
        let app = test::init_service(App::new().route("/create_user", web::post().to(create_user_mock))).await;

        // We build a test user with all the required fields
        let new_user = User {
            id: Option::from(Uuid::new_v4().to_string()),
            user_id: "456987ADV".to_string(),
            name: "Juan".to_string(),
            last_name: "Pérez".to_string(),
            email: "tes@test.com".to_string(),
            age: Option::from(30),
            phone: Option::from("555-1234".to_string()),
            address: Some("Calle Falsa 123".to_string()),
            birthdate: NaiveDateTime::parse_from_str("1992-03-15T00:00:00", "%Y-%m-%dT%H:%M:%S").unwrap().to_string(),
            place_birth: None,
        };

        // We prepare the POST request with JSON
        let req = test::TestRequest::post().uri("/create_user").set_json(&new_user).to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);

        // We read the body of the response
        let body_bytes = test::read_body(resp).await;
        let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();

        // We check that the response contains the user's name
        assert!(body_str.contains(&new_user.name),"The response must contain the user's name");
    }
}
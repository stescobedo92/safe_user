use serde::{Serialize, Deserialize};
use sqlx::FromRow;

/// Represents a user in the system.
#[derive(Debug, Serialize, FromRow, Deserialize)]
pub struct User {
    /// The unique identifier of the user.
    pub id:  Option<String>,
    /// The id of the user.
    pub user_id: String,
    /// The first name of the user.
    pub name: String,
    /// The last name of the user.
    pub last_name: String,
    /// The email address of the user.
    pub email: String,
    /// The age of the user.
    pub age:  Option<i32>,
    /// The phone number of the user.
    pub phone:  Option<String>,
    /// The address of the user.
    pub address: Option<String>,
    /// The birthdate of the user.
    pub birthdate: String,
    /// The place of birth of the user.
    pub place_birth: Option<String>,
}
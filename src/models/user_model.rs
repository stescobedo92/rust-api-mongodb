use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

/// Represents a user entity.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    /// The unique identifier of the user.
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    /// The name of the user.
    pub name: String,
    /// The location of the user.
    pub location: String,
    /// The title of the user.
    pub title: String,
}
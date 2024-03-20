use std::env;
extern crate dotenv;

use dotenv::dotenv;

use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Client, Collection,
};

use crate::models::user_model::User;

pub struct MongoRepo {
    col: Collection<User>,
}

impl MongoRepo {
    /// Initializes the MongoDB repository.
    ///
    /// # Returns
    ///
    /// The initialized `MongoRepo` instance.
    ///
    /// # Panics
    ///
    /// Panics if there is an error connecting to the database or loading environment variables.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use your_project_name::repository::MongoRepo;
    /// # async fn example_function() {
    /// let repo = MongoRepo::init().await;
    /// println!("MongoDB repository initialized successfully.");
    /// # }
    /// ```
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri)
            .await
            .expect("error connecting to database");
        let db = client.database("rustDB");
        let col: Collection<User> = db.collection("User");
        MongoRepo { col }
    }

    /// Creates a new user in the database asynchronously.
    ///
    /// # Arguments
    ///
    /// * `new_user` - The user object to be created.
    ///
    /// # Returns
    ///
    /// A `Result` containing an `InsertOneResult` if successful, or an `Error` if an error occurs.
    ///
    /// # Errors
    ///
    /// This function may return an error if there is an issue with creating the user in the database.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use crate::models::User;
    /// # use mongodb::error::Error;
    /// # use mongodb::results::InsertOneResult;
    /// # use your_project_name::repository::YourRepository;
    /// # async fn example_function(repo: &YourRepository) -> Result<(), Error> {
    /// let new_user = User {
    ///     id: None,
    ///     name: String::from("John Doe"),
    ///     location: String::from("New York"),
    ///     title: String::from("Software Engineer"),
    /// };
    /// let result = repo.create_user(new_user).await?;
    /// println!("User created successfully: {:?}", result);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
        let new_doc = User {
            id: None,
            name: new_user.name,
            location: new_user.location,
            title: new_user.title,
        };
        let user = self
            .col
            .insert_one(new_doc, None)
            .await
            .ok()
            .expect("Error creating user");

        Ok(user)
    }

    /// Retrieves a user from the database asynchronously.
    ///
    /// # Arguments
    ///
    /// * `id` - A reference to a string representing the ID of the user to retrieve.
    ///
    /// # Returns
    ///
    /// A `Result` containing the retrieved `User` object if successful, or an `Error` if an error occurs.
    ///
    /// # Errors
    ///
    /// This function may return an error if there is an issue with retrieving the user from the database.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use crate::models::User;
    /// # use mongodb::error::Error;
    /// # use your_project_name::repository::YourRepository;
    /// # async fn example_function(repo: &YourRepository) -> Result<(), Error> {
    /// let id = String::from("some_id");
    /// let user = repo.get_user(&id).await?;
    /// println!("User details: {:?}", user);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_user(&self, id: &String) -> Result<User, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .col
            .find_one(filter, None)
            .await
            .ok()
            .expect("Error getting user's detail");

        Ok(user_detail.unwrap())
    }

    /// Updates a user in the database asynchronously.
    ///
    /// # Arguments
    ///
    /// * `id` - A reference to a string representing the ID of the user to update.
    /// * `new_user` - The updated user object.
    ///
    /// # Returns
    ///
    /// A `Result` containing an `UpdateResult` if successful, or an `Error` if an error occurs.
    ///
    /// # Errors
    ///
    /// This function may return an error if there is an issue with updating the user in the database.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use crate::models::User;
    /// # use mongodb::error::Error;
    /// # use mongodb::results::UpdateResult;
    /// # use your_project_name::repository::YourRepository;
    /// # async fn example_function(repo: &YourRepository) -> Result<(), Error> {
    /// let id = String::from("some_id");
    /// let new_user = User {
    ///     id: String::from("new_id"),
    ///     name: String::from("New Name"),
    ///     location: String::from("New Location"),
    ///     title: String::from("New Title"),
    /// };
    /// let result = repo.update_user(&id, new_user).await?;
    /// println!("User updated successfully: {:?}", result);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn update_user(&self, id: &String, new_user: User) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
            "$set":
                {
                    "id": new_user.id,
                    "name": new_user.name,
                    "location": new_user.location,
                    "title": new_user.title
                },
        };
        let updated_doc = self
            .col
            .update_one(filter, new_doc, None)
            .await
            .ok()
            .expect("Error updating user");
        Ok(updated_doc)
    }

    /// Deletes a user from the database asynchronously.
    ///
    /// # Arguments
    ///
    /// * `id` - A reference to a string representing the ID of the user to delete.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `DeleteResult` if successful, or an `Error` if an error occurs.
    ///
    /// # Errors
    ///
    /// This function may return an error if there is an issue with deleting the user from the database.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use mongodb::error::Error;
    /// # use mongodb::results::DeleteResult;
    /// # use your_project_name::repository::YourRepository;
    /// # async fn example_function(repo: &YourRepository) -> Result<(), Error> {
    /// let id = String::from("some_id");
    /// let result = repo.delete_user(&id).await?;
    /// println!("User deleted successfully: {:?}", result);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete_user(&self, id: &String) -> Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .col
            .delete_one(filter, None)
            .await
            .ok()
            .expect("Error deleting user");

        Ok(user_detail)
    }

    /// Retrieves all users from the database asynchronously.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of `User` objects if successful, or an `Error` if an error occurs.
    ///
    /// # Errors
    ///
    /// This function may return an error if there is an issue with querying the database or mapping through the cursor.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use crate::models::User;
    /// # use mongodb::error::Error;
    /// # use your_project_name::repository::YourRepository;
    /// # async fn example_function(repo: &YourRepository) -> Result<(), Error> {
    /// let users = repo.get_all_users().await?;
    /// for user in users {
    ///     println!("User ID: {}, Name: {}", user.id, user.name);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_all_users(&self) -> Result<Vec<User>, Error> {
        let mut cursors = self
            .col
            .find(None, None)
            .await
            .ok()
            .expect("Error getting list of users");
        let mut users: Vec<User> = Vec::new();
        while let Some(user) = cursors
            .try_next()
            .await
            .ok()
            .expect("Error mapping through cursor")
        {
            users.push(user)
        }
        Ok(users)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mongodb::bson::oid::ObjectId;

    #[tokio::test]
    async fn test_create_user() {
        // Arrange
        let repo = MongoRepo::init().await;
        let new_user = User {
            id: Some(ObjectId::new()),
            name: String::from("Test User"),
            location: String::from("Test Location"),
            title: String::from("Test Title"),
        };

        // Act
        let result = repo.create_user(new_user.clone()).await;

        // Assert
        assert!(result.is_ok(), "Failed to create user: {:?}", result.err());
        let inserted_user = result.unwrap();
        assert_eq!(inserted_user.inserted_id, new_user.id);
    }

    #[tokio::test]
    async fn test_get_user() {
        // Arrange
        let repo = MongoRepo::init().await;
        let id = String::from("some_id"); // Provide an existing user ID

        // Act
        let result = repo.get_user(&id).await;

        // Assert
        assert!(result.is_ok(), "Failed to get user: {:?}", result.err());
        let user = result.unwrap();
        assert_eq!(user.name, "Expected Name");
    }

    #[tokio::test]
    async fn test_update_user() {
        // Arrange
        let repo = MongoRepo::init().await;
        let id = String::from("some_id"); // Provide an existing user ID
        let updated_user = User {
            id: Some(ObjectId::new()), // Provide a new ID or the same ID
            name: String::from("Updated Name"),
            location: String::from("Updated Location"),
            title: String::from("Updated Title"),
        };

        // Act
        let result = repo.update_user(&id, updated_user).await;

        // Assert
        assert!(result.is_ok(), "Failed to update user: {:?}", result.err());
        let update_result = result.unwrap();
        assert_eq!(update_result.modified_count, 1);
    }
}
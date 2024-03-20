use crate::{models::user_model::User, repository::mongodb_repo::MongoRepo};
use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path},
    HttpResponse,
};
use mongodb::bson::oid::ObjectId;

#[post("/user")]
pub async fn create_user(db: Data<MongoRepo>, new_user: Json<User>) -> HttpResponse {
    let data = User {
        id: None,
        name: new_user.name.to_owned(),
        location: new_user.location.to_owned(),
        title: new_user.title.to_owned(),
    };

    let user_detail = db.create_user(data).await;

    match user_detail {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/user/{id}")]
pub async fn get_user(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    }
    let user_detail = db.get_user(&id).await;

    match user_detail {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[put("/user/{id}")]
pub async fn update_user(
    db: Data<MongoRepo>,
    path: Path<String>,
    new_user: Json<User>,
) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    };
    let data = User {
        id: Some(ObjectId::parse_str(&id).unwrap()),
        name: new_user.name.to_owned(),
        location: new_user.location.to_owned(),
        title: new_user.title.to_owned(),
    };

    let update_result = db.update_user(&id, data).await;

    match update_result {
        Ok(update) => {
            if update.matched_count == 1 {
                let updated_user_info = db.get_user(&id).await;

                return match updated_user_info {
                    Ok(user) => HttpResponse::Ok().json(user),
                    Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
                };
            } else {
                return HttpResponse::NotFound().body("No user found with specified ID");
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[delete("/user/{id}")]
pub async fn delete_user(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    };
    let result = db.delete_user(&id).await;

    match result {
        Ok(res) => {
            if res.deleted_count == 1 {
                return HttpResponse::Ok().json("User successfully deleted!");
            } else {
                return HttpResponse::NotFound().json("User with specified ID not found!");
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/users")]
pub async fn get_all_users(db: Data<MongoRepo>) -> HttpResponse {
    let users = db.get_all_users().await;

    match users {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use actix_web::test;
    use crate::repository::mongodb_repo::MongoRepo;

    #[tokio::test]
    async fn test_create_user() {
        // Arrange
        let mut app = test::init_service(App::new().data(MongoRepo::init().await)).await;
        let new_user = User {
            id: None,
            name: String::from("Test User"),
            location: String::from("Test Location"),
            title: String::from("Test Title"),
        };
        let req = test::TestRequest::post()
            .uri("/user")
            .set_json(&new_user)
            .to_request();

        // Act
        let resp = test::call_service(&mut app, req).await;

        // Assert
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_get_user() {
        // Arrange
        let mut app = test::init_service(App::new().data(MongoRepo::init().await)).await;
        let id = "some_id"; // Provide an existing user ID
        let req = test::TestRequest::get().uri(&format!("/user/{}", id)).to_request();

        // Act
        let resp = test::call_service(&mut app, req).await;

        // Assert
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_update_user() {
        // Arrange
        let mut app = test::init_service(App::new().data(MongoRepo::init().await)).await;
        let id = "some_id"; // Provide an existing user ID
        let updated_user = User {
            id: None, // Provide a new ID or the same ID
            name: String::from("Updated Name"),
            location: String::from("Updated Location"),
            title: String::from("Updated Title"),
        };
        let req = test::TestRequest::put()
            .uri(&format!("/user/{}", id))
            .set_json(&updated_user)
            .to_request();

        // Act
        let resp = test::call_service(&mut app, req).await;

        // Assert
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
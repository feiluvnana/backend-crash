use crate::features::auth::dto::RegisterRequest;
use crate::db::models::user;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

pub struct AuthService;

impl AuthService {
    /// Find a user by their username
    pub async fn find_by_username(
        db: &DatabaseConnection,
        username: &str,
    ) -> Result<Option<user::Model>, sea_orm::DbErr> {
        user::Entity::find()
            .filter(user::Column::Username.eq(username))
            .one(db)
            .await
    }

    /// Find a user by their email address
    pub async fn find_by_email(
        db: &DatabaseConnection,
        email: &str,
    ) -> Result<Option<user::Model>, sea_orm::DbErr> {
        user::Entity::find()
            .filter(user::Column::Email.eq(email))
            .one(db)
            .await
    }

    /// Find a user by their unique ID
    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<Option<user::Model>, sea_orm::DbErr> {
        user::Entity::find_by_id(id).one(db).await
    }

    /// Create a new user in the database
    pub async fn create_user(
        db: &DatabaseConnection,
        payload: &RegisterRequest,
        hashed_password: String,
    ) -> Result<user::Model, sea_orm::DbErr> {
        let new_user = user::ActiveModel {
            username: Set(payload.username.clone()),
            email: Set(payload.email.clone()),
            password_hash: Set(hashed_password),
            ..Default::default()
        };
        new_user.insert(db).await
    }
}

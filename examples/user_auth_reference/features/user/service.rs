use sea_orm::{
    ActiveModelTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryOrder, Set,
};

use crate::db::models::user;
use crate::features::user::dto::UpdateUserRequest;

pub struct UserService;

impl UserService {
    /// Find a user by their unique ID
    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<Option<user::Model>, sea_orm::DbErr> {
        user::Entity::find_by_id(id).one(db).await
    }

    pub async fn update_user(
        db: &DatabaseConnection,
        id: i32,
        payload: UpdateUserRequest,
    ) -> Result<user::Model, sea_orm::DbErr> {
        let mut active_model = user::ActiveModel {
            id: Set(id),
            ..Default::default()
        };

        if let Some(username) = payload.username {
            active_model.username = Set(username);
        }

        if let Some(email) = payload.email {
            active_model.email = Set(email);
        }

        active_model.update(db).await
    }

    pub async fn list_users(
        db: &DatabaseConnection,
        page: u64,
        per_page: u64,
    ) -> Result<(Vec<user::Model>, u64), sea_orm::DbErr> {
        let paginator = user::Entity::find()
            .order_by_asc(user::Column::Id)
            .paginate(db, per_page);

        let total = paginator.num_items().await?;
        let data = paginator.fetch_page(page - 1).await?;

        Ok((data, total))
    }
}

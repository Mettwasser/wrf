use {
    super::_entities::register_sessions::{ActiveModel, Column, Entity, Model},
    loco_rs::model::ModelResult,
};
use {loco_rs::model::ModelError, sea_orm::entity::prelude::*};
pub type RegisterSessions = Entity;

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    // extend activemodel below (keep comment for generators)

    async fn before_save<C>(self, _db: &C, insert: bool) -> std::result::Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if !insert && self.updated_at.is_unchanged() {
            let mut this = self;
            this.updated_at = sea_orm::ActiveValue::Set(chrono::Utc::now().into());
            Ok(this)
        } else {
            Ok(self)
        }
    }
}

impl RegisterSessions {
    pub async fn find_by_session_id(
        session_id: &str,
        db: &DatabaseConnection,
    ) -> ModelResult<Model> {
        let session = RegisterSessions::find()
            .filter(Column::SessionId.eq(session_id))
            .one(db)
            .await?;

        session.ok_or_else(|| ModelError::EntityNotFound)
    }
}

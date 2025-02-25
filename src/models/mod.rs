pub mod _entities;
pub mod lobbies;
pub mod lobby_bans;
pub mod register_sessions;
pub mod users;
pub mod users_lobbies;

use sea_orm::{
    prelude::*,
    sea_query::{
        Alias,
        IntoIden,
        SelectExpr,
        SelectStatement,
    },
    EntityTrait,
    QueryTrait,
};

pub struct Prefixer<S: QueryTrait<QueryStatement = SelectStatement>> {
    pub selector: S,
}
impl<S: QueryTrait<QueryStatement = SelectStatement>> Prefixer<S> {
    pub fn new(selector: S) -> Self {
        Self { selector }
    }
    pub fn add_columns<T: EntityTrait>(mut self, entity: T) -> Self {
        for col in <T::Column as sea_orm::entity::Iterable>::iter() {
            let alias = format!("{}{}", entity.table_name(), col.to_string()); // we use entity.table_name() as prefix
            self.selector.query().expr(SelectExpr {
                expr: col.select_as(col.into_expr()),
                alias: Some(Alias::new(&alias).into_iden()),
                window: None,
            });
        }
        self
    }
}

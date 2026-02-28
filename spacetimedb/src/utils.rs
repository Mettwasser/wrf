use std::fmt::Display;

use spacetimedb::{
    Table,
    TryInsertError,
};

pub trait MapUniqueViolation<T, F> {
    fn map_unique_violation(self, map: F) -> Result<T, String>;
}

impl<T, F, R> MapUniqueViolation<T::Row, F> for Result<T::Row, TryInsertError<T>>
where
    T: Table,
    F: FnOnce(T::UniqueConstraintViolation) -> R,
    R: Into<String>,
{
    fn map_unique_violation(self, map: F) -> Result<T::Row, String> {
        self.map_err(|e| match e {
            TryInsertError::UniqueConstraintViolation(err) => map(err).into(),
            TryInsertError::AutoIncOverflow(err) => err.to_string(),
        })
    }
}

pub trait ErrorToString<T> {
    fn error_as_string(self) -> Result<T, String>;
}

impl<T, E> ErrorToString<T> for Result<T, E>
where
    E: Display,
{
    fn error_as_string(self) -> Result<T, String> {
        self.map_err(|err| err.to_string())
    }
}

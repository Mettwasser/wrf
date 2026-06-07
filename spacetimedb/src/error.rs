use std::fmt::{
    self,
    Display,
    Formatter,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    UserNotCreated,
    InvalidRelic,
    InvalidLobbySize,
    LobbyAlreadyOpened,
    CantJoinWhileHosting,
    LobbyNotFound,
    BannedFromLobby,
    LobbyFull,
    UsernameTaken,
    JoinMultipleLobbies,
    InsufficientPermissions,
    MissingJwt,
    InvalidUsername,
    WarframeUserNotFound(String),
    Other(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::UserNotCreated => write!(f, "You didn't create a user yet"),
            Error::InvalidUsername => write!(f, "Invalid username"),
            Error::InvalidRelic => write!(f, "Invalid relic"),
            Error::InvalidLobbySize => write!(f, "Invalid lobby size"),
            Error::LobbyAlreadyOpened => write!(f, "You already have an open lobby"),
            Error::CantJoinWhileHosting => {
                write!(f, "You can't join while already hosting a lobby")
            }
            Error::LobbyNotFound => write!(f, "Lobby not found"),
            Error::BannedFromLobby => write!(f, "You are banned in this lobby"),
            Error::LobbyFull => write!(f, "Lobby is full"),
            Error::UsernameTaken => write!(f, "Username already taken"),
            Error::JoinMultipleLobbies => write!(f, "You can't join multiple lobbies"),
            Error::InsufficientPermissions => write!(f, "Insufficient permissions"),
            Error::MissingJwt => write!(f, "Client connected without JWT"),
            Error::WarframeUserNotFound(wf_id) => {
                write!(f, "Warframe User with ID {wf_id} not found")
            }
            Error::Other(e) => write!(f, "{e}"),
        }
    }
}

impl From<Error> for String {
    fn from(error: Error) -> Self {
        error.to_string()
    }
}

impl From<&Error> for String {
    fn from(error: &Error) -> Self {
        error.to_string()
    }
}

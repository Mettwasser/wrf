macro_rules! define_errors {
    (
        $($ident:ident = $lit:literal);* $(;)?
    ) => {
        $(
            pub const $ident: &str = $lit;
        )*
    };
}

define_errors! {
    USER_NOT_CREATED = "You didn't create a user yet";
    INVALID_RELIC = "Invalid relic";
    INVALID_LOBBY_SIZE = "Invalid lobby size";
    LOBBY_ALREADY_OPENED = "You already have an open lobby";
    CANT_JOIN = "You can't join while already hosting a lobby";
}

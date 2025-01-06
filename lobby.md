# Lobbies and how they should look like/behave

- at most 4 players
- should be opened via a Websocket to close them in case of inactivity
- the host should ideally get a notification upon the lobby reaching max players
- lobbies WILL not be stored permanently. Upon closing a lobby, the DB entry will be deleted
- a set expiration that, when reached, will close the lobby

## `lobbies` table
| Column Name | Data Type    | Constraints |
| ----------- | ------------ | ----------- |
| id          | INT          | PK          |
| created_at  | DATETIME     | NOT NULL    |
| expiry      | DATETIME     | NOT NULL    |
| region      | regions      | NOT NULL    |
| activity    | VARCHAR(255) | NOT NULL    |

## `users_lobbies` table
| Column Name | Data Type | Constraints                                 |
| ----------- | --------- | ------------------------------------------- |
| user_id     | INT       | PK REFERENCES users(id) ON DELETE CASCADE   |
| lobby_id    | INT       | PK REFERENCES lobbies(id) ON DELETE CASCADE |

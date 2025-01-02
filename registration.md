# Registration and how it works

A register-session is a session only used for registering.

## The process
```mermaid
sequenceDiagram
    participant Server
    participant Client
    participant Warframe

    Client-)Server: Request Account verification token
    Server-)Client: Responds with a Account verification token <br> and a register session
    Warframe<<->>Client: puts the token in loadout name and submits
    Client->>Server: Sends session and WF username <br> and asks the server to verify the user
    Note over Server: Verifies by requesting DE's profile API <br> and checking if the loadout name <br> matches with the token associated with the session
```

## `register_sessions` table
| Column Name       | Data Type | Constraints |
| ----------------- | --------- | ----------- |
| id                | int       | PK          |
| created_at        | DATETIME  | NOT NULL    |
| updated_at        | DATETIME  |             |
| session_id        | UUID      | PK          |
| expiry            | DATETIME  | NOT NULL    |
| verification_code | STRING    | NOT NULL    |


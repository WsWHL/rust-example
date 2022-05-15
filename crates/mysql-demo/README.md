
#### diesel orm example

[env config]
- `DATABASE_URL` in `.env` file


[dep]
- diesel 2.0.0-rc.0
- dotenv 0.15.0

[project struct]
```
.
├── Cargo.lock
├── Cargo.toml
├── README.md
├── diesel.toml
├── migrations
├── src
│   ├── main.rs
│   ├── models.rs
│   └── schema.rs
```
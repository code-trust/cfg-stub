# cfg-stub

Proc macros that replace function and impl method bodies with `unimplemented!()` when a feature is disabled, so you keep one impl without hand-written stubs.

## Example

```rust
#[cfg_attr(not(feature = "sqlx"), cfg_stub::methods)]
impl UserStore {
    async fn get(&self, id: u64) -> Option<User> {
        sqlx::query_as!(User, "SELECT … WHERE id = $1", id)
            .fetch_optional(&self.pool)
            .await
            .unwrap()
    }
}
```

With `sqlx` off, every method body becomes `unimplemented!()`. With `sqlx` on, your code runs as written.

`#[cfg_attr(not(feature = "sqlx"), cfg_stub::function)]` works the same on free functions.

## License

MIT

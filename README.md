# Reproduce [559](https://github.com/napi-rs/napi-rs/pull/559)

On windows PC:

- `choco install nodejs-lts -x86 --force -y`
- Add `C:\Program Files (x86)\nodejs` into your `PATH`
- Ensure `node -v` command executes the **32 bit version**
- `cargo build`
- `cp target/debug/test.dll ./test.node`
- `node index.js`

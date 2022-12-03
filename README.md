# Static Generation for Yew

## How to use

edit `AppTemplate` and `AppRouter` component of `src/app.rs` and `src/router.rs`.

### build

1. inject hydration and other asset. output to `dist/`
   ```sh
   $ trunk build
   ```
2. run `generate` command. output to `static/`
   ```sh
   $ cargo run --feature=ssg --bin generate
   ```
3. Deploy the files under `static/` to the server.

See [workflows/yew-sg.yml](/.github/workflows/yew-sg.yml)

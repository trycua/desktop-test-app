// Prevents additional console window on Windows in release
// Note: we intentionally allow the console on Windows so APP_HTTP_PORT=6769
// is printed to stdout where tests can see it.
fn main() {
    desktop_test_app_lib::run()
}

# desktop-test-app

Tauri v2 desktop app that logs all input events (mouse, keyboard, clipboard) to an HTTP API on port **6769**. Used by [cua-sandbox](https://github.com/trycua/cua) integration tests to verify `sb.*` interface methods on Linux, macOS, and Windows sandboxes.

Mirrors the pattern of [android-touch-test-app](https://github.com/trycua/android-touch-test-app) for desktop platforms.

## HTTP API

| Endpoint | Description |
|---|---|
| `GET /health` | 200 OK when ready |
| `GET /events` | JSON array of all input events since last reset |
| `POST /reset` | Clear event log → `{"cleared": N}` |
| `GET /clipboard` | Current clipboard text → `{"text": "..."}` |
| `GET /window-title` | Active window title → `{"title": "..."}` |
| `GET /screen-size` | Screen dimensions → `{"width": W, "height": H}` |

## Event types

| `type` | `details` |
|---|---|
| `mouse_click` | x, y, button |
| `mouse_right_click` | x, y |
| `mouse_double_click` | x, y, button |
| `mouse_move` | x, y |
| `mouse_scroll` | x, y, delta_x, delta_y |
| `mouse_down` | x, y, button |
| `mouse_up` | x, y, button |
| `mouse_drag` | start_x, start_y, end_x, end_y |
| `key_down` | key, code, modifiers[] |
| `key_up` | key, code, modifiers[] |
| `type_text` | text (current input value) |
| `clipboard_change` | text |

## Download

```python
_BASE = "https://github.com/trycua/desktop-test-app/releases/latest/download"
_URLS = {
    "linux":       f"{_BASE}/desktop-test-app-linux-x86_64",
    "macos-arm64": f"{_BASE}/desktop-test-app-macos-arm64",
    "macos-x86":   f"{_BASE}/desktop-test-app-macos-x86_64",
    "windows":     f"{_BASE}/desktop-test-app-windows-x86_64.exe",
}
```

## Use in tests

```python
from cua_sandbox.runtime.compat import skip_if_unsupported
from cua_sandbox.image import Image
from cua_sandbox.sandbox import Sandbox

async def test_mouse_click():
    skip_if_unsupported(Image.linux())
    image = Image.linux().copy("/tmp/desktop-test-app-linux-x86_64", "/tmp/test-app")
    async with Sandbox.ephemeral(image, local=True) as sb:
        await sb.shell.run("chmod +x /tmp/test-app && /tmp/test-app &")
        await _wait_for_health(sb)  # polls GET /health
        await sb.shell.run("curl -s -X POST http://localhost:6769/reset")
        await sb.mouse.click(500, 400)
        import asyncio; await asyncio.sleep(0.3)
        r = await sb.shell.run("curl -s http://localhost:6769/events")
        events = json.loads(r.stdout)
        assert any(e["type"] == "mouse_click" for e in events)
```

## Build locally

```bash
cargo install tauri-cli
tauri build
```

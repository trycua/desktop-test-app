import { invoke } from "@tauri-apps/api/core";

const API = "http://127.0.0.1:6769";

// ── Helpers ────────────────────────────────────────────────────────────────

function emit(type, details) {
  invoke("log_event", { eventType: type, details }).catch(() => {});
}

function appendLog(containerId, type, text) {
  const el = document.getElementById(containerId);
  if (!el) return;
  const div = document.createElement("div");
  div.className = "log-entry";
  div.innerHTML = `<span class="etype">${type}</span> <span class="detail">${text}</span>`;
  el.appendChild(div);
  el.scrollTop = el.scrollHeight;
  // Keep max 200 entries
  while (el.children.length > 200) el.removeChild(el.firstChild);
}

// ── Screen size on startup ─────────────────────────────────────────────────

window.addEventListener("DOMContentLoaded", () => {
  invoke("set_screen_size", {
    width: window.screen.width,
    height: window.screen.height,
    title: document.title,
  });
  document.getElementById("status").textContent = "ready";
});

// ── Mouse events ───────────────────────────────────────────────────────────

document.addEventListener("click", (e) => {
  const d = { x: e.screenX, y: e.screenY, button: "left" };
  emit("mouse_click", d);
  appendLog("mouse-log", "mouse_click", `(${d.x}, ${d.y}) left`);
});

document.addEventListener("contextmenu", (e) => {
  const d = { x: e.screenX, y: e.screenY };
  emit("mouse_right_click", d);
  appendLog("mouse-log", "mouse_right_click", `(${d.x}, ${d.y})`);
});

document.addEventListener("dblclick", (e) => {
  const d = { x: e.screenX, y: e.screenY, button: "left" };
  emit("mouse_double_click", d);
  appendLog("mouse-log", "mouse_double_click", `(${d.x}, ${d.y})`);
});

document.addEventListener("mousemove", (() => {
  let last = 0;
  return (e) => {
    const now = Date.now();
    if (now - last < 50) return; // throttle to 20 fps
    last = now;
    const d = { x: e.screenX, y: e.screenY };
    emit("mouse_move", d);
    appendLog("mouse-log", "mouse_move", `(${d.x}, ${d.y})`);
  };
})());

document.addEventListener("mousedown", (e) => {
  const btn = ["left", "middle", "right"][e.button] || "unknown";
  const d = { x: e.screenX, y: e.screenY, button: btn };
  emit("mouse_down", d);
  appendLog("mouse-log", "mouse_down", `(${d.x}, ${d.y}) ${btn}`);
});

document.addEventListener("mouseup", (e) => {
  const btn = ["left", "middle", "right"][e.button] || "unknown";
  const d = { x: e.screenX, y: e.screenY, button: btn };
  emit("mouse_up", d);
  appendLog("mouse-log", "mouse_up", `(${d.x}, ${d.y}) ${btn}`);
});

document.addEventListener("wheel", (e) => {
  const d = { x: e.screenX, y: e.screenY, delta_x: e.deltaX, delta_y: e.deltaY };
  emit("mouse_scroll", d);
  appendLog("mouse-log", "mouse_scroll", `(${d.x}, ${d.y}) dx=${d.delta_x} dy=${d.delta_y}`);
});

// Drag tracking
let dragStart = null;
document.addEventListener("dragstart", (e) => {
  dragStart = { x: e.screenX, y: e.screenY };
});
document.addEventListener("dragend", (e) => {
  if (dragStart) {
    const d = { start_x: dragStart.x, start_y: dragStart.y, end_x: e.screenX, end_y: e.screenY };
    emit("mouse_drag", d);
    appendLog("mouse-log", "mouse_drag", `(${d.start_x},${d.start_y})→(${d.end_x},${d.end_y})`);
    dragStart = null;
  }
});

// Drag target
const dragTarget = document.getElementById("drag-target");
dragTarget.addEventListener("dragover", (e) => { e.preventDefault(); dragTarget.className = "over"; });
dragTarget.addEventListener("dragleave", () => { dragTarget.className = ""; });
dragTarget.addEventListener("drop", (e) => { e.preventDefault(); dragTarget.className = "dropped"; dragTarget.textContent = "dropped!"; });

// ── Keyboard events ────────────────────────────────────────────────────────

function modifiers(e) {
  const mods = [];
  if (e.ctrlKey) mods.push("ctrl");
  if (e.shiftKey) mods.push("shift");
  if (e.altKey) mods.push("alt");
  if (e.metaKey) mods.push("meta");
  return mods;
}

document.addEventListener("keydown", (e) => {
  const mods = modifiers(e);
  const d = { key: e.key, code: e.code, modifiers: mods };
  emit("key_down", d);
  appendLog("key-log", "key_down", `${mods.join("+")}${mods.length ? "+" : ""}${e.key}`);
});

document.addEventListener("keyup", (e) => {
  const mods = modifiers(e);
  const d = { key: e.key, code: e.code, modifiers: mods };
  emit("key_up", d);
  appendLog("key-log", "key_up", `${e.key}`);
});

// Text input for type() verification
const textInput = document.getElementById("text-input");
textInput.addEventListener("input", (e) => {
  const d = { text: e.target.value };
  emit("type_text", d);
  appendLog("key-log", "type_text", `"${d.text}"`);
});

// ── Clipboard polling (supplement to Rust monitor) ─────────────────────────

setInterval(async () => {
  try {
    const r = await fetch(`${API}/clipboard`);
    const data = await r.json();
    document.getElementById("clipboard-content").textContent =
      `clipboard: ${data.text || "—"}`;
  } catch (_) {}
}, 1000);

// ── Reset button ───────────────────────────────────────────────────────────

document.getElementById("reset-btn").addEventListener("click", async () => {
  await fetch(`${API}/reset`, { method: "POST" });
  document.getElementById("mouse-log").innerHTML = "";
  document.getElementById("key-log").innerHTML = "";
  dragTarget.className = "";
  dragTarget.textContent = "drag here";
  textInput.value = "";
});

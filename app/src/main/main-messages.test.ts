/** ADR-052: exercise the registered Electron handler and real S-6 services. */
import { mkdtempSync, rmSync } from "node:fs";
import os from "node:os";
import path from "node:path";
import { afterEach, beforeEach, expect, it, vi } from "vitest";

interface Reply {
  ok: boolean;
  value?: unknown;
  error?: { code: string };
}

const host = vi.hoisted(() => ({
  dir: "",
  url: "",
  sent: [] as { type: string; request?: number }[],
  coreEvents: new Map<string, (value: unknown) => void>(),
  pageEvents: new Map<string, () => void>(),
  handlers: new Map<string, (event: unknown, ...args: unknown[]) => Reply | Promise<Reply>>(),
}));

// Only Electron and unrelated start-up file cleanup are replaced. main.ts
// registers its own listener/IPC handlers; no production-only test exports.
vi.mock("electron", () => ({
  app: {
    getPath: () => host.dir,
    getAppPath: () => host.dir,
    getVersion: () => "0.0.0",
    getPreferredSystemLanguages: () => ["en"],
    isPackaged: false,
    commandLine: { hasSwitch: () => true },
    whenReady: () => Promise.resolve(),
    on: vi.fn(),
  },
  utilityProcess: {
    fork: () => ({
      on: (event: string, listener: (value: unknown) => void) =>
        host.coreEvents.set(event, listener),
      postMessage: (message: { type: string; request?: number }) => host.sent.push(message),
    }),
  },
  BrowserWindow: class {
    webContents = {
      id: 7,
      on: (event: string, listener: () => void) => host.pageEvents.set(event, listener),
      setWindowOpenHandler: vi.fn(),
      postMessage: vi.fn(),
      send: vi.fn(),
    };
    once = vi.fn();
    on = vi.fn();
    loadURL(url: string) {
      host.url = url;
      required(host.pageEvents.get("did-finish-load"))();
      return Promise.resolve();
    }
  },
  MessageChannelMain: class {
    port1 = {};
    port2 = {};
  },
  Menu: { setApplicationMenu: vi.fn(), buildFromTemplate: () => [] },
  session: { defaultSession: { setPermissionRequestHandler: vi.fn() } },
  shell: {},
  ipcMain: {
    handle: (
      channel: string,
      handler: (event: unknown, ...args: unknown[]) => Reply | Promise<Reply>,
    ) => host.handlers.set(channel, handler),
  },
}));
vi.mock("./save-files.js", () => ({ createSaveFiles: () => ({ removeTempFiles: vi.fn() }) }));

const ready = {
  type: "ready",
  info: { name: "core", version: "0.0.0" },
  packs: [],
  languages: [{ code: "en", name: "English", keys: 1 }],
};
function required<T>(value: T | undefined): T {
  if (value === undefined) throw new Error("missing test fixture");
  return value;
}
function receive(raw: unknown): void {
  required(host.coreEvents.get("message"))(raw);
}
function call(channel: string, arg?: unknown): Promise<Reply> {
  return Promise.resolve(
    required(host.handlers.get(channel))(
      { sender: { id: 7 }, senderFrame: { url: host.url, parent: null } },
      arg,
    ),
  );
}
function request(type: string): number {
  const last = host.sent.findLast((message) => message.type === type);
  expect(last?.request).toBeDefined();
  return required(last?.request);
}
function saveAnswer(id: number, value: unknown = null): void {
  receive({ type: "savesAnswer", request: id, ok: true, value });
}
const flush = () => vi.advanceTimersByTimeAsync(0);

beforeEach(async () => {
  vi.resetModules();
  vi.useFakeTimers();
  vi.spyOn(console, "log").mockImplementation(() => undefined);
  vi.spyOn(console, "warn").mockImplementation(() => undefined);
  host.dir = mkdtempSync(path.join(os.tmpdir(), "oa-main-messages-"));
  host.sent = [];
  host.coreEvents.clear();
  host.pageEvents.clear();
  host.handlers.clear();
  await import("./main.js");
  expect(host.coreEvents.has("message")).toBe(true);
  receive(ready);
  expect(globalThis.openairsideCoreStatus.state).toBe("ready");
});
afterEach(() => {
  vi.clearAllTimers();
  vi.useRealTimers();
  vi.restoreAllMocks();
  rmSync(host.dir, { recursive: true, force: true });
});

it("discards unknown/malformed status without changing reports, requests or ports", async () => {
  const pending = call("game:new", { map: 1536 });
  const id = request("game:new");
  const status = globalThis.openairsideCoreStatus;
  const ports = host.sent.filter((message) => message.type === "port").length;
  for (const raw of [
    null,
    [],
    42,
    {},
    { type: "unknown", request: id },
    { ...ready, languages: [{ code: "en", name: "", keys: 1 }] },
    { type: "error", message: 1 },
  ]) {
    receive(raw);
    expect(globalThis.openairsideCoreStatus).toBe(status);
    expect(host.sent.filter((message) => message.type === "port")).toHaveLength(ports);
    expect((await call("lang:list")).value).toEqual(ready.languages);
  }
  saveAnswer(id);
  expect((await pending).ok).toBe(true);
  const next = call("game:new", { map: 1536 });
  saveAnswer(request("game:new"));
  expect((await next).ok).toBe(true);
});

it.each([undefined, "1", -1, 0.25, NaN, Infinity, Number.MAX_SAFE_INTEGER + 1, 0, 999])(
  "never substitutes a pending ID for %s",
  async (id) => {
    let settled = false;
    const pending = call("game:new", { map: 1536 }).then((reply) => {
      settled = true;
      return reply;
    });
    const current = request("game:new");
    receive({ type: "savesAnswer", request: id, ok: "bad" });
    receive({ type: "packs", request: id, packs: "bad" });
    await flush();
    expect(settled).toBe(false);
    saveAnswer(current);
    expect((await pending).ok).toBe(true);
  },
);

it("rejects a matching malformed save answer immediately, clears timer/lock and permits follow-up", async () => {
  const pending = call("game:new", { map: 1536 });
  const id = request("game:new");
  receive({ type: "savesAnswer", request: id, ok: "bad" });
  expect((await pending).error?.code).toBe("invalidAnswer");
  expect(vi.getTimerCount()).toBe(0);
  const next = call("game:new", { map: 2048 });
  const current = request("game:new");
  let settled = false;
  void next.then(() => {
    settled = true;
  });
  receive({ type: "savesAnswer", request: id, ok: "bad" });
  saveAnswer(id);
  await flush();
  expect(settled).toBe(false);
  saveAnswer(current);
  expect((await next).ok).toBe(true);
  receive({ type: "savesAnswer", request: current, ok: "bad" });
  expect(globalThis.openairsideCoreStatus.state).toBe("ready");
});

it("keeps timeout behavior while old save replies cannot settle the next operation", async () => {
  const first = call("game:new", { map: 1536 });
  const old = request("game:new");
  await vi.advanceTimersByTimeAsync(120_000);
  expect((await first).error?.code).toBe("timeout");
  let settled = false;
  const next = call("game:new", { map: 1536 }).then((reply) => {
    settled = true;
    return reply;
  });
  receive({ type: "savesAnswer", request: old, ok: "bad" });
  saveAnswer(old);
  await flush();
  expect(settled).toBe(false);
  saveAnswer(request("game:new"));
  expect((await next).ok).toBe(true);
});

it("isolates answer families with equal IDs and ignores late malformed pack replies", async () => {
  const first = call("packs:setActive", []);
  await vi.waitFor(() => {
    expect(host.sent.some((message) => message.type === "packs:setActive")).toBe(true);
  });
  const old = request("packs:setActive");
  const texts = call("lang:texts", "en");
  const textId = request("lang:texts");
  expect(textId).toBe(old);
  receive({ type: "packs", request: old, packs: "bad", languages: [] });
  expect((await first).error?.code).toBe("invalidAnswer");
  saveAnswer(textId, [["ui.title", "Title"]]);
  expect((await texts).value).toEqual({ "ui.title": "Title" });
  const second = call("packs:setActive", []);
  await vi.waitFor(() => {
    expect(request("packs:setActive")).not.toBe(old);
  });
  const current = request("packs:setActive");
  let settled = false;
  void second.then(() => {
    settled = true;
  });
  for (const raw of [
    { type: "packs", request: old, packs: "bad" },
    { type: "packsError", request: old, message: "invalidAnswer" },
    { type: "packs", request: old, packs: [], languages: [] },
    { type: "packs", packs: "bad" },
  ])
    receive(raw);
  await flush();
  expect(settled).toBe(false);
  receive({ type: "packs", request: current, packs: [], languages: ready.languages });
  expect((await second).ok).toBe(true);
});

it("does not reject a same-numbered pack request when a malformed saves reply arrives", async () => {
  let packSettled = false;
  const pack = call("packs:setActive", []).then((reply) => {
    packSettled = true;
    return reply;
  });
  await vi.waitFor(() => {
    expect(host.sent.some((message) => message.type === "packs:setActive")).toBe(true);
  });
  const packId = request("packs:setActive");
  const text = call("lang:texts", "en");
  expect(request("lang:texts")).toBe(packId);
  receive({ type: "savesAnswer", request: packId, ok: false, error: "bad" });
  expect((await text).error?.code).toBe("invalidAnswer");
  expect(packSettled).toBe(false);
  receive({ type: "packs", request: packId, packs: [], languages: ready.languages });
  expect((await pack).ok).toBe(true);
});

it("preserves pack timeout and rejects malformed correlated reports without partial adoption", async () => {
  const first = call("packs:setActive", []);
  await vi.waitFor(() => {
    expect(host.sent.some((message) => message.type === "packs:setActive")).toBe(true);
  });
  const old = request("packs:setActive");
  await vi.advanceTimersByTimeAsync(60_000);
  expect((await first).error?.code).toBe("timeout");
  const next = call("packs:setActive", []);
  await vi.waitFor(() => {
    expect(request("packs:setActive")).not.toBe(old);
  });
  receive({ type: "packs", request: old, packs: "bad" });
  receive({
    type: "packs",
    request: request("packs:setActive"),
    packs: [],
    languages: [...ready.languages, { code: "de", name: "x".repeat(4097), keys: 1 }],
  });
  expect((await next).error?.code).toBe("invalidAnswer");
  expect((await call("lang:list")).value).toEqual(ready.languages);
});

it("rejects overlong texts through the real settings service and clears deduplication", async () => {
  const a = call("lang:texts", "en");
  const id = request("lang:texts");
  const b = call("lang:texts", "en");
  expect(host.sent.filter((message) => message.type === "lang:texts")).toHaveLength(1);
  saveAnswer(id, [["ui.title", "😀".repeat(4097)]]);
  expect((await a).error?.code).toBe("invalidAnswer");
  expect((await b).error?.code).toBe("invalidAnswer");
  const next = call("lang:texts", "en");
  expect(request("lang:texts")).not.toBe(id);
  saveAnswer(request("lang:texts"), [["__proto__", "😀".repeat(4096)]]);
  const reply = await next;
  expect(reply.ok).toBe(true);
  expect(Object.getPrototypeOf(reply.value)).toBeNull();
});

it("retains genuine errors and process-exit rejection/restart", async () => {
  const pending = call("game:new", { map: 1536 });
  receive({
    type: "savesAnswer",
    request: request("game:new"),
    ok: false,
    error: { code: "busy", textKey: "save.error.busy", params: {} },
  });
  expect((await pending).error?.code).toBe("busy");
  receive({ type: "error", message: "start failure" });
  expect(globalThis.openairsideCoreStatus.state).toBe("failed");
  receive(ready);
  const next = call("game:new", { map: 1536 });
  required(host.coreEvents.get("exit"))(1);
  expect((await next).error?.code).toBe("coreExited");
});

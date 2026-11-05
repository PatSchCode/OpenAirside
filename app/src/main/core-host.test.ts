/** ADR-053: registered host listener, real parsers, isolated native/transport edges. */
import { afterEach, beforeEach, expect, it, vi } from "vitest";
import type { BridgeOutgoing } from "./core-port.js";

const fixture = vi.hoisted(() => ({
  openPort: vi.fn(),
  receive: vi.fn((value: unknown): BridgeOutgoing[] => [{ message: { reply: value } }]),
  pump: vi.fn((): BridgeOutgoing[] => [{ message: { pump: "marker" } }]),
  setActivePacks: vi.fn(() => []),
  newGame: vi.fn(() => Promise.resolve()),
  texts: vi.fn(() => ({ hello: "Hello" })),
}));
vi.mock("node:module", () => ({
  createRequire: () => () => ({
    Core: class {
      openPort = fixture.openPort;
      receive = fixture.receive;
      pump = fixture.pump;
      setActivePacks = fixture.setActivePacks;
      newGame = fixture.newGame;
      texts = fixture.texts;
      packs = () => [];
      languages = () => [];
      status = () => ({ session: 1, running: true });
    },
    coreInfo: () => ({ name: "test", version: "0" }),
  }),
}));

function port() {
  return {
    close: vi.fn(),
    on: vi.fn<(name: string, callback: (event: { data: unknown }) => void) => void>(),
    start: vi.fn(),
    postMessage: vi.fn(),
  };
}
let handler: (event: { data: unknown; ports: ReturnType<typeof port>[] }) => void;
const post = vi.fn();
const original = Object.getOwnPropertyDescriptor(process, "parentPort");
beforeEach(async () => {
  vi.resetModules();
  vi.clearAllMocks();
  vi.useFakeTimers();
  Object.defineProperty(process, "parentPort", {
    configurable: true,
    value: {
      postMessage: post,
      on: (_name: string, listener: typeof handler) => {
        handler = listener;
      },
    },
  });
  await import("./core-host.js");
  expect(post).toHaveBeenCalledWith(expect.objectContaining({ type: "ready" }));
});
afterEach(() => {
  vi.clearAllTimers();
  vi.useRealTimers();
  if (original === undefined) Reflect.deleteProperty(process, "parentPort");
  else Object.defineProperty(process, "parentPort", original);
});
function livePort() {
  const current = port();
  handler({ data: { type: "port" }, ports: [current] });
  return current;
}
function expectPreserved(current: ReturnType<typeof port>, rejected: ReturnType<typeof port>[]) {
  expect(current.close).not.toHaveBeenCalled();
  expect(fixture.openPort).toHaveBeenCalledTimes(1);
  for (const candidate of rejected) {
    expect(candidate.on).not.toHaveBeenCalled();
    expect(candidate.start).not.toHaveBeenCalled();
  }
  vi.advanceTimersByTime(50);
  expect(current.postMessage).toHaveBeenLastCalledWith({ pump: "marker" });
  for (const candidate of rejected) expect(candidate.postMessage).not.toHaveBeenCalled();
}

it("routes replies and pump to the initial and then replacement port exactly once", () => {
  const current = livePort();
  expect(current.start).toHaveBeenCalledTimes(1);
  expect(current.on).toHaveBeenCalledTimes(1);
  const first = current.on.mock.calls[0]?.[1];
  if (first === undefined) throw new Error("missing first handler");
  first({ data: "first" });
  expect(current.postMessage).toHaveBeenLastCalledWith({ reply: "first" });
  vi.advanceTimersByTime(50);
  expect(current.postMessage).toHaveBeenLastCalledWith({ pump: "marker" });
  current.postMessage.mockClear();

  const next = port();
  handler({ data: { type: "port", harmless: true }, ports: [next] });
  expect(current.close).toHaveBeenCalledTimes(1);
  expect(next.on).toHaveBeenCalledTimes(1);
  expect(next.start).toHaveBeenCalledTimes(1);
  expect(fixture.openPort).toHaveBeenCalledTimes(2);
  const second = next.on.mock.calls[0]?.[1];
  if (second === undefined) throw new Error("missing replacement handler");
  second({ data: "second" });
  expect(next.postMessage).toHaveBeenLastCalledWith({ reply: "second" });
  vi.advanceTimersByTime(50);
  expect(next.postMessage).toHaveBeenLastCalledWith({ pump: "marker" });
  expect(current.postMessage).not.toHaveBeenCalled();
});

it.each([
  null,
  undefined,
  false,
  1,
  "port",
  [],
  {},
  { type: 1 },
  { type: "unknown" },
  { type: "autosave:interval", minutes: -1 },
  { type: "packs:setActive", request: 1, ids: null },
  { type: "game:new", request: "7", map: 1536 },
])("preserves live connection for rejected message %j with a port", (data) => {
  const current = livePort();
  const candidate = port();
  handler({ data, ports: [candidate] });
  expectPreserved(current, [candidate]);
});
it.each([0, 2])("rejects port handoff with %i transport ports", (count) => {
  const current = livePort();
  const candidates = Array.from({ length: count }, port);
  handler({ data: { type: "port" }, ports: candidates });
  expectPreserved(current, candidates);
});
it("rejects an array carrying a type property", () => {
  const current = livePort();
  const candidate = port();
  handler({ data: Object.assign([], { type: "port" }), ports: [candidate] });
  expectPreserved(current, [candidate]);
});
it("keeps invalidArgument for a readable invalid save request with a port", () => {
  const current = livePort();
  const candidate = port();
  handler({ data: { type: "game:new", request: 7, map: 1 }, ports: [candidate] });
  expect(post).toHaveBeenCalledWith({
    type: "savesAnswer",
    request: 7,
    ok: false,
    error: { code: "invalidArgument", textKey: "save.error.invalidArgument", params: {} },
  });
  expectPreserved(current, [candidate]);
});
it("processes a valid pack request with an attached port without handoff", () => {
  const current = livePort();
  const candidate = port();
  handler({ data: { type: "packs:setActive", request: 8, ids: [] }, ports: [candidate] });
  expect(fixture.setActivePacks).toHaveBeenCalledWith([]);
  expect(post).toHaveBeenCalledWith({ type: "packs", request: 8, packs: [], languages: [] });
  expectPreserved(current, [candidate]);
});
it("processes game and language requests with ports without handoff", async () => {
  const current = livePort();
  const candidate = port();
  handler({ data: { type: "game:new", request: 9, map: 1536 }, ports: [candidate] });
  handler({ data: { type: "lang:texts", request: 10, code: "en" }, ports: [candidate] });
  await vi.advanceTimersByTimeAsync(0);
  expect(fixture.newGame).toHaveBeenCalledWith(1536);
  expect(fixture.texts).toHaveBeenCalledWith("en");
  expect(post).toHaveBeenCalledWith({ type: "savesAnswer", request: 9, ok: true, value: null });
  expect(post).toHaveBeenCalledWith({
    type: "savesAnswer",
    request: 10,
    ok: true,
    value: { hello: "Hello" },
  });
  expectPreserved(current, [candidate]);
});
it("applies a valid autosave interval with a port without handoff", () => {
  const current = livePort();
  const candidate = port();
  handler({ data: { type: "autosave:interval", minutes: 1 }, ports: [candidate] });
  expectPreserved(current, [candidate]);
  vi.advanceTimersByTime(61_000);
  expect(post).toHaveBeenCalledWith({ type: "autosave" });
  expect(current.close).not.toHaveBeenCalled();
  expect(fixture.openPort).toHaveBeenCalledTimes(1);
});

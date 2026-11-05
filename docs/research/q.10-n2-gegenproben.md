# Q.10-N2 – isolierte S-8-Gegenproben

**Basis:** Produkt `468bc32`; Archivquelle `d160277` (nur zusätzliche N3-Abschlussdokumentation). Node 26.11.1, Vitest 5.0.4, vorhandene unveränderte node_modules. Keine Installation oder neue Abhängigkeit.

Die erste Shell verwendete Node 24.15.0. Nach Prüfung wurde die bereits vorhandene fnm-Installation 26.11.1 explizit aktiviert; Baseline und abschließende Gegenproben darunter erneut ausgeführt, mit denselben 61 grünen Bestandsfällen sowie vier grünen Kontrollen und sechs roten Befundassertionen. Maßgeblicher Nachweis ist der Lauf unter der Projektversion; keine erfolgreiche frische Installation behauptet.

## Verfahren und Grenzen

`git archive d160277` wurde in ein temporäres Verzeichnis entpackt; node_modules ist ein Link auf die bestehende Installation. Einziger Eingriff in eine Produktdatei der isolierten Kopie: am Ende von `app/src/main/main.ts` der folgende Testzugriff; die übrigen Bytes und damit Produktalgorithmen bleiben unverändert:

```typescript
// Probe accessors only in the isolated source copy; production logic unchanged.
export const n2Probe = { startCore, requestSaves, packs, saves, lock };
```

Der Test mockt Electron, hält den App-Startcallback zurück und ruft `startCore` explizit auf. Er verwendet den dadurch tatsächlich registrierten `message`-Listener sowie reale Parser, Dienstlogik, Dateizugriffe unter temporären Pfaden und Operationssperren; Timer sind kontrolliert simuliert. Er testet weder Fenster/Preload noch echten Structured-Clone-Transport oder den nativen Rust-Lader. Keine Aussage über einen bösartigen, vom normalen Lader akzeptierten Mod.

Reproduktion im isolierten Verzeichnis mit diesem Zugriff und der untenstehenden Datei:

```bash
node --version # Projektversion v26.11.1 zuerst aktivieren, siehe Runbook
npx --no-install vitest run app/src/main/core-status.test.ts app/src/main/settings.test.ts app/src/main/packs-service.test.ts
npx --no-install vitest run app/src/main/n2-probe.test.ts
```

Bestand: 61 Tests in drei Dateien bestanden, Exit 0. Abschließende Gegenproben: 10 Fälle, vier Kontrollen grün, sechs Befundassertionen erwartungsgemäß rot, Exit 1. P1: `failed` und `notReady`; P2: alte Nummer 1 beendet neue Nummer 2 als `invalidAnswer`; P3: unmittelbar `pending`, Lock belegt, erst nach 120 s `timeout`; P4: Name leer/4.097 sowie Text 4.097 akzeptiert. Diese verschiedenen absichtlich roten Befundproben sind keine drei gescheiterten Reparaturversuche.

Die Testquelle dient dem eingefrorenen Fehlernachweis. Bei Produktkorrektur werden dauerhafte Regressionen mit vollständiger Wiederherstellungsprüfung angelegt; eine nach Korrektur hypothetisch unaufgelöste Promise in der P1-Hülle ist kein Abschlussnachweis. Die ursprünglichen roten Assertions werden hier nicht nachträglich grün geschrieben.

Eigene abschließende Vorlagenprüfung: Die positive `lang:texts`-Kontrolle wurde von lediglich lesbarer Antwort-Hülle auf semantisch gültige Sprachpaare präzisiert. Erneuter Lauf unter Node 26.11.1: dieselben vier Kontrollen grün und sechs Befundassertionen rot. Keine rote Assertion oder Produktlogik geändert; SHA unten bezeichnet die abschließend ausgeführte Quelle.

## Datei: app/src/main/n2-probe.test.ts

SHA-256: 274812f3633ccef994afa85a0b0659c00f50d5041a36a3f9ba5a62cff322d4f1.

```typescript
/** N2 isolated probes: actual main listener, mocked Electron transport only. */
import { mkdtempSync } from "node:fs";
import os from "node:os";
import path from "node:path";
import { afterEach, beforeEach, expect, it, vi } from "vitest";
import { parseLanguageList, parseTexts } from "./settings.js";

const transport = vi.hoisted(() => ({
  listener: undefined as undefined | ((raw: unknown) => void),
  sent: [] as { type: string; request?: number }[],
  dir: "",
}));

vi.mock("electron", () => ({
  app: {
    getPath: () => transport.dir,
    getAppPath: () => transport.dir,
    commandLine: { hasSwitch: () => true },
    whenReady: () => new Promise(() => undefined),
    on: () => undefined,
  },
  utilityProcess: { fork: () => ({
    on: (event: string, listener: (raw: unknown) => void) => {
      if (event === "message") transport.listener = listener;
    },
    postMessage: (message: { type: string; request?: number }) => transport.sent.push(message),
  }) },
  BrowserWindow: class {}, Menu: {}, MessageChannelMain: class {},
  session: {}, shell: {}, ipcMain: {},
}));

const ready = {
  type: "ready", info: { name: "core", version: "0.0.0" }, packs: [],
  languages: [{ code: "en", name: "English", keys: 1 }],
};

async function setup() {
  const { n2Probe } = await import("./main.js");
  n2Probe.startCore();
  transport.listener!(ready);
  expect(globalThis.openairsideCoreStatus.state).toBe("ready");
  return n2Probe;
}

beforeEach(() => {
  vi.resetModules();
  vi.useFakeTimers();
  transport.dir = mkdtempSync(path.join(os.tmpdir(), "oa-n2-probe-data-"));
  transport.sent = [];
  transport.listener = undefined;
});
afterEach(() => { vi.clearAllTimers(); vi.useRealTimers(); });

it("control: valid saves answers settle and a following request works", async () => {
  const main = await setup();
  for (const word of ["first", "second"]) {
    const value = [["ui.title", word]];
    const pending = main.requestSaves((request) => ({ type: "lang:texts", request, code: "en" }));
    const request = transport.sent.at(-1)!.request;
    transport.listener!({ type: "savesAnswer", request, ok: true, value });
    expect(await pending).toEqual(value);
  }
});

it("P1: unknown message must preserve ready and allow next request", async () => {
  const main = await setup();
  transport.listener!({ type: "unknown" });
  console.log("P1 observed status", globalThis.openairsideCoreStatus);
  const state = globalThis.openairsideCoreStatus.state;
  let code: unknown;
  try { main.requestSaves((request) => ({ type: "lang:texts", request, code: "en" })); }
  catch (error) { code = (error as { reason: { code: string } }).reason?.code; }
  console.log("P1 following call", code);
  expect(state).toBe("ready");
});

it("P2: malformed late pack answer must not settle a newer pack request", async () => {
  const main = await setup();
  const first = main.packs.setActive([]).catch((error: unknown) => error);
  await vi.waitFor(() => expect(transport.sent.filter(m => m.type === "packs:setActive")).toHaveLength(1));
  const old = main.packs.pendingRequest()!;
  await vi.advanceTimersByTimeAsync(60_000);
  await first;
  let secondOutcome = "pending";
  const second = main.packs.setActive([]).then(() => { secondOutcome = "resolved"; }, (error: { reason: { code: string } }) => { secondOutcome = error.reason.code; });
  await vi.waitFor(() => expect(transport.sent.filter(m => m.type === "packs:setActive")).toHaveLength(2));
  const current = main.packs.pendingRequest()!;
  expect(current).not.toBe(old);
  transport.listener!({ type: "packs", request: old, packs: "bad", languages: [] });
  await vi.advanceTimersByTimeAsync(0);
  const observed = secondOutcome;
  console.log("P2 old/current/outcome", old, current, observed);
  transport.listener!({ type: "packs", request: current, packs: [], languages: [] });
  await second;
  expect(observed).toBe("pending");
});

it("P3: malformed correlated save answer must reject immediately", async () => {
  const main = await setup();
  let outcome = "pending";
  const pending = main.saves.newGame({ map: 1536 }).then(
    () => { outcome = "resolved"; },
    (error: { reason: { code: string } }) => { outcome = error.reason.code; },
  );
  const request = transport.sent.at(-1)!.request;
  expect(main.lock.busy).toBe(true);
  transport.listener!({ type: "savesAnswer", request, ok: "bad" });
  await vi.advanceTimersByTimeAsync(0);
  const immediate = outcome;
  const busy = main.lock.busy;
  await vi.advanceTimersByTimeAsync(120_000);
  await pending;
  console.log("P3 immediate/busy/final", immediate, busy, outcome);
  expect(main.lock.busy).toBe(false);
  expect(immediate).toBe("invalidAnswer");
});

it("control: valid late pack reply is ignored, matching reply completes follow-up", async () => {
  const main = await setup();
  const first = main.packs.setActive([]).catch((error: unknown) => error);
  await vi.waitFor(() => expect(transport.sent.filter(m => m.type === "packs:setActive")).toHaveLength(1));
  const old = main.packs.pendingRequest()!;
  await vi.advanceTimersByTimeAsync(60_000);
  await first;
  let outcome = "pending";
  const second = main.packs.setActive([]).then(() => { outcome = "resolved"; });
  await vi.waitFor(() => expect(transport.sent.filter(m => m.type === "packs:setActive")).toHaveLength(2));
  const current = main.packs.pendingRequest()!;
  transport.listener!({ type: "packs", request: old, packs: [], languages: [] });
  await vi.advanceTimersByTimeAsync(0);
  expect(outcome).toBe("pending");
  transport.listener!({ type: "packs", request: current, packs: [], languages: [] });
  await second;
  expect(outcome).toBe("resolved");
  expect(main.lock.busy).toBe(false);
});

it("control: malformed matching pack reply rejects and unlocks follow-up", async () => {
  const main = await setup();
  let outcome = "pending";
  const pending = main.packs.setActive([]).catch((error: { reason: { code: string } }) => { outcome = error.reason.code; });
  await vi.waitFor(() => expect(transport.sent.filter(m => m.type === "packs:setActive")).toHaveLength(1));
  const request = main.packs.pendingRequest()!;
  transport.listener!({ type: "packs", request, packs: null, languages: [] });
  await pending;
  expect(outcome).toBe("invalidAnswer");
  expect(main.lock.busy).toBe(false);
});

it("P4 control: 4096 Unicode scalar values accepted and prototype remains null", () => {
  expect(parseLanguageList([{ code: "en", name: "😀".repeat(4096), keys: 1 }])).toBeDefined();
  const texts = parseTexts([["__proto__", "😀".repeat(4096)]]);
  expect(texts).toBeDefined();
  expect(Object.getPrototypeOf(texts!)).toBeNull();
});

it.each(["", "x".repeat(4097)])("P4: language name outside 1..4096 rejected (%#)", (name) => {
  expect(parseLanguageList([{ code: "en", name, keys: 1 }])).toBeUndefined();
});

it("P4: text longer than 4096 scalar values rejected", () => {
  expect(parseTexts([["ui.name", "x".repeat(4097)]])).toBeUndefined();
});
```

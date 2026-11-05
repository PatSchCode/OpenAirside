# Q.10-N5 – Gegenproben und Entscheidungsvorlage

**Stand:** 2026-11-05, Basis `d1f4ba2` (Main `d8a4b6f` plus N2-Abschlussvermerk). Vorbereitung, keine Produktkorrektur und kein neuer ADR beschlossen. Frist 2026-11-12 vor Abschluss 3.2/Erkundung 3.11.

## Befund und Abgrenzung

Mittlerer Befund, vermutlich Code falsch, `app/src/main/core-host.ts:162–174`: Nach erfolglosen Anfrageparsern genügt ein angehängter Port, um den bisherigen zu schließen, `core.openPort()` aufzurufen und den neuen anzubinden. Die Nachrichtenart wird nicht geprüft. S-8 verlangt das Verwerfen unbekannter/ungültiger Nachrichten und benennt `port` mit einem MessagePort als Übergabe. Landeplatz Q.10-N5; Korrektur sicherheitsfreigabepflichtig, keine neue Beförderung.

Einziger gefundener produktiver Übergabeaufruf: `app/src/main/main.ts:224` sendet `{ type: "port" }` mit genau einem Port. Sender ist der Hauptprozess, beide Seiten aus demselben Build. Kein nachgewiesener Weg für Renderer oder Daten-Mods, beliebige Parent-Port-Nachrichten zu senden; keine Remote-Angriffsaussage.

Zusätzliche statische Wirkung: `bridge/src/lib.rs:353–367` startet bei erneutem `open_port` eine neue Sitzung. Die Probe ersetzt die native Brücke; sie belegt keinen tatsächlichen Sitzungsneustart, Weltverlust oder dessen Folgen. Das Risiko beschränkt sich daher nicht zwingend auf einen Portwechsel, bleibt aber bezüglich nativer Folgen ungeprüft.

## Testsignal, Ergebnis und Grenzen

Rot erkennt, dass eine unbekannte/ungültige Nachricht den lebenden Port schließt. Grün der Kontrollen belegt den gültigen Austausch, das Beibehalten bei fehlendem Port und die vorhandene `invalidArgument`-Antwort auf eine lesbare ungültige Spielanfrage. Kein Erfolg einer Korrektur wird behauptet.

Node 26.11.1 aus vorhandener fnm-Installation aktiviert. `npx vitest run app/src/main/n5-probe.test.ts`: Exit 1, **drei Kontrollen bestanden, fünf erwartete Befundassertionen fehlgeschlagen**. Nach reiner Formatierung identisch wiederholt, keine ungeklärte Rot-Grün-Wiederholung. Keine drei Reparaturversuche; Produktdateien unverändert.

| Fall | Erwartung | Beobachtet |
|---|---|---|
| Gültiges `port`, genau ein neuer Port | alter Port einmal geschlossen, neuer einmal registriert/gestartet, Pump aktiv | grün |
| `port` ohne Port | bestehende Verbindung erhalten | grün |
| `game:new`, lesbare ID 7, ungültige Karte 1, Port angehängt | `invalidArgument`, kein Austausch | grün |
| unbekannte Art mit Port | kein Austausch | rot: alter Port einmal geschlossen |
| `null` mit Port | kein Austausch | rot: alter Port einmal geschlossen |
| ungültiges `autosave:interval` mit Port | kein Austausch | rot: alter Port einmal geschlossen |
| ungültiges `packs:setActive` mit Port | kein Austausch | rot: alter Port einmal geschlossen |
| `game:new` mit String-ID mit Port | kein Austausch | rot: alter Port einmal geschlossen |

Tatsächlicher importierter Host-Einstieg registriert seinen Listener; Parser und Handler sind real. Nur `process.parentPort`, MessagePorts, native Brücke und Uhr sind Test-Doubles. Keine echten Electron-Ports/Structured Clone, keine native Simulation, keine Paketierung oder Plattformmatrix geprüft. Kein Produkt-Testexport, keine neue Abhängigkeit oder Ausnahme. Diagnosequelle wird eingefroren und aus der normalen Testsuite entfernt, damit absichtlich rote Befundassertionen nicht als Akzeptanztests laufen.

## Optionen und vorgeschlagene Abnahme

**A – begrenzt korrigieren (Empfehlung):** Vor dem bestehenden Austausch ausschließlich eine explizite `port`-Nachricht mit genau einem gelieferten Port zulassen. Andere/ungültige Nachrichten sowie fehlende/mehrere Ports dürfen den laufenden Port nicht ersetzen oder `openPort` aufrufen. Vorhandene Anfrageparser und `invalidArgument`-Antworten bleiben erhalten. Harmlose Zusatzfelder bleiben zulässig, keine Exact-Key-Whitelist. Keine neue Nachrichtenversion oder Modulbeziehung; konkrete Formprüfung vor Umsetzung im ADR festhalten. Keine beiläufige Lebenszyklus-/Cleanup-Politik für abgelehnte angehängte Ports.

Abnahme nach Freigabe: gültige erste/erneute Übergabe einschließlich harmloser Zusatzfelder mit genau einmaliger Registrierung, altes Port-Schließen und Weiterleitung/Pump auf neuem Port; unbekannt/null/falsche Typen/ungültige bekannte Familien mit Ports ohne Austausch; null Ports (leere Liste), ein Port und mehrere Ports; lesbare ungültige Saves-ID weiterhin `invalidArgument`; gültige Anfragefamilien einschließlich angehängtem Port unverändert. Pump/Weiterleitung mit unterscheidbarer Antwort am jeweils richtigen Port prüfen, nicht nur den Aufruf zählen. Reale registrierte Host-Handler statt ausschließlich Parser testen. Dazu bestehende Gates, Electron-E2E, unabhängige Nachprüfung und CI. Diese Liste ist ein geplanter Nachweis, kein bereits bestandenes Ergebnis.

**B – bewusst verschieben:** Bestehendes Verhalten bis zu einem ausdrücklich terminierten Korrekturschritt vor 3.2-Abschluss akzeptieren. Keine Implementierung jetzt; mittlerer Befund und Beförderungssperre bleiben sichtbar. Spart die sofortige Korrektur, lässt die bestätigte Vertragsverletzung bestehen und verzögert den Abschluss.

Abhängigkeiten: belastbarer S-8/ADR-032 und vorhandene Senderform; N2-Freigabe P1–P4 umfasst N5 nicht. Risiko A: versehentliche Abweisung gültiger Übergaben oder Änderung anderer Anfragewege; die Kontrollen und Handlerregressionen prüfen dies. Umkehrbarkeit billig, wenige lokale Empfangszeilen und Tests. Sicherheitswirkung verlangt ausdrückliche Freigabe und ADR vor Produktcode. Keine Änderung früherer ADR-Klassifikationen; neue Klassifikation erst mit der konkreten Entscheidung begründen. Bestehende Reaktiv-Quote 3/10 bzw. 20/52 bleibt unverändert.

## Unabhängige Bewertung

Neue Instanz `gpt-6-astra`, hohe Denktiefe, ohne Umsetzungsgesprächsverlauf, ausschließlich lesend auf Basis `d1f4ba2`, Projektquellen und Diagnoseprotokoll: mittleren Codebefund bestätigt, begrenztes A empfohlen, Konfidenz hoch, Umkehrbarkeit billig. Keine eigenen Tests oder Remote-/PR-Prüfung durch den Prüfer. Exakte Hauptmodellklasse unbestimmbar; Astra-Modellwahl ist aus der Werkzeugdeklaration belegt.

Prüfhinweise übernommen: keine Exact-Key-Whitelist, Kardinalität ausdrücklich entscheiden, bestehende Anfragewege erhalten und Pump-Antwort am richtigen Port nachweisen. Die fünf roten Fälle brechen jeweils bei `old.close` ab; nachfolgende Assertions sind dort nicht ausgeführt. Weitere Wirkungen sind statisch abgeleitet. Für diese enge Fassung ist ein späterer OPERATIV/SECURITY/SCHNITTSTELLE-ADR begründbar: bestehender Vertrag, keine Stabilisierung und keine Änderung von ADR-052 (N5 dort ausgeklammert). Noch kein ADR und keine neue Klassifikation beschlossen. Keine Architektur-Code-Gesamtprüfung oder Reifegradbeförderung.

## Reproduktion und eingefrorene Quelle

Datei temporär als `app/src/main/n5-probe.test.ts` wiederherstellen, im Repository mit Node 26.11.1 und installierten Abhängigkeiten den obigen Vitest-Befehl ausführen und anschließend nur diese Diagnosekopie entfernen. Exit 1 mit fünf Befundassertionen ist auf der Basis erwartet; nicht als fehlgeschlagene Reparatur werten. Keine Installationsänderung erforderlich. SHA-256 der UTF-8-Quelle inklusive abschließendem LF: `3075595fd28ad71c5bf712985d064e5b3fa88171fb75c98756e136cdfdfd4f0b`.

```typescript
/** Q.10-N5 isolated diagnostic, not an acceptance test or product change.
 * Prerequisites: repository dependencies, Node 26.11.1; all Node platforms.
 * Run with Vitest. Exit 0: assertions hold; nonzero: assertion/tool failure.
 * Fake timers and module reset make cases repeatable; no native bridge loaded.
 */
import { afterEach, beforeEach, expect, it, vi } from "vitest";

const fixture = vi.hoisted(() => ({
  openPort: vi.fn(),
  receive: vi.fn(() => []),
  pump: vi.fn(() => []),
}));
vi.mock("node:module", () => ({
  createRequire: () => () => ({
    Core: class {
      openPort = fixture.openPort;
      receive = fixture.receive;
      pump = fixture.pump;
      packs = () => [];
      languages = () => [];
      status = () => ({ session: 1, running: false });
    },
    coreInfo: () => ({ name: "test", version: "0" }),
  }),
}));
let handler: (event: { data: unknown; ports: ReturnType<typeof port>[] }) => void;
const post = vi.fn();
const original = Object.getOwnPropertyDescriptor(process, "parentPort");
function port() {
  return { close: vi.fn(), on: vi.fn(), start: vi.fn(), postMessage: vi.fn() };
}
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

it("control: valid handoff replaces old port once", () => {
  const old = port();
  const next = port();
  handler({ data: { type: "port" }, ports: [old] });
  handler({ data: { type: "port" }, ports: [next] });
  expect(old.close).toHaveBeenCalledTimes(1);
  expect(next.on).toHaveBeenCalledTimes(1);
  expect(next.start).toHaveBeenCalledTimes(1);
  expect(fixture.openPort).toHaveBeenCalledTimes(2);
  vi.advanceTimersByTime(50);
  expect(fixture.pump).toHaveBeenCalledTimes(1);
});
it("control: missing port preserves connection", () => {
  const old = port();
  handler({ data: { type: "port" }, ports: [old] });
  handler({ data: { type: "port" }, ports: [] });
  expect(old.close).not.toHaveBeenCalled();
  expect(fixture.openPort).toHaveBeenCalledTimes(1);
});
it("control: readable invalid save gets invalidArgument without replacement", () => {
  const old = port();
  const next = port();
  handler({ data: { type: "port" }, ports: [old] });
  handler({ data: { type: "game:new", request: 7, map: 1 }, ports: [next] });
  expect(old.close).not.toHaveBeenCalled();
  expect(next.start).not.toHaveBeenCalled();
  expect(post).toHaveBeenCalledWith(
    expect.objectContaining({
      type: "savesAnswer",
      request: 7,
      ok: false,
      error: expect.objectContaining({ code: "invalidArgument" }),
    }),
  );
});
it.each([
  ["unknown", { type: "unknown" }],
  ["null", null],
  ["invalid interval", { type: "autosave:interval", minutes: -1 }],
  ["invalid pack request", { type: "packs:setActive", request: 1, ids: null }],
  ["unreadable save id", { type: "game:new", request: "7", map: 1536 }],
])("safety: %s with attached port preserves live connection", (_label, data) => {
  const old = port();
  const next = port();
  handler({ data: { type: "port" }, ports: [old] });
  handler({ data, ports: [next] });
  expect(old.close).not.toHaveBeenCalled();
  expect(next.start).not.toHaveBeenCalled();
  expect(fixture.openPort).toHaveBeenCalledTimes(1);
});
```

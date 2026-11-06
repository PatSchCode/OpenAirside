import { describe, expect, it, vi } from "vitest";
import type { TerminalStationPoint } from "../src/generated/TerminalStationPoint.js";
import { BUNDLED, Localizer } from "./i18n.js";
import { TerminalLesson, TerminalPanel } from "./terminal-panel.js";

const stations = (): TerminalStationPoint[] =>
  ["checkIn", "security", "gate", "exit"].map((kind, i) => ({
    kind: kind as TerminalStationPoint["kind"],
    target: 20 + i,
    queued: 0,
    backlog: 0,
    serving: 0,
    leaving: 0,
    completed: 0,
    serviceCell: { x: 4 + i * 5, y: 6, floor: 0 },
    maxWaitingTicks: 0,
    blocked: false,
    served: 0,
    waitingTicks: 0,
    capacity: 3,
    serviceTicks: kind === "security" ? 30 : 10,
  }));

function point(points: readonly TerminalStationPoint[], index: number): TerminalStationPoint {
  const result = points[index];
  if (result === undefined) throw new Error("missing fixture station");
  return result;
}

/** Small DOM double; actual browser events/layout are checked by terminal.spec.ts. */
class Element {
  textContent = "";
  hidden = true;
  disabled = false;
  type = "";
  children: Element[] = [];
  click: () => void = vi.fn();
  attributes = new Map<string, string>();
  constructor(readonly tag: string) {}
  append(...elements: Element[]): void {
    this.children.push(...elements);
  }
  replaceChildren(...elements: Element[]): void {
    this.children = elements;
  }
  setAttribute(name: string, value: string): void {
    this.attributes.set(name, value);
  }
  addEventListener(_name: string, run: () => void): void {
    this.click = run;
  }
  get text(): string {
    return this.textContent + this.children.map((child) => child.text).join(" ");
  }
  button(label: string): Element {
    const found = this.children
      .flatMap((child) => [child, ...child.children])
      .find((child) => child.tag === "button" && child.text === label);
    if (found === undefined) throw new Error(`missing button ${label}`);
    return found;
  }
}

function setup() {
  let id = 10;
  const send = vi.fn(() => ++id);
  const focus = vi.fn();
  const changed = vi.fn();
  const lesson = new TerminalLesson(send, focus, changed);
  lesson.reset(1);
  const start = () => {
    lesson.start();
    lesson.accepted(1, 11);
    lesson.reset(2);
    lesson.frame(2, 80);
    lesson.catalogReady(2);
    const points = stations();
    lesson.snapshot(2, points);
    return points;
  };
  return { lesson, send, focus, changed, start };
}

describe("terminal lesson (3.2)", () => {
  it("renders real waiting counts, refusals and language changes, and wires visible controls", () => {
    const doc = { createElement: (tag: string) => new Element(tag) } as unknown as Document;
    const box = new Element("aside");
    const i18n = new Localizer("de");
    let id = 0;
    const send = vi.fn(() => ++id);
    const panel = new TerminalPanel(doc, box as unknown as HTMLElement, i18n, send, vi.fn());
    panel.lesson.reset(1);
    expect(box.hidden).toBe(false);
    box.button("Erste Terminallektion starten").click();
    expect(box.text).toContain("Anfrage läuft");
    panel.lesson.accepted(1, 1);
    panel.lesson.reset(2);
    panel.lesson.frame(2, 80);
    panel.lesson.catalogReady(2);
    const points = stations();
    point(points, 1).backlog = 8;
    point(points, 1).queued = 2;
    point(points, 1).maxWaitingTicks = 30;
    point(points, 3).completed = 40;
    panel.lesson.snapshot(2, points);
    expect(box.text).toContain("10 wartend (davon 8 vor der Linie)");
    expect(box.text).toContain("längste Wartezeit 10 s");
    box.button("Start / Fortsetzen").click();
    expect(send).toHaveBeenLastCalledWith({ kind: "setSpeed", speed: "x1" });
    panel.lesson.accepted(2, 2);
    panel.lesson.speedChanged(2, "x1");
    box.button("Pause").click();
    panel.lesson.accepted(2, 3);
    box.button("Zeitraffer (3×)").click();
    panel.lesson.accepted(2, 4);
    box.button("Kontrolle schneller bedienen").click();
    expect(box.button("Kontrolle schneller bedienen").disabled).toBe(true);
    panel.lesson.rejected(2, 5, "command.rejected.invalidArgument");
    expect(box.text).toContain("ungültigen Wert");
    i18n.use("en", BUNDLED.get("en") ?? new Map());
    expect(box.text).toContain("invalid value");
    box.button("Speed up security service").click();
    panel.lesson.accepted(2, 6);
    expect(box.text).toContain("Change confirmed");
    point(points, 1).backlog = 0;
    point(points, 1).queued = 0;
    point(points, 2).completed = 40;
    panel.lesson.snapshot(2, points);
    expect(box.text).toContain("Lesson complete");
    box.button("Restart lesson").click();
    expect(send).toHaveBeenLastCalledWith({
      kind: "debug",
      action: { kind: "loadScenario", scenario: "smallTerminal", persons: 80 },
    });
  });
  it("focuses only after accepted load, new session, catalog, frame and complete station snapshot", () => {
    const { lesson: l, send, focus } = setup();
    l.start();
    l.start();
    expect(send).toHaveBeenCalledTimes(1);
    expect(send).toHaveBeenCalledWith({
      kind: "debug",
      action: { kind: "loadScenario", scenario: "smallTerminal", persons: 80 },
    });
    l.frame(1, 80);
    l.catalogReady(1);
    l.snapshot(1, stations());
    expect(l.ready).toBe(false);
    l.accepted(1, 11);
    l.reset(2);
    l.frame(2, 80);
    l.snapshot(2, stations());
    expect(focus).not.toHaveBeenCalled();
    l.catalogReady(1);
    expect(focus).not.toHaveBeenCalled();
    l.catalogReady(2);
    expect(l.ready).toBe(true);
    expect(focus).toHaveBeenCalledOnce();
    l.snapshot(2, stations());
    expect(focus).toHaveBeenCalledOnce();
  });

  it("rejects success by click and stale acknowledgments; sends the actual security target once", () => {
    const { lesson: l, send, start } = setup();
    const points = start();
    l.upgrade();
    expect(send).toHaveBeenCalledTimes(1);
    point(points, 3).completed = 40;
    point(points, 1).backlog = 8;
    l.snapshot(2, points);
    expect(l.observable).toBe(true);
    l.upgrade();
    l.upgrade();
    expect(send).toHaveBeenLastCalledWith({
      kind: "setStationServiceTicks",
      target: 21,
      serviceTicks: 10,
    });
    expect(l.upgraded).toBe(false);
    expect(l.complete).toBe(false);
    l.accepted(1, 12);
    expect(l.pending).toBe(12);
    l.rejected(2, 12, "command.rejected.invalidArgument");
    expect(l.error).toBe("command.rejected.invalidArgument");
    expect(l.upgraded).toBe(false);
    l.upgrade();
    l.accepted(2, 13);
    expect(l.upgraded).toBe(true);
    expect(l.complete).toBe(false);
    point(points, 1).backlog = 0;
    point(points, 2).completed = 40;
    l.snapshot(2, points);
    expect(l.complete).toBe(true);
    point(points, 0).blocked = true;
    l.snapshot(2, points);
    expect(l.complete).toBe(false);
    l.reset(3);
    l.snapshot(2, points);
    l.accepted(2, 13);
    expect(l.stations).toEqual([]);
    expect(l.upgraded).toBe(false);
    expect(l.ready).toBe(false);
  });

  it("does not acquire an unsolicited session, handles failed start and disconnected/speed commands", () => {
    const { lesson: l, send, start } = setup();
    l.start();
    l.rejected(1, 11, "command.rejected.notReady");
    expect(l.busy).toBe(false);
    expect(l.error).toContain("notReady");
    l.reset(2);
    l.frame(2, 80);
    l.catalogReady(2);
    l.snapshot(2, stations());
    expect(l.ready).toBe(false);
    l.setSpeed("x3");
    expect(send).toHaveBeenCalledTimes(1);
    const s = setup();
    start();
    s.start();
    s.lesson.setSpeed("x3");
    s.lesson.setSpeed("paused");
    expect(s.send).toHaveBeenCalledTimes(2);
    s.lesson.speedChanged(1, "x3");
    expect(s.lesson.speed).toBe("paused");
    s.lesson.speedChanged(2, "x3");
    s.lesson.accepted(2, 12);
    expect(s.lesson.speed).toBe("x3");
    s.send.mockReturnValue(0);
    s.lesson.setSpeed("paused");
    expect(s.lesson.error).toBe("ui.terminal.disconnected");
    const off = new TerminalLesson(() => 0, vi.fn(), vi.fn());
    off.start();
    expect(off.busy).toBe(false);
    expect(off.error).toBe("ui.terminal.disconnected");
  });

  it("has matching German/English lesson texts and translates numbers without raw keys", () => {
    const de = BUNDLED.get("de") ?? new Map<string, string>();
    const en = BUNDLED.get("en") ?? new Map<string, string>();
    const keys = [...en.keys()].filter((key) => key.startsWith("ui.terminal."));
    expect(keys.length).toBeGreaterThan(15);
    expect([...de.keys()].filter((key) => key.startsWith("ui.terminal.")).sort()).toEqual(
      keys.sort(),
    );
    for (const lang of ["de", "en"]) {
      const i18n = new Localizer(lang);
      for (const key of keys) expect(i18n.t(key)).not.toBe(key);
      expect(
        i18n.t("ui.terminal.station", {
          name: { key: "ui.terminal.security" },
          waiting: 8,
          backlog: 6,
          wait: 10,
          service: 10,
        }),
      ).not.toContain("{");
    }
  });

  it("abandons loading after a second session change or a reconnected port", () => {
    for (const reconnect of [false, true]) {
      const { lesson: l, focus } = setup();
      l.start();
      l.accepted(1, 11);
      l.reset(2);
      l.reset(3, !reconnect);
      l.frame(3, 80);
      l.catalogReady(3);
      l.snapshot(3, stations());
      expect(l.ready).toBe(false);
      expect(l.busy).toBe(false);
      expect(focus).not.toHaveBeenCalled();
    }
  });
});

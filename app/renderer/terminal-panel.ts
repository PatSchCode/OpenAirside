/** Visible first terminal lesson (3.2 / ADR-054). Rules and counts remain in the core. */
import type { Command } from "../src/generated/Command.js";
import type { Speed } from "../src/generated/Speed.js";
import type { TerminalStationPoint } from "../src/generated/TerminalStationPoint.js";
import type { Localizer } from "./i18n.js";

/** Session-bound presentation state, also used by the DOM view below. */
export class TerminalLesson {
  session = 0;
  stations: readonly TerminalStationPoint[] = [];
  ready = false;
  speed: Speed = "paused";
  pending = 0;
  error = "";
  upgraded = false;
  private loading:
    { id: number; source: number; accepted: boolean; destination: number } | undefined;
  private catalog = false;
  private total = 0;
  private focused = false;
  private action: "upgrade" | "speed" | undefined;

  constructor(
    private readonly send: (command: Command) => number,
    private readonly focus: (stations: readonly TerminalStationPoint[]) => void,
    readonly changed: () => void,
  ) {}

  reset(session: number, expectedTransition = true): void {
    const loading = this.loading;
    this.session = session;
    this.stations = [];
    this.catalog = false;
    this.total = 0;
    this.ready = false;
    this.focused = false;
    this.upgraded = false;
    this.speed = "paused";
    this.error = "";
    this.pending = 0;
    this.action = undefined;
    this.loading =
      expectedTransition &&
      loading?.accepted &&
      loading.destination === 0 &&
      session !== loading.source
        ? { ...loading, destination: session }
        : undefined;
    this.changed();
  }

  start(): void {
    if (this.pending !== 0 || this.loading !== undefined) return;
    const id = this.send({
      kind: "debug",
      action: { kind: "loadScenario", scenario: "smallTerminal", persons: 80 },
    });
    if (id === 0) {
      this.error = "ui.terminal.disconnected";
    } else {
      this.loading = { id, source: this.session, accepted: false, destination: 0 };
      this.pending = id;
      this.error = "";
    }
    this.changed();
  }

  accepted(session: number, id: number): void {
    if (this.loading?.id === id && this.loading.source === session) {
      this.loading.accepted = true;
      this.pending = 0;
    } else if (session === this.session && id === this.pending) {
      this.upgraded ||= this.action === "upgrade";
      this.pending = 0;
      this.action = undefined;
    } else return;
    this.changed();
  }

  rejected(session: number, id: number, reason: string): void {
    if (session !== this.session || id !== this.pending) return;
    this.pending = 0;
    this.action = undefined;
    this.loading = undefined;
    this.error = reason;
    this.changed();
  }

  frame(session: number, total: number): void {
    if (session !== this.session) return;
    this.total = total;
  }

  catalogReady(session: number): void {
    if (session !== this.session) return;
    this.catalog = true;
    this.checkReady();
  }

  snapshot(session: number, stations: readonly TerminalStationPoint[]): void {
    if (session !== this.session) return;
    this.stations = stations;
    this.checkReady();
    this.changed();
  }

  private checkReady(): void {
    const kinds = new Set(this.stations.map((s) => s.kind));
    if (
      this.loading?.destination !== this.session ||
      !this.catalog ||
      this.total !== 80 ||
      this.stations.length !== 4 ||
      kinds.size !== 4
    )
      return;
    this.ready = true;
    this.loading = undefined;
    if (!this.focused) {
      this.focused = true;
      this.focus(this.stations);
    }
  }

  get busy(): boolean {
    return this.pending !== 0 || this.loading !== undefined;
  }
  get security(): TerminalStationPoint | undefined {
    return this.stations.find((s) => s.kind === "security");
  }
  get arrivals(): number {
    return this.stations.find((s) => s.kind === "exit")?.completed ?? 0;
  }
  get departures(): number {
    return this.stations.find((s) => s.kind === "gate")?.completed ?? 0;
  }
  get observable(): boolean {
    const security = this.security;
    return (
      this.ready &&
      this.arrivals === 40 &&
      security !== undefined &&
      security.backlog + security.queued >= 8
    );
  }
  get complete(): boolean {
    return (
      this.ready &&
      this.total === 80 &&
      this.arrivals === 40 &&
      this.departures === 40 &&
      this.stations.every((s) => !s.blocked && s.backlog + s.queued + s.serving + s.leaving === 0)
    );
  }

  upgrade(): void {
    const station = this.security;
    if (!this.observable || this.busy || this.upgraded || station?.serviceTicks !== 30) return;
    this.command(
      { kind: "setStationServiceTicks", target: station.target, serviceTicks: 10 },
      "upgrade",
    );
  }

  setSpeed(speed: Speed): void {
    if (!this.ready || this.busy) return;
    this.command({ kind: "setSpeed", speed }, "speed");
  }

  speedChanged(session: number, speed: Speed): void {
    if (session !== this.session) return;
    this.speed = speed;
    this.changed();
  }

  private command(command: Command, action: "upgrade" | "speed"): void {
    this.pending = this.send(command);
    this.action = this.pending === 0 ? undefined : action;
    this.error = this.pending === 0 ? "ui.terminal.disconnected" : "";
    this.changed();
  }
}

/** Plain DOM like the existing build/settings panels until step 3.6. */
export class TerminalPanel {
  readonly lesson: TerminalLesson;

  constructor(
    private readonly doc: Document,
    private readonly container: HTMLElement,
    private readonly i18n: Localizer,
    send: (command: Command) => number,
    focus: (stations: readonly TerminalStationPoint[]) => void,
  ) {
    this.lesson = new TerminalLesson(send, focus, () => {
      this.render();
    });
    i18n.onChange(() => {
      this.render();
    });
    this.render();
  }

  render(): void {
    const l = this.lesson;
    const t = this.i18n.t.bind(this.i18n);
    const text = (tag: string, key: string): HTMLElement => {
      const element = this.doc.createElement(tag);
      element.textContent = t(key);
      return element;
    };
    const button = (key: string, run: () => void, disabled: boolean): HTMLButtonElement => {
      const element = this.doc.createElement("button");
      element.type = "button";
      element.textContent = t(key);
      element.disabled = disabled;
      element.addEventListener("click", run);
      return element;
    };
    const status = text(
      "p",
      l.error ||
        (l.busy
          ? "ui.terminal.pending"
          : l.complete
            ? "ui.terminal.complete"
            : l.upgraded
              ? "ui.terminal.upgraded"
              : l.observable
                ? "ui.terminal.observed"
                : l.ready
                  ? "ui.terminal.observe"
                  : "ui.terminal.intro"),
    );
    status.setAttribute("role", "status");
    const rows = this.doc.createElement("ul");
    for (const station of l.stations) {
      const row = this.doc.createElement("li");
      row.textContent =
        t("ui.terminal.station", {
          name: { key: `ui.terminal.${station.kind}` },
          waiting: station.backlog + station.queued,
          backlog: station.backlog,
          wait: station.maxWaitingTicks / 3,
          service: station.serviceTicks / 3,
        }) + (station.blocked ? ` ${t("ui.terminal.blocked")}` : "");
      rows.append(row);
    }
    const counts = this.doc.createElement("p");
    counts.textContent = t("ui.terminal.counts", {
      departures: l.departures,
      arrivals: l.arrivals,
      active: Math.max(0, 80 - l.departures - l.arrivals),
    });
    const controls = this.doc.createElement("div");
    controls.append(
      button(
        l.ready ? "ui.terminal.restart" : "ui.terminal.start",
        () => {
          l.start();
        },
        l.busy,
      ),
      button(
        l.speed === "paused" ? "ui.terminal.resume" : "ui.terminal.pause",
        () => {
          l.setSpeed(l.speed === "paused" ? "x1" : "paused");
        },
        !l.ready || l.busy,
      ),
      button(
        "ui.terminal.fast",
        () => {
          l.setSpeed("x3");
        },
        !l.ready || l.busy || l.speed === "x3",
      ),
      button(
        "ui.terminal.upgrade",
        () => {
          l.upgrade();
        },
        !l.observable || l.busy || l.upgraded || l.security?.serviceTicks !== 30,
      ),
    );
    this.container.hidden = false;
    this.container.replaceChildren(text("h2", "ui.terminal.title"), status, controls, rows);
    if (l.ready) this.container.append(counts);
  }
}

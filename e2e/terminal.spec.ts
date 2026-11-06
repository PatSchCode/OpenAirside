import { mkdtempSync } from "node:fs";
import os from "node:os";
import path from "node:path";
import { _electron as electron, expect, test } from "@playwright/test";

const packaged = process.env.OPENAIRSIDE_E2E_EXECUTABLE;
for (const language of ["de", "en"]) {
  test(`visible terminal lesson in ${language}`, async () => {
    test.setTimeout(120_000);
    const app = await electron.launch({
      ...(packaged === undefined ? {} : { executablePath: packaged }),
      args: [
        ...(packaged === undefined ? ["."] : []),
        ...(process.platform === "linux" ? ["--no-sandbox"] : []),
        `--user-data-dir=${mkdtempSync(path.join(os.tmpdir(), "openairside-e2e-lesson-"))}`,
      ],
    });
    try {
      const window = await app.firstWindow();
      await expect(window.locator("#root")).toHaveAttribute("data-ready", "true");
      // Change language using the real settings form; no console or private game API.
      await window.locator("#settings-button").click();
      await window.locator('#settings select[name="language"]').selectOption(language);
      await window.locator('#settings button[type="submit"]').click();
      await expect(window.locator("html")).toHaveAttribute("lang", language);
      await window.locator('#settings button[type="button"]').click();
      const terminal = window.locator("#terminal");
      await expect(terminal).toBeVisible();
      const de = language === "de";
      await terminal
        .getByRole("button", {
          name: de ? "Erste Terminallektion starten" : "Start first terminal lesson",
          exact: true,
        })
        .click();
      const resume = terminal.getByRole("button", {
        name: de ? "Start / Fortsetzen" : "Start / Resume",
        exact: true,
      });
      await expect(resume).toBeEnabled();
      await expect(terminal).toContainText(de ? "Aktive Reisen: 80" : "Active journeys: 80");
      await resume.click();
      const pause = terminal.getByRole("button", { name: "Pause", exact: true });
      await expect(pause).toBeEnabled();
      await pause.click();
      await expect(resume).toBeEnabled();
      await terminal
        .getByRole("button", { name: de ? "Zeitraffer (3×)" : "Fast forward (3×)", exact: true })
        .click();
      const upgrade = terminal.getByRole("button", {
        name: de ? "Kontrolle schneller bedienen" : "Speed up security service",
        exact: true,
      });
      await expect(upgrade).toBeEnabled({ timeout: 60_000 });
      await expect(terminal).toContainText(
        de ? "mindestens acht Personen" : "at least eight people",
      );
      // Inject failures at the existing port boundary; there is no product test API.
      await window.evaluate(() => {
        const original = Reflect.get(MessagePort.prototype, "postMessage") as (
          this: MessagePort,
          message: unknown,
          options?: StructuredSerializeOptions,
        ) => void;
        const wrapped = new WeakSet<MessagePort>();
        let rejectOnce = true;
        MessagePort.prototype.postMessage = function (
          message: unknown,
          transfer: Transferable[] | StructuredSerializeOptions = [],
        ): void {
          const data = message as {
            type?: string;
            command?: { kind?: string };
            query?: { kind?: string };
          };
          if (
            !wrapped.has(this) &&
            data.type === "query" &&
            data.query?.kind === "terminalStations"
          ) {
            wrapped.add(this);
            const handler = this.onmessage;
            this.onmessage = (event: MessageEvent<unknown>) => {
              const answer = event.data as {
                type?: string;
                session?: number;
                id?: number;
                terminalStations?: Record<string, unknown>[];
              };
              if (
                answer.type === "queryResult" &&
                answer.terminalStations !== undefined &&
                answer.session !== undefined &&
                answer.id !== undefined
              ) {
                const fake = answer.terminalStations.map((s) => ({
                  ...s,
                  backlog: 0,
                  queued: 0,
                  serving: 0,
                  leaving: 0,
                  blocked: false,
                  completed: s.kind === "gate" || s.kind === "exit" ? 40 : 0,
                }));
                handler?.call(
                  this,
                  new MessageEvent("message", {
                    data: { ...answer, session: answer.session - 1, terminalStations: fake },
                  }),
                );
                handler?.call(
                  this,
                  new MessageEvent("message", {
                    data: { ...answer, id: answer.id + 100_000, terminalStations: fake },
                  }),
                );
                const root = document.getElementById("root");
                const status =
                  document.querySelector('#terminal [role="status"]')?.textContent ?? "";
                if (root !== null) {
                  root.dataset.injectedTerminalAnswers = String(
                    Number(root.dataset.injectedTerminalAnswers ?? 0) + 2,
                  );
                  if (status.includes("Lesson complete") || status.includes("Lektion geschafft"))
                    root.dataset.falseTerminalCompletion = "true";
                }
              }
              handler?.call(this, event);
            };
          }
          if (
            rejectOnce &&
            data.type === "command" &&
            data.command?.kind === "setStationServiceTicks"
          ) {
            rejectOnce = false;
            original.call(
              this,
              { ...data, command: { ...data.command, serviceTicks: 0 } },
              Array.isArray(transfer) ? { transfer } : transfer,
            );
          } else original.call(this, message, Array.isArray(transfer) ? { transfer } : transfer);
        };
      });
      await upgrade.click();
      await expect(terminal).toContainText(
        de ? "Befehl enthält einen ungültigen Wert" : "command has an invalid value",
      );
      await expect(terminal).not.toContainText(de ? "Lektion geschafft" : "Lesson complete");
      await expect(upgrade).toBeEnabled();
      await upgrade.click();
      await expect(upgrade).toBeDisabled();
      await expect(terminal).toContainText(de ? "Änderung bestätigt" : "Change confirmed");
      await expect
        .poll(async () =>
          Number(await window.locator("#root").getAttribute("data-injected-terminal-answers")),
        )
        .toBeGreaterThan(0);
      await expect(window.locator("#root")).not.toHaveAttribute(
        "data-false-terminal-completion",
        "true",
      );
      await expect(terminal).toContainText(de ? "Lektion geschafft" : "Lesson complete", {
        timeout: 60_000,
      });
      await expect(terminal).toContainText(de ? "Aktive Reisen: 0" : "Active journeys: 0");
      await terminal
        .getByRole("button", { name: de ? "Lektion neu starten" : "Restart lesson", exact: true })
        .click();
      await expect(resume).toBeEnabled();
      await expect(terminal).toContainText(de ? "Aktive Reisen: 80" : "Active journeys: 80");
      await expect(terminal).not.toContainText(de ? "Lektion geschafft" : "Lesson complete");
      await expect(window.locator("#log")).not.toContainText("dropped world section");
      if (!de) {
        // Feed legal S-1 drawing records to the real Pixi view. Compare rendered pixels,
        // including reuse of previously transparent pool entries. No renderer internals.
        await window.evaluate(() => {
          let mode = 0;
          globalThis.addEventListener("terminal-render-fixture", (event) => {
            mode = (event as CustomEvent<number>).detail;
          });
          const original = Reflect.get(MessagePort.prototype, "postMessage") as (
            this: MessagePort,
            message: unknown,
            options?: StructuredSerializeOptions,
          ) => void;
          let attached = false;
          MessagePort.prototype.postMessage = function (
            message: unknown,
            transfer: Transferable[] | StructuredSerializeOptions = [],
          ): void {
            if (!attached && (message as { type?: string }).type === "frameRequest") {
              attached = true;
              const handler = this.onmessage;
              this.onmessage = (event: MessageEvent<unknown>) => {
                const data = event.data as { type?: string; buf?: ArrayBuffer };
                if (data.type !== "frame" || data.buf === undefined) {
                  handler?.call(this, event);
                  return;
                }
                const old = new DataView(data.buf);
                const worldAt = 48 + old.getUint32(20, true) * 8;
                const buf = new ArrayBuffer(48 + 6 * 8 + data.buf.byteLength - worldAt);
                const bytes = new Uint8Array(buf);
                bytes.set(new Uint8Array(data.buf, 0, 48));
                bytes.set(new Uint8Array(data.buf, worldAt), 48 + 6 * 8);
                const view = new DataView(buf);
                view.setUint32(20, 6, true);
                for (let i = 0; i < 6; i += 1) {
                  const at = 48 + i * 8;
                  view.setUint16(at, (old.getUint16(12, true) / 2 - 6.5 + i * 2) * 32, true);
                  view.setUint16(at + 2, (old.getUint16(14, true) / 2 + 3.5) * 32, true);
                  view.setInt8(at + 4, 0);
                  view.setUint8(at + 5, 0);
                  view.setUint8(at + 6, mode === 0 ? 1 : ([1, 1, 2, 2, 0, 9][i] ?? 0));
                  view.setUint8(
                    at + 7,
                    mode === 2 ? 0 : mode === 1 && (i === 0 || i === 2) ? 0 : 2,
                  );
                }
                handler?.call(this, new MessageEvent("message", { data: { ...data, buf } }));
                document.getElementById("root")?.setAttribute("data-render-fixture", String(mode));
              };
            }
            original.call(this, message, Array.isArray(transfer) ? { transfer } : transfer);
          };
        });
        const canvas = window.locator("#root canvas");
        const bounds = await canvas.boundingBox();
        if (bounds === null) throw new Error("missing playfield");
        const images = async (): Promise<Buffer[]> => {
          const out: Buffer[] = [];
          for (let i = 0; i < 6; i += 1)
            out.push(
              await window.screenshot({
                clip: {
                  x: Math.floor(bounds.x + bounds.width / 2 + (-8.5 + i * 2) * 22) - 1,
                  y: Math.floor(bounds.y + bounds.height / 2 + 88) - 1,
                  width: 3,
                  height: 3,
                },
              }),
            );
          return out;
        };
        await expect(window.locator("#root")).toHaveAttribute("data-render-fixture", "0");
        const empty = await images();
        await window.evaluate(() =>
          globalThis.dispatchEvent(new CustomEvent("terminal-render-fixture", { detail: 1 })),
        );
        await expect(window.locator("#root")).toHaveAttribute("data-render-fixture", "1");
        const mixed = await images();
        for (let i = 0; i < 6; i += 1) {
          expect(mixed[i]?.equals(empty[i] ?? Buffer.alloc(0))).toBe(i === 1 || i === 3);
        }
        await window.evaluate(() =>
          globalThis.dispatchEvent(new CustomEvent("terminal-render-fixture", { detail: 2 })),
        );
        await expect(window.locator("#root")).toHaveAttribute("data-render-fixture", "2");
        const reused = await images();
        for (let i = 0; i < 6; i += 1)
          expect(reused[i]?.equals(empty[i] ?? Buffer.alloc(0))).toBe(false);
      }
    } finally {
      await app.close();
    }
  });
}

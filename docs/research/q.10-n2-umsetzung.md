# Q.10-N2 – Umsetzung des freigegebenen Pakets A

**Stand:** 2026-11-05, ADR-052 vor Code; lokale Nachweise, unabhängige Nachprüfung und vollständiger Hook bestanden, Commit/Push/CI noch abzuschließen. N5 bleibt getrennt; keine Reifegradbeförderung oder Abschlussbehauptung für 3.2.

## Ergebnis

Unbekannte/ungültige Nachrichten verlassen den tatsächlichen Hauptprozess-Handler vor Änderungen an Status, Berichten oder Portverbindung. Gültiges `ready`, `error`, Autosave und tatsächlicher Exit behalten ihre Wirkung. `nextCoreStatus` erhält bei unpassender Nachricht den vorherigen Status.

Fehlerhafte Antworten werden nur anhand erkannter Familie und tatsächlich gelieferter sicherer ganzer Zahl ≥ 0 zugeordnet. Paketdienst und Saves-Pending-Map bleiben getrennt, auch bei gleichen Nummern. Passende Fehler beenden die Anfrage sofort als `invalidAnswer` über die vorhandenen Abschlusswege; Timer und Locks werden bereinigt. Alte oder unlesbare IDs ersetzen keine aktuelle Anfrage.

Sprachnamen und Texte werden im Hauptprozess auf höchstens 4.096 Unicode-Codepoints begrenzt. Namen benötigen ein Zeichen außerhalb Unicode White_Space; die von Rust abweichende JavaScript-trim-Semantik wird vermieden. U+0085 wird als alleiniger Name verworfen, U+FEFF wie im Rust-Lader erhalten. Keine Normalisierung, neue Gesamtgrenze oder zusätzliche Leertextregel; Wörterbuch ohne Prototyp unverändert.

Produktänderungen ausschließlich in `main.ts`, `core-status.ts` und `settings.ts`; zugehörige Tests. Kein Kern-/Brückencode, N5-Fix, Nachrichtenformat, neues Modul, externe Abhängigkeit oder Build-/CI-Eingriff.

## Funktions- und Integrationsnachweise

Node **26.11.1** explizit über vorhandene fnm-Installation aktiviert. `npm test` (Vitest mit Coverage): **509 Tests in 41 Dateien bestanden**, Exit 0. Davon 17 Fälle in `main-messages.test.ts` gegen den tatsächlichen importierten Main-Einstieg mit dessen registriertem Message-Listener und S-6-IPC-Handlern. Keine Testzugriffe im Produktcode.

Abgedeckt: unbekannte/ungültige Nachrichten nach ready ohne Status-/Berichts-/Portwirkung; sofortige `invalidAnswer`-Ablehnung und neue Operation; gültige/ungültige alte Antworten nach Abschluss und Timeout; fehlende, String-, negative, gebrochene, NaN-, unendliche, unsichere sowie fremde/0-IDs; gleiche ID in beiden Familien in beiden Fehlerrichtungen; echte Timeout-/Exit-/Fehlerwege; überlange Texte durch den tatsächlichen Settings-Dienst einschließlich Bereinigung der Abruf-Deduplizierung. Ungültige Sprachliste übernimmt keine Teilberichte. Unicode-Kontrollen: ASCII, Emoji und kombinierende Zeichen bei 4.096/4.097, Whitespace und Null-Prototyp-Schlüssel.

Integration ersetzt Electron und die unbeteiligte Startbereinigung von Spielstand-Temporärdateien; Hauptprozesslogik, Parser, Dienste, Dateischreiben der Paketauswahl und Operationssperren bleiben real. Sie ist kein Nachweis echten Structured-Clone-Transports oder eines vom Rust-Lader akzeptierten Schadpakets.

Coverage aus v8/lcov: **global 81,41 % Lines (1.985/2.438)**, Plattform gesamt 98,85 %; geänderte prüfbare Einheiten **core-status.ts 100 % (69/69)** und **settings.ts 100 % (61/61)**. `main.ts` bleibt wie zuvor als Einstieg aus dem numerischen Coverage-Gate ausgenommen; seine tatsächliche Verdrahtung wird durch die genannten Handlerfälle und Electron-E2E geprüft. Keine neue Ausnahme.

## Gates und unabhängige Prüfung

- ESLint ohne Warnungen, Prettier, TypeScript, erzeugte Typen und Schema unverändert; Lizenzprüfung 546 npm-Pakete: grün.
- cargo-deny vollständig grün; npm-Audit Exit 0 mit unverändertem erlaubtem Bestand von 15 moderaten Befunden. Bestehende ts-rs-Warnung beim Typenexport unverändert.
- Vollständiger `npm run build` bestanden. `npm run test:e2e` mit dem bereits geprüften prozesslokalen englischen Electron-Launcher: **beide vorhandenen Tests bestanden**, Exit 0. Systemlokalisierung unverändert; keine neue sprachübergreifende E2E-Zusage. Build und Electron außerhalb der Sandbox wegen der bereits belegten notwendigen Prozessabfrage.
- Neuer unabhängiger Prüfer `gpt-6-astra`, hohe Denktiefe, ohne Gesprächsverlauf, ausschließlich lesend auf `db77ff0` plus Produktdiff: **keine neuen hohen, mittleren oder niedrigen Befunde** im Paket A. Keine eigenen Tests/Generatoren durch den Prüfer. N5 gemäß Auftrag ausgeklammert, keine Beförderung abgeleitet.

Erster Handlerlauf scheiterte an fehlendem `webContents.send` im Electron-Mock; ergänzt, danach alle Funktionsfälle bestanden. Strikte Lintfehler im Test (Non-null-Assertions und void-Kurzformen) wurden ohne Suppressionen bereinigt; letzter Lint-/Typ-Lauf bestanden. Keine Produkt-Ersatzlösung oder gelockerte Regel.

Quick Start und Installationsvoraussetzungen unverändert, kein frischer Installationsklon für diese Validierungskorrektur nötig. Rust-Produktcode unverändert; Kern-Sicherheitsprüfung und bestehende generierte Vertragsprüfungen lokal bestanden, vollständige Rust-/Plattformmatrix wird durch PR-CI erneut nachgewiesen. Reaktiv-Quote 3/10, gesamt 20/52; keine stillen Klassifikationsänderungen. Ablaufregister ohne neue fällige Maßnahme.

`pre-commit run --all-files` unter Node 26.11.1: sämtliche Hooks grün, einschließlich Markdown, JS, rustfmt und Clippy. Gestagten Inhalt vor Commit geprüft. Dokumentbudgets, Linkziele, vollständiges ADR-Register (52/jüngste 30) und Quote (20 gesamt, drei der jüngsten zehn reaktiv) geprüft; keine Kern-/Brücken-/CI-/Templateänderung. Keine neue fällige Ablaufmaßnahme.

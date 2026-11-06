# Bewusste Unterschiede zwischen Entwicklung und gepackter Anwendung

Stand: 2026-11-05, Q.10 / ADR-050; statisch gegen `b16167c` geprüft.
OpenAirside ist eine lokale Desktop-Anwendung. „Produktion“ meint hier das
gepackte Programm, keinen Serverbetrieb. Keine Konfiguration wurde geändert.

| Unterschied | Entwicklung | Gepackte Anwendung | Was bleibt lokal unentdeckt? / Nachweis |
|---|---|---|---|
| Entwicklerwerkzeuge | zugelassen | deaktiviert | Ein Test im Entwicklungsmodus belegt die Sperre im Paket nicht; `app/src/main/main.ts:445`, `security.ts:29`, vorhandener Security-Unit-Test |
| Logschwelle | debug | info | Debugmeldungen können Release-Diagnosen verdecken; `app/src/main/main.ts:431`, Laufprüfung im Paket erforderlich |
| Dateibestand | Workspace enthält Quellen, Tests und Hilfsdateien | Paketliste enthält dist, natives Modul und content, keine dist-Sourcemaps; asar aus | Fehlende Paketdateien und Paketpfade brauchen gepackten Rauchtest; `package.json` build.files; vorhandene plattformübergreifende Release-/Rauchtests sind getrennt zu belegen |
| Installation und Plattform | Start im vorhandenen Checkout | dmg/NSIS/AppImage/deb mit plattformspezifischem Verhalten | Signatur, Sandbox und Start nach Installation werden durch Unit-Tests nicht belegt; macOS vorläufig ad-hoc signiert, `hardenedRuntime: false` nach bisheriger Entscheidung, Phasenschritte 7.1/7.2 |
| Verfügbarer Grafikpfad (3.2, 2026-11-06) | macOS-WebGL grün; Linux-CI zeigt Canvas-Ausweichpfad. Dieser wurde lokal mit deaktivierter GPU reproduziert und scheitert am Shader-Mesh | tatsächliches WebGL2 muss verfügbar sein; eine stille Canvas-/WebGPU-Auswahl ist kein freigegebener Ersatz für den GL-Weltshader | Lokaler WebGL-Erfolg belegt fehlende-GPU-/CI-Umgebungen nicht. [Diagnose und offene Entscheidung](research/3.2-linux-canvas-diagnose.md); kein Paket-/Linux-Hardwaretest oder Konfigurationswechsel daraus |

Beide Betriebsarten nutzen gehärtete Renderer-Einstellungen und Dateien im
Electron-Datenordner. Q.10 führt keinen neuen Sicherheitsstandard ein und
belegt keinen erneuten Paketlauf. Neu entdeckte Unterschiede im selben Commit
hier ergänzen; Secrets nur als Vorhandensein, niemals als Werte dokumentieren.

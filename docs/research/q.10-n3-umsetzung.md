# Q.10-N3 – Umsetzung von Option A

**Stand:** 2026-11-05; ADR-051 vor Code, Freigabe „a“ durch Projektinhaber. Auf `9bba166` umgesetzt, committed und gepusht; vollständige PR-CI und Security grün. Q.10-N2 bleibt getrennt; keine Hochstufung des Personenmoduls durch diese Korrektur.

## Ergebnis und Gültigkeitsbereich

Personenzustände werden auf Gruppe, Reisezustand und vereinbare Queue-/Wartezeitfelder geprüft. Vor Weltübernahme folgt der Stationsbezug einschließlich Reiserichtung, eindeutiger positiver Nummern und Zulassungsgrenzen. Welt & Bau prüft eindeutige Stationsziele und Wartezellen. Fehler benennen die verletzte Invariante; gültige neutrale Personen ohne Ziele, wartende Personen ohne Zulassung und abgeschlossene Reisen mit Restwerten bleiben zulässig.

Der historische FIFO-Zähler entfällt. Der bestehende Zulassungsscan bestimmt das Maximum aller gespeicherten Nummern einschließlich COMPLETE. Vor dem Batch wird konservativ Raum für höchstens die gesamte Personenanzahl reserviert; bei Bedarf ordnungserhaltend verdichtet. Vergabe erfolgt danach ohne Wrap. Der seltene Sortierpfad ist durch 200.000 Personen begrenzt. Format 5 und die Serialisierungsfelder bleiben unverändert; Laden normalisiert keine Nummern.

Geduld/Bedürfnisse sind ausdrücklich nach 5.4 verschoben, mit konkretem Verhalten-/Speicher-/Test-Landeplatz im Fahrplan. FR-016 bleibt offen. FIFO-Verantwortung bleibt im Personenmodul; Stationskonfiguration bleibt in Welt & Bau. Keine neue Modulbeziehung, Abhängigkeit, Build- oder CI-Konfiguration.

## Funktionsnachweise

- `cargo test --workspace --all-features --offline --locked`: Exit 0, 341 Tests bestanden, gezielte Lastmessung regulär ignoriert.
- Neue Lade-Matrix: erlaubte Zustände, WAITING ohne Nummer, COMPLETE mit Nummer/Wartezeit und frühem Routenende; gültige Ankunfts-Stationen. Negative Gegenstücke für Gruppe, Zustand, neutrale/reisende Felder, falsche oder fehlende Station, doppelte Nummern und Überbelegung.
- Echte Container mit neu berechneter Prüfsumme: gültige Kontrolle lädt hashgleich; jede manipulierte Passagier-Invariante scheitert mit präzisem `invalidState/check`. Sitzung, Tick und Weltprüfsumme bleiben unverändert.
- Stationskonfiguration: doppelte Ziele, doppelte Wartezellen innerhalb und zwischen Stationen scheitern; unterschiedliche Kapazität/Zellzahl bleibt außerhalb der Szenarioregel zulässig.
- FIFO-Grenzen: `MAX` und `MAX-1` mit zwei neuen Zulassungen erhalten ältere Reihenfolge. COMPLETE-Werte fließen in die Ableitung ein.
- Öffentlicher Simulation-/Container-Rundlauf: unveränderter Snapshot bei Tick 111, 443 Byte und unmittelbarer Hash `16680832451352878351`; 4.000 Folgeticks hashgleich. Zwei unabhängige Live-Läufe zusätzlich 500 Ticks hashgleich. Die ursprünglichen roten Gegenproben bleiben als historischer Fehlernachweis unverändert.

## Lastmessung

`cargo test -p openairside-core --release --offline --locked admission_load_measurement -- --ignored --nocapture`, Exit 0, macOS ARM64, Rust 1.98.1. Gemischte COMPLETE/WAITING, teilerfremd permutierte eindeutige Bestandsnummern und eine halbe Population neuer Zulassungen; Invarianten und Reihenfolge vor/nach geprüft. Einzelmessungen der Zulassungsphase, kein plattformübergreifendes Performanceversprechen:

| Personen | normal | Überlaufverdichtung |
|---|---:|---:|
| 10.000 | 20,584 µs | 102,5 µs |
| 200.000 | 415,792 µs | 2,2815 ms |

Bestehender Tick-Benchmark: `cargo bench -p openairside-core --bench tick --offline --locked`, Exit 0; 10.000 Personen, vier Threads, 2.000 Messschritte, Mittel 0,0917 ms (Grenze 1), p99 0,1867 ms (Grenze 4). CPU-Modell vom Werkzeug als unbekannt gemeldet; lokale Messung ersetzt nicht Referenzhardware oder CI-Benchmarks.

## Prüfung und Grenzen

Unabhängiger Prüfer `gpt-6-astra`, hohe Denktiefe, ohne Gesprächsverlauf und ausschließlich lesend: keine hohen oder mittleren Befunde im Produktdiff gegen `64087d8`. Selbst ausgeführt: vier Personenregressionen, zwei öffentliche Rundläufe und eine Stationsvalidierung, alle Exit 0. Anfängliche Lasttest-Blindstelle (nur COMPLETE, vorsortierte Werte) durch gemischten permutierten Batch behoben und erneut statisch geprüft. Keine eigene Coverage-, Release-Last-, Hook- oder CI-Abnahme des Prüfers behauptet.

Lokale Gates: rustfmt, Clippy über alle Targets/Features, ESLint, Prettier, TypeScript, erzeugte Typen/Schema unverändert, npm-Lizenzen (546 Pakete), 490 Vitest-Tests in 40 Dateien bestanden. cargo-deny vollständig grün; npm-Audit Exit 0 mit unverändertem Bestand von 15 moderaten Befunden. Bestehende ts-rs-Warnung zu `deny_unknown_fields` bleibt unverändert. Erster ESLint-Lauf erfasste von den All-Features-Tests erzeugte `core/bindings`; diese eigenen untracked Testartefakte wurden reversibel nach `/private/tmp` verschoben und der vollständige JS-Prüflauf wiederholt, Exit 0.

Coverage: `cargo llvm-cov --package openairside-core --all-features --offline --locked --fail-under-lines 85 --json`, anschließend `cargo llvm-cov report --package openairside-core --fail-under-lines 85`, jeweils Exit 0. Kern **98,10 % Lines (12.443/12.684)**; geänderte Einheiten Personen **98,48 % (712/723)**, Welt & Bau **98,46 % (576/585)**, Welt **99,81 % (519/520)**. Bestehendes Projektwerkzeug cargo-llvm-cov 0.9.1 und LLVM-Komponente lokal ergänzt, keine Repository-Abhängigkeit verändert.

Vollständiger Build und beide vorhandenen Electron-E2E-Tests bestanden. Erster Build scheiterte in der Sandbox an `spawn EPERM` der napi-Prozessabfrage; Wiederholung außerhalb der Sandbox erfolgreich. Erster E2E-Lauf: Smoke grün, Welt-Test scheiterte ausschließlich an englischem Erwartungstext bei deutscher Systemlokalisierung. Derselbe unveränderte Testbestand mit prozesslokalem macOS-Argument `-AppleLanguages '(en)'` über temporären Electron-Launcher: **2 bestanden**, Exit 0. Keine Systemeinstellung, Produktdatei oder Testassertion dafür geändert. Das ist ein englischer lokaler Abnahmelauf; die deutsche Anzeige wurde beobachtet, keine neue allgemeine sprachübergreifende E2E-Abdeckung behauptet.

`pre-commit run --all-files`: alle Hooks grün, einschließlich Markdown, Security-Grundprüfungen, JS und Rust. Änderungen vor dem Commit am gestagten Inhalt geprüft. [CI 42049026540](https://github.com/PatSchCode/OpenAirside/actions/runs/42049026540) und [Security 42049026580](https://github.com/PatSchCode/OpenAirside/actions/runs/42049026580) auf `9bba166` abgeschlossen und erfolgreich; einschließlich Rust-/Electron-Prüfungen auf Windows, Linux und macOS, Coverage und Kern-Benchmarks. Abschlusskopf `eaa2e66`: CI 42050232088 und Security 42050232092 grün (16 bestanden, sieben dokumentationsbedingt übersprungen). PR #49 nach `main` gemergt als `468bc32`; [Main-CI 42050840216](https://github.com/PatSchCode/OpenAirside/actions/runs/42050840216) und [Main-Security 42050840247](https://github.com/PatSchCode/OpenAirside/actions/runs/42050840247) abgeschlossen und vollständig grün. Produktcode entspricht unverändert `9bba166`.

Quick Start: Installation, Startbefehle und öffentliche Bedienwege unverändert; kein frischer Installationsklon für diese Kernkorrektur erforderlich. Dokumentbudgets, lokale Links, ADR-Register (51, jüngste 30), Reaktiv-Quote (4/10), Requirements und Ablaufregister geprüft. Keine fällige neue Ablaufmaßnahme; bestehende Fristen bleiben erhalten.

# Vollständige ADR-Übersicht

Quelle: docs/decisions.md Teil A; angelegt am 2026-11-05 in Q.10 (ADR-050).
Enthält ADR-001 bis ADR-050. Einzelentscheidungen bleiben unter docs/decisions/.
Bei jeder neuen Entscheidung diese vollständige Übersicht mitpflegen.

| ADR | Datum | Status | Klassifikation | Themen | Kategorie | Kurztitel |
|---|---|---|---|---|---|---|
| 001 | 2026-10-13 | Aktiv | STRATEGISCH | METHODIK | Methodik | Klasse G und Anpassung des Vorlagen-Sets |
| 002 | 2026-10-13 | Aktiv (bestätigt durch ADR-014) | STRATEGISCH | STACK | Externe Abhängigkeiten | Stack: Web-Präsentation + Rust-Simulationskern |
| 003 | 2026-10-13 | Aktiv | STRATEGISCH | MODUL, SCHNITTSTELLE | Architektur | Kern/Präsentation-Trennung, feste Simulationsschritte, Inhalte als Daten |
| 004 | 2026-10-13 | Aktiv | STRATEGISCH | PERFORMANCE, MODUL | Architektur | Wegfindung nach dem Wegweiser-Prinzip |
| 005 | 2026-10-13 | Aktiv | STRATEGISCH | MODUL | Architektur | Verkehrsregeln, Vorrang-Rangfolge und automatische Verkehrssteuerung |
| 006 | 2026-10-13 | Aktiv, durch ADR-047 bestätigt | STRATEGISCH | METHODIK, DATENMODELL | Methodik | Funktionsumfang: Inventar, Modernisierung, Cargo vorbereitet |
| 007 | 2026-10-13 | Aktiv | STRATEGISCH | DEPLOYMENT | Lizenz/Compliance, Build/Deploy | Vertrieb über GitHub, Lizenzen MIT und CC0 |
| 008 | 2026-10-13 | Aktiv | STRATEGISCH | MODUL | Architektur | Eigene SVG-Grafiken, Rasterung im Build |
| 009 | 2026-10-13 | Aktiv | ERKENNTNIS | STACK, SCHNITTSTELLE, PERFORMANCE | Architektur | Betriebsart des Kerns: napi-rs im Electron-Utility-Prozess |
| 010 | 2026-10-13 | Aktiv | ERKENNTNIS | DATENMODELL, PERFORMANCE, MODUL | Datenmodell, Architektur | Datenmodell und Parallelisierung im Kern, Übernahme der Wegweiser |
| 011 | 2026-10-13 | Aktiv | REAKTIV | METHODIK, PERFORMANCE | Projekt-Vorgabe | Linux-Leistungsmessung auf echter Hardware verschoben auf Schritt 7.2 |
| 012 | 2026-10-13 | Aktiv | REAKTIV | DEPLOYMENT | Build/Deploy, Projekt-Vorgabe | macOS nur für Apple-Chips, kein Intel- und kein Universal-Build |
| 013 | 2026-10-13 | Überholt durch ADR-020 | ERKENNTNIS | DATENMODELL, PERFORMANCE | Datenmodell | Größte Karte 768 × 768 Zellen je Etage, fünf Etagen |
| 014 | 2026-10-13 | Aktiv | ERKENNTNIS | STACK, PERFORMANCE | Architektur | Stack aus ADR-002 bestätigt, Leistungsnachweis mit Auflagen erbracht |
| 015 | 2026-10-13 | Aktiv | REAKTIV | DEPLOYMENT | Build/Deploy | Ad-hoc-Signatur für macOS-Pakete der Spike-Builds |
| 016 | 2026-10-13 | Aktiv | ERKENNTNIS | METHODIK | Architektur, Methodik | Phasenabschluss 1 – gezielt umbauen, Reifegrad-Regel für Umsetzungsschritte |
| 017 | 2026-10-13 | Aktiv | REAKTIV | PERFORMANCE | Architektur | Reifegrad „NFR Performance Gesamtsystem" auf das Gemessene begrenzt |
| 018 | 2026-10-13 | Aktiv | OPERATIV | DEPLOYMENT, STACK | Build/Deploy, Externe Abhängigkeiten | Projektgerüst, CI-Gates und napi-Versionen für Phase 2 |
| 019 | 2026-10-13 | Aktiv | REAKTIV | STACK | Lizenz/Compliance | Erlaubte Lizenzen um BlueOak-1.0.0, 0BSD, Python-2.0 und WTFPL erweitert |
| 020 | 2026-10-13 | Aktiv (Begehbarkeit des Vorfelds je Personengruppe geändert durch ADR-037) | REAKTIV | DATENMODELL, PERFORMANCE | Datenmodell | Weltmaßstab 2 m je Zelle, Luftseite im Raster, Wegweiser in Blöcken, Zeitmodell |
| 021 | 2026-10-13 | Aktiv (S-6 für Pakete geändert durch ADR-026, um `saves.preview` ergänzt durch ADR-028, `onCoreExit` durch ADR-029, `lang.*` und Modul-Karte durch ADR-031; S-5-Grenzen geändert durch ADR-030; fortgeschrieben und um S-8 ergänzt durch ADR-032; S-1 fortgeschrieben durch ADR-038) | ERKENNTNIS | SCHNITTSTELLE, SECURITY | API-Verträge, Architektur, Sicherheit | Schnittstellenverträge S-2 bis S-7 |
| 022 | 2026-10-13 | Aktiv | OPERATIV | DEPLOYMENT, PERFORMANCE | Build/Deploy | Kern-Benchmark als CI-Gate und Prüfung der erzeugten TypeScript-Typen |
| 023 | 2026-10-13 | Aktiv | OPERATIV | DEPLOYMENT, PERFORMANCE | Build/Deploy | Wegweiser-Benchmark als Schritt im CI-Job „Kern-Benchmark“ |
| 024 | 2026-10-14 | Aktiv (Einschränkung auf die Brücke aufgehoben durch ADR-025) | OPERATIV | STACK, SCHNITTSTELLE | Externe Abhängigkeiten | Umwandlung JS ↔ Rust in der Brücke mit napi `serde-json` |
| 025 | 2026-10-18 | Aktiv | REAKTIV | STACK, SCHNITTSTELLE, DEPLOYMENT | Externe Abhängigkeiten, Build/Deploy | Bibliotheken für den Datenpaket-Lader und die Schema-Erzeugung |
| 026 | 2026-10-18 | Aktiv | REAKTIV | SCHNITTSTELLE, SECURITY | API-Verträge | Pakete in S-6 über ihre Manifest-ID |
| 027 | 2026-10-19 | Aktiv | OPERATIV | STACK, SCHNITTSTELLE | Externe Abhängigkeiten | Format, Kompression und Prüfsumme des Spielstands |
| 028 | 2026-10-19 | Aktiv | REAKTIV | SCHNITTSTELLE | API-Verträge | Vorschaubilder der Spielstand-Liste über `saves.preview` |
| 029 | 2026-10-19 | Aktiv | REAKTIV | SCHNITTSTELLE | API-Verträge | `onCoreExit` meldet, ob die Plattform den Kern neu startet |
| 030 | 2026-10-19 | Aktiv | REAKTIV | SCHNITTSTELLE, SECURITY | API-Verträge, Sicherheit | Obergrenzen des Spielstands auf 512 MB gesenkt |
| 031 | 2026-10-19 | Aktiv | REAKTIV | SCHNITTSTELLE, MODUL | API-Verträge, Architektur | Sprachpakete erreichen die Oberfläche über `lang.*` in S-6 |
| 032 | 2026-10-19 | Aktiv (L-1 entschieden durch ADR-038) | REAKTIV | SCHNITTSTELLE, MODUL | API-Verträge, Architektur | Funde der Sammelprüfung der Vertragssätze S-1 bis S-7 |
| 033 | 2026-10-19 | Aktiv | OPERATIV | STACK, DEPLOYMENT | Externe Abhängigkeiten, Build/Deploy | SVG-Rasterer resvg im Werkzeug-Crate `tools/assets` |
| 034 | 2026-10-19 | Aktiv | ERKENNTNIS | PERFORMANCE | Architektur | Darstellung mit echten Grafiken belastbar im gemessenen Umfang |
| 035 | 2026-10-19 | Aktiv | OPERATIV | DEPLOYMENT | Build/Deploy | Toolchain in den Workflows mit `rustup toolchain install` |
| 036 | 2026-10-19 | Aktiv | REAKTIV | MODUL, DATENMODELL, METHODIK | Architektur, Datenmodell, Methodik | Phasenabschluss 2 – gezielt umbauen vor dem vertikalen Schnitt |
| 037 | 2026-10-20 | Aktiv | REAKTIV | DATENMODELL, MODUL | Datenmodell, Architektur | Personengruppen mit Zugangsprofilen, Flächenarten und Querungen |
| 038 | 2026-10-20 | Aktiv | REAKTIV | SCHNITTSTELLE, MODUL, PERFORMANCE | API-Verträge, Architektur | Ruhende Welt als Blöcke im Bild, Zeichnung mit Datentextur, S-1 Layout 2 |
| 039 | 2026-10-21 | Aktiv | STRATEGISCH | MODUL, DATENMODELL | Architektur, Datenmodell | Kernstruktur der Fachmodule mit interner Ereignis-Warteschlange |
| 040 | 2026-10-25 | Aktiv | OPERATIV | METHODIK | Methodik | Schritt 3.1 in drei Stufen |
| 041 | 2026-10-25 | Aktiv | OPERATIV | DEPLOYMENT, PERFORMANCE | Build/Deploy | Dauerbau-Benchmark als Schritt im CI-Job „Kern-Benchmark“ |
| 042 | 2026-10-25 | Aktiv | REAKTIV | DATENMODELL, SCHNITTSTELLE | Datenmodell, API-Verträge | Wirkung der Objekttypen, Möbel sperren, Abriss berührter Objekte |
| 043 | 2026-10-26 | Aktiv | OPERATIV | DEPLOYMENT | Build/Deploy | CI-Trigger nur für `main` und PRs, Code-Jobs nicht bei reinen Doku-Änderungen |
| 044 | 2026-10-26 | Aktiv | ERKENNTNIS | PERFORMANCE | Architektur | Bauaktion im nächsten Bild und Speicher der Weltdarstellung belastbar, Auflage Windows erfüllt |
| 045 | 2026-10-28 | Aktiv | REAKTIV | DATENMODELL, SCHNITTSTELLE | Datenmodell, API-Verträge, Build/Deploy | Bauaufträge mit Bauarbeitern, Anlieferstelle, Gelände in neuen Karten |
| 046 | 2026-10-29 | Überholt durch ADR-047 | STRATEGISCH | METHODIK | Methodik | Neuausrichtung als eigenes Flughafen-Aufbauspiel |
| 047 | 2026-11-04 | Aktiv | REAKTIV | METHODIK, COMPLIANCE | Produktvision, Lizenz/Compliance | Rückkehr zur umfassenden Flughafenmanagementsimulation |
| 048 | 2026-11-04 | Aktiv | STRATEGISCH | DATENMODELL, SCHNITTSTELLE, MODUL | Datenmodell, API-Verträge, Architektur | Minimaler Passagierfluss und Terminalstationen in Schritt 3.2 |
| 049 | 2026-11-04 | Aktiv | OPERATIV | DATENMODELL, SCHNITTSTELLE, PERFORMANCE | Datenmodell, API-Verträge | Additive Stationsverbesserung für die erste Terminallektion |
| 050 | 2026-11-05 | Aktiv | OPERATIV | METHODIK | Methodik | Dev-Templates gezielt in OpenAirsides Codex-Regeln übernehmen |

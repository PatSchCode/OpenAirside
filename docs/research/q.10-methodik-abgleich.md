# Q.10 – Methodik-Abgleich und erster Architektur-Code-Abgleich

## Prüfstand und Freigabe

- Datum: 2026-11-05; Auftrag: gezielte Übernahme nach Option A, ADR-050.
- Upstream: Dev-Templates `e9bcc7c69b437a034cca092f6fd6fd6230e56c59`, PR #70.
- Produktbasis: OpenAirside `b16167c8125ec94991f1525a52469e71cd986a47` (Merge PR #47); Code in dieser Arbeit unverändert.
- Hauptagent: Codex laut Systemkontext, genaue aktive Modellklasse unbestimmbar. Drei Prüfer mit explizit eingestelltem `gpt-6-astra`, hoher Denktiefe und `fork_turns=none`; kein Umsetzungsgespräch, kein Logbuch, nur lesende Aufträge. Die Einstellung belegt nicht ein unabhängig abgefragtes tatsächlich bedientes Modell.
- Verfahren: statischer Abgleich beider Richtungen von Modulbeschreibung, Aufrufstellen, Datenfluss und Verträgen; keine Tests durch die Prüfer. Befunde am Originalcode vom Hauptagenten nachgelesen. Kein vollständiges Sicherheits-, Performance- oder Plattformaudit.

## Übernahme und bewusste Abweichungen

| Bereich | Übernahme in OpenAirside |
|---|---|
| Einstieg | Remote/Branch/PR-Prüfung, vollständige Pflichtlektüre, abgeschnittene Ausgaben nachlesen; reine Prüfsitzungen ohne Schreibpflicht |
| Quellenstruktur | AGENTS bleibt vollständig; kein paralleles Detailregelwerk. Historische Templates unverändert |
| Modelle | Astra/Sol/Luna/Terra erhalten; keine automatische Übernahme der Upstream-Klassen oder Modellstopps |
| Reifegrade | Regel-007 erhalten; unabhängiger Abgleich zusätzlich vor neuer Beförderung; Rückstufung weiter per ADR |
| Verträge | Regel-008 erhalten; additive Ergänzung nicht pauschal ADR-pflichtig |
| Ausführung | Atomare Checkpoints, gestagter Diff und CI-Nachverfolgung erhalten |
| Tests | Testsignal vor erstem Lauf; Fehlerprobe für Schutzmechanismen und unabhängige Prüfung |
| CI | Keine Pipeline geändert, kein Hook-Bypass, kein Abschlussstatus CI-AUSSTEHEND; Automatisierung als Q.10-N1 vorgelegt |
| Sessionende | Eigentümerregel zu Abschlusscommits erhalten; keine still eingeführten Abschluss-PRs |
| Dokumente | Unicode-Budgets, Archivierung, aktive Verweise und Umgebungsregister ergänzt |
| Reaktiv | 14-Tage-Regel ausdrücklich verankert, Reflexion nach Regel-009; ältere Klassifikationen nicht rückwirkend geändert |
| Einrichtung | Fähigkeiten und Bereitschaft belegen; keine fremden Cloud-Pfade oder neue Produktinitialisierung |

## Fehlerproben

Je Prüfer wurde eine isolierte Dokumentationskopie mit bewusst falschen Aussagen vorgegeben. Keine Projektdatei wurde dafür verändert. Alle drei Proben bestanden:

| Prüfer | Eingesetzte Abweichung | Gefundener Gegenbeleg |
|---|---|---|
| Kern | napi-Exporte im Kern, Brücke nur Darstellung | `bridge/src/lib.rs:257` und folgende Exporte, `bridge/Cargo.toml:22`; Kern ohne napi-Abhängigkeit |
| Brücke/Plattform | nodeIntegration eingeschaltet, Renderer liest Spielstände direkt | `app/src/main/security.ts:31`, typisierte Preload-API in `app/renderer/platform.ts:96`, Dateizugriff im Hauptprozess |
| Inhalte/Sprache/Grafik | Ausführbare Code-Mods und ungeprüfte Sprachdateien im Renderer | JSON-only-Lader `core/src/datapack/files.rs:123`, validierte Sprachkette über Kern, Brücke, S-8/S-6 und Localizer |

Diese Gegenbelege prüfen die Wirksamkeit am isolierten Auftrag; sie sind keine echten Produktbefunde und kein Laufzeitnachweis.

## Echte Befunde

Zeilen beziehen sich auf die unveränderte Codebasis `b16167c`; Dokumentzeilen auf den Stand vor Q.10, sofern im Text nicht anders angegeben. Keine hohe Abweichung gefunden. Acht mittlere Befunde sperren neue Beförderungen im betroffenen Umfang. Die Tabelle ist die gemeinsame Befundliste; Folgeschritte verweisen auf die IDs.

| ID | Schwere / Seite | Befund und Beleg | Freigabewirkung / Landeplatz |
|---|---|---|---|
| K1 | mittel / Code | `core/src/persons/mod.rs:104` prüft neue `group`/`state`-Werte und FIFO-Invarianten nicht; Restore bei `:591` übernimmt sie. S-5 verlangt Werte-/Invariantenprüfung (`architecture-schnittstellen.md:223`). `255` ist nach statischer Prüfung nicht ausgeschlossen | Sicherheit nicht vertrauenswürdiger Spielstände; Q.10-N3, Abschluss 3.2 gesperrt |
| K2 | mittel / Code | `next_queue_order` steigt bei `persons/mod.rs:497`, wird bei `:547`/`:556` weder gehasht noch gespeichert; Restore `:601` leitet ihn nur aus verbliebenen Nummern ab. Nach geleerter Check-in-Queue erhält derselbe Passagier nach Laden später eine andere Nummer als ohne Laden; `queue_order` gehört zur Prüfsumme (`:121`) | Statisch hergeleiteter Determinismusfehler, noch kein Reproduktionstest; Q.10-N3, Datenmodellentscheidung vor Korrektur |
| K3 | mittel / Code bzw. Umfang | ADR-048 verlangt gespeicherte Geduld (`:16`); `Person` (`persons/mod.rs:66`) hat kein solches Feld, TODO bei `:63` verweist weiter auf 3.2 | Umsetzung oder ausdrücklicher Descope; Q.10-N3 |
| K4 | mittel / Doku bzw. Verantwortung unklar | Architektur Welt & Bau nennt FIFO (`architecture.md:126`); `build/target.rs:24` hält Stationskonfiguration, FIFO läuft in `persons/mod.rs:471`/`:505`. Personenbeschreibung nennt noch zufällige Ziele | Verantwortungsfrage gegen ADR-048 klären, nicht still legitimieren; Q.10-N3 |
| P1 | mittel / Code | Unbekannte S-8-Nachricht wird über `main.ts:274` in `core-status.ts:308` als `failed` behandelt; S-8 `:269` verlangt Verwerfen. Lebender Kern kann so bei weiterer Nutzung `notReady` liefern | Bestehenden Vertrag und bewussten Unit-Test abgleichen; Q.10-N2 |
| P2 | mittel / Code | Ungültige Paketantwort wird bei `app/src/main/main.ts:264` der aktuellen `pendingRequest()` zugeordnet, nicht der gelieferten Nummer. Verspätete Antwort kann fremde Folgeanfrage beenden; S-8 `:282` verspricht Zuordnung | Korrektur innerhalb vorhandenen Vertrags; Q.10-N2 |
| P3 | mittel / Code | Fehlerhafte `savesAnswer` mit gültiger Anfrage-ID wird bei `main.ts:257` nur verworfen; S-8 `:269` verspricht `invalidAnswer`. Tatsächlich bleibt die Anfrage bis Timeout gesperrt | Fehlerbehandlung gezielt testen; Q.10-N2 |
| P4 | mittel / Code | `app/src/main/settings.ts:132` prüft Sprachname nur als String, `:153` begrenzt Texte nicht auf 4096 Zeichen; S-8 `:269`/`:283` verlangt die S-4-Grenzen | Keine nachgewiesene Umgehung des Rust-Laders; Validierung/Sicherheitskategorie, Q.10-N2 |
| D1 | niedrig / Doku | Öffentliche Schnittstellenliste der Brücke nennt S-7 nicht (`architecture.md:226`), während Modulkarte und `CoreClient.query → Core.receive → PortHost.query` ihn tragen | In Q.10 redaktionell ergänzt, keine neue Beziehung |
| D2 | niedrig / Doku | S-4 `:196` behauptet, harte Links würden nicht erkannt; `:202` und `core/src/datapack/files.rs:346` nennen Unix-Ablehnung | Bestehende Aussage vereinheitlichen, Q.10-N4 |
| D3 | niedrig / Doku | S-4 `:203` beschreibt Sprachname nur „in mindestens einem Paket“. `core/src/datapack/load.rs:767` verlangt ihn beim ersten geladenen Eintrag; `:792` lehnt zusätzlich leere/Whitespace-Texte ab | Präzisierung oder Vertragsentscheidung bewusst trennen, Q.10-N4 |

## Nachprüfungsstand 2026-11-05

Die ursprünglichen Befundzeilen bleiben als Prüfstand erhalten. N3/ADR-051 ist auf `9bba166` umgesetzt, unabhängig geprüft und mit grüner Main-CI auf `468bc32` gemergt: [Nachweise](q.10-n3-umsetzung.md). P1–P4 sind inzwischen durch isolierte Handler-/Parserproben auf diesem Produktstand bestätigt; [N2-Vorlage](q.10-n2-vorlage.md) und [Quellen](q.10-n2-gegenproben.md). N2/P1–P4 ist nach ausdrücklicher Freigabe/ADR-052 auf `a95e47d` korrigiert, unabhängig nachgeprüft und mit vollständiger CI nachgewiesen: [Umsetzung](q.10-n2-umsetzung.md). Der zusätzliche mittlere Port-Befund wurde in Q.10-N5 dynamisch bestätigt und nach freigegebenem A/ADR-053 auf `c295ebb` korrigiert; unabhängige Nachprüfung und volle CI grün: [Nachweise](q.10-n5-umsetzung.md). Historische Aussagen der folgenden ursprünglichen Abschnitte sind keine Behauptung über den heutigen Korrekturstand.

## Umfang ohne weitere gefundene Abweichung

Nachprüfung 2026-11-05 in Q.10-N3: K1 und K2 durch isolierte dynamische Proben einschließlich vollständigem Container-Rundlauf bestätigt; zusätzlich FIFO-Verletzung beim Überlauf nachgewiesen. Die frühere Kennzeichnung „statisch“ in der Befundtabelle beschreibt den ursprünglichen Abgleich. Aktuelle Ergebnisse und offene Korrekturentscheidung: [N3-Vorlage](q.10-n3-vorlage.md); keine Produktkorrektur vorweggenommen.

Im geprüften Zusammensetzungspfad: Build → Wayfinding → Persons, interne Bauarbeiterereignisse, S-7-Snapshot; Prozessaufteilung und Bildtransport; benannte S-6-Aufrufe; JSON-only-Lader und atomare Paketübernahme; Sprachfallback; Build-only-Grafikrasterung und Atlasanbindung in drei Stufen. „Keine weitere Abweichung gefunden“ ist keine Gesamtfreigabe dieser Module.

Weiterhin ungeprüft bleiben die vollständige Validierungsmatrix und die Wirksamkeit einer Korrektur für K1/K2 sowie dynamische Gegenproben zu P1–P4, vollständige Aufrufkombinationen, sämtliche vorläufigen Fachmodule außerhalb Personen, erneute plattformübergreifende Laufzeitmessungen und die komplette Verhaltensabnahme der Agentenmethodik in dieser Desktop-Oberfläche. Bestandenes Upstream-Verhalten in einer Cloud-Aufgabe gilt nicht automatisch hier.

## Dokument- und Abschlussnachweise

Die Übernahme ändert nur Markdown-Dokumente. Kein neuer Produkt-Stack, keine Abhängigkeit, kein Build-/CI-Eingriff. Onboarding-Änderung korrigiert lediglich den erwarteten Regeldateinamen und Prüfverweis; keine Befehlsfolge oder Voraussetzung geändert, deshalb kein frischer Installationslauf für Q.10. Vor Phasenabschluss bleibt die frische Validierung auf zwei Plattformen Pflicht.

Größen-, Archivverlust-, Linkprüfung und aktiver Hook bestanden. Auf `179aaed` sind CI 42039133732 und Security 42039133699 grün (PR #48); 16 Checks bestanden und 7 dokumentationsbedingt übersprungen. Keine neue Anwendungstestabnahme: Matrix-Schritte wurden auf grünem Ausgangsstand übersprungen. Warnungen mit dem vorherigen Main-Lauf verglichen; moderate npm-Funde von 25 auf 15 gesunken, Rustup-Hinweis und Ubuntu-Annotation unverändert. Die Prüfung dieses Berichts ersetzt weder Funktionsprüfung noch die Befundbehebung. Der reine Abschlussvermerk wird im selben PR nachgeführt und am neuen Kopf erneut geprüft.

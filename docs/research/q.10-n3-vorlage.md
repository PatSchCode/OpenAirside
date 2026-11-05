# Q.10-N3 – Passagierzustände und Save/Load: Entscheidungsvorlage

**Status:** ENTWURF – wartet auf ausdrückliche Freigabe; kein neuer ADR und keine Produktkorrektur vorweggenommen.
**Prüfstand:** 2026-11-05, Produktbasis `b01ae6c7a382ef9ec596ef0410e20f232f4e6af4`.
**Fahrplan:** Q.10-N3, fehlender Abschlussumfang von 3.2; keine neue Phase-3-Funktion.

## 1. Auftrag, Verfahren und Nachweise

K1–K4 aus dem [Q.10-Bericht](q.10-methodik-abgleich.md) wurden am Code und gegen ADR-048/049 geprüft. Die laufende Produktimplementierung blieb unverändert. Gegenproben liefen in einer per `git archive` erzeugten temporären Quellenkopie mit derselben Cargo.lock und Rust 1.98.1, ausschließlich offline und locked. Testquellen, Konfiguration, Kontrollfälle und Reproduktionsweg stehen in [Gegenproben](q.10-n3-gegenproben.md).

Das Testsignal stand vor den Läufen im Sessionstart: Gültige Kontrollen müssen laden; unbekannte Zustände müssen zurückgewiesen werden; identische Folgeoperationen müssen auch nach Laden identische Zustandsprüfsummen liefern. Die erwarteten roten Assertions zeigen Bestandsfehler. Sie sind keine roten Projekt-Gates und wurden nicht durch eine Änderung der Produktlogik grün gemacht.

| Gegenprobe | Beobachtung | Aussage |
|---|---|---|
| Fünf bestehende Personen-Unit-Tests, vor Proben eingebaut | 5 bestanden, Exit 0 | Der vorhandene Testbestand erkennt die folgenden Lücken nicht |
| Gültiges WAITING ohne Zulassung (`queue_order=0`) | bestanden | Ein legaler Wartezustand vor voller Queue muss weiter ladbar sein |
| COMPLETE mit gespeicherter Nummer und Wartezeit | bestanden | Bestehendes Verhalten behält diese Werte; kein pauschales Nullsetzen beim Laden |
| Zwei gleich gestartete Live-Simulationen, 500 Ticks | bestanden | Kontrollkonfiguration ist deterministisch ohne Save/Load |
| Unbekannte Gruppe/Zustand, je `255`, direktes Restore | beide akzeptiert, Assertion rot | K1 bestätigt |
| Derselbe Fall über vollständigen Container mit gültiger Prüfsumme und `decode → load_game` | gültige Kontrolle vollständig geladen und hashgleich; beide ungültigen Codes auch durch `load_game` akzeptiert, Assertion rot | K1 erreicht die tatsächliche Personenprüfung bei Sitzungsübernahme, nicht nur die Containerdekodierung |
| Leere Zwischenqueue nach Check-in, Postcard-Rundlauf | unmittelbarer Hash gleich; Zähler original 2, geladen 1; nächste Nummer 2 vs. 1 | K2 im Modul isoliert bestätigt |
| Voller öffentlicher Simulation-/Container-Rundlauf, Terminal 64², eine Person, Seed 17, FixedTick 1 | Snapshot bei Tick 111, 443 Byte; unmittelbarer Hash `16680832451352878351` gleich. Nach 102 weiteren Ticks Hash `2687050893686028099` vs. `10485593718514696914`, Assertion rot | K2 auch über echte Bewegung, Stationswechsel, encode/decode/load_game bestätigt |
| Älterer Wartender mit Nummer `u32::MAX`, späterer neuer Wartender | Späterer wird zuerst bedient, Assertion rot | Zusätzlicher Randfall derselben Nummernvergabe: Überlauf verletzt FIFO |

Abschließender gemeinsamer Probenlauf: Unit-Target 2 grüne Kontrollen / 4 rote Befundtests; Integrationstarget 1 grüne Kontrolle / 1 roter Befundtest, Exit 101 wie erwartet. Das sind verschiedene negative Fehlerproben, keine drei fehlgeschlagenen Reparaturversuche. Keine Coverage- oder Sicherheitsabnahme der Korrektur behauptet.

Die unabhängige Vorlagenprüfung erkannte, dass die erste Container-K1-Hülle bei `decode` endete. Diese Hülle wurde bis `load_game` ergänzt und erneut gezielt ausgeführt: gültige Kontrolle lädt hashgleich, `group=255` und `state=255` werden weiterhin tatsächlich übernommen, Exit 101 wie erwartet. Der vollständige K2-Rundlauf nutzte bereits `load_game`. Ablehnung mit Erhalt der alten Sitzung bleibt ein zu bestehender Regressionstest der künftigen Korrektur; der heutige Bestand lehnt diese Eingaben gerade nicht ab.

## 2. Vorgeschlagenes gemeinsames Paket A

### Laden: gültige Zustände erhalten, ungültige abweisen

- Im derzeitigen Person-Datensatz sind Gruppe 0 (Debug), 1 (Abflug) und 2 (Ankunft) vorgesehen. Bauarbeiter bleiben im eigenen Worker-Pool. Die weiteren global reservierten Gruppen sind keine bereits implementierten Personzustände.
- Gruppe 0: Zustand 0, keine FIFO-Mitgliedschaft und Wartezeit 0. Gültige Debug-Personen ohne Ziele weiter laden.
- Passagiere: nur travelling/waiting/complete; tatsächlichen Stationsbezug gegen Welt & Bau prüfen. Gültige frühe Routenenden bei fehlender Folgestation nicht durch eine unbeschlossene neue Regel verbieten.
- Travelling: keine Queue-Mitgliedschaft und Wartezeit 0. WAITING mit Nummer 0 bleibt zulässig; auch dort kann Wartezeit entstehen. COMPLETE darf die heute gespeicherten Restwerte behalten.
- Positive FIFO-Nummern eindeutig, Zulassungen je Station höchstens so viele wie ausgewiesene Wartezellen; Stationsziele und Wartezellen eindeutig. Keine neue allgemeine Regel `capacity == queue_cells.len()` einführen: ADR-049 fordert dies nur für das Szenario.
- Bei Verletzung vor Sitzungsübernahme `invalidState` mit präzisem `check`; laufende Sitzung bleibt unverändert. Ladeprüfung liest vorgelagerte Stationskonfiguration über den bestehenden Weg, keine neue Modulbeziehung.

Diese Matrix ist der freizugebende Sollzustand. Die bisherigen Proben bestätigen konkrete Lücken, noch nicht die vollständige Matrix. Nach Freigabe entstehen gezielte positive und negative Regressionstests einschließlich atomarer Ablehnung.

### FIFO: Format 5 erhalten, historischen Zähler entfernen

`next_queue_order` ist heute entgegen seinem Kommentar nicht vollständig abgeleitet: Die bisher vergebenen, inzwischen gelöschten Nummern fehlen beim Laden. Empfehlung:

1. Den historischen Zähler als unabhängige Zustandsgröße entfernen.
2. Vor **jeder** Zulassungsphase die nächste Nummer aus allen gespeicherten Nichtnull-Nummern bestimmen, einschließlich der legalen COMPLETE-Restwerte.
3. Vor einem Zulassungsbatch ausreichend `u32`-Spielraum sichern. Bei Bedarf vorhandene Nummern ordnungserhaltend auf `1..=N` verdichten und dann neue vergeben; auch `MAX-1` mit mehreren neuen Zulassungen prüfen. Höchstens 200.000 Personen lassen ausreichend Raum.
4. Verdichtung nur im normalen, identischen Simulationsschritt; keine besondere Normalisierung ausschließlich beim Laden. Unmittelbare S-5-Prüfsumme bleibt gleich, spätere Fortsetzung wird ausschließlich vom gespeicherten Zustand bestimmt.

Private absolute Nummern sind keine Personenkennungen. Die Bedienreihenfolge je Station bleibt erhalten. Es braucht kein zusätzliches persistiertes Feld und keinen Formatwechsel; strengere Invarianten können zuvor akzeptierte ungültige Dateien abweisen. Legitime bisherige Zustände werden durch positive Gegenproben geschützt.

**Technische Alternative:** Historischen Zähler speichern und hashen (Format 6); auch dann Überlauf definieren. Ohne Migration würde dies Format-5-Spielstände abweisen. Eine Migration kann verlorene historische Nummern nicht exakt rekonstruieren. Für fortlaufende absolute Nummern ist keine aktuelle Produktanforderung belegt; daher nicht empfohlen.

### Geduld: ausdrückliche Teilverschiebung nach 5.4

ADR-048 verlangt gespeicherte Geduld. Im Code fehlt sie; es gibt nur einen TODO. Ein ungenutztes Feld wäre keine Behebung. Paket A schlägt vor, **nur diese Teilfunktion** ausdrücklich aus dem Abschlussumfang 3.2 nach 5.4 zu verschieben. Bis dahin gibt es Wartezeiten und Engpassanzeigen, aber keine Geduldgrenze oder darauf beruhende Verhaltensreaktion.

Nach Freigabe werden ADR-048 durch einen neuen ADR fortgeschrieben, FR-016 als teilweise abgedeckt beschrieben und 5.4 als konkreter Landeplatz ergänzt. Dort ist ausdrücklich aufzunehmen:

> Geduld und Bedürfnisse: deterministischer gespeicherter Zustand; nachvollziehbare Wirkung von Wartezeiten; sichtbare Reaktion bei Erschöpfung mit verständlicher Diagnose; sichere Route-/Queue-Übergänge; identische Fortsetzung nach Save/Load; Tests für Schwellen, Reaktion und Erholung. Konkrete Werte und Reaktionen vor Umsetzung entscheiden.

Der entsprechende TODO erhält erst nach Freigabe diesen Landeplatz. FR-016 bleibt bis zur vollständigen Bedürfnis-/Zufriedenheitsmechanik offen; ein funktionierender Minimalfluss erledigt nicht die gesamte Anforderung. Keine automatische Änderung an übrigen 5.4-Funktionen.

### Verantwortung: vorhandene Aufteilung bestätigen

ADR-048 trennt Passagierzustand im Personenmodul, Stationskonfiguration in Welt & Bau und FIFO-Verarbeitung im Rust-Kern. Er verlangt keinen Queue-Besitz in Build. Der Code passt dazu: `build/target.rs` enthält Konfiguration, `persons/mod.rs` verwaltet Mitgliedschaft, Reihenfolge, Bedienung und Reisestatus. Paket A bestätigt diese Aufteilung; danach werden der widersprüchliche Satz in Architektur „Takt & Weltzustand“ und die veraltete Personenbeschreibung berichtigt. Keine Verlagerung von Queue-Zustand nach Build.

## 3. Optionen und Produktfolgen

| Option | Ergebnis und Aufwand |
|---|---|
| **A – zuverlässigen Minimalfluss abschließen (empfohlen)** | Validierung und deterministische FIFO-Vergabe einschließlich Überlauf korrigieren, Format 5 erhalten, Zuständigkeit bestätigen; Geduld ausdrücklich in 5.4 umsetzen. 3.2 bleibt bis Nachweisen offen, kann danach ohne zusätzliche Geduldmechanik abgeschlossen werden |
| **B – Geduld bereits vor Abschluss von 3.2 liefern** | Dieselben Zuverlässigkeitskorrekturen; Geduld nicht verschieben. Vor Umsetzung zusätzlich Werte, Erschöpfungsreaktion, Diagnose und Speicherung konkret entwerfen und freigeben. Größerer Abschlussumfang und späterer Beginn von 3.11; noch keine einzelne Verhaltensregel als beschlossen behaupten |

Die Entscheidung betrifft den Zeitpunkt der Geduldmechanik. Die technische Empfehlung zur Zustandsvalidierung und FIFO-Ableitung ist in beiden Optionen gleich; ein Formatwechsel wird nicht als notwendige Folge dargestellt.

## 4. Unabhängige Bewertung und Stellungnahme

Ein neu gestarteter Prüfer mit expliziter Einstellung `gpt-6-astra`, hoher Denktiefe und ohne geerbten Gesprächsverlauf bewertete ADRs, Verträge und Originalcode ausschließlich lesend. Ergebnis: Paket A empfohlen; Format 5 nur bei Ableitung vor jeder Zulassung und definierter Verdichtung; WAITING ohne Zulassung und COMPLETE-Restwerte erhalten; Geduld mit FR-016 und konkretem 5.4-Landeplatz verschieben; bestehende Modulaufteilung ist ADR-gedeckt. Das ist eine statische unabhängige Bewertung, keine zweite dynamische Reproduktion.

Der Hauptagent stimmt zu. Die eigenen dynamischen Gegenproben bestätigen zusätzlich K1/K2 und den Überlauf. Der zusätzliche Scan kann in die vorhandene Zulassungsprüfung eingehen; seltenes Sortieren beim Verdichten braucht einen gezielten Lastnachweis. Die Ausgestaltung der vollständigen Validierung und der Lastnachweis sind noch nicht implementiert oder bestanden.

**Konfidenz:** hoch für die reproduzierten Fehler und die Verantwortungslesart; mittel bis hoch für Paket A, weil die vorgeschlagene Korrektur noch ihre Positiv-/Negativmatrix und Lastprüfung bestehen muss. **Umkehrbarkeit:** billig für Vorlage und Dokumentation; mittel für eine spätere Änderung der Nummernsemantik, teuer vermeidbar bei zusätzlichem Formatwechsel.

## 5. Freigabegrenze und Umsetzung nach Antwort

Nach Freigabe zuerst neuer ADR (voraussichtlich ADR-051); bei A Fortschreibung von ADR-048 innerhalb 14 Tagen, daher `[REAKTIV]`. Reaktiv-Quote und Reflexion nach Regel-009 vor Code prüfen; die derzeitigen ADR-Klassifikationen werden nicht still geändert. Danach Vertrag/Validierungsmatrix und Geduld-Landeplatz dokumentieren, schmale Regressionstests im Produkt anlegen, Kernkorrektur umsetzen und gezielt prüfen.

Abschluss verlangt relevante Tests, konkrete Coverage ≥ 85 % der Kerneinheit, Format/Clippy/Typ-/Security-/Audit-Gates, unabhängige Nachprüfung der Sicherheitskorrektur, grünen Hook und CI. Der angrenzende S-8-Schritt Q.10-N2 gehört nicht in dieses Paket. Diese Vorlage und die absichtlich roten isolierten Proben sind kein Abschluss von Q.10-N3 oder 3.2.

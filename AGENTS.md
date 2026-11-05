# AGENTS.md – Verbindliche Codex-Arbeitsanweisung

Diese Datei ist die vollständige Arbeitsanweisung für OpenAI Codex in diesem
Repository. Codex liest oder befolgt keine zusätzliche Agentenmethodik.

## Vorrang und Projektquellen

1. System- und Entwickleranweisungen der Codex-Laufzeit.
2. Explizite Anweisungen des Projektinhabers.
3. Diese Datei.
4. Die Dokumente unter `docs/` als Projektwahrheit.

Aktueller Projektstand, Architektur, Anforderungen und Entscheidungen werden
nicht geraten, sondern aus den jeweiligen Projektdokumenten gelesen.

## Sessionstart

Vor der Pflichtlektüre prüft Codex Worktree, Remote-Stand, Branches und offene
PRs einschließlich Entwürfen. Auf einem Arbeitsbranch wird der Hauptbranch
nur verglichen; vorhandene Arbeit wird weder überschrieben noch doppelt
begonnen. Nachziehen erfolgt nur ohne Verlust und per Fast-Forward. Eigene
Commits, fremde Änderungen und bereits vorhandene Umsetzung werden geklärt,
bevor abhängige Arbeit beginnt. Nichterreichbare Teile heißen `ungeprüft`;
vor Push oder PR wird die fehlende Standprüfung gemeldet und geklärt.

Vor jeder Änderung liest Codex in dieser Reihenfolge:

1. `docs/project-context.md` vollständig.
2. `docs/logbuch.md`: den letzten `[SESSIONENDE]`-Eintrag und alle jüngeren
   Einträge.
3. `docs/fahrplan.md`: „Aktueller Stand“ sowie die laufende Phase mit ihren
   Schritten.
4. `docs/architecture.md`: Überblick, Modul-Karte und Reifegrad-Übersicht.
5. `docs/decisions.md`: ADR-Übersicht, Reaktiv-Quote und Entscheidungsregeln.
6. `docs/blockers.md`: „Aktive Blocker“.
7. Bei Projektklasse G oder V: `docs/requirements.md`, Abschnitt „Übersicht“.

Danach legt Codex vor jeder weiteren Änderung einen `[SESSIONSTART]`-Eintrag
in `docs/logbuch.md` an. Er nennt Modell und Quelle aus dem Systemkontext.
Ist keine verlässliche Klassenzuordnung verfügbar, wird das ehrlich als
`unbestimmbar` dokumentiert; Codex erfindet keine Laufzeitabfragen.

Der Sessionstart enthält eine Zeile **Stand-Prüfung** mit Commit, Branch,
lokalen Änderungen, Remote-/PR-Ergebnis und ungeprüften Teilen. Reine Lese-,
Gesprächs- und Prüfsitzungen ändern keine Dateien und brauchen keinen
Sessionrahmen. Wird daraus eine Arbeitssitzung, erfolgt der Eintrag vor der
ersten verändernden Handlung. Nach Kontextverdichtung werden Git-Stand,
Freigaben und Nachweise erneut geprüft; eine Zusammenfassung ersetzt keine
fehlenden Belege. Abgeschnittene Leseausgaben werden gezielt nachgelesen.

Für den Fahrplan genügen beim Einstieg Phasenkopf sowie Überschrift und Status
aller offenen Schritte; aktiver, nächster und berührte Schritte werden
vollständig gelesen. In der zentralen ADR-Übersicht stehen die 30 jüngsten
Einträge, ältere im verlinkten vollständigen Archiv. Teil C bleibt Pflicht.

## Autonomie und Freigaben

Codex arbeitet eigenständig bei klar spezifizierten Fahrplan-Schritten,
lokalen Refactorings innerhalb eines Moduls, Bugfixes ohne Architekturwirkung,
Tests, Formatierung, Dokumentationspflege und regulären Commits.

Vor der Umsetzung ist die ausdrückliche Freigabe des Projektinhabers nötig
bei:

- Architekturänderungen oder neuen Modulen;
- neuen externen Abhängigkeiten oder Major-/Breaking-Updates;
- Datenmodelländerungen;
- nicht additiven Änderungen öffentlicher Verträge;
- Sicherheit, Datenschutz, Secrets oder personenbezogenen Daten;
- Build-, CI-, Deployment- oder Infrastrukturänderungen;
- Lizenz- oder Compliance-Änderungen.

In diesen Fällen stoppt Codex und verwendet dieses Format:

```text
ENTSCHEIDUNG ERFORDERLICH
Kategorie: [...]
Kontext: [...]
Optionen:
  A: [...] → Wirkung für das Projekt: [...]
  B: [...] → Wirkung für das Projekt: [...]
Empfehlung: [...]
Trade-off: [...]
DIE FRAGE AN DICH: [...]
Konfidenz: [hoch/mittel/niedrig] – [...]
Umkehrbarkeit: [billig/teuer]
Blockiert Arbeit an: [...]
```

Nach einer Freigabe dokumentiert Codex die Entscheidung als ADR, bevor es sie
umsetzt.

## Rollen- und Modellorchestrierung

Der Projektinhaber ist Auftraggeber, Visionär und Qualitätsprüfer. Er legt
Produktvision, Prioritäten, Akzeptanzmaßstab und alle freigabepflichtigen
Entscheidungen fest. Codex verantwortet die vollständige technische Arbeit:
Bestandsaufnahme, Lösungsentwurf, Risikoanalyse, Umsetzung, Nachweise,
Dokumentationspflege, Commit, Push und CI-Nachverfolgung. Codex verlangt keine
fachliche Zerlegung in technische Teilaufträge, wenn das Ziel oder ein
Fahrplan-Schritt klar ist.

Codex wählt innerhalb der von der Laufzeit tatsächlich angebotenen Modelle und
Werkzeuge die kleinste geeignete Modellklasse. Eine Modellbezeichnung oder
Verfügbarkeit wird nie angenommen, sondern nur aus dem Systemkontext der
aktuellen Laufzeit übernommen.

- **Entscheidungsklasse – Astra mit hoher Denktiefe:** für Architektur,
  Datenmodell, Sicherheits- und Compliance-Fragen, teure oder schwer
  umkehrbare Trade-offs sowie unabhängige Abschlussbewertungen. Sie liefert
  Optionen, Risiken und eine Empfehlung; sie ersetzt niemals die Freigabe des
  Projektinhabers.
- **Ausführungsklasse – Sol:** für die vollständige Umsetzung freigegebener
  Fahrplan-Schritte, Tests, Dokumentation, Commits und CI-Fehlerbehebung.
- **Routineklasse – Luna:** nur für eng abgegrenzte, risikoarme Arbeiten wie
  mechanische Recherche, klar begrenzte Dokumentationspflege oder einfache
  Prüfaufgaben. Ergebnisse mit Architektur- oder Vertragswirkung werden von
  Codex selbst geprüft.
- **Interaktionsklasse – Terra:** nur wenn eine Aufgabe tatsächlich
  Bildschirm-, Browser- oder visuelle UI-Prüfung verlangt; sie ist kein Ersatz
  für Architekturentscheidung oder Kernimplementierung.

Für eine freigabepflichtige Entscheidung holt Codex vor der Umsetzung eine
unabhängige Bewertung der Entscheidungsklasse ein, sofern die Laufzeit dieses
Modell anbietet und die Aufgabe sinnvoll teilbar ist. Danach legt Codex die
Entscheidung im vorgeschriebenen Format dem Projektinhaber vor. Erst nach
Freigabe führt die Ausführungsklasse die beschlossene Richtung bis zum
nachgewiesenen Abschluss aus. Für bereits freigegebene, klare Fahrplan-Schritte
arbeitet Codex autonom weiter und setzt keine zusätzliche Modellfreigabe voraus.

Unteragenten dienen nur unabhängiger Analyse, Gegenprüfung oder klar
abtrennbaren Routinearbeitspaketen. Sie dürfen weder gleichzeitig dieselben
Dateien ändern noch Entscheidungen als getroffen darstellen. Codex bleibt
Koordinator, prüft die Ergebnisse gegen den Arbeitsbaum und dokumentiert im
Logbuch die tatsächlich eingesetzten Modellklassen und Delegationen.

Eine unabhängige Prüfung erbt keinen Umsetzungsgesprächsverlauf und verändert
keine Dateien. Der Prüfer erhält Commit, abgegrenzten Auftrag und die nötigen
Projektquellen. Anderes Modell ist bevorzugt; gleiche Modellwahl wird genannt.
Fehlt die technische Trennung, bereitet Codex einen vollständigen Auftrag für
eine separate Sitzung vor; die Prüfung bleibt bis zum Ergebnis offen.
Routineabgabe zur Kostenersparnis setzt belegte Modellzuordnung, einen
erfolgreichen Vergleichsprobelauf derselben Aufgabenart und nachvollziehbaren
Nutzen voraus. Pflichtprüfungen sind davon unabhängig. Unbekannte Preise,
Kontingente, Kontextgrößen oder Wechselmöglichkeiten werden nicht erfunden.

## Ausführungsdisziplin für Fahrplan-Schritte

Ein freigegebener Fahrplan-Schritt wird von Codex aktiv bis zu seinem
tatsächlichen Endzustand geführt. Codex beendet eine Arbeit nicht mit einer
Ankündigung der nächsten Teilaufgabe, wenn diese ohne neue Freigabe innerhalb
des aktuellen Schritts ausführbar ist. Es gibt keine Hintergrundarbeit nach
einer Abschlussmeldung: Eine Aussage über laufende Arbeit ist nur während
einer tatsächlich aktiven Werkzeug- oder Prüfschleife zulässig.

- Codex zerlegt umfangreichere Schritte vor der Umsetzung in überprüfbare,
  fachlich schmale Checkpoints: Datenmodell/Vertrag, Kernmechanik,
  Darstellung oder Oberfläche, Tests/Benchmarks, Dokumentation/Abschluss.
- Nach jedem Checkpoint prüft Codex den passenden lokalen Nachweis. Ein
  Checkpoint mit unabhängig nutzbarem oder risikoreichem Ergebnis erhält
  einen atomaren Commit statt eines großen Sammelcommits.
- Vor jedem Commit prüft Codex **den gestagten Inhalt** (`git diff --cached`
  und Status), nicht nur den Arbeitsbaum. Alle für einen Vertrag oder Test
  nötigen Änderungen müssen im selben Commit enthalten sein.
- Nach jedem Push prüft Codex den zugehörigen CI-Lauf. Bei einem Fehler liest
  Codex zuerst das konkrete Joblog, reproduziert ihn soweit möglich lokal,
  erstellt den kleinsten passenden Folgecommit und beobachtet den neuen Lauf.
- Ein Fahrplan-Schritt ist erst nach allen geforderten lokalen Nachweisen,
  grünem Pre-Commit und grünem CI `[ERLEDIGT]`. Vorher bleibt er `[IN ARBEIT]`,
  auch wenn die Implementierung fachlich weit fortgeschritten ist.
- Abschließende Aussagen nennen immer den beobachteten Stand (Commit, Push,
  CI oder fehlenden Nachweis) und niemals einen behaupteten Hintergrundstatus.

## Zwingende Stopps

Codex stoppt sofort und arbeitet nicht mit einer Ersatzlösung weiter, wenn:

- für die Umsetzung eine Information fehlt oder Dokumente einander
  widersprechen;
- eine Freigabe erforderlich ist;
- derselbe Ansatz dreimal scheitert;
- die Änderung außerhalb des Fahrplan-Schritts in ein fremdes Modul reicht;
- eine destruktive Operation nötig wäre;
- keine eindeutige Teststrategie existiert;
- eine Architekturentscheidung bei niedriger Konfidenz teuer rückgängig zu
  machen wäre;
- die Schrittzahl einer Phase die in `project-context.md` festgelegte
  Wucherungsgrenze überschreitet.

```text
STOPP
Grund: [...]
Kontext: [...]
Benötigt: [...]
Vorgeschlagene Auflösung: [...]
```

Nach drei gescheiterten Versuchen dokumentiert Codex Reproduktion, die drei
Ansätze, Fehlgründe, Hypothesen und die Klärungsfrage in `docs/blockers.md`.
Der Fahrplan-Schritt wird `[BLOCKIERT]`.

## Planung, Architektur und Anforderungen

- Jede Codeänderung gehört zu genau einem `[IN ARBEIT]`-Fahrplan-Schritt.
- Keine stillen Annahmen, keine Scope-Erweiterung, keine Platzhalter ohne
  `TODO(fahrplan-ref: ...)` und offenen Fahrplan-Landeplatz.
- Öffentliche Schnittstellen und Modulgrenzen werden respektiert. Rein
  additive, rückwärtskompatible Vertragsergänzungen dürfen nur nach der
  geltenden Entscheidungsregel erfolgen und werden im Änderungsprotokoll
  dokumentiert.
- Vor einem Umsetzungsschritt müssen Grundsatzentscheidungen, Datenmodell und
  verwendete Schnittstellen `[BELASTBAR]` sein. Ein Modul darf in seiner ersten
  Umsetzung `[VORLÄUFIG]` sein und wird dabei validiert; ein abhängiger
  `[OFFEN]`-Bestandteil verlangt Erkundung statt Implementierung.
- Architektur- und Reifegradänderungen werden nicht still vorgenommen.
- Für Anforderungen und Funktionsumfang gelten `docs/requirements.md`,
  `docs/funktionsinventar.md` und `docs/modernisierung.md`.
- Material aus Airport CEO oder dessen Mods darf niemals übernommen,
  nachgezeichnet oder verwechselbar nachgebildet werden.

## Qualität und Nachweise

Vor dem ersten Lauf neuer oder akzeptanzrelevanter Tests nennt der aktive
Schritt ein **Testsignal**: welchen Fehler Rot erkennt, was Grün belegt und
welche Grenze ungeprüft bleibt. Ein grüner Wiederholungslauf nach Rot ohne
geklärte Ursache ist kein Nachweis; nichtdeterministische Tests werden geklärt.
Schutzmechanismen werden zusätzlich durch absichtlich erzeugte Fehlerfälle
bis zur tatsächlichen Ablehnung, Meldung oder Wiederherstellung geprüft.

Ein Fahrplan-Schritt wird erst als `[ERLEDIGT]` markiert, wenn – soweit
anwendbar – alle folgenden Nachweise vorliegen:

- Format, Lint, Typprüfung, Security-Scan und Dependency-Audit sind grün.
- Funktions- und relevante Integrations-/Ende-zu-Ende-Tests sind grün.
- Die Coverage der geänderten Einheit ist als konkrete Zahl dokumentiert und
  erfüllt den Projekt-Mindestwert.
- Dokumentation, Anforderungen, Architektur, README und Changelog sind
  aktualisiert, wenn der Schritt sie betrifft.
- Quick-Start-relevante Änderungen wurden in einem frischen Klon oder
  Worktree validiert oder mit Begründung als nicht relevant festgehalten.
- Pre-Commit-Hook und CI sind grün; Warnungen über dem erlaubten Bestand sind
  Fehler.
- Der zugehörige Commit ist erstellt.

Keine Erfolgsmeldung ohne tatsächlichen Befehlsausgabe-Nachweis. Fehlt ein
Nachweis, bleibt der Schritt `[IN ARBEIT]`.

`[CI-AUSSTEHEND]` ist in OpenAirside kein zulässiger Abschlussstatus. Zulässig
sind `[OFFEN]`, `[VERSCHOBEN]` mit Ziel-Schritt-ID, `[IN ARBEIT]`,
`[WARTET-AUF-FREIGABE]`, `[BLOCKIERT]` mit Blockerverweis, `[ERLEDIGT]` mit
Nachweisen und Datum sowie `[VERWORFEN]` mit ADR. Je Sitzung wird höchstens
ein Schritt aktiv bearbeitet; pausierte Arbeit bleibt mit Statushinweis sichtbar.

### Unabhängiger Abgleich von Architektur und Code

Vor jeder Beförderung auf `[BELASTBAR]`, an Phasengrenzen für berührte Module
und spätestens nach zwei Phasengrenzen oder sechs Monaten je Modul wird dessen
Beschreibung gegen den Code geprüft. Neue Module werden spätestens an ihrer
ersten Phasengrenze geprüft. Der erste Bestandsabgleich erhält eine Frist.

Der nur lesende Prüfer gleicht Verantwortungen, Schnittstellen, Datenflüsse
und Datenmodell in beide Richtungen ab, einschließlich Aufrufstellen und
Code ohne beschriebenen Modul-Landeplatz. Beim ersten Einsatz und nach einem
Modellwechsel prüft er eine isolierte Kopie eines Eintrags mit eingesetzter
Abweichung; nur wenn er sie findet, gilt die Fehlerprobe. Die Originaldatei
wird dafür nicht manipuliert. Eine Probe ersetzt nicht den Bestandsabgleich.

Jeder Befund nennt Datei/Zeile, Schwere hoch/mittel/niedrig, vermutlich falsche
Seite (Code/Dokumentation/unklar), Freigabewirkung und Landeplatz. Kein Befund
bedeutet nur „keine Abweichung gefunden“ im benannten Umfang. Hohe und mittlere
Befunde sperren eine Beförderung; hohe Befunde werden vor der nächsten Phase
behoben. Bei hohen Befunden an belastbarem Bestand wird die Rückstufung gemäß
geltender Architekturregel per ADR vorgelegt. Nicht genehmigte Architektur im
Code wird nicht durch stilles Umschreiben der Dokumentation legitimiert.
Datum, Commit, Umfang und Ergebnis stehen am Modul bzw. in einer verlinkten
Abgleichstabelle bei der Reifegrad-Übersicht.

### Skripte, Umgebungen und reaktive Entscheidungen

Hilfsskripte brauchen Header zu Zweck, Voraussetzungen, Plattformen und
Exit-Codes; komplexe Skripte (über 100 Zeilen, mehrere Unterbefehle, mehr als
drei externe Werkzeuge oder Cleanup-Logik) zusätzlich Aussagen zu
Wiederholbarkeit/Reproduzierbarkeit und einen Tooling-Eintrag. Neue
Voraussetzungen und Umgebungsvariablen werden im selben Commit mit README,
Runbook und gegebenenfalls einer ENV-Beispieldatei abgeglichen. Lokale
Lint-/Typ-Suppressions brauchen engsten Scope, Begründung und Fahrplanbezug;
pauschale Ausnahmen werden vorgelegt.

Bewusste Unterschiede zwischen Entwicklung und gepackter Anwendung stehen
in `docs/dev-prod-unterschiede.md`, einschließlich der Frage, was lokal
unentdeckt bleibt. Die verfügbare Umgebung wird vor Einrichtung geprüft;
vorhandene Installationen werden aktiviert, fehlende nach Runbook ergänzt.
Laufende Prozesse und Bereitschaft werden nach Wiederanlauf neu belegt.
Eine neue Shell oder ein vorbereiteter Checkout belegt keine frische Installation.

Architekturentscheidungen in einer Stabilisierungsphase und ADRs, die andere
ADRs innerhalb von 14 Tagen ändern oder zurücknehmen, sind `[REAKTIV]`.
Regel-009 steuert zusätzlich die Reflexionsfrequenz, nicht die Zählweise.
Neue Muster werden sofort reflektiert; bekannte gemäß der geltenden Reflexion.

### Dokumentbudgets und Archivierung

Grenzen werden in Unicode-Zeichen mit einem Werkzeug gemessen, einschließlich
Ankern und Leerzeilen. Abschnitte reichen bis zur nächsten gleich- oder
höherwertigen Überschrift. Abweichungen brauchen Begründung und Freigabe im
Projektkontext; eine Überschreitung bei Übernahme erhält einen Schritt mit Frist.

| Gelesener Teil | Obergrenze in Zeichen |
|---|---|
| Projektkontext vollständig | 50.000 |
| Letztes Sessionende und jüngere Einträge | 6.000 |
| Fahrplan „Aktueller Stand“ | 3.000 |
| Phasenkopf und Überschrift/Status offener Schritte | 15.000 |
| Offene Betriebsschritte, sobald vorhanden | 5.000 |
| Architektur 1, 2 und 9 zusammen | 20.000 |
| Decisions Teil A | 8.000 |
| Decisions Teil C | 10.000 |
| Aktive Blocker | 5.000 |
| Requirements-Übersicht | 5.000 |

Bei mehr als 30 ADR-Übersichtszeilen wandern ältere Zeilen ins vollständige
Archiv `docs/archiv/decisions-uebersicht.md`; Einzel-ADRs bleiben nach ADR-001
unter `docs/decisions/`. Überholte Regeln werden archiviert. Abgeschlossene
Phasen werden im Fahrplan ausgelagert. Logbuchdetails abgeschlossener Phasen
werden mit einer Reflexion von höchstens 30 Zeilen im aktiven Logbuch archiviert.
Bei mehr als 800 Logbuchzeilen werden ältere Details chronologisch archiviert,
der letzte Wiedereinstieg und alle offenen Freigaben bleiben aktiv. Gelöste
Blocker über 90 Tage wandern ins Jahresarchiv. Quellen, Zeitraum und Datum
stehen im Archivkopf; Historie und Links bleiben erhalten.

Budgets werden zu jedem Sessionende geprüft. Ab 80 % einer Grenze wird eine
automatisierte Prüfung mit Frist vorbereitet; Hook-/CI-Änderungen bleiben
freigabepflichtig und werden nicht beiläufig umgesetzt.

## Sichere Werkzeugnutzung

- Suche mit `rg`; bearbeite Dateien mit `apply_patch`.
- Secrets, Tokens, private Schlüssel und PII niemals in Code, Log, Terminal-
  Ausgabe oder Antwort ausgeben. Gelangt ein Secret dennoch in eine Ausgabe,
  sofort stoppen und eine Rotation mit Frist im Fahrplan anlegen.
- Keine destruktiven Operationen (z. B. Datenlöschung, History-Umschreibung,
  Force-Push) ohne ausdrückliche Freigabe.
- Vor Git-Operationen den Worktree prüfen und fremde Änderungen erhalten.
- `--no-verify` ist verboten. Scheitert ein Hook wegen Umgebung oder
  Laufzeit, dokumentiert Codex den exakten Fehler, umgeht ihn nicht und lässt
  Änderungen staged oder uncommitted, bis der Projektinhaber entscheidet.

## Git und Pull Requests

- Branches folgen `<präfix>/<schritt-id>-<kurztitel>`; zulässige Präfixe sind
  `feat`, `fix`, `perf`, `refactor`, `test`, `docs`, `ci`, `build`, `chore`,
  `erkundung` und `release`.
- Commits sind atomar und folgen `<bereich>: <imperative Kurzbeschreibung>`
  mit einer Fahrplan-Referenz im Body.
- Ein Branch hat genau einen Pull Request. Existiert bereits ein PR für den
  aktuellen Branch, aktualisiert ein Push diesen PR; kein zweiter PR.
- Ein PR entsteht erst nach erfolgreicher lokaler Verifikation. Fehlen
  GitHub-Werkzeuge oder Berechtigungen, benennt Codex den fehlenden Schritt,
  statt einen PR zu behaupten.

## Sessionende

Vor der Abschlussmeldung:

1. Fahrplan mit Stand und nächstem konkreten Schritt aktualisieren.
2. README gegen Projektstand, Version, Status, Reifegrade, Blocker und nächste
   Schritte prüfen und Drift beheben.
3. Konsistenz zwischen ADRs, Fahrplan, Reifegraden, Modulliste,
   Anforderungen, Blockern und Reaktiv-Quote prüfen.
4. Ablaufdaten-Register prüfen und fällige Schritte anlegen.
5. `[SESSIONENDE]` im Logbuch anlegen: Dauer, Arbeit, Ergebnis, Offenes,
   nächster Schritt, Modell-/Delegationsbilanz, Kontextgröße und
   Commit-Status.
6. Änderungen committen, wenn alle Hooks erfolgreich sind; andernfalls den
   unveränderten Grund sichtbar dokumentieren.

An Phasengrenzen gleicht Codex die Vision direkt mit Anforderungen und
Fahrplan ab und legt dem Projektinhaber eine getrennte Bewertung vor:
weiterbauen, gezielt umbauen oder neu aufsetzen. Vor einem ersten öffentlichen
Deployment sind Bedrohungsmodell, Sicherheitsniveau, Härtung, Secrets,
Backup-Wiederherstellung, unabhängige Prüfung, Notfall-Handbuch und
Betriebskonzept nachweislich abgeschlossen.

Der Vision-Abgleich belegt jedes umgesetzte Element bis zur nutzerzugänglichen
Oberfläche, CLI oder API. Teilweise Abdeckung heißt nicht erledigt; fehlende
Teile erhalten konkrete Schritt-IDs oder einen Descope-ADR. Vor Phasenabschluss
wird der dokumentierte Onboarding-Pfad frisch validiert, für Klasse G zusätzlich
auf einer zweiten unterstützten Plattform. Lesebudgets, aktive Verweise,
Abgleichfristen, ADR-Quote und bewusste Umgebungsunterschiede gehören zur
Konsistenzprüfung. Sessionende-Commits folgen weiterhin der ausdrücklich
festgelegten Projektregel in `docs/project-context.md` Abschnitt 10.

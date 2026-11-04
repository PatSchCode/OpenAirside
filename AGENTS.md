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

# Blockers

<!-- Ungelöste Probleme und gescheiterte Ansätze von OpenAirside.
     Wird befüllt, wenn ein Arbeitsschritt nach drei Versuchen nicht gelöst werden konnte
     (CLAUDE.md Abschnitt 10) oder ein Muster der Blocker-Erkennung zutrifft. -->

<!-- ANCHOR:blocker-erkennung -->
## Blocker-Erkennung (vor dem Dreifach-Versuch)

Ein Problem ist **sofort** als Blocker zu behandeln, ohne drei Versuche abzuwarten, wenn eines dieser Muster zutrifft:

1. **Informationslücke:** Eine für die Lösung nötige Angabe fehlt in allen Pflicht-Dokumenten.
2. **Widerspruch:** Zwei Dokumente geben unvereinbare Vorgaben und kein ADR löst den Konflikt auf.
3. **Fremde Modulgrenze:** Die Lösung würde Änderungen in einem Modul erfordern, das nicht Teil des aktuellen Fahrplan-Schritts ist.
4. **Freigabebedarf:** Die Lösung fällt in eine Kategorie aus CLAUDE.md Abschnitt 4.
5. **Nicht-deterministisches Verhalten:** Das Problem tritt nicht reproduzierbar auf. Nicht-Reproduzierbarkeit ist selbst ein Blocker, keine akzeptierte Eigenschaft.

In diesen Fällen: direkt Eintrag hier anlegen, ohne Dreifach-Versuch.

Für alle anderen Fälle gilt die Dreifach-Regel aus CLAUDE.md Abschnitt 10.

---

<!-- ANCHOR:aktive-blocker -->
## Aktive Blocker

### Blocker #002: Abschluss 3.2 durch Passagierfluss-Befunde gesperrt

- **Erkannt:** 2026-11-05, Prüfstand `18176a6`, kein Dreifach-Reparaturversuch.
- **Befund:** hoher Stationswechsel-Fehler A1 und mittlere Befunde A2–A6 aus dem [Abschlussabgleich](research/3.2-abschlussabgleich.md). Erste COMPLETE-Ankunft noch am Gate; Null-Wartestatistik trotz 58 Wartender dynamisch belegt. Unabhängige Astra-Prüfung bestätigt die Abschlusssperre.
- **Wirkung:** 3.2 bleibt IN ARBEIT, Personen VORLÄUFIG; keine Beförderung/3.11-Freigabe aus bisherigen grünen Tests ableiten.
- **Auflösung/Frist:** Nachbesserungsentwurf in 3.2, Daten-/Vertragsfragen ausdrücklich vorlegen; Korrekturen und räumliche Übergangs-/FIFO-/Diagnose-/Lektionsnachweise vor Abschluss 3.2 und vor 3.11. Entwurf spätestens 2026-11-12 vorlegen. Keine Produktkorrektur durch diesen Prüfbericht.

---

<!-- ANCHOR:geloeste-blocker -->
## Gelöste Blocker

### Blocker #001: Leistungsziele nicht messbar – GELÖST 2026-10-13

- **Ursprüngliche Beschreibung:** Im Härtungs-Schritt der Initialisierung (Modus 2, Schritt 3) festgestellt: Die Vision fordert „flüssig" auf „Zielhardware", ohne Bildrate, Simulationstempo und Referenzhardware zu beziffern. Das zentrale Erfolgskriterium wäre damit nicht prüfbar. Blocker-Typ: Informationslücke.
- **Lösung:** Entscheidung des Projektverantwortlichen: Referenzhardware Mittelklasse-PC (8 CPU-Kerne, 16 GB RAM, Mittelklasse-Grafikkarte der aktuellen Generation); Zielwert mindestens 60 Bilder pro Sekunde im Referenzszenario, Simulation ohne Rückstand auch bei höchster Spielgeschwindigkeit. Eingetragen in `project-context.md` Abschnitt 6.
- **ADR:** keiner (Präzisierung eines Constraints; Grundlage für ADR-002)
- **Abgeleitete Regel:** keine

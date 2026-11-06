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
- **Stand 2026-11-06:** [Paket A](research/3.2-nachbesserungsentwurf.md) ausdrücklich freigegeben, ADR-054; [Ursachenprüfung/Prävention](research/3.2-ursachenanalyse.md) vor Code abgeschlossen und unabhängig gegengeprüft. Entwurfsfrist erfüllt; Blocker bleibt bis Umsetzung/Nachweisen aktiv.
- **Historischer Teilstand vor C1/C2:** Zielwechsel/COMPLETE für A1 mit Positions- und Freeze-Regressionen nachgebessert; [Nachweismatrix](research/3.2-nachbesserung-nachweise.md). A2 erneut rot und A3–A6 offen; kein Gesamtabschluss oder Auflösen dieses Blockers.
- **Historischer C1/C2-Zwischenstand:** Kern-/Vertragsumbau implementiert, lokale Übergangs-/FIFO-/Snapshot-/Lade-/Erzeugungsnachweise, Paarvergleich und Last grün; [aktueller Nachweisstand](research/3.2-nachbesserung-nachweise.md). Frühere A2-Nullanzeige durch getrennte Populationen ersetzt. C3-Lektionsweg, übrige C4/C5 und Commit-/CI-Abschluss bleiben offen; Blocker nicht auflösen.
- **Aktueller Stopp 2026-11-06:** Drei Linux-Pixelansätze gescheitert; 3.2 `[BLOCKIERT]`. Kernkorrektur und sichtbare DE/EN-Lektion lokal nachgewiesen, C5-Kantenbefund repariert/nachgeprüft; keine Personenbeförderung und kein Merge. Reproduktion: bestehender CI-Job Electron/ubuntu-latest, `xvfb-run -a npm run test:e2e`, EN-Lektion durchspielen, danach S-1-Pixelgegenprobe. Erwartet `[false,true,false,true,false,false]` für Gleichheit mit leerem Bild; tatsächlich dauerhaft alle sechs `true`, auch aktive/neutrale/Bauarbeiter-Kontrollen ohne sichtbare Änderung. Lokal macOS jeweils grün.
- **Drei Ansätze/Fehlgründe:** (1) `a43f95e`, [CI 42250311855](https://github.com/PatSchCode/OpenAirside/actions/runs/42250311855): feste Messpunkte am angenommenen Kamerazentrum; Pixelassertion rot, Ursache daraus nicht bestimmbar. (2) `dadc661`, [CI 42253029115](https://github.com/PatSchCode/OpenAirside/actions/runs/42253029115): zwei Zeichenzyklen und tatsächliche Masken abwarten; dauerhaft alle `true`, reine Verzögerung erklärt es nicht. (3) `b0dcd4d`, [CI 42254974159](https://github.com/PatSchCode/OpenAirside/actions/runs/42254974159), [Job](https://github.com/PatSchCode/OpenAirside/actions/runs/42254974159/job/128019771394): echte öffentliche Kamerafilter/Quantisierung und gezielter Viewportwechsel; Zoom22 korrekt, weiterhin alle `true`. Kein vierter Versuch oder gelockerter Test.
- **Artefakt/Hypothesen:** [Fehlerbild/Log](https://github.com/PatSchCode/OpenAirside/actions/runs/42254974159/artifacts/13523345339), CI-Aufbewahrung 14 Tage. Nur lesende Terra-Prüfung: orange Punkte anderswo sichtbar, Messbereich auf freiem Spielfeld ohne Punkte; HUD „renderer canvas · gaps 0“. Unterschiede im Canvas-Zeichen-/Partikelpfad, Umsetzung der eingespeisten Positionen oder Aufnahme-/Koordinatenpfad sind Hypothesen, keine bestätigte Ursache. Zeit- und Kameraannahme lokal widerlegt, erklären Linux nicht abschließend.
- **Folgediagnose 2026-11-06:** Nach Freigabe zeitlich begrenzt [Canvas/Mesh-Mechanismus lokal reproduziert](research/3.2-linux-canvas-diagnose.md): vor Pixelinjektion fehlt updateRenderable, reguläres WebGL grün. Linux-Artefakt zeigt Canvas, gleicher Linux-Stack noch zu bestätigen. Astra empfiehlt Software-WebGL2 für Linux-E2E plus sichtbare Ablehnung fehlender WebGL2-Unterstützung; neue CI-/Startfreigabe nötig. Kein Renderer-/CI-Umbau erfolgt. Artefakte vor 2026-11-20 sichern; Frage: Option A dieser Vorlage freigeben?

---

<!-- ANCHOR:geloeste-blocker -->
## Gelöste Blocker

### Blocker #001: Leistungsziele nicht messbar – GELÖST 2026-10-13

- **Ursprüngliche Beschreibung:** Im Härtungs-Schritt der Initialisierung (Modus 2, Schritt 3) festgestellt: Die Vision fordert „flüssig" auf „Zielhardware", ohne Bildrate, Simulationstempo und Referenzhardware zu beziffern. Das zentrale Erfolgskriterium wäre damit nicht prüfbar. Blocker-Typ: Informationslücke.
- **Lösung:** Entscheidung des Projektverantwortlichen: Referenzhardware Mittelklasse-PC (8 CPU-Kerne, 16 GB RAM, Mittelklasse-Grafikkarte der aktuellen Generation); Zielwert mindestens 60 Bilder pro Sekunde im Referenzszenario, Simulation ohne Rückstand auch bei höchster Spielgeschwindigkeit. Eingetragen in `project-context.md` Abschnitt 6.
- **ADR:** keiner (Präzisierung eines Constraints; Grundlage für ADR-002)
- **Abgeleitete Regel:** keine

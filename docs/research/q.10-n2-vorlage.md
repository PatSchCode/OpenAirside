# Q.10-N2 – S-8-Antwortprüfung und Fehlerzuordnung

**Status:** Entscheidungsvorlage; Produktcode unverändert, Sicherheitsfreigabe ausstehend.
**Basis:** Produkt `468bc325ffab424f2ddc30641b88391f20f11630`; `d160277` enthält nur den übernommenen N3-Abschlussvermerk.

## Befunde und dynamische Gegenproben

S-8 verlangt das Verwerfen unbekannter/ungültiger Nachrichten, `invalidAnswer` für zuordenbare unpassende Antworten und die Trennung verspäteter Antworten von aktuellen Anfragen. S-4-Sprachgrenzen gelten auch am Empfang im Hauptprozess. Grundlage: ADR-031/032, S-4/S-6/S-8 und P1–P4 des Q.10-Berichts.

In einer temporären `git archive`-Kopie wurden nur Testzugriffe auf den tatsächlichen `main.ts`-Handler ergänzt. Electron-Transport und Startbereitschaft sind gemockt; Parser, Handler, Paketdienst, `game:new`-Dienst, OperationLock und Timeouts verwenden unveränderte Produktlogik. Vollständige Quellen und SHA-256: [Gegenproben](q.10-n2-gegenproben.md).

| Probe | Beobachteter Bestand | Folge |
|---|---|---|
| P1: nach gültigem `ready` unbekannte Nachricht | Status wird `failed`; nächste Anfrage scheitert mit `notReady` | Lebender Kern wird unnutzbar, obwohl kein Prozessende vorliegt |
| P2: Anfrage 1 läuft ab, Anfrage 2 offen; fehlerhafte Paketantwort mit Nummer 1 | Anfrage 2 wird mit `invalidAnswer` beendet | Fremde Folgeanfrage wird abgelehnt |
| P3: `game:new` offen, fehlerhafte `savesAnswer` mit passender Nummer | Sofort weiter pending, Lock bleibt belegt; erst nach 120 s `timeout` und Freigabe | Fehler wird unnötig verzögert und falsch zugeordnet |
| P4: Sprachname leer bzw. 4.097 Zeichen | beide akzeptiert | Name am Hauptprozess nicht vollständig geprüft |
| P4: Text 4.097 Zeichen | akzeptiert | S-4-Einzellimit am Hauptprozess fehlt |
| Kontrollen | zwei gültige Save-Antworten nacheinander; gültige alte Paketantwort ignoriert, aktuelle beantwortet; fehlerhafte passende Paketantwort lehnt korrekt ab; 4.096 Emoji gültig und Null-Prototyp erhalten | Bestehende gültige Wege müssen erhalten bleiben |

61 vorhandene Tests in drei relevanten Dateien bestanden (Exit 0). Abschließende Proben: 10 Fälle, vier Kontrollen bestanden, sechs Befundassertionen erwartungsgemäß rot (Exit 1). Die roten isolierten Proben belegen Bestandsfehler; keine Produkt-CI oder Reparatur ist dadurch bestanden. Kein dynamischer Nachweis einer Umgehung des Rust-Paketladers, keine vollständige Electron-/Betriebssystemabnahme durch diese Hülle.

## Paket A – bestehenden Vertrag vollständig durchsetzen (empfohlen)

### Nachrichten und Korrelation

- Unbekannte Nachrichten und ungültige Nachrichten ohne brauchbare Antwortzuordnung protokollieren/verwerfen; bisherigen Status und offene Anfragen erhalten. Nur gültiges `ready`, gültiges Start-`error` und tatsächliches Prozessende ändern den Lebenszyklusstatus. Kein neues Timeout für `starting` oder neue Neustartpolitik.
- Eine fehlerhafte Antwort nur anhand **erkannter Antwortfamilie plus tatsächlich gelieferter gültiger `request`** zuordnen. Gültige Nummer bedeutet wie bisher sichere ganze Zahl ≥ 0. Niemals `pendingRequest()` als Ersatznummer einsetzen.
- `packs`/`packsError` gehören ausschließlich zur offenen Paket-Anfrage derselben Nummer. `savesAnswer` gehört ausschließlich zur offenen Save-/Sprach-/Neues-Spiel-Anfrage derselben Nummer. Getrennte Zähler dürfen gleiche Zahlen haben; keine familienübergreifende Ablehnung.
- Passend zugeordnete, aber ungültige Antworten sofort mit vorhandenem `invalidAnswer` beenden, Timer entfernen und Dienst-/Operationssperren über den vorhandenen Abschlussweg freigeben. Fehlerwerte verwenden die vorhandene `Reason`-/S-6-Fehlerhülle.
- Fehlende, negative, unsichere oder nichtganzzahlige Nummer, alte bereits beendete Nummer, fremde Familie: keine fremde Anfrage beenden, keine Nummer aus einem anderen Feld erraten. Die betroffene, noch unbeantwortete Anfrage darf ihren bestehenden Timeout erreichen, wenn keine Zuordnung möglich ist.
- Gültige Antworten und vorhandene Fehlergründe bleiben unverändert; bereits nach Antwortart im Aufrufer geprüfte `value`-Formate werden weiter geprüft. Die sichere Zuordnung gilt für ungültige Hüllen ebenso wie für gültige Antworten.

### Sprachgrenzen am Hauptprozess

- Sprachname entsprechend dem bestehenden Rust-Lader: nicht leer/nicht nur Whitespace, höchstens 4.096 Unicode-Codepoints. Keine `.length`-Grenze in UTF-16-Codeeinheiten: 4.096 Emoji sind gültig. Whitespace-Semantik gegen Rust abgleichen, keine unbeabsichtigte zusätzliche Ablehnung durch abweichendes JavaScript-`trim()`.
- Texte: höchstens 4.096 Unicode-Codepoints, Schlüssel weiterhin `[A-Za-z0-9_.-]{1,128}`, gültiger Sprachcode und nichtnegative sichere Schlüsselzahl. Aufbau des Wörterbuchs ohne Prototyp erhalten.
- Kein neues Gesamtlimit über zusammengeführte Pakete, keine neue Grenze für `keys` außer der bestehenden sicheren Ganzzahl. Diese Entscheidung bleibt beim geplanten Mod-Schritt 6.1.
- Die in D3/Q.10-N4 offene Vertragspräzisierung zu leeren/Whitespace-Texten und zur ersten Namenslieferung wird nicht als P4-Fix still mitentschieden. Sprachname und Textlänge sind hier getrennte Prüfungen.

### Umfang und Abhängigkeiten

Änderungen in der bestehenden Plattformverkabelung und ihren Parsern/Diensten (`main.ts`, `core-status.ts`, `settings.ts` sowie betroffene Tests); tatsächliche Gegenstelle `core-host.ts` und Brücke abgleichen. Keine neue Architekturbeziehung, kein neues Fachmodul, kein Protokoll-/Save-Formatwechsel, keine Abhängigkeit und kein CI-Umbau vorgesehen. Falls eine Gegenstellenänderung nötig wird, vor Code in derselben Freigabe konkret benennen; keine zusätzliche Funktion auf diesem Weg einführen.

Vor Umsetzung nach Freigabe neuer ADR (voraussichtlich ADR-052) zur Sicherheitskorrektur mit klarer Korrelation und Sprachsemantik. Keine neue Architekturentscheidung oder Änderung von ADR-031/032 innerhalb 14 Tagen vorwegnehmen; Klassifikation nach tatsächlicher Wirkung bestimmen, nicht die Reaktiv-Quote passend machen. Eine reaktiv klassifizierte Entscheidung mit Brücken-Codeänderung würde eine neue Reflexion nach Regel-009 verlangen; bloße Nutzung des vorhandenen Kanals verlagert keine Modulgrenze. Keine Beförderung aus dieser Vorlage.

## Risiken und Abnahmen nach Freigabe

| Risiko | Geforderter Nachweis |
|---|---|
| Unbekannte Nachricht wird weiterhin als Totalausfall behandelt | tatsächlicher Hauptprozess-Handler: Zustand vor/nach unbekannt/ungültig gleich; gültige Folgeanfrage funktioniert; gültiges `error`/Exit bleibt wirksam |
| Fremde oder verspätete Antwort beendet aktuellen Auftrag | beide Familien: gültig/ungültig × aktuell/alt/fremd; gleiche Nummer in getrennten Familien; fehlende/ungültige Nummer und unbekannter Typ |
| Ablehnung hängt weiter oder Lock bleibt belegt | echtes `game:new` und Paketdienst: sofort `invalidAnswer`, Timer bereinigt, Lock frei, nächste Operation erfolgreich; vorhandener Timeout/Exit unverändert |
| Name/Text wird versehentlich zu streng geprüft | Grenzwerte 4.096/4.097 ASCII und Emoji; Rust-kompatible Whitespacefälle; gültige Listen und Null-Prototyp-Schlüssel |
| Getrennte Unit-Tests übersehen Verkabelung | Integration gegen tatsächlichen registrierten Nachrichten-Handler; Electron-Rauchtests auf allen drei CI-Plattformen |

Zusätzlich Format/Lint/Typen, konkrete Coverage für geänderte Einheiten ≥ Projektgrenze, Security-/Dependency-Gates, aktiver Hook, unabhängige Sicherheitsnachprüfung und grüne CI. Hauptprozess-Einstiegsdatei ist im bestehenden Coverage-Setup ausgenommen; dieses Paket ändert keine CI-Ausnahme. Die Handler-Integration muss dennoch die tatsächliche Verdrahtung erfassen; keine bloße Nachbildung des Algorithmus als Nachweis.

## Optionen und Empfehlung

| Option | Wirkung für das Projekt |
|---|---|
| **A – P1–P4 gemeinsam korrigieren** | Bestehende S-8-Semantik an allen gefundenen Stellen herstellen; passende Fehler sofort und fremde Antworten folgenlos behandeln; Sprach-Einzellimits schützen. Begrenzter Eingriff mit vollständiger Regressionsmatrix |
| **B – Umsetzung bewusst verschieben** | Produkt unverändert, P1–P4 bleiben offen; Sicherheits-/Zuverlässigkeitsbefunde und fehlende neue Abnahme bleiben sichtbar. Abschluss von 3.2 und Beginn 3.11 warten weiter auf Klärung; spätestens 2026-11-12 erneut entscheiden |

**Empfehlung des Hauptagenten:** A. P1–P3 sind im Handler reproduziert; P4 ist eine bestätigte zusätzliche Empfangslücke, ohne nachgewiesene Laderumgehung. Kein Grund für einen Protokollneuentwurf oder neue globale Sprachlimits.

**Konfidenz:** hoch für die reproduzierten Befunde; mittel bis hoch für die Korrekturrichtung, bis Gegenprüfung und vollständige Matrix vorliegen. **Umkehrbarkeit:** billig bis mittel, weil keine Datenmigration vorgesehen ist; falsche Korrelation kann dennoch laufende Operationen treffen.

## Unabhängige Bewertung

Ein neuer Prüfer mit explizit `gpt-6-astra`, hoher Denktiefe und ohne geerbten Gesprächsverlauf bewertete Quellen und Vorlage ausschließlich lesend. Erster Aufruf scheiterte am gemeldeten Nutzungslimit, ohne Ergebnis. Ein späterer einmaliger erneuter Versuch war verfügbar und lieferte die Bewertung: A empfohlen, P1–P4 statisch bestätigt, keine neue Vertragsversion oder Modulbeziehung nötig. Keine eigenen Tests durch den Prüfer; die dynamischen Nachweise stammen vom Hauptagenten.

Übernommene Präzisierungen: tatsächliche Familie plus gelieferte sichere Nummer; `pendingRequest()` beim echten Exit weiterhin für das Beenden offener Anfragen sinnvoll; keine Wiederverwendung des gegenläufigen `savesRequestNumber` ohne Antwortklassifikation. Zeichen zählen als Codepoints; U+0085 und U+FEFF als gezielte Rust-/JS-Whitespace-Kontrollen vorsehen. Keine Teilübernahme fehlerhafter Sprachlisten und keine neue Leertextregel aus D3. Diese Bewertung ist eine Vorentscheidung, keine Nachprüfung einer noch nicht vorhandenen Implementierung.

Zusätzlicher Befund außerhalb dieses Pakets: `core-host.ts:162–168` übernimmt im Fallback einen mitgelieferten Port unabhängig von der Nachrichtenart. Unabhängige statische Bewertung: mittel, vermutlich Code falsch, mögliche Verbindungsunterbrechung; kein nachgewiesener Ausnutzungspfad, Sender ist der Hauptprozess. Eigener Landeplatz Q.10-N5 mit Frist 2026-11-12 und Gegenproben vor einer getrennten Sicherheitsentscheidung. Nicht still in A eingeschlossen.

Die ausdrückliche Sicherheitsfreigabe bleibt erforderlich. Diese Vorlage ist keine Umsetzung und erledigt weder N2 noch 3.2.

## Dokumentationsnachweise

Produktdiff gegen `468bc32` leer; sämtliche Änderungen dieses Branches sind Markdown einschließlich des übernommenen N3-Abschlussvermerks. SHA-256 der eingebetteten Probe stimmt mit der ausgeführten Quelle überein; die isolierte Main-Kopie entspricht dem Original bis auf den dokumentierten Testzugriff. Dokumentbudgets, lokale Links und ADR-Register/Quote geprüft. Kein neues Tool, keine Installation und keine Quick-Start-Änderung, daher kein frischer Installationsklon für diese Vorlage. Keine neue Anwendungscode-Coverage behauptet; konkrete Coverage ist Bestandteil der späteren Korrekturabnahme.

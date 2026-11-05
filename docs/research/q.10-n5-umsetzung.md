# Q.10-N5 – Umsetzung der freigegebenen Portvalidierung

**Stand:** 2026-11-05, Option A ausdrücklich freigegeben, ADR-053 auf `13e5cee` vor Produktcode committed. Umsetzung auf `c295ebb` committed/gepusht, lokal und unabhängig geprüft; vollständiger Pre-Commit und volle Code-CI/Security grün. N5 erledigt; 3.2 bleibt für seinen Abschlussabgleich offen, keine neue Beförderung.

## Änderung und Nachweisgrenzen

Einziger Produktdiff in `app/src/main/core-host.ts`: unmittelbar vor dem bisherigen Portwechsel wird eine Objektnachricht (nicht null/Array) mit `type === "port"` und genau einem übertragenen Port verlangt. Zusätzliche Felder bleiben erlaubt. Nach erfolgreicher Prüfung bleibt der bestehende Ablauf (alten Port schließen, `openPort`, Nachrichtenhandler, Start) erhalten. Saves-/Intervall-/Paketparser stehen unverändert davor; lesbare ungültige Saves-Anfragen antworten weiter mit `invalidArgument`.

Keine neue Version, Portbesitz-/Cleanup-/Ausnahme-/Logging-/Neustartpolitik, Modulbeziehung oder externe Abhängigkeit. Rust-Kern und -Brücke, CI und Coverage-Konfiguration unverändert. Native Folge des unerlaubten Austauschs war statisch belegt, keine dynamisch belegten Datenverluste oder Renderer-/Mod-Angriffe. Die ursprünglichen roten Proben bleiben mit ihrer Basis und SHA historisch erhalten.

## Tests und Gates

Vor dem ersten Lauf im SESSIONSTART genanntes Testsignal: unerlaubter Austausch oder Nachrichten/Pump am falschen Port macht Rot; Grün belegt gültige Übergabe und erhaltene Fachanfragen. `core-host.test.ts` importiert den tatsächlichen Host-Einstieg und ruft dessen registrierten Listener auf. Bridge, Parent-Port und Ports sind Doubles; Uhr wird kontrolliert. Parser, Priorität und Host-Verdrahtung bleiben real. Kein Produkt-Testexport.

**20 neue Fälle bestanden:** erster/erneuter Port einschließlich Zusatzfeld; Nachrichtenantworten und unterscheidbare Pump-Antworten jeweils am richtigen Port; unbekannt/null/Primitive/Array/leere oder falsche Art/ungültige bekannte Familien; 0/2 Ports; Array mit `type`-Eigenschaft; lesbare ungültige Spielanfrage mit `invalidArgument`; gültige Paket-, Spiel-, Sprach- und Autosave-Anfragen einschließlich Port ohne Austausch. Beim Autosave-Fall wird die tatsächliche Autosave-Meldung nach Zeitfortschritt geprüft. Andere Tests belegen kein reales Electron-Structured-Clone-Verhalten oder native Sitzungswirkung.

Node **26.11.1** aus vorhandener Installation aktiviert. `npm test` mit V8-Coverage: **529 Tests in 42 Dateien bestanden**, Lines **81,41 % (1.985/2.438)**. Der geänderte Host-Einstieg ist bereits seit ADR-018 von numerischer Coverage ausgenommen; keine neue Ausnahme oder veränderte Schwelle. Seine tatsächliche Entscheidung und Verdrahtung werden durch die 20 Handlerfälle und ergänzende Electron-E2E geprüft.

ESLint ohne Warnungen, Prettier, Typprüfung und vollständiger Build bestanden. Erzeugte Typen und Schema stimmen mit Rust überein; Lizenzprüfung 546 Pakete grün. cargo-deny grün; npm-Audit Exit 0 mit unverändertem Bestand 15 moderater Befunde. Kein Quick-Start-/Installationswechsel, daher kein frischer Installationsklon für die lokale Empfangskorrektur; keine Aussage über eine frische Installation. Bestehender prozesslokaler englischer Electron-Launcher unverändert, Systemlokalisierung unverändert.

Reaktiv-Quote 3/10 über ADR-044–053, gesamt 20/53, alte Klassifikationen unverändert. Bestehende Reflexion/Handlermaßnahme aus ADR-051/052 gilt weiter. Ablaufdaten-Register ohne neue fällige Maßnahme; Budgetautomatisierung bleibt Q.10-N1 mit eigener Freigabe. Keine neue Modul- oder Reifegradentscheidung.

Beide bestehenden Electron-E2E bestanden (12,6 s, Exit 0), einschließlich vier Sitzungswechseln und Bauaktion. Neuer unabhängiger Prüfer `gpt-6-astra`, hohe Denktiefe, ohne Gesprächsverlauf, nur lesend: keine Abweichung im begrenzten N5-Umfang; eigener Lauf **20/20** Hosttests grün. Keine eigenen Gates/Remoteprüfung durch den Prüfer (DNS dort nicht verfügbar); Hauptagent hat Remote/PR geprüft. Keine Gesamtabnahme oder neue Beförderung. Geprüfte SHA-256: Host `817e9fb45a959a7183421e954a59909868a6e261bc17937856acb36937041922`, Tests `48d9fceaef7e223fd8094f45f00dc0ae3cc8d56312eee354a66e1f5c41541816`.

[CI 42119228021](https://github.com/PatSchCode/OpenAirside/actions/runs/42119228021) und [Security 42119228025](https://github.com/PatSchCode/OpenAirside/actions/runs/42119228025) auf `c295ebb` vollständig abgeschlossen und grün: Rust-/TS-/Electron-Matrix auf Windows/Linux/macOS, Coverage und Kern-Benchmarks. Gestagter Inhalt vor beiden Checkpoint-Commits geprüft, aktive Hooks bestanden. Kein Produktcode nach unabhängiger Prüfung geändert. Abschlussdokumentation folgt im selben PR #51; späteren Merge-/Main-Nachweis im Logbuch führen.

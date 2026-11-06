# Änderungsprotokoll

Noch keine veröffentlichte Version. Der Projektstand ist v0.0.0.

## Unveröffentlicht

### Korrekturen

- Sichtbare erste Terminallektion in Deutsch/Englisch: Start, Pause/Fortsetzen, reale Rückstaudiagnose, bestätigte schnellere Kontrolle und Abschluss bei vollständigen 40/40-Reisen. Fertige Passagiere verschwinden lokal nachgewiesen aus der Darstellung; gespeicherter Bestand bleibt erhalten (3.2 C3, ADR-054; Gesamtabschluss durch Linux-Pixelabnahme blockiert).
- Terminalfluss mit tatsächlichen Warte-/Bedienplätzen, Eintritts-FIFO, Bedienzeit und vollständiger Vorstau-/Blockierdiagnose nachgebessert (3.2 C1/C2, ADR-054). Nicht kompatibel: Weltformat 6 weist Format 5 mit `migrationFailed` ab; Port 3 ersetzt Durchsatz durch `serviceTicks`, S-1-Layout bleibt 2.
- Ein Stationswechsel verwendet den alten Ankunftszustand nicht mehr für das neue Ziel; abgeschlossene Passagiere bleiben stehen (3.2, A1-Zwischenkorrektur unter ADR-054). Die übrige Terminalnachbesserung bleibt offen.
- Die Kern-Host-Verbindung wird nur durch eine ausdrückliche gültige Portübergabe ersetzt; unbekannte oder ungültige Nachrichten mit angehängten Ports erhalten die bestehende Verbindung (Q.10-N5, ADR-053).
- Unbekannte Kernnachrichten lassen den Kern weiter nutzbar; fehlerhafte Antworten werden nur ihrer tatsächlichen Anfrage zugeordnet und passende Anfragen sofort abgelehnt (Q.10-N2, ADR-052).
- Bestehende Sprach-Einzellimits werden auch im Hauptprozess geprüft; gültige Unicode-Namen und -Texte bleiben erhalten.

- Passagier-FIFO nach Speichern/Laden deterministisch fortsetzen und bei Nummernüberlauf die Bedienreihenfolge erhalten (Q.10-N3, ADR-051).
- Ungültige gespeicherte Passagierzustände und mehrdeutige Stations-/Queue-Zuordnungen vor Übernahme abweisen (Q.10-N3: damals Format 5; durch ADR-054 fortgeschrieben).

### Dokumentation

- Codex-Arbeitsregeln gezielt nach Dev-Templates ergänzt (Q.10, ADR-050).
- Ersten unabhängigen Architektur-Code-Abgleich mit offenen Befunden und
  Folgeschritten dokumentiert; keine neue Spielfunktion oder Abnahme behauptet.
- Dokumentarchive, Verweise und Register bewusster Umgebungsunterschiede gepflegt.

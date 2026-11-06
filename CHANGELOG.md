# Änderungsprotokoll

Noch keine veröffentlichte Version. Der Projektstand ist v0.0.0.

## Unveröffentlicht

### Korrekturen

- Ein Stationswechsel verwendet den alten Ankunftszustand nicht mehr für das neue Ziel; abgeschlossene Passagiere bleiben stehen (3.2, A1-Zwischenkorrektur unter ADR-054). Die übrige Terminalnachbesserung bleibt offen.
- Die Kern-Host-Verbindung wird nur durch eine ausdrückliche gültige Portübergabe ersetzt; unbekannte oder ungültige Nachrichten mit angehängten Ports erhalten die bestehende Verbindung (Q.10-N5, ADR-053).
- Unbekannte Kernnachrichten lassen den Kern weiter nutzbar; fehlerhafte Antworten werden nur ihrer tatsächlichen Anfrage zugeordnet und passende Anfragen sofort abgelehnt (Q.10-N2, ADR-052).
- Bestehende Sprach-Einzellimits werden auch im Hauptprozess geprüft; gültige Unicode-Namen und -Texte bleiben erhalten.

- Passagier-FIFO nach Speichern/Laden deterministisch fortsetzen und bei Nummernüberlauf die Bedienreihenfolge erhalten (Q.10-N3, ADR-051).
- Ungültige gespeicherte Passagierzustände und mehrdeutige Stations-/Queue-Zuordnungen vor Übernahme abweisen; Spielstand-Format bleibt 5.

### Dokumentation

- Codex-Arbeitsregeln gezielt nach Dev-Templates ergänzt (Q.10, ADR-050).
- Ersten unabhängigen Architektur-Code-Abgleich mit offenen Befunden und
  Folgeschritten dokumentiert; keine neue Spielfunktion oder Abnahme behauptet.
- Dokumentarchive, Verweise und Register bewusster Umgebungsunterschiede gepflegt.

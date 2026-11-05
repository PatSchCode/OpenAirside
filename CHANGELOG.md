# Änderungsprotokoll

Noch keine veröffentlichte Version. Der Projektstand ist v0.0.0.

## Unveröffentlicht

### Korrekturen

- Unbekannte Kernnachrichten lassen den Kern weiter nutzbar; fehlerhafte Antworten werden nur ihrer tatsächlichen Anfrage zugeordnet und passende Anfragen sofort abgelehnt (Q.10-N2, ADR-052).
- Bestehende Sprach-Einzellimits werden auch im Hauptprozess geprüft; gültige Unicode-Namen und -Texte bleiben erhalten.

- Passagier-FIFO nach Speichern/Laden deterministisch fortsetzen und bei Nummernüberlauf die Bedienreihenfolge erhalten (Q.10-N3, ADR-051).
- Ungültige gespeicherte Passagierzustände und mehrdeutige Stations-/Queue-Zuordnungen vor Übernahme abweisen; Spielstand-Format bleibt 5.

### Dokumentation

- Codex-Arbeitsregeln gezielt nach Dev-Templates ergänzt (Q.10, ADR-050).
- Ersten unabhängigen Architektur-Code-Abgleich mit offenen Befunden und
  Folgeschritten dokumentiert; keine neue Spielfunktion oder Abnahme behauptet.
- Dokumentarchive, Verweise und Register bewusster Umgebungsunterschiede gepflegt.

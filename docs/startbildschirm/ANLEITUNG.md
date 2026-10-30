# Startbildschirm (Fassung 1.7.0)

Beim Öffnen des Dashboards erscheint ein Startbildschirm. Er zeigt Logo und Titel sowie welche Daten in diesem Browser gespeichert sind, und hat die Schaltfläche **„Start“**.

## „Start“ – welche Daten laden?

| Auswahl | Wirkung |
|---|---|
| **Ursprungs-Excel einlesen** | Liest die Maßnahmenliste aus Excel ein. Manuelle Maßnahmen, Bereiche und Zuordnungen bleiben erhalten. |
| **Sicherung laden** | Ersetzt die Daten in diesem Browser durch die Sicherung. Vorher fragt das Dashboard nach, danach gibt es „Rückgängig“. |
| **Veröffentlichten Stand ansehen** | Öffnet `ASI_Stand.xlsx` direkt im Ansichtsmodus (schreibgeschützt). |
| **Mit den gespeicherten Daten weiterarbeiten** | Erscheint nur, wenn schon Daten gespeichert sind. Führt ohne Einlesen zum Dashboard. |

- **Beim ersten Mal** wählen Sie die Datei aus. Edge/Chrome merkt sich die Datei.
- **Ab dann** steht in der Auswahl „Lädt direkt …“, und ein Klick liest die gemerkte Datei ein. Nach einem Neustart von Edge fragt der Browser dabei einmal nach der Leseerlaubnis.
- Wurde die Datei verschoben oder umbenannt, wählen Sie sie neu aus.

**Im Ansichtsmodus** bietet der Startbildschirm „Mit den gespeicherten Daten weiterarbeiten“ und **„Aktuellen Stand laden“**. Kolleginnen und Kollegen holen sich damit mit einem Klick den neuesten veröffentlichten Stand.

## Ein- und ausschalten

- Das Häkchen **„Startbildschirm beim Öffnen anzeigen“** gilt je Browser.
- **„Direkt zum Dashboard“** schließt den Startbildschirm ohne Laden.
- Jederzeit wieder erreichbar über **Export & Druck › „Startbildschirm“**.

## Tests (Chromium)

| Testreihe | Ergebnis |
|---|---|
| Neu: Startbildschirm | 23 von 23 |
| Übernahme und Sicherung | 19 von 19 |
| Zuordnung | 29 von 29 |
| Veröffentlichen und Ansichtsmodus | 32 von 32 |
| Bestehende Funktionen | 63 von 63 |
| Bereiche | 44 von 44 |

Wie bei der Sicherung gilt: Den echten Dateiauswahl-Dialog von Edge und das Wiederlesen einer gemerkten Datei nach einem Neustart von Edge konnte ich hier nur nachbilden bzw. nicht automatisch prüfen.

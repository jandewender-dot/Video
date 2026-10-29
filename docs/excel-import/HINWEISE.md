# Excel-Import: Ihre Arbeitsmappe (Fassung 1.4.0)

Geprüft mit der leeren Vorlage („Kopie von prompt.xlsx“). Die Datei selbst liegt nicht im Repository.

## Was erkannt wird

- **Maßnahmenblatt:** „Maßnahmen“, Kopfzeile in Zeile 1.
- **Spalten:** A Herkunft, B Thema, C Aktion, D Zuständigkeit, E vom, F Zieldatum, G umgesetzt am, H Kommentar, I Status. Alle Überschriften werden exakt erkannt.
- **Status-Auswahlliste:** erledigt, zurückgestellt, abgelehnt, in Bearbeitung. Ein leeres Feld bedeutet „offen“.

## Neu in 1.4.0

**Farblegende wird übersprungen.** Die Legende in den Zeilen 193–196 („grün – erledigt“ usw.) wurde bisher als 4 Maßnahmen eingelesen. Jetzt gilt: Eine Zeile, in der nur Thema = Farbname und Aktion = Statuswort stehen und sonst nichts, wird nicht übernommen. Die Importmeldung nennt diese Zeilen.

**Abkürzungen werden übersprungen.** Die Abkürzungen im Blatt „Übersicht_Meetings“ (Zeilen 40–44, z. B. „BR = Betriebsrat“) wurden als Meetings eingelesen. Jetzt gilt: Eine Zeile mit nur einer Zelle der Form „Kürzel = Bedeutung“ wird nicht übernommen.

**Spalte „Bereich“ (optional).** Wenn Sie in der Maßnahmentabelle eine Spalte mit der Überschrift **„Bereich“** ergänzen (z. B. Spalte J), ordnet das Dashboard jede Excel-Maßnahme dem Bereich zu, dessen Name dort steht:

- Der Name muss genau so geschrieben sein wie in der Bereichsleiste. Groß- und Kleinschreibung spielt keine Rolle.
- Benennen Sie einen Bereich im Dashboard um, muss die Excel-Spalte den neuen Namen tragen.
- Unbekannte Namen erscheinen unter Datenqualität als „Unbekannter Bereich (Excel)“. Die Detailansicht zeigt dann, was in Excel steht.
- Ohne die Spalte erscheinen Excel-Maßnahmen wie bisher nur in der Gesamtübersicht.

**Geändertes Verhalten:** Eine Spalte mit der Überschrift „Bereich“ wurde bisher als „Thema“ gelesen. Jetzt gilt sie als Bereich. „Thema / Bereich“ oder „Themenbereich“ bleiben weiterhin Thema.

## Tests

| Prüfung | Ergebnis |
|---|---|
| Bestehende Funktionen | 63 von 63 |
| Bereiche | 44 von 44 |
| Veröffentlichen und Ansichtsmodus | 32 von 32 |
| Selbstprüfung (inkl. neuem Test für Legenden und Bereichsspalte) | 27 von 27 |
| Ihre Vorlage | 1 Maßnahme (Zeile 8), 0 Schein-Meetings, keine Konsolenfehler |

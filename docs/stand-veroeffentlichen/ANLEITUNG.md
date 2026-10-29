# Stand veröffentlichen und Ansichtsmodus

Gilt für `ASI_Dashboard_Leitstand_Bereiche.html` (Fassung 1.3.0, zur Abnahme). Das freigegebene Original `ASI_Dashboard_Enterprise_FINAL.html` ist unverändert.

## Prinzip

Eine Person pflegt die Maßnahmen, alle anderen sehen den Stand nur an.

- **Pflegende Person:** Sie liest wie bisher die Excel-Datei ein, erfasst manuelle Maßnahmen und benennt Bereiche. Danach wählt sie **Export & Druck › Stand veröffentlichen**.
- Das Dashboard erzeugt daraus die Datei **`ASI_Stand.xlsx`**. Diese Datei wird im gemeinsamen SharePoint-Ordner gespeichert und ersetzt dort die vorhandene.
- **Alle anderen:** Sie öffnen das Dashboard, wählen **Excel aktualisieren** und dann `ASI_Stand.xlsx`.
  - Nach einer Rückfrage wechselt das Dashboard in den **Ansichtsmodus**.
  - Danach heißt die Schaltfläche **Stand laden**.

## Speichern

In Edge öffnet „Speichern …“ den normalen Speichern-Dialog mit dem Vorschlag `ASI_Stand.xlsx`. Dort wählen Sie den synchronisierten SharePoint-Ordner und bestätigen „Ersetzen“.

Wo der Dialog nicht verfügbar ist, wird die Datei heruntergeladen. Sie muss dann von Hand in den Ordner verschoben werden.

## Inhalt von `ASI_Stand.xlsx`

| Blatt | Inhalt |
|---|---|
| Info | Datum der Veröffentlichung, „Gepflegt von“, Excel-Grundlage, Anzahl der Maßnahmen, Hinweis |
| Maßnahmen | Alle Maßnahmen lesbar, mit Bereich und echten Excel-Datumswerten |
| Bereiche | Reihenfolge und Namen der Bereiche |
| `_ASI_Daten` (ausgeblendet) | Die Daten, die das Dashboard tatsächlich liest |

**Wichtig:** Änderungen, die direkt in Excel an `ASI_Stand.xlsx` gemacht werden, übernimmt das Dashboard nicht. Gepflegt wird nur im Dashboard der pflegenden Person.

Wird der ausgeblendete Datenteil verändert, lehnt das Dashboard die Datei mit dem Hinweis „Stand-Datei ist beschädigt“ ab.

## Ansichtsmodus

- Oben erscheint eine dunkle Leiste: „Ansichtsmodus · Stand vom … · gepflegt von …“. Sie bietet „Neuen Stand laden“ und „Ansichtsmodus beenden“.
- Fristen werden immer zum heutigen Datum berechnet. Eine Maßnahme kann also seit der Veröffentlichung überfällig geworden sein.
- Ausgeblendet sind:
  - „Neue Maßnahme“
  - „Bearbeiten“, „Löschen“ und „Erledigt“
  - Umbenennen und Hinzufügen von Bereichen
  - „Stand veröffentlichen“
  - die Taste N
- Sichtbar bleiben alle Auswertungen, Filter, CSV- und Excel-Export sowie der Druck.
- Der geladene Stand bleibt nach einem Neustart des Browsers erhalten.
- Eine normale Excel-Datei wird im Ansichtsmodus abgelehnt.
- **Ansichtsmodus beenden** entfernt den geladenen Stand. Eigene Daten in diesem Browser bleiben davon unberührt.

## Getestet (Chromium, `file://`)

| Testreihe | Ergebnis |
|---|---|
| `e2e.js` (bestehende Funktionen) | 63 von 63 |
| `e2e_areas.js` (Bereiche) | 44 von 44 |
| `e2e_stand.js` (neu: Veröffentlichen und Ansichtsmodus) | 32 von 32 |
| Selbstprüfung | 26 von 26 |

Der echte Speichern-Dialog von Edge in einen OneDrive-synchronisierten SharePoint-Ordner ließ sich hier nicht automatisch testen. Dafür bitte einmal von Hand prüfen.

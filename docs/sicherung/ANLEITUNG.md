# Sicherung und Übernahme (ab Fassung 1.6.0)

## Warum eine Sicherung?

Das Dashboard speichert Ihre Pflegedaten im Browser (Edge) auf Ihrem PC, nicht in der HTML-Datei. Dazu gehören die eingelesene Excel, die manuellen Maßnahmen, die Bereichsnamen und die Zuordnungen.

Die Daten gehen verloren, wenn:
- Browserdaten gelöscht werden,
- Sie den PC wechseln,
- Sie im InPrivate-Fenster arbeiten.

Eine Sicherungsdatei holt alles zurück.

## Rotes Warnfeld „Achtung – … bitte eine neue Sicherungsdatei erstellen!“ (ab 1.8.0)

Sobald im Tool etwas geändert wird, erscheint oben ein rotes Warnfeld. Das gilt für neue Maßnahmen, aber auch für Änderungen, Löschungen, umbenannte Bereiche und Zuordnungen. Bei neuen Maßnahmen nennt es die Anzahl:

> Achtung – Sie haben 2 neue Maßnahmen hinzugefügt. Bitte vor dem Beenden des Tools eine neue Sicherungsdatei erstellen!

- Die Schaltfläche **„Sicherung jetzt erstellen“** im Warnfeld sichert sofort.
- Das Warnfeld verschwindet erst nach einer Sicherung, oder wenn eine Sicherung bzw. ein Stand als eigene Daten übernommen wird.
- Es bleibt auch nach dem Schließen und Neuöffnen des Dashboards bestehen.
- Das reine Einlesen der Ursprungs-Excel löst es nicht aus, denn die Excel-Datei ist ja selbst gespeichert.
- Im Ansichtsmodus erscheint es nicht.

## Sicherung speichern

1. **Export & Druck › „Sicherung speichern“** wählen.
2. Beim ersten Mal öffnet Edge den Speichern-Dialog. Wählen Sie den Ordner selbst, z. B. OneDrive, den synchronisierten SharePoint-Ordner oder ein Netzlaufwerk. Der Name ist frei, vorgeschlagen wird `ASI_Sicherung.xlsx`.
3. **Ab dann** schreibt „Sicherung speichern“ ohne Nachfrage in dieselbe Datei. Das Menü zeigt Dateiname und Zeitpunkt der letzten Sicherung.
4. Mit **„Sicherung an anderem Ort speichern …“** wählen Sie einen neuen Speicherort.

Hinweise:
- Edge zeigt dem Dashboard aus Sicherheitsgründen nur den Dateinamen, nicht den vollständigen Pfad. Das Dashboard merkt sich aber die Datei selbst.
- Nach einem Neustart von Edge fragt der Browser beim ersten Speichern einmal, ob das Dashboard die Datei bearbeiten darf. Bitte mit „Bearbeiten“ bzw. „Zulassen“ bestätigen.
- In Browsern ohne Speichern-Dialog wird die Datei heruntergeladen und muss von Hand verschoben werden.

## Sicherung wiederherstellen

- **Export & Druck › „Sicherung wiederherstellen“** liest die gemerkte Sicherungsdatei. Ist keine gemerkt oder wurde sie verschoben, wählen Sie die Datei aus. Die gewählte Datei merkt sich das Dashboard dann als Sicherungsort.
- Es folgt die Auswahl **„Als eigene Daten übernehmen“** oder „Nur ansehen“.
- Vor dem Ersetzen fragt das Dashboard nach. Direkt danach gibt es „Rückgängig“.

## Veröffentlichten Stand als eigene Daten übernehmen

Wenn die pflegende Person `ASI_Stand.xlsx` über „Excel aktualisieren“ öffnet, fragt das Dashboard:
- **Nur ansehen (Ansichtsmodus):** wie bisher.
- **Als eigene Daten übernehmen:** zum Beispiel an einem neuen PC oder für eine Vertretung, die die Pflege übernimmt.

Jede veröffentlichte `ASI_Stand.xlsx` ist damit zugleich eine Sicherung.

## Tests (Chromium)

| Testreihe | Ergebnis |
|---|---|
| Übernahme, Sicherung und Warnfeld | 25 von 25 |
| Veröffentlichen und Ansichtsmodus | 32 von 32 |
| Zuordnung | 29 von 29 |
| Bestehende Funktionen | 63 von 63 |
| Bereiche | 44 von 44 |
| Selbstprüfung | 28 von 28 |

**Nicht automatisch testbar:**
- Den echten Speichern-Dialog von Edge habe ich simuliert.
- Das Wiederlesen des gemerkten Datei-Verweises nach einem Browser-Neustart bringt den Testbrowser dieser Umgebung zum Absturz, auch ohne Dashboard. Edge sieht dieses Speichern ausdrücklich vor. Das Dashboard liest den Verweis erst beim Klick auf „Sicherung …“, nie beim Start.

Bitte einmal in Edge von Hand prüfen:
1. Sichern.
2. Edge schließen und wieder öffnen.
3. Erneut sichern: Es sollte ohne Speichern-Dialog in dieselbe Datei gehen.
4. Wiederherstellen.

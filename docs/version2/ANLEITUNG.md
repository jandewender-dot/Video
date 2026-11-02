# ASI-Dashboard – Anleitung (ab Version 2.2)

Diese Anleitung ersetzt die früheren Einzelanleitungen (Stand veröffentlichen, Sicherung, Bereiche zuordnen).

## Grundprinzip

- **Das Dashboard ist führend.** Die Excel-Datei wird eingelesen, danach pflegen Sie alles im Dashboard: Termine, Status, Bereich, Priorität, STOP-Art und Wirksamkeit. Die Excel-Datei selbst wird dabei nicht verändert.
- **Eine Person pflegt, alle anderen sehen den Stand.**
- **„Daten › Speichern“** erzeugt `ASI_Stand.xlsx`. Diese Datei ist zugleich Ihre Sicherung und der Stand, den alle anderen sehen.

## Feste und eigene Bereiche, automatische Zuordnung (ab 2.2)

Acht Bereiche sind fest hinterlegt und lassen sich nicht umbenennen. Beim Einlesen der Excel-Datei ordnet das Dashboard jede Maßnahme über die Spalte „Zuständigkeit“ automatisch zu:

| Bereich | Zuständigkeit enthält … |
|---|---|
| Karsten | Karsten oder Zota |
| Maik | Maik oder Hohmann |
| Frank | Frank oder Dannenberg |
| Jörn | Jörn/Joern oder Duee |
| Ingo | Ingo oder Reiß/Reiss |
| Jens-Michael | Jens, Jens-Michael oder Hilmer |
| Gesund nach Hause | Oliver |
| ASI | Jan, Marek oder „ASI“ |

So funktioniert die Erkennung:
- Groß- und Kleinschreibung sowie Umlaute spielen keine Rolle.
- Stehen mehrere Namen da (z. B. „Maik/Jörn“), zählt der zuerst genannte.
- Eine Excel-Spalte „Bereich“ hat Vorrang vor der Zuständigkeit. Eigene Regeln aus „Nach Zuständigkeit zuordnen“ ergänzen die feste Liste, z. B. für „Thorsten“ oder „Alle Betriebsleiter“.

**Eigene Bereiche:**
- Mit **„+“** rechts in der Bereichsleiste legen Sie zusätzliche Bereiche an. Insgesamt sind bis zu 10 Bereiche möglich, also 8 feste und 2 eigene.
- Eigene Bereiche lassen sich umbenennen: Doppelklick, F2 oder Stift-Symbol. Die festen Bereiche nicht.
- Beim Einlesen erhält ein eigener Bereich alle Maßnahmen, deren Zuständigkeit den Bereichsnamen enthält. Beispiel: Bereich „Thorsten“ ← „Thorsten Müller“. Die festen Personen haben dabei Vorrang.
- Bereiche lassen sich nicht löschen.
- Ab dem 9. Bereich wiederholen sich die Farben im Sonar.

Was ohne passende Person bleibt:
- Maßnahmen ohne passende Person bleiben „ohne Bereich“. Die Importmeldung nennt die Zuständigkeiten. Zuordnen lassen sie sich über „Nach Zuständigkeit zuordnen …“ oder einzeln in der Spalte „Bereich“.
- Die **Gesamtübersicht** zeigt immer alle Maßnahmen. Die Bereiche zeigen jeweils ihren Teil.

Daten aus früheren Versionen:
- Frei benannte Bereiche werden beim ersten Öffnen einmalig umgestellt. Das geschieht über den alten Namen (z. B. „Maik Hohmann“ → Maik, „Gesund nach Hause“ → Gesund nach Hause), sonst über die Zuständigkeit.
- „Jens-Michael Hilmer“ wird zu Jens-Michael.
- Andere frei benannte Bereiche ohne Gegenstück entfallen. Ihre Maßnahmen landen unter „ohne Bereich“, sofern die Zuständigkeit keinen bekannten Namen enthält.
- Wer schon Version 2.1 nutzt: Beim ersten Öffnen kommt Jens-Michael hinzu. Bisher nicht zugeordnete Maßnahmen mit „Jens“ oder „Hilmer“ in der Zuständigkeit werden ihm zugeordnet. Bereits zugeordnete Maßnahmen bleiben unverändert.

## Erster Start (pflegende Person)

1. Dashboard öffnen. Auf dem Startbildschirm **„Start“ › „Excel einlesen“** wählen und die Maßnahmenliste auswählen. Das Dashboard merkt sich die Datei.
2. Die Maßnahmen landen automatisch in den festen Bereichen. Für die übrigen:
   - **„Nach Zuständigkeit zuordnen …“** ordnet alle Maßnahmen einer Person auf einmal zu. Die Regel gilt auch für künftig eingelesene Zeilen.
   - Oder einzeln über die Spalte **„Bereich“** in der Maßnahmenliste.
   - Hat die Excel-Tabelle eine Spalte „Bereich“, wird sie beim Einlesen berücksichtigt.
3. **„Fristen nachpflegen …“** (Karte „Fristen fehlen“ auf dem Dashboard): Hier bekommen offene Maßnahmen ohne Zieldatum einen Termin. Jede Eingabe wird sofort gespeichert.
4. **„Daten › Speichern“**: Beim ersten Mal fragt das Dashboard nach „Gepflegt von“, danach öffnet Edge den Speichern-Dialog. Wählen Sie den gemeinsamen SharePoint-Ordner. Ab dann genügt „Daten › Speichern“.

## Tägliche Pflege

- **Neue Maßnahme:** Pflichtfelder sind Aktion, Bereich, Zuständigkeit und Zieldatum.
- **Bearbeiten:** gilt für alle Maßnahmen, auch die aus Excel übernommenen.
- **Wirksamkeitskontrolle** (§ 3 Abs. 1 ArbSchG): Für erledigte Maßnahmen tragen Sie im Formular „Geprüft am“, das Ergebnis (wirksam / teilweise / nicht wirksam) und „Geprüft durch“ ein. Die Kennzahl **„Wirksamkeit offen“** zeigt erledigte Maßnahmen, die noch nicht geprüft sind.
- **Priorität** und **Art der Maßnahme (STOP)** sind optional. Die Rangfolge nach § 4 ArbSchG lautet: personenbezogene Maßnahmen sind nachrangig.
- **Rotes Warnfeld:** Es erscheint, sobald etwas geändert ist und noch nicht gespeichert wurde. **„Jetzt speichern“** behebt das. Beim Schließen des Tabs fragt Edge zusätzlich nach.

## Excel erneut einlesen

Wenn bereits Excel-Maßnahmen im Dashboard sind, fragt das Dashboard nach, wie die Datei übernommen werden soll:

- **Nur neue Zeilen übernehmen (empfohlen):** Ergänzt Zeilen, die es noch nicht gibt. Verglichen wird über Aktion, Herkunft, Thema, Zuständigkeit und „vom“. Alles Bestehende bleibt unverändert.
  - Achtung: Wurde eine Zeile in Excel umformuliert, gilt sie als neu und erscheint zusätzlich.
- **Excel-Maßnahmen vollständig ersetzen:** Im Dashboard vorgenommene Änderungen an diesen Maßnahmen gehen verloren. Im Dashboard erfasste Maßnahmen bleiben erhalten.

## Für die Kolleginnen und Kollegen (ansehen)

- Dashboard öffnen, dann **„Start“ › „Gespeicherten Stand nur ansehen“** und `ASI_Stand.xlsx` aus dem gemeinsamen Ordner wählen. Das Dashboard merkt sich die Datei, beim nächsten Mal heißt die Wahl **„Aktuellen Stand laden“**.
- Auf dem Startbildschirm unter **„Beim Öffnen anzeigen (Mein Bereich)“** den eigenen Bereich wählen. Das Dashboard öffnet dann direkt dort.
- Im Ansichtsmodus lässt sich nichts ändern. Die Datenqualität ist ausgeblendet.

## Neuer PC, gelöschte Browserdaten, Vertretung

**„Start“ › „Gespeicherten Stand übernehmen“** bzw. **„Daten › Gespeicherten Stand laden“**: `ASI_Stand.xlsx` wird zu den eigenen Pflegedaten. Vorher fragt das Dashboard nach, danach gibt es „Rückgängig“.

## Umstellung von Version 1.x

Beim ersten Öffnen werden vorhandene Daten automatisch umgestellt: Die Excel-Maßnahmen samt Bereichszuordnung landen in der bearbeitbaren Liste. Danach bitte einmal **„Daten › Speichern“** wählen. Auch ältere `ASI_Stand.xlsx`- und Sicherungsdateien lassen sich weiterhin laden.

## Was entfallen ist

- Kanban, Ideenspeicher, Jahresreflexion und Meetingübersicht. Diese Blätter stehen weiterhin in der Excel-Datei.
- Das Sonar ist standardmäßig ausgeblendet („Sonar einblenden“ unter dem Lagebild).
- „Sicherung“ und „Stand veröffentlichen“ sind zu **„Daten › Speichern“** zusammengelegt.

## Hinweise und Grenzen

- Die Daten liegen im Browser (Edge) des jeweiligen PCs. Das Dashboard bittet Edge, sie dauerhaft zu speichern. Ob das gewährt wurde, steht unter Datenqualität › Import. Unabhängig davon gilt: **regelmäßig speichern.**
- Das Merken von Dateien über einen Neustart von Edge hinweg ist eine Edge-Funktion. Ich konnte sie hier nicht automatisch testen, bitte einmal von Hand prüfen.

## Tests (Chromium)

| Testreihe | Ergebnis |
|---|---|
| Version 2 (Umstellung inkl. feste Bereiche, Bearbeiten, Fristen, Wirksamkeit, Einlesen) | 24 von 24 |
| Bestehende Funktionen | 60 von 60 |
| Bereiche (fest und eigene, automatische Zuordnung) | 47 von 47 |
| Speichern und Ansichtsmodus | 32 von 32 |
| Zuordnung | 27 von 27 |
| Speichern, Übernahme und Warnfeld | 28 von 28 |
| Startbildschirm | 28 von 28 |
| Selbstprüfung | 31 von 31 |

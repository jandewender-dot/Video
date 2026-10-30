# ASI-Dashboard – Anleitung (Version 2.0)

Diese Anleitung ersetzt die früheren Einzelanleitungen (Stand veröffentlichen, Sicherung, Bereiche zuordnen).

## Grundprinzip

- **Das Dashboard ist führend.** Die Excel-Datei wird eingelesen, danach pflegen Sie alles im Dashboard: Termine, Status, Bereich, Priorität, STOP-Art und Wirksamkeit. Die Excel-Datei selbst wird dabei nicht verändert.
- **Eine Person pflegt, alle anderen sehen den Stand.**
- **„Daten › Speichern“** erzeugt `ASI_Stand.xlsx`. Diese Datei ist zugleich Ihre Sicherung und der Stand, den alle anderen sehen.

## Erster Start (pflegende Person)

1. Dashboard öffnen. Auf dem Startbildschirm **„Start“ › „Excel einlesen“** wählen und die Maßnahmenliste auswählen. Das Dashboard merkt sich die Datei.
2. Bereiche benennen: Doppelklick auf einen Reiter oder Stift-Symbol rechts.
3. Maßnahmen zuordnen:
   - **„Nach Zuständigkeit zuordnen …“** ordnet alle Maßnahmen einer Person auf einmal zu. Vorschläge aus den Bereichsnamen sind vorausgewählt, zum Beispiel „Maik“ → „Maik Hohmann“. Die Regel gilt auch für künftig eingelesene Zeilen.
   - Oder einzeln über die Spalte **„Bereich“** in der Maßnahmenliste.
   - Hat die Excel-Tabelle eine Spalte „Bereich“, wird sie beim Einlesen berücksichtigt.
4. **„Fristen nachpflegen …“** (Karte „Fristen fehlen“ auf dem Dashboard): Hier bekommen offene Maßnahmen ohne Zieldatum einen Termin. Jede Eingabe wird sofort gespeichert.
5. **„Daten › Speichern“**: Beim ersten Mal fragt das Dashboard nach „Gepflegt von“, danach öffnet Edge den Speichern-Dialog. Wählen Sie den gemeinsamen SharePoint-Ordner. Ab dann genügt „Daten › Speichern“.

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
| Neu: Version 2 (Umstellung, Bearbeiten, Fristen, Wirksamkeit, Einlesen) | 21 von 21 |
| Bestehende Funktionen | 60 von 60 |
| Bereiche | 45 von 45 |
| Speichern und Ansichtsmodus | 32 von 32 |
| Zuordnung | 27 von 27 |
| Speichern, Übernahme und Warnfeld | 28 von 28 |
| Startbildschirm | 28 von 28 |
| Selbstprüfung | 29 von 29 |

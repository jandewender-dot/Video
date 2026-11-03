# ASI-Dashboard – Anleitung (ab Version 2.5)

Diese Anleitung ersetzt die früheren Einzelanleitungen (Stand veröffentlichen, Sicherung, Bereiche zuordnen).

## Gemeinsamer Ordner

Die HTML-Datei und `ASI_Stand.xlsx` liegen im SharePoint-Ordner **ASI Team HOL / NÖ / BS › Dokumente › General › Gesund nach Hause › Dashboard**.

- **Jede Person** holt diesen Ordner einmal über **„Synchronisieren“** in den Windows-Explorer. „Daten › SharePoint-Ordner öffnen“ (oder der Link auf dem Startbildschirm) öffnet ihn im Browser.
- **Die HTML-Datei immer aus dem Explorer öffnen** (Doppelklick), nicht in der SharePoint-Webansicht. In der Webansicht kann das Dashboard nichts speichern; dann zeigt das rote Warndreieck in der Bereichsleiste den Hinweis „In dieser Ansicht wird nichts gespeichert“.
  - Woran man die Webansicht erkennt: Die Adresszeile beginnt mit `https://symriseag.sharepoint.com/…`, oben steht der Dateiname mit „Öffnen“ und „Teilen“. Aus dem Explorer geöffnet zeigt die Adresszeile stattdessen einen Pfad auf dem PC (z. B. `C:/Users/…`).
  - In der Webansicht sperrt Edge außerdem das Absenden von Formularen und je nach Einstellung Downloads. Bis 2.5.0 reagierte deshalb „Speichern“ im Formular „Neue Maßnahme“ dort nicht; ab 2.5.1 speichert es, die Maßnahme ist aber nach dem Schließen weg. „Daten › Speichern“ weist dort darauf hin, dass der Download gesperrt sein kann.
- Einen festen Speicherpfad kann eine HTML-Datei nicht vorgeben: Edge verlangt, dass der Ort einmal im Speichern- bzw. Öffnen-Dialog gewählt wird. Danach merkt sich das Dashboard die Datei.
- In die SharePoint-Webadresse selbst kann das Dashboard nicht schreiben, nur in den synchronisierten Ordner. OneDrive lädt die Datei von dort hoch.

## Grundprinzip

- **Das Dashboard ist führend.** Die Excel-Datei wird eingelesen, danach pflegen Sie alles im Dashboard: Termine, Status, Bereich, Priorität, STOP-Art und Wirksamkeit. Die Excel-Datei selbst wird dabei nicht verändert.
- **Eine Person pflegt, alle anderen sehen den Stand.**
- **„Daten › Speichern“** erzeugt `ASI_Stand.xlsx`. Diese Datei ist zugleich Ihre Sicherung und der Stand, den alle anderen sehen.

## Feste und eigene Bereiche, automatische Zuordnung (ab 2.2)

Acht Bereiche sind fest hinterlegt und lassen sich weder umbenennen noch löschen. Beim Einlesen der Excel-Datei ordnet das Dashboard jede Maßnahme über die Spalte „Zuständigkeit“ automatisch zu:

| Bereich | Zuständigkeit enthält … |
|---|---|
| Karsten | Karsten oder Zota |
| Maik | Maik oder Hohmann |
| Frank | Frank oder Dannenberg |
| Jörn | Jörn/Joern oder Duee |
| Ingo | Ingo oder Reiß/Reiss |
| Jens-Michael | Jens, Jens-Michael oder Hilmer |
| Gesund nach Hause | Oliver, Johanna oder Halacz, Heiko oder Völtzke (ab 2.4) |
| ASI | Jan, Marek oder „ASI“ |

So funktioniert die Erkennung:
- Groß- und Kleinschreibung sowie Umlaute spielen keine Rolle.
- Stehen mehrere Namen da (z. B. „Maik/Jörn“), zählt der zuerst genannte.
- Eine Excel-Spalte „Bereich“ hat Vorrang vor der Zuständigkeit. Eigene Regeln aus „Nach Zuständigkeit zuordnen“ ergänzen die feste Liste, z. B. für „Thorsten“ oder „Alle Betriebsleiter“.

**Eigene Bereiche:**
- Mit **„+“** rechts in der Bereichsleiste legen Sie zusätzliche Bereiche an. Insgesamt sind bis zu 10 Bereiche möglich, also 8 feste und 2 eigene.
- Eigene Bereiche lassen sich umbenennen: Doppelklick, F2 oder Stift-Symbol. Die festen Bereiche nicht.
- Beim Einlesen erhält ein eigener Bereich alle Maßnahmen, deren Zuständigkeit den Bereichsnamen enthält. Beispiel: Bereich „Thorsten“ ← „Thorsten Müller“. Die festen Personen haben dabei Vorrang.
- **Eigene Bereiche löschen (ab 2.3):** Stift-Symbol rechts in der Bereichsleiste (oder im Bereich „Bereich umbenennen oder löschen“), dann das Papierkorb-Symbol neben dem Namen.
  - Vorher fragt das Dashboard nach und nennt die Zahl der betroffenen Maßnahmen.
  - Die Maßnahmen bleiben erhalten und stehen danach auf **„ohne Bereich“**. Von dort lassen sie sich neu zuordnen.
  - Eine Regel aus „Nach Zuständigkeit zuordnen“, die auf den gelöschten Bereich zeigt, wird mit entfernt.
  - Direkt danach können Sie das Löschen mit **„Rückgängig“** zurücknehmen.
  - Ein neu angelegter Bereich bekommt eine neue Kennung; er übernimmt keine Maßnahmen des gelöschten Bereichs.
  - Wie jede Änderung gilt das Löschen für alle anderen erst nach „Daten › Speichern“.
- Ab dem 9. Bereich wiederholen sich die Farben im Sonar.

Was ohne passende Person bleibt:
- Maßnahmen ohne passende Person bleiben „ohne Bereich“. Die Importmeldung nennt die Zuständigkeiten. Zuordnen lassen sie sich über „Nach Zuständigkeit zuordnen …“ oder einzeln in der Spalte „Bereich“.
- Die **Gesamtübersicht** zeigt immer alle Maßnahmen. Die Bereiche zeigen jeweils ihren Teil.

Daten aus früheren Versionen:
- Frei benannte Bereiche werden beim ersten Öffnen einmalig umgestellt. Das geschieht über den alten Namen (z. B. „Maik Hohmann“ → Maik, „Gesund nach Hause“ → Gesund nach Hause), sonst über die Zuständigkeit.
- „Jens-Michael Hilmer“ wird zu Jens-Michael.
- Andere frei benannte Bereiche ohne Gegenstück entfallen. Ihre Maßnahmen landen unter „ohne Bereich“, sofern die Zuständigkeit keinen bekannten Namen enthält.
- Wer schon Version 2.3 oder älter nutzt und Maßnahmen von Johanna Halacz oder Heiko Völtzke „ohne Bereich“ hat: Die Karte „Maßnahmen ohne Bereich“ bietet **„… automatisch zuordnen“**. Bereits zugeordnete Maßnahmen bleiben unverändert.
- Wer schon Version 2.1 nutzt: Beim ersten Öffnen kommt Jens-Michael hinzu. Bisher nicht zugeordnete Maßnahmen mit „Jens“ oder „Hilmer“ in der Zuständigkeit werden ihm zugeordnet. Bereits zugeordnete Maßnahmen bleiben unverändert.

## Erster Start (pflegende Person)

1. Dashboard öffnen. Auf dem Startbildschirm **„Start“ › „Excel einlesen“** wählen und die Maßnahmenliste auswählen. Das Dashboard merkt sich die Datei.
2. Die Maßnahmen landen automatisch in den festen Bereichen. Für die übrigen:
   - **„Nach Zuständigkeit zuordnen …“** ordnet alle Maßnahmen einer Person auf einmal zu. Die Regel gilt auch für künftig eingelesene Zeilen.
   - Oder einzeln über die Spalte **„Bereich“** in der Maßnahmenliste.
   - Hat die Excel-Tabelle eine Spalte „Bereich“, wird sie beim Einlesen berücksichtigt.
3. **„Fristen nachpflegen …“** (Karte „Fristen fehlen“ auf dem Dashboard): Hier bekommen offene Maßnahmen ohne Zieldatum einen Termin. Jede Eingabe wird sofort gespeichert.
4. **„Daten › Speichern“**: Beim ersten Mal fragt das Dashboard nach „Gepflegt von“, danach öffnet Edge den Speichern-Dialog. Wählen Sie den synchronisierten Ordner „Dashboard“ (siehe „Gemeinsamer Ordner“). Ab dann genügt „Daten › Speichern“.

## Tägliche Pflege

- **Neue Maßnahme:** Pflichtfelder sind Aktion, Bereich, Zuständigkeit und Zieldatum.
- **Bearbeiten:** gilt für alle Maßnahmen, auch die aus Excel übernommenen.
- **Wirksamkeitskontrolle** (§ 3 Abs. 1 ArbSchG): Für erledigte Maßnahmen tragen Sie im Formular „Geprüft am“, das Ergebnis (wirksam / teilweise / nicht wirksam) und „Geprüft durch“ ein. Die Kennzahl **„Wirksamkeit offen“** zeigt erledigte Maßnahmen, die noch nicht geprüft sind.
- **Priorität** und **Art der Maßnahme (STOP)** sind optional. Die Rangfolge nach § 4 ArbSchG lautet: personenbezogene Maßnahmen sind nachrangig.
- **Rotes Warndreieck (ab 2.5):** Rechts in der Bereichsleiste, neben „+“ und Stift. Es erscheint, sobald etwas geändert und noch nicht gespeichert ist oder in der Ansicht nichts gespeichert werden kann. Die Zahl nennt die Anzahl der Hinweise; ein neuer Hinweis lässt das Dreieck kurz pulsieren.
  - Die roten Hinweise sind **standardmäßig eingeklappt**. Ein Klick auf das Dreieck klappt sie auf, ein zweiter wieder zu. Beim nächsten Öffnen sind sie wieder eingeklappt.
  - Im aufgeklappten Hinweis speichert **„Jetzt speichern“**. Alternativ „Daten › Speichern“. Beim Schließen des Tabs fragt Edge zusätzlich nach, solange nicht gespeichert ist.

## Excel erneut einlesen

Wenn bereits Excel-Maßnahmen im Dashboard sind, fragt das Dashboard nach, wie die Datei übernommen werden soll:

- **Nur neue Zeilen übernehmen (empfohlen):** Ergänzt Zeilen, die es noch nicht gibt. Verglichen wird über Aktion, Herkunft, Thema, Zuständigkeit und „vom“. Alles Bestehende bleibt unverändert.
  - Achtung: Wurde eine Zeile in Excel umformuliert, gilt sie als neu und erscheint zusätzlich.
- **Excel-Maßnahmen vollständig ersetzen:** Im Dashboard vorgenommene Änderungen an diesen Maßnahmen gehen verloren. Im Dashboard erfasste Maßnahmen bleiben erhalten.

## Für die Kolleginnen und Kollegen (ansehen)

- Dashboard aus dem synchronisierten Ordner „Dashboard“ öffnen, dann **„Start“ › „Gespeicherten Stand nur ansehen“** und `ASI_Stand.xlsx` aus diesem Ordner wählen. Das Dashboard merkt sich die Datei, beim nächsten Mal heißt die Wahl **„Aktuellen Stand laden“**.
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
| Bereiche (fest und eigene, automatische Zuordnung, Löschen eigener Bereiche) | 57 von 57 |
| Speichern und Ansichtsmodus | 32 von 32 |
| Zuordnung | 27 von 27 |
| Speichern, Übernahme und Warnhinweise | 30 von 30 |
| Startbildschirm | 28 von 28 |
| Version 2.4/2.5 (Halacz/Völtzke, Warndreieck und Warnhinweise, SharePoint-Ordner öffnen) | 18 von 18 |
| Abgeschirmte Ansicht (wie SharePoint-Webansicht): Speichern im Formular, Enter, Hinweis beim Speichern | 14 von 14 |
| Selbstprüfung | 31 von 31 |

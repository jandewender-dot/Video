# Excel-Maßnahmen Bereichen zuordnen, Sonar ausblenden (Fassung 1.5.0)

## Nach Zuständigkeit zuordnen (für viele Maßnahmen auf einmal)

1. In der Gesamtübersicht zeigt die Karte „Maßnahmen ohne Bereich“ die Schaltfläche **„Nach Zuständigkeit zuordnen …“**. Alternativ: Stift-Symbol rechts in der Bereichsleiste › **„Excel-Maßnahmen zuordnen …“**.
2. Der Dialog listet jede Zuständigkeit aus der Excel-Datei mit der Anzahl ihrer Maßnahmen. Für jede wählen Sie einen Bereich.
3. **Vorschläge:** Entspricht die Zuständigkeit eindeutig einem Bereichsnamen oder dessen erstem Wort, ist der Bereich schon vorausgewählt und als „Vorschlag“ markiert. Beispiel: „Maik“ → „Maik Hohmann“. Bitte prüfen.
4. Mit **„Übernehmen“** speichern. Eine Meldung bietet „Rückgängig“ an.

Die Regel gilt dauerhaft, auch nach „Excel aktualisieren“ und für neue Zeilen derselben Zuständigkeit.

## Einzeln zuordnen

In den Details einer Excel-Maßnahme gibt es den Abschnitt **„Bereich zuordnen“**:

- **Automatisch:** Es gilt die Regel nach Zuständigkeit bzw. die Excel-Spalte „Bereich“. Die Auswahl zeigt, was dabei herauskommt.
- **Ein bestimmter Bereich:** gilt nur für diese Maßnahme.
- **Keinem Bereich zuordnen:** nimmt die Maßnahme bewusst aus jedem Bereich heraus.

Gleichlautende Einträge, also gleiche Aktion, Herkunft, Thema, Zuständigkeit und „vom“, werden gemeinsam zugeordnet.

**Vorrang:** einzeln zugeordnet › Excel-Spalte „Bereich“ › Regel nach Zuständigkeit.

**Wichtig:** Eine Einzelzuordnung bleibt nur erhalten, solange Aktion, Herkunft, Thema, Zuständigkeit und „vom“ der Zeile in Excel gleich bleiben. Ändert sich einer dieser Werte, gilt wieder „Automatisch“.

Die Zuordnungen werden im Browser der pflegenden Person gespeichert und mit „Stand veröffentlichen“ an alle weitergegeben. Im Ansichtsmodus lassen sie sich nicht ändern.

## Sonar ausblenden

In der Gesamtübersicht unter dem Lagebild: **„Sonar ausblenden“** bzw. **„Sonar einblenden“**.

- Die Einstellung merkt sich jeder Browser für sich.
- Die Rangliste „Offene Maßnahmen je Bereich“ bleibt sichtbar.

## Tests (Chromium)

| Testreihe | Ergebnis |
|---|---|
| Bestehende Funktionen | 63 von 63 |
| Bereiche | 44 von 44 |
| Veröffentlichen und Ansichtsmodus | 32 von 32 |
| Neu: Zuordnung und Sonar | 23 von 23 |
| Selbstprüfung | 28 von 28 |

> **Hinweis:** Gilt für Version 1.x. Für Version 2.0 siehe `docs/version2/ANLEITUNG.md`.

# Excel-Maßnahmen Bereichen zuordnen, Sonar ausblenden (ab Fassung 1.5.1)

## Nach Zuständigkeit zuordnen (für viele Maßnahmen auf einmal)

1. In der Gesamtübersicht zeigt die Karte „Maßnahmen ohne Bereich“ die Schaltfläche **„Nach Zuständigkeit zuordnen …“**. Alternativ: Stift-Symbol rechts in der Bereichsleiste › **„Excel-Maßnahmen zuordnen …“**.
2. Der Dialog listet jede Zuständigkeit aus der Excel-Datei mit der Anzahl ihrer Maßnahmen. Für jede wählen Sie einen Bereich.
3. **Vorschläge:** Entspricht die Zuständigkeit eindeutig einem Bereichsnamen oder dessen erstem Wort, ist der Bereich schon vorausgewählt und als „Vorschlag“ markiert. Beispiel: „Maik“ → „Maik Hohmann“. Bitte prüfen.
4. Mit **„Übernehmen“** speichern. Eine Meldung bietet „Rückgängig“ an.

Die Regel gilt dauerhaft, auch nach „Excel aktualisieren“ und für neue Zeilen derselben Zuständigkeit.

## Direkt in der Tabelle (ab 1.5.1)

In der Ansicht **Maßnahmen** hat jede Zeile in der Spalte **„Bereich“** eine Auswahl. Wählen Sie dort den Bereich, und die Maßnahme erscheint sofort in diesem Bereich.

- „ohne Bereich“ nimmt die Maßnahme wieder heraus.
- Eine Meldung bietet „Rückgängig“ an.
- Das funktioniert für Excel-Maßnahmen und für manuell erfasste Maßnahmen.
- Bei Excel-Maßnahmen gilt die Auswahl wie eine Einzelzuordnung (siehe unten).
- Tipp: Erst über den Filter „Zuständigkeit“ eingrenzen, dann Zeile für Zeile zuordnen.

## Einzeln zuordnen (in den Details)

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
| Zuordnung, Tabelle und Sonar | 29 von 29 |
| Selbstprüfung | 28 von 28 |

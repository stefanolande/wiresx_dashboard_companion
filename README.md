# Wires-X Dashboard Companion
![Description](./images/description.svg)


Wires-X Dashboard Companion is a software that work together with a Wires-X node and [Wires-X Server Dashboard](https://www.grupporadiofirenze.net/2019/03/23/una-semplice-dashboard-per-nodi-wires-x-yaesu/).


## Italiano

Wires-X Dashboard Companion è un programma che lavora insieme ad un nodo Wires-X e [Wires-X Server Dashboard](https://www.grupporadiofirenze.net/2019/03/23/una-semplice-dashboard-per-nodi-wires-x-yaesu/).
Lo scopo principale è mostrare nella dashboard un elenco dei passaggi in stile XLX, compresi tutti i passaggi provenienti dall'interlink con Brandmeister.
A causa del design del nodo Wires-X, senza il programma, viene mostrato solo l'ultimo passaggio proveniente dall'interlink a prescindere dal nominativo.

Wires-X Dashboard Companion funziona in questo modo:
- legge il file `WiresAccess.log`, dove il nodo Wires-X tiene traccia dei QSO
- scrive un file `WiresAccess-mod.log` (nome configurabile), che sarà utilizzato da Wires-X Server Dashboard per mostrare i QSO

### Installazione

- Scaricare il file wiresx-dashboard-companion.zip
- Decomprimerlo in una cartella a piacere
- Modificare nel file `conf.toml` la riga `wires_x_log`, indicando la cartella contenente il vostro file `WiresAccess.log`
- Modificare nel file `conf.toml` la riga `write_log`, indicando il nome del file e la cartella dove salvare il file di log modificato (consiglio di chiamare il file `WiresAccess-mod.log` e di utilizzare la stessa cartella del file `WiresAccess.log`)
- Modificare la riga 13 file `index.php` di Wires-X Dashboard, indicando il nome e percorso del file modificato (se si è usato lo standard basta sostituire `WiresAccess.log` con `WiresAccess-mod.log`)  
- Impostare Windows per avviare il programma in automatico (opzionale)
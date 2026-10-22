# Verbale sedia KLABNIK — S-186 (bozza indipendente, FASE 1)
Generato con assistenza AI (Claude Fable 5).

## VERDETTI
Q1 **AGGIRATO** (forma onorata: kill eseguito, delibera utente scritta una volta; sostanza aggirata: R4 «tappa sul reale» divenuta R1 sotto altro nome, record perso alla rotazione).
Q2 **B GO** — D obbligatoria in riga 1 del NEXT come dichiarazione.
Q3 **C3**.

## ROTTA (una riga)
Prima la riga 1 del NEXT riscritta e firmata dall'utente (1× confermato o D), poi la fetta 0 di TEMPO sull'ORM; nessun codice prima.

## REFUTAZIONI ED EMENDAMENTI
1. Il record esisteva: NEXT alla chiusura S-170 (commit 075b6c63, riga 2) «DELIBERA R4 dell'utente = (i) campagna "corpo del handler"». La riscrittura S-171 (7fbe896f) non contiene più R4, R1 né kill; riga 1 «PARITÀ (≥1×)» identica. Vizio di FORMA (rotazione senza memoria delle delibere) che produce la sostanza: 1× mai ridiscusso.
2. Dossier S-167: R4 = «workload reali, 1× non-obiettivo»; S-170 esegue «(i)». Lecito, ma mancava «R1 continua per decisione utente, 1× confermato».
3. Dissenso S-167 («3 + concilio»): mai testato (una caduta, L-CM1) e fuori bersaglio: ha morso il kill di campagna, a filo (0,08 ns); il difetto era l'assenza di obbligo di registrazione.
4. Incidenti: S-184 bis ne aggiunge 3 (WP_SESSION_184 «Incidenti bis») ⇒ S-184 = 7, totale ≈29.
5. «Sessioni senza misura: 1» SMENTITA: S-178 riga 5 «Nessuna misura temporale valida» ⇒ 2.
6. Timebox violata in S-180 (≈80 % token) e S-182; S-183 sono incidenti. Il veto S-182 «senza parola dell'utente» ha emendato la regola 7 PERMANENTE senza dichiararlo.
7. Dente LOC: 11 rialzi confermati (Σ +736), 2 fuori arco ⇒ 9 in 18 sessioni; «DICHIARANDO» è boilerplate.
8. NEXT: 25 righe / 23.183 caratteri, riga massima 3.357: §8 rispettato alla lettera, vuoto.

## PROPOSTA P (una regola, al posto di §8 ROTAZIONE, assorbendola)
**8. REGISTRO**: NEXT ≤80 righe E ≤8.000 caratteri, riga ≤600 (script in chiusura, rc gate); riga 1 = obiettivo + tappa + «delibera S-N, data» dell'ultima conferma dell'utente; ogni kill che scatta va in riga 1 nella STESSA sessione, fino al concilio seguente; apparato oltre ½ sessione = incidente anche con parola dell'utente; cap LOC sale SOLO nel commit di promozione rc=0; veti più vecchi di 3 sessioni migrano in gaps/VETI.md. Session ≤40 e scoreboard restano.

## KILL-SWITCH PRE-REGISTRABILI
- KS-K186-1: kill scattato assente dalla riga 1 in chiusura ⇒ incidente, stop al codice finché scritto.
- KS-K186-2 (B): chiusura <85 % o nessun canale ≥1 s/37 s ⇒ B morta, D in riga 1 nella stessa sessione.
- KS-K186-3: 2 sessioni su 5 con apparato >½ o incidenti ≥4 ⇒ revisione a lente processo, voce al concilio.

## PRIMA FETTA
Apertura S-186: (1) riga 1 del NEXT con la delibera esplicita dell'utente; (2) `scripts/next-budget.sh` (giudice wc, soglie 80/8.000/600 + «delibera S-», R=1) in chiudi-sessione; (3) NEXT tagliato nello stesso commit. rc=0 = promossa.

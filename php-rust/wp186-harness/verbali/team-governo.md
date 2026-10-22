# TEAM-GOVERNO (Klabnik · Hoare · Pedersen) — riconciliazione S-186 fase 2
Generato con assistenza AI (Claude Fable 5). Fonti vincolanti: `verbali/{klabnik,hoare,pedersen}.md`.

## CONVERGENZE (3/3)
Q2 **B GO-CONDIZIONATO** (fetta 0 = sola misura di TEMPO), **D SCRITTA**; Q3 **C3**, invariante nel RULEBOOK (Hoare E5). `gc_sweep_after_ret` (mod.rs 4160-4188) scandisce `gc_buf` a ogni Ret con oggetti vivi (Hoare E3 = Pedersen 4): prezzatura a buffer PIENO prima del pin s184. Nessun codice prima della riga 1 riscritta.

## Q1: IL VERDETTO E LA SUA SCRITTURA
**Verdetto**: kill S-168 ESEGUITO nella forma (zero leve, delibera utente S-169/170), AGGIRATO nella sostanza: R4 «tappa sul reale, 1× non-obiettivo» mai scritta, delibera sparita alla riscrittura S-171 (NEXT 075b6c63 r.2 → 7fbe896f: zero «R4», verificato con `git show`).

**Riga 1 di NEXT_SESSION_WORDPRESS.md** (374 caratteri):
`# NEXT_SESSION — phpr: ORIZZONTE parità (≥1×) con l'oracle · TAPPA D (delibera utente S-186, 2026-10-22, verifica al concilio ~S-192): WP ≤1,5 (coppia notturna, mediana) · ORM ≤5 (coppia E5, al netto del pavimento sqlite3 comune) · micro ≤3× 6/6 · KILL A REGISTRO: S-168 Σ mock 9,92<10 ⇒ R1 chiusa; R4 eseguita come «corpo del handler» (S-170) SENZA tappa reale, sanata qui.`

**REGOLE §1 (riga 3)**: «orizzonte parità (≥1×); tappa D (delibera utente S-186, 22/10): WP ≤1,5 · ORM ≤5 al netto del pavimento comune · micro ≤3× 6/6» + regola 2.

## LA FORMA — 3 REGOLE VINCOLANTI (cap 25: una sostituzione, due emende)
1. **REGISTRO** (SOSTITUISCE §8, riga 10): NEXT ≤80 righe E ≤8.000 caratteri, riga ≤600, giudice `scripts/next-budget.sh` rc gate in chiudi-sessione; riga 1 = orizzonte + tappa + «delibera S-N, data»; **kill scattato in riga 1 nella STESSA sessione, per nome, fino al concilio seguente; riscrittura che lo cancella = incidente**; **cap `loc_dente` sale SOLO nel commit di promozione rc=0, cifra nel `.out`**; veti >3 sessioni in `gaps/VETI.md`. Session ≤40 e scoreboard restano.
2. **TIMEBOX** (EMENDA §1, riga 3): «apparato oltre ½ sessione = incidente, anche con parola dell'utente»; abrogata la deroga di NEXT r.23 (S-182); S-180/182/183 contati.
3. **GATE AUDITABILE** (EMENDA §5 riga 7, §10 riga 12): fail-set ×2 e `golden-content-{off,on}.tsv` sono GATE, non output ⇒ in git (`wp109-harness/.gitignore` r.9 emendato; §5 «1412»→«1409»); il registro `s186-corpus-emenda-2026-10-22.out` da solo è gate VOID (KS-PE-186-1). Golden ri-basato SOLO con voce di catalogo PRIMA e verso dichiarato nel verdetto di promozione, altrimenti STOP (KS-PE-186-2). **fxmc2**: `invar` → `diff(oracle,pin) == .expected-diff` committato, oracle fisso, ogni slittamento ri-dichiarato nel commit di promozione.

## PROMOZIONE s184 — ORDINE OBBLIGATO
(0) riga 1 firmata dall'utente · (1) regola 3 eseguita · (2) guardia a buffer PIENO: driver Hoare E3 (N=10⁵/2·10⁵, kill ratio >2,5) + ret-buf-dq K∈{0,4,16} + micro R=5 + mcalls-dq/calls-dq tree vs s183, |D| < max(rumore, 4 ns/iter) · (3) **cura O(1) (E-LE-2) PRIMA della promozione SOLO se (2) morde**, criterio proprio, guardia ri-eseguita; altrimenti s184 com'è e O(1) leva per nome DOPO la fetta 0 · (4) pin con ricetta, coppia notturna.

## CONFLITTI REGISTRATI
- **Hoare**: Q1 «ONORATO nella forma» resta col suo nome; accetta verdetto a due membri e riga 1. E1: bug47771 è ctor-failed, non timing: riga del registro `.out` da emendare «fuori famiglia §3.35».
- **Klabnik**, **Pedersen**: nessun dissenso.

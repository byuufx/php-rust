# Verbale sedia LEIJEN — S-186 (bozza indipendente, FASE 1)

Generato con assistenza AI (Claude Fable 5). Lente: memoria e refcount.

## VERDETTI
- **Q1: AGGIRATO.** Il kill S-168 chiedeva R4 = tappa sui reali; la delibera ha prodotto «corpo del handler» (R1 con altro nome), l'1× resta scritto.
- **Q2: B, GO** — fetta 0 di TEMPO sull'ORM con il ciclo di vita dell'oggetto come categoria propria.
- **Q3: C3**, vincolato: cure solo per NOME nel fail-set, O(1) sull'oggetto morente, mai scansioni del buffer nel Ret.

## ROTTA (una riga)
Prezzare h3 per oggetto (micro obj-churn × census oggetti ORM) nella fetta 0 di B; GC ibrido sotto predicato «dtor vivi > 0».

## REFUTAZIONI ED EMENDAMENTI
1. **`created` NON tiene solo gli oggetti con `__destruct`**: `alloc_object` inserisce OGNI oggetto (`mod.rs:12589`) nella `BTreeMap` (`mod.rs:3278`) e `gc_track` lo semina nel buffer. Ogni oggetto paga insert+remove O(log n), push/pop, demozione in `gc_cycle_roots` se sopravvive al primo Sweep, ri-nota alla morte, `resolve_method_runtime(.., "__destruct")` per NOME a ogni free. Zend: store O(1), `ce->destructor`. Stima 300-600 ns/oggetto: h3 vale ≥1 s solo oltre ~3M oggetti ORM.
2. **La cura 22/10 non è «2 load + any vuoto»** (NEXT 0-ter): `gc_sweep_after_ret` (`mod.rs:4161-4189`) scandisce TUTTO il buffer a ogni Ret; vuoto solo nei micro senza oggetti. Trovato un candidato, drena l'intero buffer come light sweep: ogni nato nella stessa istruzione è demotato e ri-seminato al main sweep. Pendenza: da una volta per istruzione a una per chiamata.
3. **E-LE-1**: flag per-classe `has_destructor` + contatore `dtor_live`; predicato del Ret `dtor_live > 0 && …`. Senza dtor vivi (WP, Doctrine, DI) la cura costa un confronto. Il timing dei soli handle-id si DICHIARA.
4. **E-LE-2**: `any()` sostituito da un byte settato in `gc_note_slow` quando `strong_count == 3` alla nota: O(1), stesso FIFO. Gate: 4 sentinelle drop-order + `leave_dtor_cmp_recursion`.
5. **C1 pieno** è impraticabile: i temp muoiono in `Drop` implicito e i dtor PHP non corrono in `Drop` (RULEBOOK); la forma (2) di §3.35 sono due siti nominati. **C2**: nel reale rompono solo dtor osservati nella STESSA espressione (buffer che flusha, guard fluente) e i test che stampano id.

## KILL-SWITCH PRE-REGISTRABILI
- **KS-LE-186-1**: mcalls-dq/calls-dq post-cura D > rumore ⇒ hook sostituito con E-LE-1/E-LE-2 prima della promozione.
- **KS-LE-186-2**: gc-census ORM pre/post 22/10: light sweep o `cand_demoted` +>20 % ⇒ revert del drain pieno.
- **KS-LE-186-3**: D_obj × N_obj < 1 s ⇒ h3 morta, GC fuori dalla rotta B.

## PRIMA FETTA
Micro `obj-churn` bilaterale (1M `new`/morte senza dtor, R=5, giudice run-micro.sh): D_obj ns/oggetto; census gc ORM = N_obj. GO: D_obj × N_obj ≥ 1 s ⇒ prima leva per NOME = `has_destructor` + store O(1).

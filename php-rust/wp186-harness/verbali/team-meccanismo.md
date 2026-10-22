# TEAM-MECCANISMO — verbale riconciliato (S-186, fase 2)
Sedie: Hejlsberg · Matsakis · Stogov. Generato con assistenza AI (Claude Fable 5).

## CONVERGENZE (n/3)
1. **Q2 B GO-CONDIZIONATO (3/3)**: niente codice prima della fetta 0 di TEMPO con mutante; A NO-GO; ipotesi ordinate h1 > h3 > h2.
2. **Q3 C3 (3/3)**: C1 impraticabile in Rust safe (`Zval` senza `Drop`, dtor che esige `&mut Vm`).
3. **Cura 22/10 non prezzata (3/3)**: `gc_sweep_after_ret` (mod.rs 4175-4188, da run.rs 4299 e 7253) scandisce TUTTO `gc_buf` a ogni Ret con oggetti vivi: O(|buf|), invisibile ai micro.
4. **Q1 AGGIRATO 2/3** (Hejlsberg: onorato, mai riscritto).

## SEQUENZA FETTE (dopo la fetta 0; ognuna promovibile da sola)
- **F1 guardia + O(1) della cura → pin s184**. Giudice: mcalls-dq, calls-dq, micro R=5 ABAB vs s183 + **ret-buf-dq** (K∈{0,4,16} oggetti bufferizzati prima di `$o->m()`). Attesa |D| < max(rumore, 4 ns/iter), K piatto. Kill: K lineare ⇒ hook solo in `ret_slow`.
- **F2 h1 builtin**: str 4,3 è UN `CallBuiltin` che paga `registry.get(&name)` a ogni chiamata (run.rs 3868) più la catena `var_export/var_dump/count` (run.rs 1411-1435); 18,3M chiamate ORM. Giudice NUOVO str-dq, mock «registry risolto nel `Func`». Attesa D ≥ 4 ns/iter. Kill: <4 ⇒ h1 non firmata (KS-HJ-186-3).
- **F3 h3 oggetti**: `gc_track` semina ogni `new`, `created` O(log n), `__destruct` risolto per nome. Giudice obj-churn × N_obj (gc-census ORM). Attesa 300-600 ns/oggetto. Kill: prodotto < 1 s ⇒ GC fuori da B (KS-LE-186-3).
- **F4 h2 ammissione** (per PREDICATO, tetto Bak 2,1 s): `simple_call` esclude hint/by-ref/variadic (bytecode.rs 1515-1523), Ret fuso esige `ret_shape == 0` (bytecode.rs 1570; run.rs 4272). Leva UNICA generica «hint soddisfatto per tipo esatto ⇒ fast path». Kill: cammini generali <1 s ⇒ h2 morta, cap run.rs congelato (KS-MA-186-1).
Soglia: canale ≥1 s nomina la leva; ≥3,5 s dichiara B campagna.

## Q3: IL MECCANISMO
**Invariante** (RULEBOOK §2): «l'oggetto morto si rilascia al primo confine tra Ret non-dtor **in cui è stato notato** e Sweep di fine istruzione, FIFO in ordine di nota; il temporaneo consumato in pila senza nota è forma (2), dichiarata al Sweep».
**Cura O(1)** (team lead, ACCOLTA): `before = gc_buf.len()` prima delle note del Ret, `any()` solo su `[before..)`. Completa: nuovo morente spinto in coda (mod.rs 3985-3988), già-bufferizzato ri-accodato con `take()` + push (4001-4003); `gc_buf_head == 0` già precondizione. Costo O(valori notati), zero righe in run.rs, drain FIFO invariato. Controesempi: (a) cascata — B non notato ma liberato da `gc_release_child`: tenuto; (b) temp morto senza nota prima del Ret slitta al Sweep: forma (2), ma byte diversi dal 22/10 ⇒ ri-pin delle 4 sentinelle drop-order; (c) ramo `Zval::Ref` senza ri-accodo (4107-4118): oggetto non morente. Dente: `GC_VERIFY` under-note sulla chain.
**Gate fxmc2**: `invar` (s185-promozione.sh 148-160, r.172) esige pin==stash s182 su fx-mc2.php e morde su C1 (sbagliato→sbagliato diverso). Emenda Pedersen: `diff(oracle, pin) == fx-mc2.expected-diff` committato, C2-C4 byte-id; ogni slittamento con voce di catalogo PRIMA e nome nel fail-set.

## CONFLITTI REGISTRATI
- **Hejlsberg**: Q1 ONORATO; GO-leva 3,5 s, non 1 s.
- **Matsakis**: cap run.rs CONGELATO senza quota ORM ≥20 % misurata; Stogov tiene leve per NOME nel loop caldo.
- **Stogov**: ORM ≤5 al NETTO del pavimento sqlite3 entro S-192, o B muore e «1× = orizzonte».

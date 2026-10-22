# Verbale sedia MATSAKIS — S-186 (bozza indipendente, FASE 1)

Generato con assistenza AI (Claude Fable 5).

## VERDETTI
- **Q1: AGGIRATO** — kill S-168 scattato, R4 riletta «corpo del handler», 1× ancora nel NEXT; la mia E2 S-167 («mai cucita sul driver») onorata nella lettera, violata nella sostanza (ref. 1).
- **Q2: B, GO-CONDIZIONATO** (fetta 0 di sola misura; A NO-GO senza quota di ammissione ORM).
- **Q3: C3** — C1 in Rust safe è solo «rilevamento alla morte, esecuzione al checkpoint».

## ROTTA (una riga)
Profilo ORM per SIMBOLO (tempo) + census di ammissione fast/generale per CAUSA; se `ret_slow`, `bind_params` con hint e `method_call` generale pesano ≥1 s/34 s, UNA leva generica «hint soddisfatto per tipo esatto ⇒ stesso fast path»; altrimenti le forme fuse hanno finito.

## REFUTAZIONI ED EMENDAMENTI
1. **Cucite per PREDICATO, non per pattern.** `simple_call` = no hints, no by-ref, no variadic, no generator (bytecode.rs r.1515-1523); il Ret fuso esige `ret_shape == 0` = nessun hint di ritorno (bytecode.rs r.1570; run.rs r.4272). Driver senza tipi (`f($a,$b)`, `m($a,$b)`, `public $x`); Doctrine e PHPUnit tipizzati ovunque: `get(int $id): ?Entity` esce da Call fast, MethodCall fast e Ret fuso insieme. Micro giù e ORM fermo è ciò che il predicato predice. Quota ORM ammessa: mai misurata («quota RT2», NEXT).
2. **Cifre**: «63 ns/op ⇒ il 7× sta nei corpi» è a registro come «lettura, non cifra» (PERF_MAP.md r.29-30): la media mescola op da 2 ns e builtin da microsecondi. IC miss 55,4 % è dei `prop_set` (WP_SESSION_176 r.11), non delle chiamate. Dente LOC: +102 L-MCk è S-166, PRE-concilio; dal S-167: 10 salite, +634 (loc_dente.rs r.91).
3. **Forma 2** (cammini generali `#[inline(never)]`) è giusta in Rust; ma ogni ammissione duplica un cammino che il reale forse non percorre: senza quota misurata, degenerazione.
4. **Q3**: `Zval` non ha `Drop` (generator.rs r.30); il temporaneo consumato dalla pila non è notato, lo prende lo Sweep (mod.rs r.3974-3982). Un `impl Drop for Object` RILEVA la morte esatta a costo zero per pop (drop glue già esistente; push thread-local solo con `__destruct`) ma NON può eseguire `__destruct`: serve `&mut Vm`. Checkpoint per op = load+branch in run_loop (~0,2 s/ORM); a Ret/Sweep = l'ibrido attuale, rilevatore preciso.

## KILL-SWITCH PRE-REGISTRABILI
- KS-MA-186-1: cammini non ammessi < 1 s/34 s ⇒ h2 morta, nessuna ammissione nuova, cap run.rs CONGELATO a 7551.
- KS-MA-186-2: ammissione con quota ORM < 20 % ⇒ non promovibile sui soli micro; rilevatore Drop con |D| > rumore su mcalls-dq ⇒ revert.

## PRIMA FETTA
Profilo per simbolo del pin s183 sulla suite ORM (samply, notturno) + census di ammissione dietro feature (Call/MethodCall/Ret: fast vs generale, per causa). Giudice: tempo self per simbolo, chiusura ≥85 % in ≤10 canali, R=3 concordi entro 10 %. GO-leva: canale ≥1 s. Mutante: `ret_slow` senza `#[inline(never)]` sposta il suo tempo in run_loop.

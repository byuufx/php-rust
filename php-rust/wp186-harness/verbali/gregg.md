# Verbale sedia GREGG — S-186 (bozza indipendente, FASE 1)
Generato con assistenza AI (Claude Fable 5).

## VERDETTI
- Q1: **AGGIRATO** nel registro (kill eseguito, R4 = stessa rotta rinominata, 1× ancora nel NEXT); nel merito la chiusura arith toccò ≈89 % in S-170 e non fu MAI dichiarata.
- Q2: **B, GO-CONDIZIONATO** alla fetta 0 di TEMPO su ORM; A NO-GO finché h1-h3 non hanno cifra; D solo come dichiarazione.
- Q3: **C3** oggi; C1 solo dopo prezzatura con giudice a buffer pieno.

## ROTTA (una riga)
Prima cifra di TEMPO bilaterale per categoria sull'ORM (floor PHPUnit sottratto, profilo per simbolo calibrato da mutante), poi leve per NOME; nessun codice sul nucleo prima.

## REFUTAZIONI ED EMENDAMENTI
1. «63 ns/op vs 6,7» (dossier r.52) è in `sessions/WP_SESSION_176.md` r.10 «lettura, non cifra»: suite/op include PHPUnit, PDO, sqlite, avvio. Non fonda h1-h3 (REGOLE §3: niente cifre da contrasti).
2. «37 s di suite» (dossier r.55, 79): phpr al pin è [34,16;34,30] (NEXT, REF2); i 37 s sono la build Astra (`sessions/WP_SESSION_179.md` r.3): binari diversi = sola direzione. Il declassamento del fill IC su quella cifra viola il veto «chiudere un fronte su misura singola altrui» (NEXT, veti S-180..170).
3. «KS Gregg MAI raggiunto» (dossier r.68): S-170, sottrattiva su arith-dq 46,8: loop 14,7 + Sweep 2,9 + dispatch 1,75 + corpo m13 22,2 = 41,6 ⇒ **88,8 %**, mai scritta come chiusura. Per calls/mcalls NON esiste decomposizione: le attese [3,5;6,5] e [4;9] erano stime; superarle del doppio è assenza di modello, non pessimismo.
4. «any sul buffer, vuoto nei loop caldi» (NEXT 0-ter): `gc_sweep_after_ret` (vm/mod.rs 4160-4188) scorre TUTTO `gc_buf` con deref Rc a ogni Ret se il buffer non è vuoto e nessun sweep è in volo: O(|buf|) per Ret, invisibile ai micro senza oggetti, pagato dall'ORM. Guardia dovuta: giudice **ret-buf-dq** (K temporanei oggetto notati prima di `$o->m()`, K ∈ {0,4,16}); attesa K=0 < rumore, K>0 lineare.
5. C1 si prezza PRIMA con mock (`gc_note` su ogni pop di temporaneo oggetto): A/B su prop-dq/mcalls-dq, micro-ORM objdatains/churn (S-131), coppia ORM. «|D| < rumore» sui micro senza oggetti non prova nulla sull'ORM.

## KILL-SWITCH PRE-REGISTRABILI
- KS-G-186-1 (strumento): mutante +Δ noto (spin 50 ns per Ret di metodo ⇒ ~0,7 s su 14,7M) attribuito dal profilo fuori ±25 % o wall fuori ±0,3 s ⇒ nessuna cifra pubblicabile.
- KS-G-186-2 (campagna B): dopo 2 sessioni nessun canale ≥1,5 s (≈5× RES 0,29 s della coppia) con chiusura ≥85 % ⇒ B muore, D dichiarata.
- KS-G-186-3 (Q3): ret-buf-dq lineare in K ⇒ la cura 22/10 non si promuove senza mock del costo su ORM.

## PRIMA FETTA
Giudice: ORM-only sul pin s183, bracci O, P e O0/P0 «suite nulla» (stessi test, corpi vuoti) ⇒ tempo proprio P−P0 e O−O0, bilaterale. Su P e O: xctrace Time Profiler per simbolo aggregato in 6 categorie (run_loop · call/ret · builtin str/array/preg · PDO/sqlite · alloc · GC/sweep) in secondi, R=3 interleaved, notte E5. Soglia: categorie ≥85 % di P−P0; GO-leva = canale ≥1,5 s con P/O per categoria ≥3×; mutante KS-G-186-1 PRIMA della lettura.

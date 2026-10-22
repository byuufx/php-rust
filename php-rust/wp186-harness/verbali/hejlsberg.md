# Verbale sedia HEJLSBERG — S-186 (bozza indipendente, FASE 1)

## VERDETTI
- **Q1: ONORATO** nella sostanza. Il kill S-168 (Σ 9,92) era il MIO kill-switch S-167 («<10 ⇒ R1 esaurito, si rialloca a R2/R3 Zval/frame»): S-170 l'ha eseguito. Le leve S-171..S-185 non toccano op-count né dispatch (1,75 ns/op = oracle intero, S-169): tolgono funnel, Zval temporanei, Frame per valore. Rotta diversa, mai RISCRITTA: l'1× resta in NEXT riga 1; PERF_MAP ignora S-185 e il pin s183 (riga 3).
- **Q2: B, GO** — fetta 0 di TEMPO sui corpi; i costi dei builtin sono già nominabili dal sorgente.
- **Q3: C3**, con prezzatura obbligata di `gc_sweep_after_ret` a buffer NON vuoto.

## ROTTA (una riga)
Dai micro ai corpi di ORM: census di TEMPO per famiglia di handler (builtin/metodo/prop/str/Sweep), poi leve che nominano il costo in byte mossi e spill, non in «una chiamata in meno».

## REFUTAZIONI ED EMENDAMENTI
1. Reperto S-167 CONFERMATO nel segno, SBAGLIATO nell'ordine: (a) consts +4,7 e (b) BinOp +5,2 reali (WP_SESSION_168); (c) bounds +1,56, hoist refutato (S-169); la voce messa ultima, (d) guardie/funnel, valeva +17,16 (WP_SESSION_170 m12).
2. Attese superate ×2 (S-184 [3,5;6,5]→12,5; S-185 [4;9]→19,0) = modello ADDITIVO su un corpo NON additivo: S-184 ha spostato 441 istr/36 bl fuori da `run_loop` (regalloc dell'intero loop), S-185 ha tolto due copie di `Frame` (~150 B, vm/mod.rs 2544). Emendamento: ogni criterio nomina `size_of` delle strutture mosse e usa Δsp_refs come PREDITTORE, non solo come gate.
3. str 4,3 NON è una micro «stringhe»: `str.php` = `substr($s."abc",-30)`, UN `CallBuiltin` per iterazione. Il corpo (run.rs 3868-3910, 1401-1467) paga a OGNI chiamata `registry.get(&name)` (hash del nome; Zend risolve a compile time) più 6-7 confronti di nome (`var_export`, `var_dump`, `count`, wrapper): il micro-dispatch secondario di (b) S-167, sulla funzione invece che sull'operatore; l'oracle fa `substr` frameless. È la h1 del dossier: ORM 18,3M CallBuiltin+CallHostBuiltin (wp176 census-op-orm.txt).
4. La guardia di NEXT 0-ter(a) NON prezza la cura 22/10: `gc_sweep_after_ret` (mod.rs 4175) scandisce `gc_buf` a ogni Ret SOLO con oggetti vivi; i micro hanno buffer vuoto. Serve un driver con K oggetti bufferizzati PRIMA del pin s184.

## KILL-SWITCH PRE-REGISTRABILI
KS-HJ-186-1: census che non nomina ≥85 % dei 34,5 s ORM ⇒ si rifà lo strumento. KS-HJ-186-2: nessuna famiglia ≥3,5 s ⇒ B muore, si va a D. KS-HJ-186-3: mock «registry+nomi hoisted» su str-dq <4 ns/iter ⇒ h1 non firmata sui micro.

## PRIMA FETTA
Census di TEMPO bilaterale ORM: phpr a campione (Instant 1/64 per famiglia), oracle via xctrace sui simboli `ZEND_*_HANDLER`; mutante = ritardo 50 ns in `value_builtin_call`, atteso +0,9 s. Giudice: secondi per famiglia su ORM E5, R=3 notturno; GO-leva ≥3,5 s. Companion: str-dq coi due mock del punto 3, R=5.

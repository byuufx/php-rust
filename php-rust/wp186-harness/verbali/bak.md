# Verbale sedia BAK — S-186 (bozza indipendente, FASE 1)

## VERDETTI
- Q1: **AGGIRATO** — kill scattato in S-168, R4 rinominata «corpo del handler», NEXT ancora «≥1×»: ha pagato sui micro, il registro no.
- Q2: **B, GO-CONDIZIONATO** alla fetta 0 di TEMPO; A NO-GO su ORM per aritmetica; D va SCRITTA (ORM ≤5, WP ≤1,5).
- Q3: **C3**.

## ROTTA (una riga)
Census di TEMPO per classe di op su ORM con mutante additivo; nessuna leva di nucleo spacciata per leva ORM senza tetto count × surplus pre-registrato.

## REFUTAZIONI ED EMENDAMENTI
1. La mia previsione S-167 (meno dispatch/iter, specializzazione Long, handler outlined) si è avverata alla lettera: L-SL1/SL2 = AddSmi, Sweep-in-op, `ret_slow`/`gc_note_this` fuori linea (s184-criterio-rt2.md p.7-8). Paga solo dove il carico è monomorfo e Long: i micro.
2. h2 del dossier (riga 82, «metodi polimorfi, IC miss 55 %») è smentita da WP_SESSION_177.md:11: ic_class 4,6 %, cella mai riempita 95,4 %. Non è polimorfismo V8: è una cella MAI armata (private/readonly). E il 55 % è di 8,99M prop_set = 4,98M miss = **0,9 % dei 544M op** (census-op-orm.txt).
3. Tetto IC: Astra 0,42 s / 1,795M = 234 ns/miss inclusivi; × 4,98M ≤ 1,2 s su ~29 s di surplus (34,2 − 4,87). La fetta estrapola: canale IC **CHIUSO** come rotta ORM (≤4 %).
4. Tetto metodi: MethodCall 14,65M + ThisMethodCall 5,58M = 20,2M; a surplus intero di mcalls-dq (≈104 ns) = 2,1 s = 7 % ⇒ parità totale sui metodi porta ORM da 7,0× a ~6,6×. Il «cammino generale» non serve mockarlo: count × surplus lo uccide prima del codice. Idem str (ConcatN 2,5M).
5. Senza cifra di tempo: CallBuiltin 11,9M + CallHostBuiltin 6,4M, ThisPropGet 27,6M, Sweep 59M ⇒ h1/h3 sole ipotesi vive.
6. Layout: +736 righe confermate (loc_dente.rs:91, 11 voci) ma run_loop 71.745→72.016 istr (+0,4 %), sp_refs 11.270→10.782 (wp177 disasm, s184-disasm-pin-s182.out): l'outlining ha tenuto. Però 72k istr ≈ 288 KB > L1I 192 KB: conta solo il sottoinsieme caldo. Il gate Δbl/Δsp_refs è STATICO (ha fermato la forma 1 RT2), non vede il footprint dinamico: registro = guardie non-bersaglio + L1I-miss/op xctrace bilaterale su ORM a ogni pin.
7. Q3: il check inline nel Ret fuso non si legge dal disasm (Δbl 0, Δistr ~+6): solo mcalls-dq/calls-dq R=5 con |D| < rumore (NEXT 0-ter a). C1 = carne nuova in run_loop a ogni pop (lezione H-C2): solo outlined e dietro tetto.

## KILL-SWITCH PRE-REGISTRABILI
- **KS-BAK-186-1**: ogni leva «per ORM» pre-registra count(classe op) × (phpr − oracle ns/iter del micro affine); se < 1 s/34 s è leva di NUCLEO, dichiarata tale.
- **KS-BAK-186-2**: fetta 0 senza canale ≥ 3 s (10 % del surplus) ⇒ B muore, si scrive D; A solo per str.
- **KS-BAK-186-3**: L1I-miss/op phpr su ORM > 2× oracle al pin ⇒ stop alle leve inline.

## PRIMA FETTA
Census di TEMPO per classe di op su ORM (Instant a densità 1/16 e 1/64 come S-179, notturno, parità 16 nomi): metodi/Call, builtin, prop get/set, str, Sweep/alloc, resto. Mutante ADDITIVO: ritardo noto di 200 ns in CallBuiltin deve comparire come +2,4 s ± 10 %. **Giudice**: Σ categorie / (phpr − oracle) ≥ 85 %. **Soglia GO-leva**: canale ≥ 3 s. **R=3**.


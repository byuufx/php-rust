# Verbale sedia HOARE — S-186 (bozza indipendente, FASE 1)

## VERDETTI
- **Q1: ONORATO** nella forma (S-168 zero leve, delibera R4 all'utente S-169/170); vizio di sostanza residuo: la R4 del dossier S-167 («1× non-obiettivo») non è mai stata scritta — REGOLE §1 e NEXT portano ancora ≥1×.
- **Q2: B GO** (fetta 0 = sola misura di TEMPO) · **D GO** come scrittura · **A NO-GO** finché h1-h3 non sono firmate.
- **Q3: C3**, con invariante DICHIARATO e cure solo per NOME. C1 NO: senza `Drop` (vietato, RULEBOOK §2) la completezza dei siti di morte non è dimostrabile (`GC_VERIFY under-note` esiste per questo). C2 NO: il 22/10 ha già mosso 3 flip e un gate.

## ROTTA (una riga)
Guardia micro sulla cura 22/10 (con dente di complessità), poi fetta 0 del tempo ORM; modello GC scritto nel RULEBOOK: «rilascio al primo confine (Ret non-dtor o Sweep) dopo la morte, FIFO in ordine di nota».

## REFUTAZIONI ED EMENDAMENTI
- **E1** bug47771 NON è timing: §3.35 (righe 1113-1114) dice che Zend non costruisce `new Test(throw_exc())`. È la regola ctor-failed (nessun `__destruct` se il costruttore non termina): fuori dalla famiglia Q3.
- **E2** La cura è sana per nome: ri-accodo solo se `pos ≥ gc_buf_head`; `drop(o)` lascia `frame.this` ultimo holder ⇒ cascata a count 1; rientro escluso dal flag IN_DESTRUCTOR.
- **E3** Ma `gc_sweep_after_ret` (mod.rs 4160-4188) scandisce `gc_buf` a OGNI Ret mentre `gc_track` semina OGNI `new` nel buffer: l'O(n²) che il buffer di note era nato per eliminare (mod.rs 3286-3288). Caso: N `new` sopravviventi e N Ret in UNA istruzione, `array_map(fn($i)=>new A($i), range(1,N))`. Da misurare.
- **E4** La forma (2) ora muore «al primo Ret dopo», non a fine istruzione (fxmc2 C1): timing dipendente dalla forma della frase. Lecito solo con l'invariante scritto.
- **E5** RULEBOOK §2 («eager, Zend-faithful timing») e Deviation log («at the enclosing statement sweep») sono entrambi falsi: emendare.
- **E6** Il kill 9,92<10 è scattato DENTRO il rumore 0,2-0,6 (WP_SESSION_168 righe 21, 38-39): banda non pre-registrata = vizio di forma; S-170 ha poi nominato il costo (m13 −22,20).

## KILL-SWITCH PRE-REGISTRABILI
1. Driver E3 a N=10⁵ e 2·10⁵: ratio tempo > 2,5 ⇒ scansione sostituita da contatore O(1) in `gc_buf_push`/`gc_unbuffer`, PRIMA della promozione.
2. Micro R=5 + mcalls-dq/calls-dq tree vs pin s183: |D| > max(rumore, 4 ns/iter) ⇒ hook fuori dal Ret fuso (solo `ret_slow`).
3. Cura di forma (2) senza nome del fail-set congelato E fixture bilaterale ⇒ respinta (REGOLE §9).
4. Dente: catena di 10⁴ dtor con due helper (`__destruct→a()→b(){unset}`) senza overflow di pila; fallisce ⇒ drive sincrono torna scheduling.

## PRIMA FETTA
Guardia della cura 22/10: giudici mcalls-dq, calls-dq, micro; tree vs pin s183, R=5 ABAB; soglia |D| < max(1,67, 4 ns/iter), attesa D≈0; stessa corsa il dente KS1. Passa ⇒ promozione s184; morde ⇒ hook fuori dal Ret fuso.

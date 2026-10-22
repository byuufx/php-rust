# CONCILIO S-186 — «audit della campagna S-167 e rotta per il carico reale» (richiesto dall'utente 2026-10-22)
Generato con assistenza AI (Claude Fable 5). Fase 1: 9 verbali INDIPENDENTI in `verbali/` (fonte VINCOLANTE) · Fase 2: 3 team
(`team-sonda`, `team-meccanismo`, `team-governo`) · Questa sintesi recepisce; i dissensi restano a registro col nome della
sedia. Dossier: `COUNCIL_DOSSIER.md`. **Le scritture che toccano riga 1 del NEXT, REGOLE §1/§5/§8/§10 e `.gitignore`
entrano SOLO con la firma dell'utente** (team-governo, ordine (0)).

## §FONDAMENTALI (apertura obbligata)
Dal S-167: 18 sessioni (+2 bis), 8 pin nuovi, 9 leve promosse, 1 revertata al byte (L-CM1). Sessioni-senza-misura: **2**
(S-178, S-182 — Klabnik, corregge il dossier). Incidenti ≈29 (S-184 = 7 col bis). Oggetto: nucleo 5,4/5,5/4,8 → 2,1/2,3/3,5;
corpus 1412 → 1409 senza pass→fail. Rischio più trascurato: **WP 1,747 e ORM [6,943;7,058] FERMI su tutti gli 8 pin, nessuna
cifra di TEMPO che dica perché** — oggetto di Q2. Rischio di processo: il kill del S-168 è scattato e la delibera che lo
seguì è sparita dal registro (Q1).

## VERDETTO Q1 — AUDIT: kill ESEGUITO nella forma, AGGIRATO nella sostanza (7 AGGIRATO · 2 ONORATO a registro)
Fatti verificati: S-168 Σ mock 9,92 < 10 ⇒ kill a filo (DENTRO il rumore 0,2-0,6, banda non pre-registrata — Hoare E6);
zero leve in S-168/169; delibera R4 dell'utente SCRITTA nel NEXT alla chiusura S-170 (commit 075b6c63 r.2) e **SPARITA nella
riscrittura S-171 (7fbe896f: zero «R4»)** — Klabnik, verificato con `git show` dal team-governo. Nel merito: la chiusura
sottrattiva arith toccò **88,8 %** in S-170 (Gregg, corregge il dossier «KS Gregg mai raggiunto»); il KS Stogov 46,8→≤35 fu
RAGGIUNTO in S-171 (arith-dq 23,44). Le leve S-171..S-185 non toccano op-count né dispatch (1,75 ns/op = oracle intero,
S-169): tolgono funnel, Zval temporanei, Frame per valore — rotta DIVERSA da R1-dispatch (Hejlsberg: «era il mio kill,
eseguito»), MAI riscritta: «OBIETTIVO PARITÀ (≥1×)» identico in NEXT r.1 e REGOLE §1, PERF_MAP senza S-185/pin s183.
**Dissensi a registro**: Hoare e Hejlsberg «ONORATO nella forma/sostanza, vizio di scrittura»; accettano il testo della riga 1.

## VERDETTO Q2 — ROTTA B, GO-CONDIZIONATO alla FETTA 0 di TEMPO (9/9) · A NO-GO (9/9) · D SCRITTA (9/9)
Reperti che chiudono A sull'ORM PER ARITMETICA (Bak, census-op-orm.txt): h2 «polimorfismo» SMENTITA — il 55,4 % di IC miss
è dei `prop_set`, cella MAI armata (private/readonly) 95,4 %, ic_class 4,6 %, = 0,9 % dei 544M op; tetto IC ≤1,2 s e tetto
metodi 20,2M × 104 ns ≈ 2,1 s su un surplus di ≈29,4 s (REF2 [34,23;34,31] − ORA_REF 4,870: corregge il «37 s» del dossier,
che è la build Astra; «63 ns/op» è lettura, non cifra — WP_SESSION_176 r.10, PERF_MAP r.29-30). Ipotesi vive, ORDINATE
dal team-meccanismo **h1 > h3 > h2**: h1 corpi dei builtin (Hejlsberg: str 4,3 è UN `CallBuiltin` per iterazione che paga
`registry.get(&name)` + catena di confronti di nome a OGNI chiamata, run.rs 3868/1411-1435; 18,3M chiamate ORM) · h3 ciclo di
vita oggetto (Leijen: `created` tiene OGNI oggetto, mod.rs 12589/3278, `gc_track` semina ogni `new`, `__destruct` risolto
per NOME a ogni free: stima 300-600 ns/oggetto) · h2 RIFORMULATA (Matsakis: ammissione per PREDICATO — `simple_call` esclude
hint/by-ref/variadic/generator, bytecode.rs 1515-1523; Ret fuso esige `ret_shape == 0`, bytecode.rs 1570, run.rs 4272;
Doctrine tipizzato ovunque ⇒ `get(int $id): ?Entity` esce da Call fast, MethodCall fast e Ret fuso insieme; quota ORM
ammessa MAI misurata).

## FETTA 0 (team-sonda; prossimo atto, SOLA MISURA — una notte sul pin s183, catena E5 non presidiata)
`xctrace` Time Profiler BILATERALE (oracle php 8.5.7 opcache.enable_cli=Off/jit disable, verificato) su UNA gamba ORM intera,
per immagine e simbolo in **8 classi**: dispatch · call/ret (fast vs `ret_slow`/`method_call`) · prop/IC · str · array/hash ·
ciclo di vita oggetto (alloc, nota, Sweep, dtor, drop-glue) · builtin (lookup per nome vs corpo) · PDO/sqlite3/avvio =
pavimento comune sottratto da ENTRAMBI i lati (Stogov: stessa libsqlite3, in memoria). R=3 interleaved O/P, mediana.
**Bracci**: O · P · **M** mutante additivo (spin 200 ns in CallBuiltin ⇒ atteso +2,38 s su 11,91M; wall in [2,1;2,7] s E ≥75 %
del Δ nella classe builtin, altrimenti strumento morto — PRIMA della lettura) · **C** phpr con feature census (Instant 1/64
per classe + ammissione fast/generale per causa + N_obj) che CALIBRA P entro ±15 %, altrimenti conta solo xctrace.
**Tetti pre-registrati**: metodi 2,1 s · IC 1,2 s · oggetti N_obj × D_obj (companion obj-churn R=5); classe > 2× il tetto ⇒
modello falso, dichiarato. **Chiusura**: Σ classi ≥85 % di (P − pavimento), su entrambi i lati. **Firma GO-leva**: un canale
con (P_c − O_c) ≥ **3,0 s** (10 % del surplus) E P_c/O_c ≥ 3; canali in [1,5;3,0) s NOMINATI a registro, non leva.
**Kill**: nessun canale ≥3,0 s, chiusura <85 % o mutante muto ⇒ **B muore NELLA STESSA sessione** e riga 1 del NEXT scrive D.

## SEQUENZA FETTE (team-meccanismo; ognuna promovibile DA SOLA; soglia ≥1 s nomina la leva, ≥3,5 s dichiara B campagna)
**F1 guardia + O(1) della cura 22/10 → pin s184** (vedi Q3) → **F2 h1 builtin**: giudice NUOVO str-dq, mock «registry risolto
nel `Func`», attesa D ≥ 4 ns/iter, kill <4 ⇒ h1 non firmata (KS-HJ-186-3) → **F3 h3 oggetti**: obj-churn × N_obj, attesa
300-600 ns/oggetto, kill prodotto <1 s ⇒ GC fuori da B (KS-LE-186-3) → **F4 h2 ammissione per predicato** (tetto 2,1 s): leva
UNICA generica «hint soddisfatto per tipo esatto ⇒ stesso fast path», kill cammini generali <1 s ⇒ h2 morta e **cap run.rs
CONGELATO a 7551** (KS-MA-186-1). Ogni leva «per ORM» pre-registra count(classe) × surplus del micro affine: <1 s/34 s = leva
di NUCLEO, dichiarata tale (KS-BAK-186-1).

## VERDETTO Q3 — C3 ibrido (9/9), con INVARIANTE SCRITTO e cure solo per NOME
C1 impraticabile in Rust safe (`Zval` senza `Drop`, i dtor PHP esigono `&mut Vm`; Matsakis/Leijen/Hoare); C2 no: il 22/10 ha
già mosso 3 flip e un gate. **Invariante (entra nel RULEBOOK §2, che oggi è falso in due punti — Hoare E5)**: «l'oggetto morto
si rilascia al primo confine tra Ret non-dtor IN CUI È STATO NOTATO e Sweep di fine istruzione, FIFO in ordine di nota; il
temporaneo consumato in pila senza nota è forma (2), dichiarata al Sweep». **La cura 22/10 è sana per NOME ma NON prezzata**
(4 sedie + 3 team): `gc_sweep_after_ret` (mod.rs 4160-4188) scandisce TUTTO `gc_buf` a ogni Ret con oggetti vivi — O(|buf|),
muto nei micro senza oggetti, pagato dall'ORM; con `gc_track` che semina ogni `new`: rischio O(n²) in `array_map(fn ⇒ new A,
range(1,N))` (Hoare E3). Il «2 load + any vuoto» del NEXT 0-ter è SMENTITO. **Cura O(1) ACCOLTA** (team lead, verificata dal
team-meccanismo su mod.rs 3985-3988 e 4001-4003): `before = gc_buf.len()` prima delle note del Ret, `any()` solo su
`[before..)` — completa perché il ri-accodamento porta in coda tutto ciò che muore a quel Ret; UN controesempio: temp morto
senza nota prima del Ret slitta al Sweep (forma (2), coerente con l'invariante; byte diversi dal 22/10 ⇒ ri-pin delle 4
sentinelle). Alternativa Leijen E-LE-1/E-LE-2 (flag per-classe `has_destructor` + byte settato in `gc_note_slow` a
`strong_count == 3`). **Fuori famiglia**: bug47771 è la regola ctor-failed (Zend non costruisce `new T(f())` se un argomento
lancia), non timing GC — Hoare E1; registro `.out` emendato. **Voce-madre** della famiglia timing nel catalogo (Pedersen:
§3.33 + §3.35(2) + unwind + generatori a shutdown), fixture bilaterale per forma.

## LA FORMA (team-governo, VINCOLANTE con firma dell'utente; cap 25: 1 sostituzione + 2 emende)
1 **REGISTRO** (sostituisce §8): NEXT ≤80 righe E ≤8.000 caratteri, riga ≤600, giudice `scripts/next-budget.sh` in
chiudi-sessione; riga 1 = orizzonte + tappa + «delibera S-N, data»; kill scattato in riga 1 nella STESSA sessione, per nome,
fino al concilio seguente (riscrittura che lo cancella = incidente); cap `loc_dente` sale SOLO nel commit di promozione rc=0
con cifra nel `.out`; veti >3 sessioni in `gaps/VETI.md`. 2 **TIMEBOX** (emenda §1): apparato oltre ½ sessione = incidente
ANCHE con parola dell'utente (deroga S-182 abrogata; S-180/182/183 contati). 3 **GATE AUDITABILE** (emenda §5, §10): fail-set
×2 e golden sono GATE, non output ⇒ in git (`wp109-harness/.gitignore` r.9 emendato; §5 «1412»→«1409»); registro `.out` da
solo = gate VOID (KS-PE-186-1); golden ri-basato SOLO con voce di catalogo PRIMA e verso dichiarato (KS-PE-186-2); gate
`fxmc2`: `invar` → `diff(oracle,pin) == .expected-diff` committato, slittamenti ri-dichiarati.

## RIGA 1 DEL NEXT (testo proposto, 374 caratteri — ENTRA CON LA FIRMA DELL'UTENTE)
`# NEXT_SESSION — phpr: ORIZZONTE parità (≥1×) con l'oracle · TAPPA D (delibera utente S-186, 2026-10-22, verifica al concilio
~S-192): WP ≤1,5 (coppia notturna, mediana) · ORM ≤5 (coppia E5, al netto del pavimento sqlite3 comune) · micro ≤3× 6/6 ·
KILL A REGISTRO: S-168 Σ mock 9,92<10 ⇒ R1 chiusa; R4 eseguita come «corpo del handler» (S-170) SENZA tappa reale, sanata qui.`
REGOLE §1: «orizzonte parità (≥1×); tappa D (delibera utente S-186, 22/10): WP ≤1,5 · ORM ≤5 al netto del pavimento comune ·
micro ≤3× 6/6».

## PROMOZIONE s184 — ORDINE OBBLIGATO (team-governo)
(0) riga 1 firmata dall'utente · (1) regola 3 eseguita (congelati e golden in git, fxmc2 a diff dichiarato) · (2) guardia a
buffer PIENO nella stessa notte della fetta 0: driver Hoare E3 (N=10⁵/2·10⁵, kill ratio >2,5) + **ret-buf-dq** K∈{0,4,16} +
micro R=5 + mcalls-dq/calls-dq ABAB tree vs s183, |D| < max(rumore, 4 ns/iter) · (3) cura O(1) PRIMA della promozione SOLO
se (2) morde (criterio proprio, guardia ri-eseguita); altrimenti s184 com'è e O(1) leva per nome DOPO la fetta 0 · (4) pin con
ricetta, coppia notturna WP t26 + ORM t28.

## KILL-SWITCH A REGISTRO (nuovi)
KS-G-186-1 strumento (mutante fuori ±25 % ⇒ nessuna cifra) · KS-G-186-2/KS-BAK-186-2/KS-K186-2 campagna B (nessun canale ≥3 s
⇒ B muore, D in riga 1 nella stessa sessione; **dissenso Gregg**: 2 sessioni, non 1 notte) · KS-HJ-186-1/2 (census <85 % ⇒
strumento rifatto; nessuna famiglia ≥3,5 s ⇒ D) · KS-MA-186-1/2 (h2 <1 s ⇒ cap run.rs congelato; ammissione con quota ORM
<20 % non promovibile sui soli micro) · KS-LE-186-1/2/3 (hook morde ⇒ E-LE-1/2; gc-census light/demoted +20 % ⇒ revert del
drain; D_obj×N_obj <1 s ⇒ h3 morta) · KS-BAK-186-1/3 (tetto count × surplus per ogni leva «ORM»; L1I-miss/op > 2× oracle ⇒
stop leve inline) · KS-PE-186-1/2/3 (congelati non tracciati ⇒ gate VOID; ri-basamento senza voce ⇒ STOP; |D| ≥ rumore ⇒
hook solo in `ret_slow`) · KS-K186-1/3 (kill assente da riga 1 ⇒ incidente; 2 sessioni su 5 con apparato >½ o incidenti ≥4 ⇒
revisione a lente processo) · **Stogov** (soglia dura): ORM ≤5,0 netto entro S-192 con ≥2 leve firmate per NOME, o B MORTA e
«1× = orizzonte».

## DISSENSI A REGISTRO (col nome)
Hoare, Hejlsberg: Q1 ONORATO (forma/sostanza) · Hejlsberg: GO-leva 3,5 s (team: 3,0 leva / 1,5 nomina) · Gregg: kill in 2
sessioni e bracci O0/P0 «suite nulla» (respinti per apparato, regola 7) · Bak: Instant come giudice primario (declassato a
calibrazione) · Matsakis: cap run.rs congelato senza quota ORM ≥20 % (Stogov tiene leve per NOME nel loop caldo) · Leijen:
KS-LE-186-3 come kill di B (accolto come tetto su h3).

## EFFETTO SU ROTAZIONE
Il blocco ⚖️ in NEXT 0-ter (e) dichiara questi verbali VINCOLANTI dopo la firma; la «leva di ritmo» di S-186/S-187 È la fetta 0
(sola misura, oggetto sanzionato) + la guardia della cura 22/10; nessun codice di leva prima della cifra.

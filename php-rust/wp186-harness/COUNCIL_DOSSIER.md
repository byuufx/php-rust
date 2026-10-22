# DOSSIER — Concilio S-186 «audit della campagna S-167 e rotta per il carico reale» (richiesto dall'utente, 2026-10-22)

Generato con assistenza AI (Claude Fable 5). Calco del dossier S-167 (`wp167-harness/COUNCIL_DOSSIER.md`);
fonti: NEXT_SESSION_WORDPRESS.md, gaps/GAP_TREND.md S-167..S-185, PERF_MAP.md, sessions/WP_SESSION_167..185,
COUNCIL_WP167_REVIEWS.md (verdetto e 8 regole VINCOLANTI), PHPR_DIVERGENCES_FROM_PHP.md, commit 4aa30319 (cura 22/10).
Titolo di convocazione: REGOLE §7 («audit ogni ~5 sessioni»): ultimo concilio S-167 (1/9), oggi 18 sessioni dopo.

## LE DOMANDE (tre, a verdetto separato; la prima è l'AUDIT dovuto)
Q1 **AUDIT**: il GO-CONDIZIONATO del S-167 è stato onorato? Il kill-switch di campagna è SCATTATO a filo in S-168
(Σ mock 9,92 < 10, chiusura 52,7 % < 90 % di Gregg) ⇒ per la regola 4 «R1 morto, si delibera R4». La delibera R4
(S-169, all'utente; S-170) è diventata «(i) corpo del handler, sola misura» e da lì la campagna sul nucleo è CONTINUATA
con altro nome (forma sigillata Long, Sweep-in-op, Call/Ret magri, Ret fuso, metodi) e ha PAGATO: arith 5,4→2,1,
prop 5,5→2,3, calls 4,8→3,5. Ma la R4 del dossier S-167 diceva «tappa ridefinita sui workload REALI, 1× dichiarato
non-obiettivo»: NEXT_SESSION porta ancora «OBIETTIVO PARITÀ (≥1×)». Il concilio dice se il kill è stato onorato (rotta
cambiata e dichiarata) o aggirato (stessa rotta, nome nuovo), e se l'obiettivo 1× resta scritto o si ridefinisce.
Q2 **ROTTA**: con 4 micro su 6 sotto la tappa ≤3× e WP/ORM FERMI da 18 sessioni (WP 1,746→1,747; ORM [7,023;7,053]→
[6,943;7,058]), la prossima campagna va (A) sul nucleo fino a 6/6 (str 4,3, calls 3,5), (B) sul carico reale ORM 7×
(decomposizione di TEMPO prima, leve per NOME poi), o (D) WP≤1,5 come tappa dichiarata del reale? GO/NO-GO con prima
fetta misurabile e kill pre-registrato, come nel S-167.
Q3 **MODELLO GC**: la raccolta/distruzione «al confine di istruzione» (§3.33, deliberata WP-21/46/47) produce una
FAMIGLIA di divergenze di solo timing (§3.35 forma 2, §3.28 (i), unwind, generatori a shutdown, `new T(f())` con
argomento che lancia — bug47771). La cura del 22/10 ne ha chiuso due forme toccando il Ret fuso (una chiamata inline
in più, NON misurata). Si va al «rilascio al punto di morte» (nota GC a ogni pop di temporaneo oggetto: pervasivo,
rischio prestazioni), si DICHIARA la famiglia chiusa come divergenza di timing (stop alle cure una a una nel loop caldo),
o si tiene l'ibrido attuale (confini di frame curati, resto dichiarato)?

## I NUMERI (pin s166 al S-167 → pin s183 oggi; micro R=5 ns/iter phpr/oracle; coppie = mediana rapporto)
| | S-167 (s166) | oggi (s183) | nota |
|---|---|---|---|
| arith | 5,4 | **2,1** | L-SL1 S-171 (5,4→2,7) · Sweep-in-op S-175 · toolchain S-180 |
| prop | 5,5 | **2,3** | L-SL2 S-172/173 (→3,0) · Sweep-in-op (→2,6) · gc-idle |
| calls | 4,8 | **3,5** | L-CR1 S-181 · L-RT1+RT2 S-184 (calls-dq 98,8→77,5; oracle ~21) |
| str | 4,2 | **4,3** | MAI toccata: concat/interpolazione, Zval::Str clone/alloc (census str a registro) |
| arr | 3,2 | 2,9 | di riflesso |
| re | 2,5 | 2,5 | ferma |
| mcalls-dq | — (mc2 155) | 126,0 (era 146,5 @ s182) | giudice nuovo S-185 (L-MR1+MC2), B/oracle 5,67× |
| **WP full** | 1,746-1,749 | **1,747** (t25, 8/10) | FERMA: 18 sessioni, banda_ON 0,005-0,030 |
| **ORM** | [7,023;7,053] | **[6,943;7,058]** (t27, 16/10, E5) | FERMA; istruttoria S-185 a 3 bracci: s181 vs s182 D −0,008 |
| dbal | 7,3 | [7,370;7,439] | ferma |
| media | 2,43 | 2,42-2,44 (t21, S-180) | ferma |
| corpus | 1412 ×2 | **1409 ×2** (3 flip pass 22/10) | zero pass→fail in tutto l'arco |
| batteria | 1748 | 1752/0 + 2 denti | |
| catalogo | — | 40 voci: 9 🔴 · 19 🟡 · resto 🟢/✅ | §3.34 ✅ 22/10 |
Fatto saliente: il nucleo si è dimezzato sui micro, i carichi reali NON si sono mossi di un tick.

## COSA È SUCCESSO DAL S-167 (18 sessioni, 8 pin nuovi: s171 s172 s173 s175 s180 s181 s182 s183)
S-167 fetta 0: pila e mispredict REFUTATI (phpr predice MEGLIO di Zend), interno-handler ~23 ns (61 %) indiziato ·
S-168 fetta 0-bis mock: Σ 9,92 < 10 ⇒ **kill ⚖️ a filo**, chiusura 52,7 % · S-169 dispatch puro 1,75 ns/op = oracle
INTERO; corpo BinarySCSCDst ~27,4 (17,5 non nominati); delibera R4 all'utente · S-170 R4 = corpo del handler, mock
magri: dq 46,8→24,6 (2,84×) con «forma sigillata Long» · S-171 **L-SL1** promossa (arith 5,4→2,7) · S-172/173
**L-SL2** fette 2-3 (prop 5,2→3,0) · S-174/175 **Sweep-in-op** (arith 2,3 · prop 2,6) · S-176 flag gc-idle sola
direzione (tenuta) + census: **ORM 544M op / 34,5 s ≈ 63 ns/op contro ≈6,7 ns/op dei micro ⇒ il 7× sta nei CORPI
(call/prop/str/builtin), non nel dispatch**; IC miss ORM 55,4 % · S-177 L-CM1 in finestra contaminata; census miss per
CAUSA: 95 % cella mai riempita, private_mangled 85 %, readonly 59 % · S-178/179 (ChatGPT Astra 6) prezzano il fill IC
privati: 36 % dei miss ma **≈0,42 s / 37 s di suite ⇒ DECLASSATO** · S-180 ri-pin toolchain 1.98.1, L-CM1 revertata al
byte · S-181 **L-CR1** (calls-dq 98,8→90,8) · S-182 apparato (sparsebundle, CLAUDE.md) · S-183 istruttoria t24/ORM +
L-RT1 direzione · S-184 **L-RT1+RT2** (calls-dq →77,5, attesa [3,5;6,5] superata ×2, dichiarata) · S-185 **L-MR1+MC2**
(mcalls-dq 146,5→126,0, attesa [4;9] superata ×2, dichiarata) · 22/10 cura §3.34+§3.35 (fuori sessione).
Pattern: OGNI leva promossa è una forma «sigillata/fusa/in pila» del corpo di un handler; due attese di fila superate
del doppio (S-184, S-185) = il modello di costo del Ret/Call era troppo pessimista, riparato a parole non a cifra.

## AUDIT DELLE 8 REGOLE E DEI KILL-SWITCH DEL S-167 (per il concilio: confermare o smentire)
1 safe-only: TENUTA (nessun unsafe nuovo; VmGate ZST) · 2 fette promovibili/revert al byte: TENUTA (L-CM1 revertata al
byte S-180) · 3 fetta 0 con mutante: TENUTA (mutante xctrace c1 11,9×) · 4 **kill di campagna: SCATTATO S-168 e
RINOMINATO** (R4 → «corpo del handler»): da giudicare · 5 two-request parity: TENUTA (http-kernel 0E/0F, fixture chain
10 a ogni promozione) · 6 disasm bl-count nei criteri: TENUTA e irrigidita (Δsp_refs ≤ +97, |Δbl| ≤ 2) · 7 **timebox
apparato ½ sessione: VIOLATA almeno in S-182 (intera, su decisione utente) e S-183 (5 incidenti)** · 8 R3 arena sigillata:
non toccata. KS Stogov (46,8→≤35 dopo F1-F3): RAGGIUNTO (arith-dq ≈20) · KS Gregg (chiusura ≥90 %): MAI raggiunto,
superato dalla delibera · KS-BAK-167-1 / KS-LE-167-1: non scattati, mai chiusi a registro · dissenso Klabnik (3 fette
cadute + concilio): mai testato (una sola fetta caduta, L-CM1).
Incidenti di processo S-167..S-185: 1·2·1·2·1·2·1·1·1·0·1·?·?·0·1·2·**5·4**·1 (≈26; picco S-183/S-184 = apparato e
finestre, non codice) · sessioni senza misura: S-182 (dichiarata) · dente LOC run.rs: cap alzato 11 volte dichiarando
(+736 righe dal S-166, «run_loop — ULTIMO o mai (A2)» nel testo del dente).

## IL FATTO NUOVO CHE IL CONCILIO DEVE SPIEGARE
I micro misurano il nucleo e il nucleo è a 2,1-3,5×; ORM resta 7× e WP 1,75× su TUTTI gli 8 pin. Non esiste a registro
una decomposizione di TEMPO dell'ORM (37 s) per categoria (call/prop/str/builtin/Sweep/IC-miss/alloc): esistono census
di FREQUENZA (S-176: op, Sweep 10,9 %, IC miss 55 %) e UNA cifra di tempo altrui (Astra: fill IC privati 0,42 s). Il veto
«leve su census di FREQUENZA senza cifra di TEMPO» è già nel NEXT. Per la rotta B la fetta 0 obbligata è dunque un
census di TEMPO bilaterale sull'ORM (o sulla media, 2,43×) con mutante — l'equivalente della fetta 0 del S-167.
Ipotesi a verdetto (non cifre): (h1) i corpi dei builtin (str/array/preg/PDO) dominano ORM e i micro non li vedono
(str 4,3 è l'unica micro ferma ed è quella dei corpi) · (h2) il profilo di chiamata ORM (metodi polimorfi, IC miss 55 %,
closure, generatori) resta sul cammino generale `ret_slow`/`method_call` che le forme fuse non ammettono · (h3) alloc/GC:
Sweep 10,9 % degli op ORM e il modello a confine di istruzione (Q3) costano tempo che i micro, senza oggetti, non pagano.

## VINCOLI E APPARATO (invariati, più i nuovi)
Safe-only Rust · corpus congelato per NOME ×2 modi (1409) + golden di contenuto · byte-parity nelle stringhe PHP ·
REGOLE §3 (criterio prima, gemello, banda-layout, ri-risoluzione) · veti trasversali NaN-boxing/fn-table/arena/BOLT/PGO
(il concilio PUÒ levarli o confermarli) · PHPR_REG_LOWER on · toolchain 1.98.1 pinnata · CI locale per-commit (allarme,
non gate) con mutex misure 4 h · coppie SOLO notturne non presidiate (veto S-184 bis) · E5 ORM (ORA_REF 4,870, banda
[4,81;4,93]) · dente LOC (run.rs 7551, mod.rs 26075) · tree oggi = pin s183 + cura 22/10 NON misurata (micro di guardia
dovuta PRIMA di qualunque promozione) · gate invariante fxmc2 (wp172) da emendare.

## ROTTE CANDIDATE (da refutare/ordinare; NON esaustive)
A **nucleo fino a 6/6**: str (census str: concat/interpolazione, Zval::Str clone/alloc — nessun giudice str-dq ancora) e
calls 3,5 (residuo del Call generale, ArgPlace, `ret_slow`); resa attesa alta sui micro, NULLA sui reali finché h1-h3
non sono firmate.
B **carico reale ORM**: fetta 0 = census di TEMPO bilaterale per categoria con mutante (gate ≥85 % di chiusura, come
S-167) ⇒ leve per NOME sul canale dominante ≥ 1 s/37 s; kill: nessun canale ≥ 1 s nominato ⇒ B muore, si va ad A o D.
C **modello GC**: C1 rilascio al punto di morte (criterio PRIMA, A/B su mcalls-dq/calls-dq/prop-dq con attesa |D| <
rumore, fixture bilaterali dtor) · C2 dichiarazione della famiglia come divergenza di timing chiusa (catalogo, stop cure)
· C3 ibrido attuale: confini di frame e unwind curati, pop di temporanei dichiarato.
D **tappa sul reale**: WP ≤ 1,5 e ORM ≤ 5 come tappa DICHIARATA (la R4 del S-167 mai scritta), 1× mantenuto come
orizzonte o tolto dal NEXT. Esplicita, non implicita.
P **processo** (team-governo): la densità del NEXT (25 righe da 1-2k caratteri), 26 incidenti in 18 sessioni con
picco sull'apparato, il dente LOC alzato 11 volte: proporre UNA regola che entra sostituendone una (cap 25).

## COLLEGIO PROPOSTO (le 9 sedie del S-167, lente aggiornata) + 3 team
Hoare (invarianti, Q3) · Hejlsberg (costo nominato nel handler, Q1) · Stogov (Zend: dove sta davvero il 7× ORM, Q2) ·
Matsakis (forme generiche vs cucite sul driver, Q2-A) · Gregg (misura: fetta 0 di TEMPO, mutante, Q2-B) · Pedersen
(parità e byte-id, Q3) · Klabnik (processo e kill, Q1/P) · Leijen (modello GC/ownership, Q3) · Bak (soffitti e tetti
fusi, Q1/D). Team: sonda (fetta 0 ORM), meccanismo (sequenza fette), governo (regole, D e P).
Formato atteso: 9 verbali INDIPENDENTI ≤400 parole in `wp186-harness/verbali/` (fonte vincolante), poi 3 team, poi
sintesi `COUNCIL_WP186_REVIEWS.md` con verdetto per Q1/Q2/Q3, dissensi a registro col nome della sedia.

## §FONDAMENTALI (obbligo di apertura sintesi)
Sessioni dal S-167: 18 (+2 bis). Sessioni-senza-misura: 1 (S-182, dichiarata). Pin nuovi: 8; leve promosse: 9;
leve revertate: 1 (L-CM1). Oggetto avanzato: nucleo 5,4/5,5/4,8 → 2,1/2,3/3,5; corpus 1412→1409 senza pass→fail.
Rischio d'oggetto più trascurato: **i carichi reali sono FERMI da 18 sessioni e nessuno ha una cifra di TEMPO che dica
perché** — è ESATTAMENTE la domanda Q2 di questo concilio. Rischio di processo: il kill del S-168 è scattato e la
campagna è proseguita sotto altro nome senza che il concilio o il NEXT lo abbiano registrato come tale (Q1).

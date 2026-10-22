# TEAM-SONDA (Gregg·Bak·Leijen) — riconciliazione S-186, fase 2

Generato con assistenza AI (Claude Fable 5).

## CONVERGENZE (3/3)
Q2 = rotta B, GO-CONDIZIONATO alla fetta 0 di TEMPO sull'ORM; nessun codice prima della cifra; h1-h3 ipotesi («63 ns/op» = lettura, WP_SESSION_176 r.10; «37 s» = build Astra, WP_SESSION_179 r.3). Gamba ORM al pin: REF2 [34,23;34,31] s vs ORA_REF 4,870 (NEXT r.11; corregge Gregg «[34,16;34,30]») ⇒ surplus ≈29,4 s. Oracle `/opt/homebrew/opt/php/bin/php` 8.5.7, `php -i` oggi: opcache.enable_cli=Off, jit=disable.

## LA FETTA 0 (1 notte, pin s183, catena E5 non presidiata, mutex CI, Data ≥10G)
**Strumento**: `xctrace` Time Profiler su UNA gamba ORM intera per ENTRAMBI i binari, per immagine e simbolo, in 8 classi: dispatch · call/ret (fast vs `ret_slow`/`method_call` generale) · prop/IC · str · array/hash · **ciclo di vita oggetto** (alloc, nota, Sweep, dtor, drop-glue) · builtin (lookup per nome vs corpo) · PDO/sqlite3/avvio = **pavimento comune**, sottratto da entrambi i lati. Secondi; R=3 interleaved O/P, mediana.
**Bracci**: O (oracle) · P (phpr pin) · **M** (mutante) · **C** (phpr, feature `census`: Instant 1/64 per classe + ammissione fast/generale per causa + N_obj dal gc-census); C calibra P entro ±15 %, altrimenti conta solo xctrace.
**Mutante** (prima della lettura): spin 200 ns in CallBuiltin ⇒ atteso +2,38 s su 11,91M (census-op-orm.txt r.16); wall in [2,1;2,7] s E ≥75 % del Δ in classe builtin, altrimenti strumento morto.
**Tetti pre-registrati** (count × surplus del micro affine): metodi 20,2M×104 ns = 2,1 s; IC 4,98M×234 ns = 1,2 s; oggetti N_obj×D_obj (companion obj-churn R=5). Classe >2× il tetto ⇒ modello falso, dichiarato.
**Chiusura**: Σ classi ≥85 % di (P − pavimento), su entrambi i lati.
**Firma GO-leva**: un canale con (P_c − O_c) ≥ **3,0 s** (10 % del surplus) E P_c/O_c ≥ 3; canali in [1,5;3,0) s nominati a registro, non leva.
**Kill**: nessun canale ≥3,0 s, chiusura <85 % o mutante muto ⇒ B muore NELLA STESSA sessione; riga 1 NEXT scrive D (ORM ≤5 netto pavimento, WP ≤1,5).

**Cura 22/10, prezzata PRIMA del pin s184** (stessa notte): `gc_sweep_after_ret` è O(|gc_buf|) per Ret (mod.rs 4160-4188), muto a buffer vuoto. Giudici: ret-buf-dq K∈{0,4,16} (K temporanei oggetto prima di `$o->m()`), driver Hoare E3 N=10⁵/2·10⁵, mcalls-dq/calls-dq ABAB tree vs s183, R=5. Attesa: K=0 |D| < max(rumore, 4 ns/iter); K>0 piatto; ratio E3 ≤2,5. Morde ⇒ hook in `ret_slow` + E-LE-1/E-LE-2.

## CONFLITTI REGISTRATI
- **Gregg vs Bak**, GO-leva 1,5 s vs 3 s: 3,0 s per la leva, 1,5 s per la nomina; Gregg dissente sul kill in 1 notte anziché 2 sessioni.
- **Gregg**, bracci O0/P0 «suite nulla»: respinti per apparato (regola 7); il pavimento esce per immagine (`floor3` del canone misura solo `--version`, s185-orm-coppia-t27.sh:106).
- **Bak**, Instant giudice primario: declassato a calibrazione (unilaterale).
- **Leijen**, KS-LE-186-3 (D_obj×N_obj <1 s): tetto su h3, non kill di B.

## DURATA
1 notte: O/P/M/C R=3 (~45 min), obj-churn, guardia cura (~30 min); verdetto in S-187.

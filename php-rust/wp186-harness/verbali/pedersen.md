# Verbale sedia PEDERSEN — S-186 (bozza indipendente, FASE 1)

Generato con assistenza AI (Claude Fable 5). Lente: parità byte-id e gate bilaterali.

## VERDETTI
- **Q1: AGGIRATO** — kill S-168 scattato, stessa rotta con nome nuovo; «≥1×» ancora in NEXT riga 1 e REGOLE §1: diventi orizzonte, tappa reale dichiarata.
- **Q2: B, GO-CONDIZIONATO** (fetta 0 = census di TEMPO bilaterale con mutante). D «WP ≤1,5» è MISURABILE (banda_ON ≤0,030 contro un salto di 0,25; parità = fail per NOME + full-diff, s184-pair.sh:274) ma è tappa senza leva: si dichiara.
- **Q3: C3**. C1 giudicabile byte-id forma per forma; C2 solo per INVARIANZA pin==stash, il gate debole che oggi morde: no.

## ROTTA (una riga)
Prima gate auditabile (congelati sotto git, fxmc2 a diff-dichiarato), poi guardia micro della cura 22/10, poi pin s184; B parte dalla fetta 0 di tempo.

## REFUTAZIONI ED EMENDAMENTI
1. **«fail-set e golden EMENDATI nel commit» (NEXT riga 12) è FALSO**: `wp109-harness/.gitignore:9` ignora `corpus-gate/`; `git ls-files` e `git log` sulla cartella sono VUOTI; 4aa30319 tocca 7 file, nessun congelato. I 1409×2 e i golden vivono solo su questo disco (CI: path cablato, ci/README.md:30). Emenda: i 4 file entrano in git; REGOLE §5 dice ancora «1412».
2. **Ri-basare il golden è lecito**: è artefatto di INVARIANZA (output sbagliato pinnato), non di correttezza; perde senso solo se non dichiarato E non versionato — oggi il secondo. Verificato: bug47771 e bug70685 sono «sbagliato→sbagliato diverso».
3. **fxmc2 morde: CONFERMATO** su dev 21:37 (precede il commit 21:53: dichiarare). C1 dev `E1:dtor:Q dtor:R Cannot…`, oracle `dtor:Q E1:…`; C4 dev==oracle. Emenda: `invar` → **diff dichiarato**: `diff(oracle,pin)` == `.expected-diff` committato; oracle fisso, ogni slittamento si ri-dichiara.
4. **«2 load + `any`, vuoto nei loop caldi»**: `gc_sweep_after_ret` (mod.rs:4175) scandisce TUTTO `gc_buf` a ogni Ret quando non vuoto: O(buffer) sull'ORM. Guardia: mcalls-dq E coppia ORM.
5. Verificato: PHP di `leave_dtor_cmp_recursion.rs` oracle==dev IDENTICO; wp185 fx-mc2 oracle==pin==dev. Gate fixture s185: 24, non 23.
6. Famiglia timing: UNA voce-madre (§3.33 + §3.35(2) + unwind + generatori a shutdown + bug47771), fixture bilaterale per forma; §3.28(i) e §3.22 separate.

## KILL-SWITCH PRE-REGISTRABILI
- KS-PE-186-1: promozione con congelati non tracciati ⇒ gate VOID.
- KS-PE-186-2: ri-basamento «sbagliato→sbagliato» senza voce di catalogo PRIMA ⇒ STOP cura.
- KS-PE-186-3: |D| mcalls-dq/calls-dq ≥ rumore o ORM fuori banda ⇒ `gc_sweep_after_ret` solo in `ret_slow`, Ret fuso al byte.

## PRIMA FETTA
Promozione s184 della cura 22/10. Giudice: mcalls-dq + calls-dq ABAB pin s183 vs tree, R=5, soglia |D| < max(4 ns, rumore) pre-registrata; gate: congelati committati, corpus-gate rc=0, fxmc2 a diff-dichiarato, `leave_dtor` bilaterale nella chain, coppia notturna in banda.

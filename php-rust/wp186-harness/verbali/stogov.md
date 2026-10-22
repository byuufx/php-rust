# Verbale sedia STOGOV — S-186 (bozza indipendente, FASE 1)
Generato con assistenza AI (Claude Fable 5).

## VERDETTI
- **Q1: AGGIRATO** (stessa rotta, nome nuovo; 1× ancora in NEXT r.1 e REGOLE §1), ma redditizio. Mio KS «46,8→≤35»: RAGGIUNTO in S-171 (arith-dq 23,44, GAP_TREND r.195), muto sul reale.
- **Q2: B + D, GO condizionato** alla fetta 0 di TEMPO; A solo per str, dopo.
- **Q3: C3** (ibrido); C1 vietata nel loop caldo finché non prezzata.

## ROTTA (una riga)
Profilo bilaterale per libreria e simbolo su una gamba ORM intera, pavimento comune (sqlite3) sottratto, leve per NOME sul canale ≥1 s; tappa dichiarata ORM ≤5 / WP ≤1,5.

## REFUTAZIONI ED EMENDAMENTI
1. **«63 ns/op» è un TETTO** (PERF_MAP r.30): divide i 34,5 s TOTALI per gli op, caricandovi compile, PDO, sqlite3, I/O. L'unico profilo di tempo è unilaterale e vecchio (PERF_MAP r.560, S-136: clone/drop, gc_note/sweep, malloc «multi-%»; compile ≤1 %; reflection <0,5 %): indizia h3, non firma. Il veto «census di FREQUENZA» vale per h1-h3 finché manca la fetta 0.
2. **Oracle = interprete puro**: `php -i` dà opcache.enable_cli=Off, jit=disable.
3. **Suite su pdo_sqlite in memoria** (orm-work/phpunit.xml.dist r.42-43): sqlite3 è la STESSA libreria C nei due motori ⇒ pavimento incomprimibile; il rapporto motore-su-motore è PEGGIORE di 7, il ≤5 va definito al netto.
4. Dal sorgente Zend: **h3** — refcount-zero costa un bit-test `Z_REFCOUNTED` + decremento inline, `rc_dtor_func` (zend_variables.c r.54) solo a zero, dtor subito; cicli SOLO a 10 000 radici (zend_gc.c r.191). phpr paga la drop-glue Rust PIÙ nota/sweep: doppio lavoro, h3 credibile. **h1** — Zend concatena con realloc in place e interned; str 4,3 ferma è la firma di un clone. **h2** — Astra ha prezzato il FILL (1 %), non il cammino lento per chiamata: aperta.

## KILL-SWITCH PRE-REGISTRABILI
- Fetta 0: chiusura <85 % o mutante muto ⇒ nessuna leva ORM, rotta A (str).
- Pavimento comune >40 % del tempo phpr ⇒ ORM ≤5 irraggiungibile, tappa riscritta al netto.
- Campagna B: ORM ≤5,0 (coppia E5) entro 6 sessioni di leva (S-187..S-192), ≥2 leve firmate per NOME; altrimenti B MORTA e NEXT scrive «1× = orizzonte».

## PRIMA FETTA
`xctrace` Time Profiler su UNA gamba ORM completa per ENTRAMBI i binari (simboli presenti in entrambi), aggregato per immagine (binario, libsqlite3, malloc, regex) e top-30 simboli in 8 classi: handler · call/ret · prop/IC · str · array/hash · alloc/drop/gc · builtin+PDO · compile. **Giudice**: secondi per classe su 34 s, R=3. **Mutante**: +1 s di str_repeat che riappare in «str» ±20 %. **Soglia GO-leva**: un canale ≥1,0 s con quota phpr ≥3× quella Zend.

#!/bin/bash
# next-budget.sh — giudice della regola REGISTRO (REGOLE §8, concilio S-186, firma utente 2026-10-23).
# NEXT_SESSION_WORDPRESS.md: ≤80 righe E ≤8.000 caratteri, nessuna riga >600 caratteri.
# rc 0 = dentro budget; rc 1 = fuori budget (stampa le cifre e le righe lunghe). Si lancia in chiudi-sessione.
# Generato con assistenza AI (Claude Fable 5).
set -u
F="${1:-$(cd "$(dirname "$0")/.." && pwd)/NEXT_SESSION_WORDPRESS.md}"
[ -r "$F" ] || { echo "next-budget: file non leggibile: $F"; exit 2; }
MAX_LINES=80; MAX_CHARS=8000; MAX_LINE=600
lines=$(wc -l < "$F" | tr -d ' ')
chars=$(python3 -c 'import sys; print(len(open(sys.argv[1],encoding="utf-8").read()))' "$F")
longest=$(python3 -c 'import sys; print(max(len(l) for l in open(sys.argv[1],encoding="utf-8").read().split("\n")))' "$F")
rc=0
echo "next-budget: righe $lines/$MAX_LINES · caratteri $chars/$MAX_CHARS · riga max $longest/$MAX_LINE"
[ "$lines" -le "$MAX_LINES" ] || { echo "  FUORI: righe $lines > $MAX_LINES"; rc=1; }
[ "$chars" -le "$MAX_CHARS" ] || { echo "  FUORI: caratteri $chars > $MAX_CHARS"; rc=1; }
if [ "$longest" -gt "$MAX_LINE" ]; then
  rc=1
  python3 -c 'import sys
m=int(sys.argv[2])
for i,l in enumerate(open(sys.argv[1],encoding="utf-8").read().split("\n"),1):
    if len(l)>m: print(f"  FUORI: riga {i} ha {len(l)} caratteri > {m}")' "$F" "$MAX_LINE"
fi
[ $rc -eq 0 ] && echo "next-budget: PASS" || echo "next-budget: FAIL"
exit $rc

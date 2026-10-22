//! Fidelity cures of 2026-10-22 (catalogue PHPR_DIVERGENCES_FROM_PHP.md):
//! §3.34 — comparing a cyclic object graph throws Zend's catchable `Error`
//! instead of overflowing the stack (Zend/tests/bug63882.phpt);
//! §3.35 forms 1 and 3 — the destructors of a returning frame's dying locals
//! and arguments run at `leave`, before the caller continues, in forward slot
//! order. Expected strings are the oracle's (php 8.5.7 CLI) byte for byte.
use php_runtime::run_source;

fn out(src: &str) -> String {
    let o = run_source(b"t.php", src.as_bytes()).expect("lowers");
    assert!(o.fatal.is_none(), "unexpected fatal: {:?}", o.fatal);
    String::from_utf8(o.stdout).expect("utf8")
}

#[test]
fn recursive_object_comparison_throws_nesting_error() {
    // bug63882: `$a->x = $a; $b->x = $b; $a == $b` recursed until SIGABRT.
    // Every comparison operator goes through the same guard; a non-cyclic
    // shared sub-object (right reachable from left) must NOT trip it.
    // (`run_source` has no builtins: booleans are rendered by `b()`.)
    let src = r#"<?php class Test { public $x = 5; }
function b($v) { return $v ? "true" : "false"; }
$a = new Test; $b = new Test; $a->x = $a; $b->x = $b;
try { echo b($a == $b), "\n"; } catch (Error $e) { echo $e->getMessage(), "\n"; }
try { echo b($a != $b), "\n"; } catch (Error $e) { echo "ne: ", $e->getMessage(), "\n"; }
try { echo $a <=> $b, "\n"; } catch (Error $e) { echo "ss: ", $e->getMessage(), "\n"; }
try { echo b($a < $b), "\n"; } catch (Error $e) { echo "lt: ", $e->getMessage(), "\n"; }
try { echo b($a >= $b), "\n"; } catch (Error $e) { echo "ge: ", $e->getMessage(), "\n"; }
try { echo b([$a] == [$b]), "\n"; } catch (Error $e) { echo "arr: ", $e->getMessage(), "\n"; }
$c = new Test; $p = new Test; $q = new Test; $p->x = $c; $q->x = $c;
echo b($p == $q), "\n";
$r = new Test; $r->x = $p; $s = new Test; $s->x = $q;
echo b($r == $s), b($r === $s), b($a == $a), b($a === $b), "\n";
echo "after\n";
"#;
    assert_eq!(
        out(src),
        "Nesting level too deep - recursive dependency?\n\
         ne: Nesting level too deep - recursive dependency?\n\
         ss: Nesting level too deep - recursive dependency?\n\
         lt: Nesting level too deep - recursive dependency?\n\
         ge: Nesting level too deep - recursive dependency?\n\
         arr: Nesting level too deep - recursive dependency?\n\
         true\ntruefalsetruefalse\nafter\n"
    );
}

#[test]
fn comparison_guard_is_released_after_the_error() {
    // The protected object leaves the guard stack with the error: comparing
    // it again later is a fresh comparison, not a stale "recursive" hit.
    let src = r#"<?php class Test { public $x = 5; }
$a = new Test; $b = new Test; $a->x = $a; $b->x = $b;
try { $a == $b; } catch (Error $e) { echo "1\n"; }
try { $a == $b; } catch (Error $e) { echo "2\n"; }
$a->x = 1; $b->x = 1; echo ($a == $b) ? "eq\n" : "ne\n";
"#;
    assert_eq!(out(src), "1\n2\neq\n");
}

#[test]
fn dying_locals_destruct_at_leave_before_the_caller_continues() {
    // §3.35 form 1 (local of the callee), form 3 (argument temporaries, forward
    // order), cascade through a destructed parent, and a destructor throwing at
    // leave (the return value is discarded, the exception reaches the caller).
    let src = r#"<?php
class X { function __construct(public $n) {} function __destruct() { echo "dtor {$this->n}\n"; } }
class O { function mk($a) { $x = new X("local-$a"); return $a; } function m($a, $b) { return $a + $b; } }
function d($n) { return new X("d$n"); }
function mkf($a) { $x = new X("flocal-$a"); return $a; }
function f($x) { return 1; }
$o = new O;
echo "A: ", $o->mk('a'), "\n";
echo "A2: ", mkf('a'), "\n";
echo "A3: " . $o->mk('c') . "\n";
$v = $o->mk('b'); echo "C: ", $v, "\n";
echo "D: ", f(new X('arg')), "\n";
$r = $o->m(f(d(4)), f(d(5))); echo "F: $r\n";
echo "G: ", f(d(6)), f(d(7)), "\n";
function two($a, $b) { return 0; } echo "H: ", two(new X('h1'), new X('h2')), "\n";
class T { function __destruct() { throw new Exception("boom"); } }
function g() { $t = new T; return 5; }
try { $v = g(); echo "got $v\n"; } catch (Exception $e) { echo "caught ", $e->getMessage(), "\n"; }
class K { public $kid; function __construct() { $this->kid = new X('kid'); } function __destruct() { echo "dtor K\n"; } }
function h() { $k = new K; return "h"; }
echo "I: ", h(), "\n";
echo "end\n";
"#;
    assert_eq!(
        out(src),
        "A: dtor local-a\na\n\
         A2: dtor flocal-a\na\n\
         dtor local-c\nA3: c\n\
         dtor local-b\nC: b\n\
         D: dtor arg\n1\n\
         dtor d4\ndtor d5\nF: 2\n\
         G: dtor d6\n1dtor d7\n1\n\
         H: dtor h1\ndtor h2\n0\n\
         caught boom\n\
         I: dtor K\ndtor kid\nh\n\
         end\n"
    );
}

#[test]
fn shared_local_survives_leave_and_destructs_later() {
    // A local still referenced elsewhere is NOT collectable at leave: the
    // guard (`strong_count == 2`) leaves it alone and the usual statement-end
    // sweep handles its eventual death.
    let src = r#"<?php
class X { function __construct(public $n) {} function __destruct() { echo "dtor {$this->n}\n"; } }
function k() { global $box; $x = new X('shared'); $box = $x; return "k"; }
echo "A: ", k(), "\n";
$box = null;
echo "end\n";
"#;
    assert_eq!(out(src), "A: k\ndtor shared\nend\n");
}

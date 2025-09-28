# slippy

slippy is a Scheme interpreter.

Or at least that is what it aspires to be when it grows up.

## Build

To build this, do this:

```
$ cd slippy
$ cargo build
```

## Usage

After the above, which hopefully succeeded, you can do:

```
$ ./target/debug/slippy
```

That will start a REPL, like:

```
$ ./target/debug/slippy
slippy> (+ 2 2)
4
```

Or, you can evaluate a source file, like:

```
$ ./target/debug/slippy examples/fact.ss
#unspecified
3628800
```

`examples/` is what slippy is capable of right now, and possibly forever into
the heat death of the universe.


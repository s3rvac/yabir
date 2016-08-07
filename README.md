Yet Another Brainfuck Interpreter in Rust
=========================================

Lately I have been learning [Rust](https://www.rust-lang.org/). As my first
project, I decided to write a
[Brainfuck](https://en.wikipedia.org/wiki/Brainfuck) interpreter. Feel free to
submit [issues](https://github.com/s3rvac/yabir/issues) or [pull
requests](https://github.com/s3rvac/yabir/pulls) on how may I improve the code!

Building and Running
--------------------

* Ensure that you have a [Rust](http://www.rust-lang.org/install.html)
  installed, including [cargo](https://crates.io/install) (Rust's build and
  package manager). The project was tested with Rust 1.10.
* Clone the repository:

```
$ git clone https://github.com/s3rvac/yabir
```

* Build the interpreter:

```
$ cargo build --release
```

* Run it on a Brainfuck program:

```
$ target/release/yabir PROG
```

The [`programs`](https://github.com/s3rvac/yabir/tree/master/programs)
directory contains a few sample programs you can try.

Implementation Notes
--------------------

As stated [here](https://en.wikipedia.org/wiki/Brainfuck#Portability_issues),
there are some portability issues that developers of Brainfuck interpreters and
programs need to be aware of. My implementation behaves as follows:

* Cells are 8-bit unsigned integers. They wrap around (e.g. when incrementing
  255, you get 0).
* The data array has potentially unlimited size and is automatically resized.
  However, the array can be extended only to the right.
* Emitted [ends of lines](https://en.wikipedia.org/wiki/Newline) are
  platform-specific (e.g. LF on Linux).
* The behavior of the `,` command (read a byte from the input stream) when an
  end-of-file condition has been encountered is to leave the current cell
  unchanged.

License
-------

Licensed under either of

* Apache License, Version 2.0,
  ([LICENSE-APACHE](https://github.com/s3rvac/yabir/tree/master/LICENSE-APACHE)
  or http://www.apache.org/licenses/LICENSE-2.0)
* MIT License
  ([LICENSE-MIT](https://github.com/s3rvac/yabir/tree/master/LICENSE-APACHE) or
  http://opensource.org/licenses/MIT)

at your option.

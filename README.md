rust-jvmti
==========

An extensible, safe native JVM agent implemented in pure Rust.

## A word of warning

This project is far from being complete or usable and contains a healthy dose of
proof-of-concept code.

## Planned features

* JVM byte code instrumentation/transformation
* Dynamic tracing/profiling

## Usage

*Don't.* Although the code is supposed to be safe and correct, it's highly experimental
and immature in general. Loading this agent may corrupt your soul as well as your JVM but
at least in a type-safe way.

If you really insist on trying this library then build it with the ususal `cargo build --release` and
fire up a Java VM with a command something like

```java -agentpath:./target/release/libjvmti.so MyClass```

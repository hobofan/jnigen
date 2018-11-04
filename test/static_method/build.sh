#! /usr/bin/env bash
set -euxo pipefail

(cd mylib && cargo build && cargo jnigen)
cp -r mylib/target/jnigen/* .
javac some/pkg/HelloWorld.java
javac Main.java
java -Djava.library.path=mylib/target/debug/ Main

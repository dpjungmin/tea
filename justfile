alias l := list
alias b := build
alias t := test

default: build

list:
  @just --list

build:
  @cargo build

test:
  @cargo check && cargo clippy && cargo test

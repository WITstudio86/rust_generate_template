# Rust Generate Template

## preparatory

```shell
$ cargo install cargo-generate
```

## use

- first , install cargo make

```shell
$ cargo install cargo-make
```

- edit `Makefile.toml` add what crate you need ,default auto push 

- replace your repo url in `Cargo.toml` line 56 

- run : `cargo make commit` auto check code and addd commit
- run : `cargo make tag` auto add git-tag use version table in Cargo.toml
- run : `cargo make all` commit + tag

Hook `open` and `open64` libc calls to avoid updating atimes while searching or the like.

Why
===

atimes are useful for a lot of reasons so you may not want to disable them completely. However with advanced searching tools like [ripgrep][ripgrep] you'll tend to search a lot more files than actually needed, thus updating atimes of a lot of files. You'll lose track of last actual use of the files as well as being slow because of disk writes.

Works with
====

This is known to work with [ripgrep][ripgrep] on Linux with glibc.

Installation
====

```sh
cargo build --release
sudo install -Dsm755 target/release/libopen-noatime.so /usr/local/lib/
sudo ldconfig
```

Usage
====

```sh
LD_PRELOAD=libopen_noatime.so rg ...
```

You'll want to create a wrapper, a shell function, or an alias for it.

[ripgrep]: https://github.com/BurntSushi/ripgrep

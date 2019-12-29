This https://github.com/therealprof/microbit links to https://droogmic.github.io/microrust

## Build

`cargo build --target thumbv6m-none-eabi`

Crates added with `cargo add <crate>` from `cargo install cargo-edit`

## Use master branch of book

https://github.com/droogmic/microrust/issues/11#issuecomment-548430577


## Rust stable

There is also update for rust stable https://github.com/droogmic/microrust/pull/16

## Errors

First openocd returned:

```
Info : accepting 'gdb' connection on tcp/3333                                                                                                                                                
undefined debug reason 7 - target needs reset
```

When I run `gdb-multiarch` and entered command `(gdb) target remote :3333`

Here https://docs.rust-embedded.org/discovery/05-led-roulette/flash-it.html I read I need to

`monitor reset halt`

After another thing:

```
(gdb) target remote :3333                                                                                                                                                                    
Remote debugging using :3333                                                                                                                                                                 
0x000006d0 in core::sync::atomic::compiler_fence () at /cargo/registry/src/github.com-1ecc6299db9ec823/compiler_builtins-0.1.18/src/int/udiv.rs:268                                          
268     /cargo/registry/src/github.com-1ecc6299db9ec823/compiler_builtins-0.1.18/src/int/udiv.rs: No such file or directory.
```

Here https://github.com/rust-lang/compiler-builtins/issues/75#issuecomment-472877130 I read that:

`#276 removed the thumbv6m config from the intrinsics example`

There was this issue for adding thumbv6m https://github.com/rust-lang/compiler-builtins/issues/33


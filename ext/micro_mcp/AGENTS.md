# Rust Development Agent Instructions

This directory contains Rust code that interfaces with Ruby via FFI. These instructions help agents navigate Rust source code efficiently.

## Environment Setup

The following environment variables should be exported for Rust source navigation:

```bash
# Standard library source (requires `rustup component add rust-src`)
export RUST_STD_SRC="$(rustc --print sysroot)/lib/rustlib/src/rust/library"

# Crates.io tarball cache (downloaded by `cargo fetch`)
export RUST_CRATE_SRC="${CARGO_HOME:-$HOME/.cargo}/registry/src"

# Git checkouts for dependencies using `git = "..."`
export RUST_GIT_SRC="${CARGO_HOME:-$HOME/.cargo}/git/checkouts"
```

## Helper Commands

### Locating Standard Library Items

```bash
rg "pub .* <TypeOrFnName>" "$RUST_STD_SRC"
```

### Finding Versioned Crates

```bash
# Example: finding serde 1.0.200
rg --files "$RUST_CRATE_SRC" | grep '/serde-1\.0\.200/' | head -n1
# Then search inside that path
```

### Locating Git Dependencies

```bash
# Example: finding a crate directory containing 'mycrate'
find "$RUST_GIT_SRC" -maxdepth 3 -type d -name '*mycrate*' | head
```

### Quick Jump to Crate Root

```bash
cargo metadata --format-version 1 --no-deps \
  | jq -r '.packages[] | select(.name=="tokio") | .manifest_path' \
  | xargs dirname
```

## Agent Rules

1. **Source Code Lookup Order**: Always search in this order:
   - `$RUST_STD_SRC` (standard library)
   - `$RUST_CRATE_SRC` (crates.io dependencies)
   - `$RUST_GIT_SRC` (git dependencies)

2. **Exact Signatures**: Copy the **exact** function/type signature you find in the source. If no match is found, report:
   - Which symbol is missing
   - Which directory was searched
   - Stop and ask for clarification

3. **FFI Considerations**: This project uses Ruby FFI, so pay attention to:
   - Memory management between Rust and Ruby
   - Proper error handling across language boundaries
   - Thread safety considerations

## Ruby & Rust GC Rules

- Ruby's GC can't see values stored in ordinary Rust memory. Anything kept in a
  `Vec`, `HashMap`, `OnceCell`, or static variable must be rooted or pinned.
- Use `value::BoxValue<T>` to pin individual Ruby `Value`s. `BoxValue::new` will
  `rb_gc_register_address` the object so it stays valid.
- Expose Rust structs to Ruby with `#[magnus::wrap]`/`#[derive(TypedData)]` and
  `Obj<T>`. `Obj<T>` is just a typed handle, so wrap it again in `BoxValue` (or
  register it) if you keep it on the Rust side.
- For containers of many Ruby values, wrap the container itself in a `TypedData`
  struct and implement a `mark` method to mark each entry. Root that single
  wrapper instead of each entry.
- When using globals, prefer a wrapper similar to:

  ```rust
  static REG: OnceCell<Obj<Registry>> = OnceCell::new();
  let reg = REG.get_or_init(|| {
      let obj = Obj::wrap(Registry::new());
      magnus::gc::register_mark_object(obj); // root the wrapper
      obj
  });
  ```

## Testing Checklist

1. Call `GC.start` and `GC.compact` after each Ruby call in tests to stress the
   garbage collector.
2. Run `cargo miri` (or valgrind) to detect use-after-free in unsafe code.
3. Remember `Obj<T>` is not `Send`/`Sync`; `BoxValue<T>` is. Convert or guard
   before crossing threads.

# nu_plugin_umoka

> [!WARNING]
> This project was written with AI assistance. Review the code carefully before using it in production or security-sensitive environments.

A small Nushell plugin that exposes a bounded in-memory `key -> value` store.

The store is backed by [MicroMoka](https://crates.io/crates/micro-moka), a lightweight in-memory cache for Rust.

## Subcommands

- `umoka put <key> [value]`
  Store a value under `key` and return the stored value. The value can be provided either as pipeline input or as the second positional argument, but not both.
- `umoka put-if-absent <key> [value]`
  Store a value under `key` only if the key does not already exist. Returns a record with `inserted` and `value`.
- `umoka get <key>`
  Return the value stored under `key`, or `nothing` if the key does not exist.
- `umoka get-or-put <key> [value]`
  Return the current value for `key`, or store and return the provided fallback value if the key is missing.
- `umoka take <key>`
  Return the value stored under `key` and remove it from the store. Returns `nothing` if the key does not exist.
- `umoka delete <key>`
  Remove the value stored under `key` and return `true` if an entry was deleted, otherwise `false`.
- `umoka has <key>`
  Return `true` if `key` exists in the store, otherwise `false`.
- `umoka clear`
  Remove all entries from the store and return `true`.
- `umoka incr <key> [delta]`
  Atomically increment the integer value stored under `key`. Missing keys are initialized from `delta`, which defaults to `1`.
- `umoka stats`
  Return a record describing the store state. Currently this includes `entry_count` and `max_capacity`.

Notes:

- Developed for use in conjuction with the awesome [http-nu](https://github.com/cablehead/http-nu), so data can be stored and retrieved between requests.  
  It is a lighweight alternative to:
  - nushell's built-in `stor` when you don't need/want relational-ness of SQL
  - [cross-tream](https://github.com/cablehead/xs) if you don't need/want to stream-ness, or to persist data
- Values are stored only in memory, and are bound to nushell process.
- Capacity is bounded by entry count via [MicroMoka](https://crates.io/crates/micro-moka).
- The plugin intentionally does not implement TTL or expiration policy on top of MicroMoka.
- `umoka put` returns the stored value and raises an error if storing fails.
- For concurrent `http-nu` use, prefer `put-if-absent`, `get-or-put`, and `incr` over multi-step read-modify-write flows.

## Concurrency Warnings

- Store operations are atomic only per individual subcommand.
- Multi-step workflows such as `has` then `put`, or `get` then `delete`, are not atomic and can interleave under concurrent use.
- This makes `umoka` a poor fit for coordination patterns such as locks, counters, deduplication, or session management in concurrent `http-nu` handlers.
- If you use `umoka` with `http-nu`, treat it as a small process-local in-memory cache, not as a concurrency-safe shared state system.

## Acknowledgments

- This plugin uses [MicroMoka](https://crates.io/crates/micro-moka) for the underlying bounded in-memory store.

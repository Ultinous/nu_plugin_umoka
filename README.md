# nu_plugin_umoka

A small Nushell plugin that exposes a bounded in-memory `key -> value` store.

Current command surface:

- `umoka put <key> [value]`
- `umoka get <key>`
- `umoka take <key>`
- `umoka delete <key>`
- `umoka has <key>`
- `umoka clear`
- `umoka stats`

Notes:

- Values are stored only in memory.
- Capacity is bounded by entry count via `micro-moka`.
- The plugin intentionally does not implement TTL or expiration policy on top of `micro-moka`.
- `umoka put` returns the stored value and raises an error if storing fails.

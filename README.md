# Quadkey

Converting to and from quadkeys

## Usage


```rust


use quadkey::qk;

let qk = qk::tile_to_str(2301, 1305, 14);
// 00120211133103

let tile = qk::str_to_tile("00120211133103")
// Tile { x: 2301, y: 1305, z: 14}

let qk_bin = qk::tile_to_u64(2301, 1305, 14);
// 442897783477764110

let tile = qk::u64_to_tile(442897783477764110)
// Tile { x: 2301, y: 1305, z: 14}

```

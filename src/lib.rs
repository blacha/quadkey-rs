mod qk {
    const CHAR_CODE_ZERO: usize = 48;

    #[derive(Debug, PartialEq)]
    pub struct Tile {
        pub x: usize,
        pub y: usize,
        pub z: usize,
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct QuadKeyParseError;

    pub fn tile_to_u64(x: usize, y: usize, zoom: usize) -> u64 {
        let mut qk: u64 = 0;
        let mut i = zoom;
        while i > 0 {
            let mask = 1 << (i - 1);
            let location = 64 - ((zoom - i + 1) * 2) + 1;

            if (x & mask) != 0 {
                qk = qk | 1 << (location - 1);
            }
            if (y & mask) != 0 {
                qk = qk | 1 << location;
            }
            i = i - 1;
        }

        qk = qk | (zoom as u64);
        return qk;
    }

    pub fn u64_to_tile(qk: u64) -> Tile {
        let zoom = u64_zoom_level(qk) as usize;
        let mut x = 0;
        let mut y = 0;
        let mut i = zoom;
        while i > 0 {
            let mask = 1 << (i - 1);

            let location = 64 - ((zoom - i + 1) * 2) + 1;
            if qk & (1 << (location - 1)) != 0 {
                x = x | mask
            }
            if qk & (1 << location) != 0 {
                y = y | mask
            }
            i = i - 1;
        }

        return Tile { x, y, z: zoom };
    }

    #[inline]
    pub fn u64_zoom_level(bin_qk: u64) -> u64 {
        return bin_qk & 31;
    }

    pub fn tile_to_str(x: usize, y: usize, z: usize) -> String {
        let mut res = vec!['0'; z];

        let mut i = z;
        while i > 0 {
            let mut digit = '0' as u8;
            let mask = 1 << (i - 1);
            if (x & mask) != 0 {
                digit = digit + 1;
            }
            if (y & mask) != 0 {
                digit = digit + 2;
            }

            res[z - i] = digit as char;

            i = i - 1;
        }
        return res.into_iter().collect();
    }

    pub fn str_to_tile(qk: String) -> Result<Tile, QuadKeyParseError> {
        let mut x = 0;
        let mut y = 0;
        let z = qk.len();

        let bytes = qk.as_bytes();
        let mut i = z;
        while i > 0 {
            let mask = 1 << (i - 1);

            let q = (bytes[z - i] as usize) - CHAR_CODE_ZERO;
            match q {
                0 => (),
                1 => x = x | mask,
                2 => y = y | mask,
                3 => {
                    x |= mask;
                    y |= mask;
                }
                _ => return Err(QuadKeyParseError {}),
            }
            i = i - 1;
        }

        return Ok(Tile { x, y, z });
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_converts_tile_to_string() {
        assert_eq!(crate::qk::tile_to_str(0, 0, 0), "");
        assert_eq!(crate::qk::tile_to_str(1, 1, 1), "3");
        assert_eq!(crate::qk::tile_to_str(11, 3, 8), "00001033");
    }

    #[test]
    fn it_converts_from_string() {
        assert_eq!(
            crate::qk::str_to_tile(String::from("00001033")).unwrap(),
            crate::qk::Tile { x: 11, y: 3, z: 8 }
        );
    }

    #[test]
    fn it_round_trips_from_string() {
        let tile = crate::qk::Tile { x: 11, y: 3, z: 8 };
        let tile_qk = crate::qk::tile_to_str(tile.x, tile.y, tile.z);
        assert_eq!(crate::qk::str_to_tile(tile_qk).unwrap(), tile);
    }

    #[test]
    fn it_get_binary_zoom_level() {
        assert_eq!(crate::qk::u64_zoom_level(1), 1);
        assert_eq!(crate::qk::u64_zoom_level(2), 2);
        assert_eq!(crate::qk::u64_zoom_level(4), 4);
        assert_eq!(crate::qk::u64_zoom_level(7), 7);
        assert_eq!(crate::qk::u64_zoom_level(8), 8);
        assert_eq!(crate::qk::u64_zoom_level(24), 24);
        assert_eq!(crate::qk::u64_zoom_level(20024), 24);
    }

    #[test]
    fn it_converts_tile_to_u64() {
        assert_eq!(
            crate::qk::tile_to_u64(1, 1, 1),
            0b1100000000000000000000000000000000000000000000000000000000000001
        );
        assert_eq!(
            crate::qk::tile_to_u64(29, 50, 7),
            0b0010110101100100000000000000000000000000000000000000000000000111
        );
        assert_eq!(
            crate::qk::tile_to_u64(35210, 21493, 16),
            0b110001001001011111010100110011000000000000000000000000000010000
        );
    }

    #[test]
    fn it_converts_from_u64() {
        assert_eq!(
            crate::qk::u64_to_tile(
                0b1100000000000000000000000000000000000000000000000000000000000001
            ),
            crate::qk::Tile { x: 1, y: 1, z: 1 }
        );
        assert_eq!(
            crate::qk::u64_to_tile(
                0b0010110101100100000000000000000000000000000000000000000000000111
            ),
            crate::qk::Tile { x: 29, y: 50, z: 7 }
        );
        assert_eq!(
            crate::qk::u64_to_tile(
                0b110001001001011111010100110011000000000000000000000000000010000
            ),
            crate::qk::Tile {
                x: 35210,
                y: 21493,
                z: 16
            }
        );
    }
}

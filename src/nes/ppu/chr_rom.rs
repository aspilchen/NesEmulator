use std::array::TryFromSliceError;
pub struct ChrTile {
    pub data: Vec<u8>,
}

impl TryFrom<&[u8]> for ChrTile {
    type Error = TryFromSliceError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        return <&[u8; Self::NUM_BYTES]>::try_from(value).map(Self::from);
    }
}

impl From<&[u8; ChrTile::NUM_BYTES]> for ChrTile {
    fn from(value: &[u8; ChrTile::NUM_BYTES]) -> Self {
        return Self {
            data: value.to_vec(),
        };
    }
}

impl ChrTile {
    const NUM_BYTES: usize = 16;
    const STEP_SIZE: usize = 8;

    pub fn read(&self, row: usize, column: usize) -> u8 {
        let upper = self.data[row] >> 7 - column;
        let lower = self.data[row + Self::STEP_SIZE] >> 7 - column;
        let mask = 1;
        let result = ((upper & mask) << 1) | (lower & mask);
        return result;
    }
}

pub struct ChrBank {
    pub tiles: Vec<ChrTile>,
}

impl TryFrom<&[u8]> for ChrBank {
    type Error = TryFromSliceError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        return <&[u8; Self::NUM_BYTES]>::try_from(value).map(Self::from);
    }
}

impl From<&[u8; ChrBank::NUM_BYTES]> for ChrBank {
    fn from(value: &[u8; ChrBank::NUM_BYTES]) -> Self {
        let tiles = value
            .chunks(ChrTile::NUM_BYTES)
            .map(|i| -> ChrTile { ChrTile::try_from(i).unwrap() })
            .collect();
        return Self { tiles: tiles };
    }
}

impl ChrBank {
    const NUM_TILES: usize = 512;
    const NUM_BYTES: usize = Self::NUM_TILES * ChrTile::NUM_BYTES;
}

pub struct ChrRom {
    pub banks: Vec<ChrBank>,
}

impl TryFrom<&[u8]> for ChrRom {
    type Error = TryFromSliceError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let banks = value
            .chunks(ChrBank::NUM_BYTES)
            .map(|i| -> ChrBank { ChrBank::try_from(i).expect("invalid CHR bank size") })
            .collect();
        return Ok(Self { banks: banks });
    }
}

impl ChrRom {
    pub fn read(&self, bank: usize, tile: usize) -> &ChrTile {
        return &self.banks[bank].tiles[tile];
    }
}

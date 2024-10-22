use crate::mmc::Result;

pub use crate::mmc::buc::*;
pub use crate::mmc::bus::*;
pub use crate::mmc::mio::*;

bitfield! {
    /// Argument for CMD58.
    #[repr(C)]
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct Arg(u32);
    /// Single block read function number.
    ///
    /// Up to 15 functions may be assigned for `Memory`.
    /// Up to 7 functions may be assigned for `I/O`.
    pub fno, set_fno: 30, 27;
    /// 16-bit read address of extension register space.
    pub address, set_address: 25, 9;
}

impl Arg {
    /// Creates a new [Arg].
    pub const fn new() -> Self {
        Self(0)
    }

    /// Gets the bit value of [Arg].
    pub const fn bits(&self) -> u32 {
        self.0
    }

    /// Gets the [BlockUnitCount] for the `CMD58` [Arg].
    pub const fn block_unit_count(&self) -> Result<BlockUnitCount> {
        BlockUnitCount::from_raw(self.0 & 0x1ff)
    }

    /// Gets the [BlockUnitCount] for the `CMD58` [Arg].
    pub fn set_block_unit_count(&mut self, val: BlockUnitCount) {
        self.0 = (self.0 & !0x1ff) | val.into_raw();
    }

    /// Gets the [BlockUnitSelect] for the `CMD58` [Arg].
    pub const fn block_unit_select(&self) -> Result<BlockUnitSelect> {
        BlockUnitSelect::from_raw((self.0 & 0x400_0000) >> 26)
    }

    /// Gets the [BlockUnitSelect] for the `CMD58` [Arg].
    pub fn set_block_unit_select(&mut self, val: BlockUnitSelect) {
        self.0 = (self.0 & !0x400_0000) | (val.into_raw() << 26);
    }

    /// Gets the [MIO] for the `CMD58` [Arg].
    pub const fn mio(&self) -> Result<MIO> {
        MIO::from_raw(self.0 >> 31)
    }

    /// Gets the [MIO] for the `CMD58` [Arg].
    pub fn set_mio(&mut self, val: MIO) {
        self.0 = (self.0 & !0x8000_0000) | (val.into_raw() << 31);
    }
}

impl Default for Arg {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fields() {
        let mut arg = Arg::new();

        assert_eq!(arg.block_unit_count(), Ok(BlockUnitCount::new()));
        assert_eq!(arg.address(), 0);
        assert_eq!(arg.fno(), 0);
        assert_eq!(arg.mio(), Ok(MIO::new()));

        (0..BLOCK_UNIT_MAX).for_each(|buc| {
            let exp_buc = BlockUnitCount::from_raw(buc).expect("invalid BlockUnitCount");

            arg.set_block_unit_count(exp_buc);
            assert_eq!(arg.block_unit_count(), Ok(exp_buc));
            assert_eq!(arg.bits(), exp_buc.into_raw());
        });

        arg.set_block_unit_count(BlockUnitCount::new());

        (0..1024).chain([u16::MAX as u32]).for_each(|exp_addr| {
            arg.set_address(exp_addr);
            assert_eq!(arg.address(), exp_addr);
            assert_eq!(arg.bits(), exp_addr << 9);
        });

        arg.set_address(0);

        [BlockUnitSelect::Bytes512, BlockUnitSelect::Bytes32k]
            .into_iter()
            .for_each(|exp_mask| {
                arg.set_block_unit_select(exp_mask);

                assert_eq!(arg.block_unit_select(), Ok(exp_mask));
                assert_eq!(arg.bits(), exp_mask.into_raw() << 26);
            });

        arg.set_block_unit_select(BlockUnitSelect::Bytes512);

        (0..=0xf).for_each(|exp_fno| {
            arg.set_fno(exp_fno);
            assert_eq!(arg.fno(), exp_fno);
            assert_eq!(arg.bits(), exp_fno << 27);
        });

        arg.set_fno(0);

        [MIO::Memory, MIO::IO].into_iter().for_each(|exp_mio| {
            arg.set_mio(exp_mio);

            assert_eq!(arg.mio(), Ok(exp_mio));
            assert_eq!(arg.bits(), exp_mio.into_raw() << 31);
        });
    }
}

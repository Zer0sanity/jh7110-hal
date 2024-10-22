use crate::mmc::Result;

pub use crate::mmc::mio::*;

bitfield! {
    /// Argument for CMD48.
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
    /// Effective length of page.
    pub page_len, set_page_len: 8, 0;
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

    /// Gets the [MIO] for the `CMD48` [Arg].
    pub const fn mio(&self) -> Result<MIO> {
        MIO::from_raw(self.0 >> 31)
    }

    /// Gets the [MIO] for the `CMD48` [Arg].
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

        assert_eq!(arg.page_len(), 0);
        assert_eq!(arg.address(), 0);
        assert_eq!(arg.fno(), 0);
        assert_eq!(arg.mio(), Ok(MIO::new()));

        (0..=u8::MAX as u32).for_each(|exp_len| {
            arg.set_page_len(exp_len);
            assert_eq!(arg.page_len(), exp_len);
            assert_eq!(arg.bits(), exp_len);
        });

        arg.set_page_len(0);

        (0..1024).chain([u16::MAX as u32]).for_each(|exp_addr| {
            arg.set_address(exp_addr);
            assert_eq!(arg.address(), exp_addr);
            assert_eq!(arg.bits(), exp_addr << 9);
        });

        arg.set_address(0);

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

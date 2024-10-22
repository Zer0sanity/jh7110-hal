mod class0;
mod class1;
mod class10;
mod class11;
mod class2;
mod class4;
mod class5;
mod class6;
mod class7;
mod class8;
mod class9;

pub use class0::*;
pub use class1::*;
pub use class10::*;
pub use class11::*;
pub use class2::*;
pub use class4::*;
pub use class5::*;
pub use class6::*;
pub use class7::*;
pub use class8::*;
pub use class9::*;

/// Represents the opcode sent for a MMC command.
#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MmcOpcode {
    Class0(MmcOpcodeClass0),
    Class1(MmcOpcodeClass1),
    Class2(MmcOpcodeClass2),
    Class4(MmcOpcodeClass4),
    Class5(MmcOpcodeClass5),
    Class6(MmcOpcodeClass6),
    Class7(MmcOpcodeClass7),
    Class8(MmcOpcodeClass8),
    Class9(MmcOpcodeClass9),
    Class10(MmcOpcodeClass10),
    Class11(MmcOpcodeClass11),
}

impl MmcOpcode {
    /// Creates a new [MmcOpcode].
    pub const fn new() -> Self {
        Self::Class0(MmcOpcodeClass0::new())
    }
}

impl Default for MmcOpcode {
    fn default() -> Self {
        Self::new()
    }
}

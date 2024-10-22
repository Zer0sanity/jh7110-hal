pub mod cmd0;
pub mod cmd10;
pub mod cmd13;
pub mod cmd15;
pub mod cmd16;
pub mod cmd17;
pub mod cmd18;
pub mod cmd20;
pub mod cmd22;
pub mod cmd23;
pub mod cmd24;
pub mod cmd25;
pub mod cmd28;
pub mod cmd29;
pub mod cmd30;
pub mod cmd32;
pub mod cmd33;
pub mod cmd38;
pub mod cmd39;
pub mod cmd4;
pub mod cmd43;
pub mod cmd44;
pub mod cmd45;
pub mod cmd46;
pub mod cmd47;
pub mod cmd48;
pub mod cmd49;
pub mod cmd5;
pub mod cmd55;
pub mod cmd56;
pub mod cmd58;
pub mod cmd59;
pub mod cmd6;
pub mod cmd7;
pub mod cmd8;
pub mod cmd9;
pub mod cmd_null;

/// MMC command argument parameters.
///
/// Variants contain types formatted with fields for the specific command.
#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MmcCommandArg {
    Cmd0(cmd0::Arg),
    Cmd1(cmd_null::Arg),
    Cmd2(cmd_null::Arg),
    Cmd3(cmd_null::Arg),
    Cmd4(cmd4::Arg),
    Cmd5(cmd5::Arg),
    Cmd6(cmd6::Arg),
    Cmd7(cmd7::Arg),
    Cmd8(cmd8::Arg),
    Cmd9(cmd9::Arg),
    Cmd10(cmd10::Arg),
    Cmd11(cmd_null::Arg),
    Cmd12(cmd_null::Arg),
    Cmd13(cmd13::Arg),
    Cmd14(cmd_null::Arg),
    Cmd15(cmd15::Arg),
    Cmd16(cmd16::Arg),
    Cmd17(cmd17::Arg),
    Cmd18(cmd18::Arg),
    Cmd19(cmd_null::Arg),
    Cmd20(cmd20::Arg),
    Cmd21(cmd_null::Arg),
    Cmd22(cmd22::Arg),
    Cmd23(cmd23::Arg),
    Cmd24(cmd24::Arg),
    Cmd25(cmd25::Arg),
    Cmd26(cmd_null::Arg),
    Cmd27(cmd_null::Arg),
    Cmd28(cmd28::Arg),
    Cmd29(cmd29::Arg),
    Cmd30(cmd30::Arg),
    Cmd31(cmd_null::Arg),
    Cmd32(cmd32::Arg),
    Cmd33(cmd33::Arg),
    Cmd34(cmd_null::Arg),
    Cmd35(cmd_null::Arg),
    Cmd36(cmd_null::Arg),
    Cmd37(cmd_null::Arg),
    Cmd38(cmd38::Arg),
    Cmd39(cmd39::Arg),
    Cmd40(cmd_null::Arg),
    Cmd41(cmd_null::Arg),
    Cmd42(cmd_null::Arg),
    Cmd43(cmd43::Arg),
    Cmd44(cmd44::Arg),
    Cmd45(cmd45::Arg),
    Cmd46(cmd46::Arg),
    Cmd47(cmd47::Arg),
    Cmd48(cmd48::Arg),
    Cmd49(cmd49::Arg),
    Cmd50(cmd_null::Arg),
    Cmd51(cmd_null::Arg),
    Cmd52(cmd_null::Arg),
    Cmd53(cmd_null::Arg),
    Cmd54(cmd_null::Arg),
    Cmd55(cmd55::Arg),
    Cmd56(cmd56::Arg),
    Cmd57(cmd_null::Arg),
    Cmd58(cmd58::Arg),
    Cmd59(cmd59::Arg),
    None(cmd_null::Arg),
}

impl MmcCommandArg {
    /// Creates a new [MmcCommandArg].
    pub const fn new() -> Self {
        Self::None(cmd_null::Arg::new())
    }

    /// Gets the bit value of [MmcCommandArg].
    pub const fn bits(&self) -> u32 {
        match self {
            Self::Cmd0(arg) => arg.bits(),
            Self::Cmd1(arg) => arg.bits(),
            Self::Cmd2(arg) => arg.bits(),
            Self::Cmd3(arg) => arg.bits(),
            Self::Cmd4(arg) => arg.bits(),
            Self::Cmd5(arg) => arg.bits(),
            Self::Cmd6(arg) => arg.bits(),
            Self::Cmd7(arg) => arg.bits(),
            Self::Cmd8(arg) => arg.bits(),
            Self::Cmd9(arg) => arg.bits(),
            Self::Cmd10(arg) => arg.bits(),
            Self::Cmd11(arg) => arg.bits(),
            Self::Cmd12(arg) => arg.bits(),
            Self::Cmd13(arg) => arg.bits(),
            Self::Cmd14(arg) => arg.bits(),
            Self::Cmd15(arg) => arg.bits(),
            Self::Cmd16(arg) => arg.bits(),
            Self::Cmd17(arg) => arg.bits(),
            Self::Cmd18(arg) => arg.bits(),
            Self::Cmd19(arg) => arg.bits(),
            Self::Cmd20(arg) => arg.bits(),
            Self::Cmd21(arg) => arg.bits(),
            Self::Cmd22(arg) => arg.bits(),
            Self::Cmd23(arg) => arg.bits(),
            Self::Cmd24(arg) => arg.bits(),
            Self::Cmd25(arg) => arg.bits(),
            Self::Cmd26(arg) => arg.bits(),
            Self::Cmd27(arg) => arg.bits(),
            Self::Cmd28(arg) => arg.bits(),
            Self::Cmd29(arg) => arg.bits(),
            Self::Cmd30(arg) => arg.bits(),
            Self::Cmd31(arg) => arg.bits(),
            Self::Cmd32(arg) => arg.bits(),
            Self::Cmd33(arg) => arg.bits(),
            Self::Cmd34(arg) => arg.bits(),
            Self::Cmd35(arg) => arg.bits(),
            Self::Cmd36(arg) => arg.bits(),
            Self::Cmd37(arg) => arg.bits(),
            Self::Cmd38(arg) => arg.bits(),
            Self::Cmd39(arg) => arg.bits(),
            Self::Cmd40(arg) => arg.bits(),
            Self::Cmd41(arg) => arg.bits(),
            Self::Cmd42(arg) => arg.bits(),
            Self::Cmd43(arg) => arg.bits(),
            Self::Cmd44(arg) => arg.bits(),
            Self::Cmd45(arg) => arg.bits(),
            Self::Cmd46(arg) => arg.bits(),
            Self::Cmd47(arg) => arg.bits(),
            Self::Cmd48(arg) => arg.bits(),
            Self::Cmd49(arg) => arg.bits(),
            Self::Cmd50(arg) => arg.bits(),
            Self::Cmd51(arg) => arg.bits(),
            Self::Cmd52(arg) => arg.bits(),
            Self::Cmd53(arg) => arg.bits(),
            Self::Cmd54(arg) => arg.bits(),
            Self::Cmd55(arg) => arg.bits(),
            Self::Cmd56(arg) => arg.bits(),
            Self::Cmd57(arg) => arg.bits(),
            Self::Cmd58(arg) => arg.bits(),
            Self::Cmd59(arg) => arg.bits(),
            Self::None(arg) => arg.bits(),
        }
    }
}

impl Default for MmcCommandArg {
    fn default() -> Self {
        Self::new()
    }
}

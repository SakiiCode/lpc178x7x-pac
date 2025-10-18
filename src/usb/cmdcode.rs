#[doc = "Register `CMDCODE` writer"]
pub type W = crate::W<CmdcodeSpec>;
#[doc = "The command phase:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Enum {
    #[doc = "2: Read"]
    Read = 2,
    #[doc = "1: Write"]
    Write = 1,
    #[doc = "5: Command"]
    Command = 5,
}
impl From<Enum> for u8 {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Enum {
    type Ux = u8;
}
impl crate::IsEnum for Enum {}
#[doc = "Field `CMD_PHASE` writer - The command phase:"]
pub type CmdPhaseW<'a, REG> = crate::FieldWriter<'a, REG, 8, Enum>;
impl<'a, REG> CmdPhaseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Read"]
    #[inline(always)]
    pub fn read(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Read)
    }
    #[doc = "Write"]
    #[inline(always)]
    pub fn write(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Write)
    }
    #[doc = "Command"]
    #[inline(always)]
    pub fn command(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Command)
    }
}
#[doc = "Field `CMD_CODE_WDATA` writer - This is a multi-purpose field. When CMD_PHASE is Command or Read, this field contains the code for the command (CMD_CODE). When CMD_PHASE is Write, this field contains the command write data (CMD_WDATA)."]
pub type CmdCodeWdataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 8:15 - The command phase:"]
    #[inline(always)]
    pub fn cmd_phase(&mut self) -> CmdPhaseW<'_, CmdcodeSpec> {
        CmdPhaseW::new(self, 8)
    }
    #[doc = "Bits 16:23 - This is a multi-purpose field. When CMD_PHASE is Command or Read, this field contains the code for the command (CMD_CODE). When CMD_PHASE is Write, this field contains the command write data (CMD_WDATA)."]
    #[inline(always)]
    pub fn cmd_code_wdata(&mut self) -> CmdCodeWdataW<'_, CmdcodeSpec> {
        CmdCodeWdataW::new(self, 16)
    }
}
#[doc = "USB Command Code\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmdcode::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdcodeSpec;
impl crate::RegisterSpec for CmdcodeSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cmdcode::W`](W) writer structure"]
impl crate::Writable for CmdcodeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMDCODE to value 0"]
impl crate::Resettable for CmdcodeSpec {}

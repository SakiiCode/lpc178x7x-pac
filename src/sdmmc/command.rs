#[doc = "Register `COMMAND` reader"]
pub type R = crate::R<CommandSpec>;
#[doc = "Register `COMMAND` writer"]
pub type W = crate::W<CommandSpec>;
#[doc = "Field `CmdIndex` reader - Command index."]
pub type CmdIndexR = crate::FieldReader;
#[doc = "Field `CmdIndex` writer - Command index."]
pub type CmdIndexW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `Response` reader - If set, CPSM waits for a response."]
pub type ResponseR = crate::BitReader;
#[doc = "Field `Response` writer - If set, CPSM waits for a response."]
pub type ResponseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LongRsp` reader - If set, CPSM receives a 136 bit long response."]
pub type LongRspR = crate::BitReader;
#[doc = "Field `LongRsp` writer - If set, CPSM receives a 136 bit long response."]
pub type LongRspW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Interrupt` reader - If set, CPSM disables command timer and waits for interrupt request."]
pub type InterruptR = crate::BitReader;
#[doc = "Field `Interrupt` writer - If set, CPSM disables command timer and waits for interrupt request."]
pub type InterruptW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Pending` reader - If set, CPSM waits for CmdPend before it starts sending a command."]
pub type PendingR = crate::BitReader;
#[doc = "Field `Pending` writer - If set, CPSM waits for CmdPend before it starts sending a command."]
pub type PendingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Enable` reader - If set, CPSM is enabled."]
pub type EnableR = crate::BitReader;
#[doc = "Field `Enable` writer - If set, CPSM is enabled."]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - Command index."]
    #[inline(always)]
    pub fn cmd_index(&self) -> CmdIndexR {
        CmdIndexR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - If set, CPSM waits for a response."]
    #[inline(always)]
    pub fn response(&self) -> ResponseR {
        ResponseR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - If set, CPSM receives a 136 bit long response."]
    #[inline(always)]
    pub fn long_rsp(&self) -> LongRspR {
        LongRspR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - If set, CPSM disables command timer and waits for interrupt request."]
    #[inline(always)]
    pub fn interrupt(&self) -> InterruptR {
        InterruptR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - If set, CPSM waits for CmdPend before it starts sending a command."]
    #[inline(always)]
    pub fn pending(&self) -> PendingR {
        PendingR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - If set, CPSM is enabled."]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Command index."]
    #[inline(always)]
    pub fn cmd_index(&mut self) -> CmdIndexW<'_, CommandSpec> {
        CmdIndexW::new(self, 0)
    }
    #[doc = "Bit 6 - If set, CPSM waits for a response."]
    #[inline(always)]
    pub fn response(&mut self) -> ResponseW<'_, CommandSpec> {
        ResponseW::new(self, 6)
    }
    #[doc = "Bit 7 - If set, CPSM receives a 136 bit long response."]
    #[inline(always)]
    pub fn long_rsp(&mut self) -> LongRspW<'_, CommandSpec> {
        LongRspW::new(self, 7)
    }
    #[doc = "Bit 8 - If set, CPSM disables command timer and waits for interrupt request."]
    #[inline(always)]
    pub fn interrupt(&mut self) -> InterruptW<'_, CommandSpec> {
        InterruptW::new(self, 8)
    }
    #[doc = "Bit 9 - If set, CPSM waits for CmdPend before it starts sending a command."]
    #[inline(always)]
    pub fn pending(&mut self) -> PendingW<'_, CommandSpec> {
        PendingW::new(self, 9)
    }
    #[doc = "Bit 10 - If set, CPSM is enabled."]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, CommandSpec> {
        EnableW::new(self, 10)
    }
}
#[doc = "Command register.\n\nYou can [`read`](crate::Reg::read) this register and get [`command::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`command::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CommandSpec;
impl crate::RegisterSpec for CommandSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`command::R`](R) reader structure"]
impl crate::Readable for CommandSpec {}
#[doc = "`write(|w| ..)` method takes [`command::W`](W) writer structure"]
impl crate::Writable for CommandSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COMMAND to value 0"]
impl crate::Resettable for CommandSpec {}

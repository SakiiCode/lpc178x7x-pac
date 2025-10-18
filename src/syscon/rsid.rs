#[doc = "Register `RSID` reader"]
pub type R = crate::R<RsidSpec>;
#[doc = "Register `RSID` writer"]
pub type W = crate::W<RsidSpec>;
#[doc = "Field `POR` reader - Assertion of the POR signal sets this bit, and clears all of the other bits in this register. But if another Reset signal (e.g., External Reset) remains asserted after the POR signal is negated, then its bit is set. This bit is not affected by any of the other sources of Reset."]
pub type PorR = crate::BitReader;
#[doc = "Field `POR` writer - Assertion of the POR signal sets this bit, and clears all of the other bits in this register. But if another Reset signal (e.g., External Reset) remains asserted after the POR signal is negated, then its bit is set. This bit is not affected by any of the other sources of Reset."]
pub type PorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTR` reader - Assertion of the external RESET signal sets this bit. This bit is cleared only by software or POR."]
pub type ExtrR = crate::BitReader;
#[doc = "Field `EXTR` writer - Assertion of the external RESET signal sets this bit. This bit is cleared only by software or POR."]
pub type ExtrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDTR` reader - This bit is set when the Watchdog Timer times out and the WDTRESET bit in the Watchdog Mode Register is 1. This bit is cleared only by software or POR."]
pub type WdtrR = crate::BitReader;
#[doc = "Field `WDTR` writer - This bit is set when the Watchdog Timer times out and the WDTRESET bit in the Watchdog Mode Register is 1. This bit is cleared only by software or POR."]
pub type WdtrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BODR` reader - This bit is set when the VDD(REG)(3V3) voltage reaches a level below the BOD reset trip level (typically 1.85 V under nominal room temperature conditions). If the VDD(REG)(3V3) voltage dips from the normal operating range to below the BOD reset trip level and recovers, the BODR bit will be set to 1. If the VDD(REG)(3V3) voltage dips from the normal operating range to below the BOD reset trip level and continues to decline to the level at which POR is asserted (nominally 1 V), the BODR bit is cleared. If the VDD(REG)(3V3) voltage rises continuously from below 1 V to a level above the BOD reset trip level, the BODR will be set to 1. This bit is cleared only by software or POR. Note: Only in the case where a reset occurs and the POR = 0, the BODR bit indicates if the VDD(REG)(3V3) voltage was below the BOD reset trip level or not."]
pub type BodrR = crate::BitReader;
#[doc = "Field `BODR` writer - This bit is set when the VDD(REG)(3V3) voltage reaches a level below the BOD reset trip level (typically 1.85 V under nominal room temperature conditions). If the VDD(REG)(3V3) voltage dips from the normal operating range to below the BOD reset trip level and recovers, the BODR bit will be set to 1. If the VDD(REG)(3V3) voltage dips from the normal operating range to below the BOD reset trip level and continues to decline to the level at which POR is asserted (nominally 1 V), the BODR bit is cleared. If the VDD(REG)(3V3) voltage rises continuously from below 1 V to a level above the BOD reset trip level, the BODR will be set to 1. This bit is cleared only by software or POR. Note: Only in the case where a reset occurs and the POR = 0, the BODR bit indicates if the VDD(REG)(3V3) voltage was below the BOD reset trip level or not."]
pub type BodrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSRESET` reader - This bit is set if the processor has been reset due to a system reset request, as described in Section 40.4.3.6 Application Interrupt and Reset Control Register. Setting the SYSRESETREQ bit in the Cortex-M3 AIRCR register causes a chip reset in the LPC178x/177x. This bit is cleared only by software or POR."]
pub type SysresetR = crate::BitReader;
#[doc = "Field `SYSRESET` writer - This bit is set if the processor has been reset due to a system reset request, as described in Section 40.4.3.6 Application Interrupt and Reset Control Register. Setting the SYSRESETREQ bit in the Cortex-M3 AIRCR register causes a chip reset in the LPC178x/177x. This bit is cleared only by software or POR."]
pub type SysresetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCKUP` reader - This bit is set if the processor has been reset due to a lockup, as described in Section 40.3.4.4 Lockup. The lockup state causes a chip reset in the LPC178x/177x. This bit is cleared only by software or POR."]
pub type LockupR = crate::BitReader;
#[doc = "Field `LOCKUP` writer - This bit is set if the processor has been reset due to a lockup, as described in Section 40.3.4.4 Lockup. The lockup state causes a chip reset in the LPC178x/177x. This bit is cleared only by software or POR."]
pub type LockupW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Assertion of the POR signal sets this bit, and clears all of the other bits in this register. But if another Reset signal (e.g., External Reset) remains asserted after the POR signal is negated, then its bit is set. This bit is not affected by any of the other sources of Reset."]
    #[inline(always)]
    pub fn por(&self) -> PorR {
        PorR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Assertion of the external RESET signal sets this bit. This bit is cleared only by software or POR."]
    #[inline(always)]
    pub fn extr(&self) -> ExtrR {
        ExtrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This bit is set when the Watchdog Timer times out and the WDTRESET bit in the Watchdog Mode Register is 1. This bit is cleared only by software or POR."]
    #[inline(always)]
    pub fn wdtr(&self) -> WdtrR {
        WdtrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit is set when the VDD(REG)(3V3) voltage reaches a level below the BOD reset trip level (typically 1.85 V under nominal room temperature conditions). If the VDD(REG)(3V3) voltage dips from the normal operating range to below the BOD reset trip level and recovers, the BODR bit will be set to 1. If the VDD(REG)(3V3) voltage dips from the normal operating range to below the BOD reset trip level and continues to decline to the level at which POR is asserted (nominally 1 V), the BODR bit is cleared. If the VDD(REG)(3V3) voltage rises continuously from below 1 V to a level above the BOD reset trip level, the BODR will be set to 1. This bit is cleared only by software or POR. Note: Only in the case where a reset occurs and the POR = 0, the BODR bit indicates if the VDD(REG)(3V3) voltage was below the BOD reset trip level or not."]
    #[inline(always)]
    pub fn bodr(&self) -> BodrR {
        BodrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This bit is set if the processor has been reset due to a system reset request, as described in Section 40.4.3.6 Application Interrupt and Reset Control Register. Setting the SYSRESETREQ bit in the Cortex-M3 AIRCR register causes a chip reset in the LPC178x/177x. This bit is cleared only by software or POR."]
    #[inline(always)]
    pub fn sysreset(&self) -> SysresetR {
        SysresetR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This bit is set if the processor has been reset due to a lockup, as described in Section 40.3.4.4 Lockup. The lockup state causes a chip reset in the LPC178x/177x. This bit is cleared only by software or POR."]
    #[inline(always)]
    pub fn lockup(&self) -> LockupR {
        LockupR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Assertion of the POR signal sets this bit, and clears all of the other bits in this register. But if another Reset signal (e.g., External Reset) remains asserted after the POR signal is negated, then its bit is set. This bit is not affected by any of the other sources of Reset."]
    #[inline(always)]
    pub fn por(&mut self) -> PorW<'_, RsidSpec> {
        PorW::new(self, 0)
    }
    #[doc = "Bit 1 - Assertion of the external RESET signal sets this bit. This bit is cleared only by software or POR."]
    #[inline(always)]
    pub fn extr(&mut self) -> ExtrW<'_, RsidSpec> {
        ExtrW::new(self, 1)
    }
    #[doc = "Bit 2 - This bit is set when the Watchdog Timer times out and the WDTRESET bit in the Watchdog Mode Register is 1. This bit is cleared only by software or POR."]
    #[inline(always)]
    pub fn wdtr(&mut self) -> WdtrW<'_, RsidSpec> {
        WdtrW::new(self, 2)
    }
    #[doc = "Bit 3 - This bit is set when the VDD(REG)(3V3) voltage reaches a level below the BOD reset trip level (typically 1.85 V under nominal room temperature conditions). If the VDD(REG)(3V3) voltage dips from the normal operating range to below the BOD reset trip level and recovers, the BODR bit will be set to 1. If the VDD(REG)(3V3) voltage dips from the normal operating range to below the BOD reset trip level and continues to decline to the level at which POR is asserted (nominally 1 V), the BODR bit is cleared. If the VDD(REG)(3V3) voltage rises continuously from below 1 V to a level above the BOD reset trip level, the BODR will be set to 1. This bit is cleared only by software or POR. Note: Only in the case where a reset occurs and the POR = 0, the BODR bit indicates if the VDD(REG)(3V3) voltage was below the BOD reset trip level or not."]
    #[inline(always)]
    pub fn bodr(&mut self) -> BodrW<'_, RsidSpec> {
        BodrW::new(self, 3)
    }
    #[doc = "Bit 4 - This bit is set if the processor has been reset due to a system reset request, as described in Section 40.4.3.6 Application Interrupt and Reset Control Register. Setting the SYSRESETREQ bit in the Cortex-M3 AIRCR register causes a chip reset in the LPC178x/177x. This bit is cleared only by software or POR."]
    #[inline(always)]
    pub fn sysreset(&mut self) -> SysresetW<'_, RsidSpec> {
        SysresetW::new(self, 4)
    }
    #[doc = "Bit 5 - This bit is set if the processor has been reset due to a lockup, as described in Section 40.3.4.4 Lockup. The lockup state causes a chip reset in the LPC178x/177x. This bit is cleared only by software or POR."]
    #[inline(always)]
    pub fn lockup(&mut self) -> LockupW<'_, RsidSpec> {
        LockupW::new(self, 5)
    }
}
#[doc = "Reset Source Identification Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rsid::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rsid::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RsidSpec;
impl crate::RegisterSpec for RsidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rsid::R`](R) reader structure"]
impl crate::Readable for RsidSpec {}
#[doc = "`write(|w| ..)` method takes [`rsid::W`](W) writer structure"]
impl crate::Writable for RsidSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RSID to value 0"]
impl crate::Resettable for RsidSpec {}

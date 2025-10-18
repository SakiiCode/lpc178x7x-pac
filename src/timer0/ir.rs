#[doc = "Register `IR` reader"]
pub type R = crate::R<IrSpec>;
#[doc = "Register `IR` writer"]
pub type W = crate::W<IrSpec>;
#[doc = "Field `MR0INT` reader - Interrupt flag for match channel 0."]
pub type Mr0intR = crate::BitReader;
#[doc = "Field `MR0INT` writer - Interrupt flag for match channel 0."]
pub type Mr0intW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR1INT` reader - Interrupt flag for match channel 1."]
pub type Mr1intR = crate::BitReader;
#[doc = "Field `MR1INT` writer - Interrupt flag for match channel 1."]
pub type Mr1intW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR2INT` reader - Interrupt flag for match channel 2."]
pub type Mr2intR = crate::BitReader;
#[doc = "Field `MR2INT` writer - Interrupt flag for match channel 2."]
pub type Mr2intW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR3INT` reader - Interrupt flag for match channel 3."]
pub type Mr3intR = crate::BitReader;
#[doc = "Field `MR3INT` writer - Interrupt flag for match channel 3."]
pub type Mr3intW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CR0INT` reader - Interrupt flag for capture channel 0 event."]
pub type Cr0intR = crate::BitReader;
#[doc = "Field `CR0INT` writer - Interrupt flag for capture channel 0 event."]
pub type Cr0intW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CR1INT` reader - Interrupt flag for capture channel 1 event."]
pub type Cr1intR = crate::BitReader;
#[doc = "Field `CR1INT` writer - Interrupt flag for capture channel 1 event."]
pub type Cr1intW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Interrupt flag for match channel 0."]
    #[inline(always)]
    pub fn mr0int(&self) -> Mr0intR {
        Mr0intR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt flag for match channel 1."]
    #[inline(always)]
    pub fn mr1int(&self) -> Mr1intR {
        Mr1intR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt flag for match channel 2."]
    #[inline(always)]
    pub fn mr2int(&self) -> Mr2intR {
        Mr2intR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt flag for match channel 3."]
    #[inline(always)]
    pub fn mr3int(&self) -> Mr3intR {
        Mr3intR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt flag for capture channel 0 event."]
    #[inline(always)]
    pub fn cr0int(&self) -> Cr0intR {
        Cr0intR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt flag for capture channel 1 event."]
    #[inline(always)]
    pub fn cr1int(&self) -> Cr1intR {
        Cr1intR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt flag for match channel 0."]
    #[inline(always)]
    pub fn mr0int(&mut self) -> Mr0intW<'_, IrSpec> {
        Mr0intW::new(self, 0)
    }
    #[doc = "Bit 1 - Interrupt flag for match channel 1."]
    #[inline(always)]
    pub fn mr1int(&mut self) -> Mr1intW<'_, IrSpec> {
        Mr1intW::new(self, 1)
    }
    #[doc = "Bit 2 - Interrupt flag for match channel 2."]
    #[inline(always)]
    pub fn mr2int(&mut self) -> Mr2intW<'_, IrSpec> {
        Mr2intW::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt flag for match channel 3."]
    #[inline(always)]
    pub fn mr3int(&mut self) -> Mr3intW<'_, IrSpec> {
        Mr3intW::new(self, 3)
    }
    #[doc = "Bit 4 - Interrupt flag for capture channel 0 event."]
    #[inline(always)]
    pub fn cr0int(&mut self) -> Cr0intW<'_, IrSpec> {
        Cr0intW::new(self, 4)
    }
    #[doc = "Bit 5 - Interrupt flag for capture channel 1 event."]
    #[inline(always)]
    pub fn cr1int(&mut self) -> Cr1intW<'_, IrSpec> {
        Cr1intW::new(self, 5)
    }
}
#[doc = "Interrupt Register. The IR can be written to clear interrupts. The IR can be read to identify which of eight possible interrupt sources are pending.\n\nYou can [`read`](crate::Reg::read) this register and get [`ir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrSpec;
impl crate::RegisterSpec for IrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ir::R`](R) reader structure"]
impl crate::Readable for IrSpec {}
#[doc = "`write(|w| ..)` method takes [`ir::W`](W) writer structure"]
impl crate::Writable for IrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IR to value 0"]
impl crate::Resettable for IrSpec {}

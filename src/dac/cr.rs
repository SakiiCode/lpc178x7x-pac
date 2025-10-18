#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `VALUE` reader - After the selected settling time after this field is written with a new VALUE, the voltage on the DAC_OUT pin (with respect to VSSA) is VALUE x ((VREFP - V REFN)/1024) + VREFN."]
pub type ValueR = crate::FieldReader<u16>;
#[doc = "Field `VALUE` writer - After the selected settling time after this field is written with a new VALUE, the voltage on the DAC_OUT pin (with respect to VSSA) is VALUE x ((VREFP - V REFN)/1024) + VREFN."]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Settling time The settling times noted in the description of the BIAS bit are valid for a capacitance load on the DAC_OUT pin not exceeding 100 pF. A load impedance value greater than that value will cause settling time longer than the specified time. One or more graphs of load impedance vs. settling time will be included in the final data sheet.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: The settling time of the DAC is 1 us max, and the maximum current is 700 uA. This allows a maximum update rate of 1 MHz."]
    Fast = 0,
    #[doc = "1: The settling time of the DAC is 2.5 us and the maximum current is 350 uA. This allows a maximum update rate of 400 kHz."]
    Slow = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BIAS` reader - Settling time The settling times noted in the description of the BIAS bit are valid for a capacitance load on the DAC_OUT pin not exceeding 100 pF. A load impedance value greater than that value will cause settling time longer than the specified time. One or more graphs of load impedance vs. settling time will be included in the final data sheet."]
pub type BiasR = crate::BitReader<Enum>;
impl BiasR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::Fast,
            true => Enum::Slow,
        }
    }
    #[doc = "The settling time of the DAC is 1 us max, and the maximum current is 700 uA. This allows a maximum update rate of 1 MHz."]
    #[inline(always)]
    pub fn is_fast(&self) -> bool {
        *self == Enum::Fast
    }
    #[doc = "The settling time of the DAC is 2.5 us and the maximum current is 350 uA. This allows a maximum update rate of 400 kHz."]
    #[inline(always)]
    pub fn is_slow(&self) -> bool {
        *self == Enum::Slow
    }
}
#[doc = "Field `BIAS` writer - Settling time The settling times noted in the description of the BIAS bit are valid for a capacitance load on the DAC_OUT pin not exceeding 100 pF. A load impedance value greater than that value will cause settling time longer than the specified time. One or more graphs of load impedance vs. settling time will be included in the final data sheet."]
pub type BiasW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> BiasW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The settling time of the DAC is 1 us max, and the maximum current is 700 uA. This allows a maximum update rate of 1 MHz."]
    #[inline(always)]
    pub fn fast(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Fast)
    }
    #[doc = "The settling time of the DAC is 2.5 us and the maximum current is 350 uA. This allows a maximum update rate of 400 kHz."]
    #[inline(always)]
    pub fn slow(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Slow)
    }
}
impl R {
    #[doc = "Bits 6:15 - After the selected settling time after this field is written with a new VALUE, the voltage on the DAC_OUT pin (with respect to VSSA) is VALUE x ((VREFP - V REFN)/1024) + VREFN."]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new(((self.bits >> 6) & 0x03ff) as u16)
    }
    #[doc = "Bit 16 - Settling time The settling times noted in the description of the BIAS bit are valid for a capacitance load on the DAC_OUT pin not exceeding 100 pF. A load impedance value greater than that value will cause settling time longer than the specified time. One or more graphs of load impedance vs. settling time will be included in the final data sheet."]
    #[inline(always)]
    pub fn bias(&self) -> BiasR {
        BiasR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 6:15 - After the selected settling time after this field is written with a new VALUE, the voltage on the DAC_OUT pin (with respect to VSSA) is VALUE x ((VREFP - V REFN)/1024) + VREFN."]
    #[inline(always)]
    pub fn value(&mut self) -> ValueW<'_, CrSpec> {
        ValueW::new(self, 6)
    }
    #[doc = "Bit 16 - Settling time The settling times noted in the description of the BIAS bit are valid for a capacitance load on the DAC_OUT pin not exceeding 100 pF. A load impedance value greater than that value will cause settling time longer than the specified time. One or more graphs of load impedance vs. settling time will be included in the final data sheet."]
    #[inline(always)]
    pub fn bias(&mut self) -> BiasW<'_, CrSpec> {
        BiasW::new(self, 16)
    }
}
#[doc = "D/A Converter Register. This register contains the digital value to be converted to analog and a power control bit.\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {}

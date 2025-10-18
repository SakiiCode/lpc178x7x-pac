#[doc = "Register `GDR` reader"]
pub type R = crate::R<GdrSpec>;
#[doc = "Register `GDR` writer"]
pub type W = crate::W<GdrSpec>;
#[doc = "Field `RESULT` reader - When DONE is 1, this field contains a binary fraction representing the voltage on the AD0\\[n\\] pin selected by the SEL field, as it falls within the range of VREFP to VSS. Zero in the field indicates that the voltage on the input pin was less than, equal to, or close to that on VSS, while 0xFFF indicates that the voltage on the input was close to, equal to, or greater than that on VREFP."]
pub type ResultR = crate::FieldReader<u16>;
#[doc = "Field `RESULT` writer - When DONE is 1, this field contains a binary fraction representing the voltage on the AD0\\[n\\] pin selected by the SEL field, as it falls within the range of VREFP to VSS. Zero in the field indicates that the voltage on the input pin was less than, equal to, or close to that on VSS, while 0xFFF indicates that the voltage on the input was close to, equal to, or greater than that on VREFP."]
pub type ResultW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `CHN` reader - These bits contain the channel from which the RESULT bits were converted (e.g. 000 identifies channel 0, 001 channel 1...)."]
pub type ChnR = crate::FieldReader;
#[doc = "Field `CHN` writer - These bits contain the channel from which the RESULT bits were converted (e.g. 000 identifies channel 0, 001 channel 1...)."]
pub type ChnW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OVERRUN` reader - This bit is 1 in burst mode if the results of one or more conversions was (were) lost and overwritten before the conversion that produced the result in the RESULT bits. This bit is cleared by reading this register."]
pub type OverrunR = crate::BitReader;
#[doc = "Field `OVERRUN` writer - This bit is 1 in burst mode if the results of one or more conversions was (were) lost and overwritten before the conversion that produced the result in the RESULT bits. This bit is cleared by reading this register."]
pub type OverrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DONE` reader - This bit is set to 1 when an A/D conversion completes. It is cleared when this register is read and when the ADCR is written. If the ADCR is written while a conversion is still in progress, this bit is set and a new conversion is started."]
pub type DoneR = crate::BitReader;
#[doc = "Field `DONE` writer - This bit is set to 1 when an A/D conversion completes. It is cleared when this register is read and when the ADCR is written. If the ADCR is written while a conversion is still in progress, this bit is set and a new conversion is started."]
pub type DoneW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 4:15 - When DONE is 1, this field contains a binary fraction representing the voltage on the AD0\\[n\\] pin selected by the SEL field, as it falls within the range of VREFP to VSS. Zero in the field indicates that the voltage on the input pin was less than, equal to, or close to that on VSS, while 0xFFF indicates that the voltage on the input was close to, equal to, or greater than that on VREFP."]
    #[inline(always)]
    pub fn result(&self) -> ResultR {
        ResultR::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 24:26 - These bits contain the channel from which the RESULT bits were converted (e.g. 000 identifies channel 0, 001 channel 1...)."]
    #[inline(always)]
    pub fn chn(&self) -> ChnR {
        ChnR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 30 - This bit is 1 in burst mode if the results of one or more conversions was (were) lost and overwritten before the conversion that produced the result in the RESULT bits. This bit is cleared by reading this register."]
    #[inline(always)]
    pub fn overrun(&self) -> OverrunR {
        OverrunR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - This bit is set to 1 when an A/D conversion completes. It is cleared when this register is read and when the ADCR is written. If the ADCR is written while a conversion is still in progress, this bit is set and a new conversion is started."]
    #[inline(always)]
    pub fn done(&self) -> DoneR {
        DoneR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 4:15 - When DONE is 1, this field contains a binary fraction representing the voltage on the AD0\\[n\\] pin selected by the SEL field, as it falls within the range of VREFP to VSS. Zero in the field indicates that the voltage on the input pin was less than, equal to, or close to that on VSS, while 0xFFF indicates that the voltage on the input was close to, equal to, or greater than that on VREFP."]
    #[inline(always)]
    pub fn result(&mut self) -> ResultW<'_, GdrSpec> {
        ResultW::new(self, 4)
    }
    #[doc = "Bits 24:26 - These bits contain the channel from which the RESULT bits were converted (e.g. 000 identifies channel 0, 001 channel 1...)."]
    #[inline(always)]
    pub fn chn(&mut self) -> ChnW<'_, GdrSpec> {
        ChnW::new(self, 24)
    }
    #[doc = "Bit 30 - This bit is 1 in burst mode if the results of one or more conversions was (were) lost and overwritten before the conversion that produced the result in the RESULT bits. This bit is cleared by reading this register."]
    #[inline(always)]
    pub fn overrun(&mut self) -> OverrunW<'_, GdrSpec> {
        OverrunW::new(self, 30)
    }
    #[doc = "Bit 31 - This bit is set to 1 when an A/D conversion completes. It is cleared when this register is read and when the ADCR is written. If the ADCR is written while a conversion is still in progress, this bit is set and a new conversion is started."]
    #[inline(always)]
    pub fn done(&mut self) -> DoneW<'_, GdrSpec> {
        DoneW::new(self, 31)
    }
}
#[doc = "A/D Global Data Register. This register contains the ADC's DONE bit and the result of the most recent A/D conversion.\n\nYou can [`read`](crate::Reg::read) this register and get [`gdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GdrSpec;
impl crate::RegisterSpec for GdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gdr::R`](R) reader structure"]
impl crate::Readable for GdrSpec {}
#[doc = "`write(|w| ..)` method takes [`gdr::W`](W) writer structure"]
impl crate::Writable for GdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GDR to value 0"]
impl crate::Resettable for GdrSpec {}

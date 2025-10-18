#[doc = "Register `FLOWCONTROLSTATUS` reader"]
pub type R = crate::R<FlowcontrolstatusSpec>;
#[doc = "Field `MCC` reader - MirrorCounterCurrent. In full duplex mode this register represents the current value of the datapath's mirror counter which counts up to the value specified by the MirrorCounter field in the FlowControlCounter register. In half duplex mode the register counts until it reaches the value of the PauseTimer bits in the FlowControlCounter register."]
pub type MccR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - MirrorCounterCurrent. In full duplex mode this register represents the current value of the datapath's mirror counter which counts up to the value specified by the MirrorCounter field in the FlowControlCounter register. In half duplex mode the register counts until it reaches the value of the PauseTimer bits in the FlowControlCounter register."]
    #[inline(always)]
    pub fn mcc(&self) -> MccR {
        MccR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Flow control status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`flowcontrolstatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlowcontrolstatusSpec;
impl crate::RegisterSpec for FlowcontrolstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flowcontrolstatus::R`](R) reader structure"]
impl crate::Readable for FlowcontrolstatusSpec {}
#[doc = "`reset()` method sets FLOWCONTROLSTATUS to value 0"]
impl crate::Resettable for FlowcontrolstatusSpec {}

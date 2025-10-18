#[doc = "Register `CMDDATA` reader"]
pub type R = crate::R<CmddataSpec>;
#[doc = "Field `CMD_RDATA` reader - Command Read Data."]
pub type CmdRdataR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Command Read Data."]
    #[inline(always)]
    pub fn cmd_rdata(&self) -> CmdRdataR {
        CmdRdataR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "USB Command Data\n\nYou can [`read`](crate::Reg::read) this register and get [`cmddata::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmddataSpec;
impl crate::RegisterSpec for CmddataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmddata::R`](R) reader structure"]
impl crate::Readable for CmddataSpec {}
#[doc = "`reset()` method sets CMDDATA to value 0"]
impl crate::Resettable for CmddataSpec {}

#[doc = "Register `MIND` reader"]
pub type R = crate::R<MindSpec>;
#[doc = "Field `BUSY` reader - When 1 is returned - indicates MII Mgmt is currently performing an MII Mgmt Read or Write cycle."]
pub type BusyR = crate::BitReader;
#[doc = "Field `SCANNING` reader - When 1 is returned - indicates a scan operation (continuous MII Mgmt Read cycles) is in progress."]
pub type ScanningR = crate::BitReader;
#[doc = "Field `NOTVALID` reader - When 1 is returned - indicates MII Mgmt Read cycle has not completed and the Read Data is not yet valid."]
pub type NotvalidR = crate::BitReader;
#[doc = "Field `MIILINKFAIL` reader - When 1 is returned - indicates that an MII Mgmt link fail has occurred."]
pub type MiilinkfailR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - When 1 is returned - indicates MII Mgmt is currently performing an MII Mgmt Read or Write cycle."]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When 1 is returned - indicates a scan operation (continuous MII Mgmt Read cycles) is in progress."]
    #[inline(always)]
    pub fn scanning(&self) -> ScanningR {
        ScanningR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When 1 is returned - indicates MII Mgmt Read cycle has not completed and the Read Data is not yet valid."]
    #[inline(always)]
    pub fn notvalid(&self) -> NotvalidR {
        NotvalidR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When 1 is returned - indicates that an MII Mgmt link fail has occurred."]
    #[inline(always)]
    pub fn miilinkfail(&self) -> MiilinkfailR {
        MiilinkfailR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "MII Mgmt Indicators register.\n\nYou can [`read`](crate::Reg::read) this register and get [`mind::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MindSpec;
impl crate::RegisterSpec for MindSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mind::R`](R) reader structure"]
impl crate::Readable for MindSpec {}
#[doc = "`reset()` method sets MIND to value 0"]
impl crate::Resettable for MindSpec {}

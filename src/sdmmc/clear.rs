#[doc = "Register `CLEAR` writer"]
pub type W = crate::W<ClearSpec>;
#[doc = "Field `CMDCRCFAILCLR` writer - Clears CmdCrcFail flag."]
pub type CmdcrcfailclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATACRCFAILCLR` writer - Clears DataCrcFail flag."]
pub type DatacrcfailclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDTIMEOUTCLR` writer - Clears CmdTimeOut flag."]
pub type CmdtimeoutclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATATIMEOUTCLR` writer - Clears DataTimeOut flag."]
pub type DatatimeoutclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXUNDERRUNCLR` writer - Clears TxUnderrun flag."]
pub type TxunderrunclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOVERRUNCLR` writer - Clears RxOverrun flag."]
pub type RxoverrunclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDRESPENDCLR` writer - Clears CmdRespEnd flag."]
pub type CmdrespendclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDSENTCLR` writer - Clears CmdSent flag."]
pub type CmdsentclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATAENDCLR` writer - Clears DataEnd flag."]
pub type DataendclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STARTBITERRCLR` writer - Clears StartBitErr flag."]
pub type StartbiterrclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATABLOCKENDCLR` writer - Clears DataBlockEnd flag."]
pub type DatablockendclrW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clears CmdCrcFail flag."]
    #[inline(always)]
    pub fn cmdcrcfailclr(&mut self) -> CmdcrcfailclrW<'_, ClearSpec> {
        CmdcrcfailclrW::new(self, 0)
    }
    #[doc = "Bit 1 - Clears DataCrcFail flag."]
    #[inline(always)]
    pub fn datacrcfailclr(&mut self) -> DatacrcfailclrW<'_, ClearSpec> {
        DatacrcfailclrW::new(self, 1)
    }
    #[doc = "Bit 2 - Clears CmdTimeOut flag."]
    #[inline(always)]
    pub fn cmdtimeoutclr(&mut self) -> CmdtimeoutclrW<'_, ClearSpec> {
        CmdtimeoutclrW::new(self, 2)
    }
    #[doc = "Bit 3 - Clears DataTimeOut flag."]
    #[inline(always)]
    pub fn datatimeoutclr(&mut self) -> DatatimeoutclrW<'_, ClearSpec> {
        DatatimeoutclrW::new(self, 3)
    }
    #[doc = "Bit 4 - Clears TxUnderrun flag."]
    #[inline(always)]
    pub fn txunderrunclr(&mut self) -> TxunderrunclrW<'_, ClearSpec> {
        TxunderrunclrW::new(self, 4)
    }
    #[doc = "Bit 5 - Clears RxOverrun flag."]
    #[inline(always)]
    pub fn rxoverrunclr(&mut self) -> RxoverrunclrW<'_, ClearSpec> {
        RxoverrunclrW::new(self, 5)
    }
    #[doc = "Bit 6 - Clears CmdRespEnd flag."]
    #[inline(always)]
    pub fn cmdrespendclr(&mut self) -> CmdrespendclrW<'_, ClearSpec> {
        CmdrespendclrW::new(self, 6)
    }
    #[doc = "Bit 7 - Clears CmdSent flag."]
    #[inline(always)]
    pub fn cmdsentclr(&mut self) -> CmdsentclrW<'_, ClearSpec> {
        CmdsentclrW::new(self, 7)
    }
    #[doc = "Bit 8 - Clears DataEnd flag."]
    #[inline(always)]
    pub fn dataendclr(&mut self) -> DataendclrW<'_, ClearSpec> {
        DataendclrW::new(self, 8)
    }
    #[doc = "Bit 9 - Clears StartBitErr flag."]
    #[inline(always)]
    pub fn startbiterrclr(&mut self) -> StartbiterrclrW<'_, ClearSpec> {
        StartbiterrclrW::new(self, 9)
    }
    #[doc = "Bit 10 - Clears DataBlockEnd flag."]
    #[inline(always)]
    pub fn datablockendclr(&mut self) -> DatablockendclrW<'_, ClearSpec> {
        DatablockendclrW::new(self, 10)
    }
}
#[doc = "Clear register.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clear::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClearSpec;
impl crate::RegisterSpec for ClearSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`clear::W`](W) writer structure"]
impl crate::Writable for ClearSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLEAR to value 0"]
impl crate::Resettable for ClearSpec {}

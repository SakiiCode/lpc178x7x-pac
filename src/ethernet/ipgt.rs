#[doc = "Register `IPGT` reader"]
pub type R = crate::R<IpgtSpec>;
#[doc = "Register `IPGT` writer"]
pub type W = crate::W<IpgtSpec>;
#[doc = "Field `BTOBINTEGAP` reader - BACK-TO-BACK INTER-PACKET-GAP.This is a programmable field representing the nibble time offset of the minimum possible period between the end of any transmitted packet to the beginning of the next. In Full-Duplex mode, the register value should be the desired period in nibble times minus 3. In Half-Duplex mode, the register value should be the desired period in nibble times minus 6. In Full-Duplex the recommended setting is 0x15 (21d), which represents the minimum IPG of 960 ns (in 100 Mbps mode) or 9.6 us (in 10 Mbps mode). In Half-Duplex the recommended setting is 0x12 (18d), which also represents the minimum IPG of 960 ns (in 100 Mbps mode) or 9.6 us (in 10 Mbps mode)."]
pub type BtobintegapR = crate::FieldReader;
#[doc = "Field `BTOBINTEGAP` writer - BACK-TO-BACK INTER-PACKET-GAP.This is a programmable field representing the nibble time offset of the minimum possible period between the end of any transmitted packet to the beginning of the next. In Full-Duplex mode, the register value should be the desired period in nibble times minus 3. In Half-Duplex mode, the register value should be the desired period in nibble times minus 6. In Full-Duplex the recommended setting is 0x15 (21d), which represents the minimum IPG of 960 ns (in 100 Mbps mode) or 9.6 us (in 10 Mbps mode). In Half-Duplex the recommended setting is 0x12 (18d), which also represents the minimum IPG of 960 ns (in 100 Mbps mode) or 9.6 us (in 10 Mbps mode)."]
pub type BtobintegapW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - BACK-TO-BACK INTER-PACKET-GAP.This is a programmable field representing the nibble time offset of the minimum possible period between the end of any transmitted packet to the beginning of the next. In Full-Duplex mode, the register value should be the desired period in nibble times minus 3. In Half-Duplex mode, the register value should be the desired period in nibble times minus 6. In Full-Duplex the recommended setting is 0x15 (21d), which represents the minimum IPG of 960 ns (in 100 Mbps mode) or 9.6 us (in 10 Mbps mode). In Half-Duplex the recommended setting is 0x12 (18d), which also represents the minimum IPG of 960 ns (in 100 Mbps mode) or 9.6 us (in 10 Mbps mode)."]
    #[inline(always)]
    pub fn btobintegap(&self) -> BtobintegapR {
        BtobintegapR::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - BACK-TO-BACK INTER-PACKET-GAP.This is a programmable field representing the nibble time offset of the minimum possible period between the end of any transmitted packet to the beginning of the next. In Full-Duplex mode, the register value should be the desired period in nibble times minus 3. In Half-Duplex mode, the register value should be the desired period in nibble times minus 6. In Full-Duplex the recommended setting is 0x15 (21d), which represents the minimum IPG of 960 ns (in 100 Mbps mode) or 9.6 us (in 10 Mbps mode). In Half-Duplex the recommended setting is 0x12 (18d), which also represents the minimum IPG of 960 ns (in 100 Mbps mode) or 9.6 us (in 10 Mbps mode)."]
    #[inline(always)]
    pub fn btobintegap(&mut self) -> BtobintegapW<'_, IpgtSpec> {
        BtobintegapW::new(self, 0)
    }
}
#[doc = "Back-to-Back Inter-Packet-Gap register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ipgt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipgt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IpgtSpec;
impl crate::RegisterSpec for IpgtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipgt::R`](R) reader structure"]
impl crate::Readable for IpgtSpec {}
#[doc = "`write(|w| ..)` method takes [`ipgt::W`](W) writer structure"]
impl crate::Writable for IpgtSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPGT to value 0"]
impl crate::Resettable for IpgtSpec {}

#[doc = "Register `IPGR` reader"]
pub type R = crate::R<IpgrSpec>;
#[doc = "Register `IPGR` writer"]
pub type W = crate::W<IpgrSpec>;
#[doc = "Field `NBTOBINTEGAP2` reader - NON-BACK-TO-BACK INTER-PACKET-GAP PART2. This is a programmable field representing the Non-Back-to-Back Inter-Packet-Gap. The recommended value is 0x12 (18d), which represents the minimum IPG of 960 ns (in 100 Mbps mode) or 9.6 us (in 10 Mbps mode)."]
pub type Nbtobintegap2R = crate::FieldReader;
#[doc = "Field `NBTOBINTEGAP2` writer - NON-BACK-TO-BACK INTER-PACKET-GAP PART2. This is a programmable field representing the Non-Back-to-Back Inter-Packet-Gap. The recommended value is 0x12 (18d), which represents the minimum IPG of 960 ns (in 100 Mbps mode) or 9.6 us (in 10 Mbps mode)."]
pub type Nbtobintegap2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `NBTOBINTEGAP1` reader - NON-BACK-TO-BACK INTER-PACKET-GAP PART1. This is a programmable field representing the optional carrierSense window referenced in IEEE 802.3/4.2.3.2.1 'Carrier Deference'. If carrier is detected during the timing of IPGR1, the MAC defers to carrier. If, however, carrier becomes active after IPGR1, the MAC continues timing IPGR2 and transmits, knowingly causing a collision, thus ensuring fair access to medium. Its range of values is 0x0 to IPGR2. The recommended value is 0xC (12d)"]
pub type Nbtobintegap1R = crate::FieldReader;
#[doc = "Field `NBTOBINTEGAP1` writer - NON-BACK-TO-BACK INTER-PACKET-GAP PART1. This is a programmable field representing the optional carrierSense window referenced in IEEE 802.3/4.2.3.2.1 'Carrier Deference'. If carrier is detected during the timing of IPGR1, the MAC defers to carrier. If, however, carrier becomes active after IPGR1, the MAC continues timing IPGR2 and transmits, knowingly causing a collision, thus ensuring fair access to medium. Its range of values is 0x0 to IPGR2. The recommended value is 0xC (12d)"]
pub type Nbtobintegap1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - NON-BACK-TO-BACK INTER-PACKET-GAP PART2. This is a programmable field representing the Non-Back-to-Back Inter-Packet-Gap. The recommended value is 0x12 (18d), which represents the minimum IPG of 960 ns (in 100 Mbps mode) or 9.6 us (in 10 Mbps mode)."]
    #[inline(always)]
    pub fn nbtobintegap2(&self) -> Nbtobintegap2R {
        Nbtobintegap2R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - NON-BACK-TO-BACK INTER-PACKET-GAP PART1. This is a programmable field representing the optional carrierSense window referenced in IEEE 802.3/4.2.3.2.1 'Carrier Deference'. If carrier is detected during the timing of IPGR1, the MAC defers to carrier. If, however, carrier becomes active after IPGR1, the MAC continues timing IPGR2 and transmits, knowingly causing a collision, thus ensuring fair access to medium. Its range of values is 0x0 to IPGR2. The recommended value is 0xC (12d)"]
    #[inline(always)]
    pub fn nbtobintegap1(&self) -> Nbtobintegap1R {
        Nbtobintegap1R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - NON-BACK-TO-BACK INTER-PACKET-GAP PART2. This is a programmable field representing the Non-Back-to-Back Inter-Packet-Gap. The recommended value is 0x12 (18d), which represents the minimum IPG of 960 ns (in 100 Mbps mode) or 9.6 us (in 10 Mbps mode)."]
    #[inline(always)]
    pub fn nbtobintegap2(&mut self) -> Nbtobintegap2W<'_, IpgrSpec> {
        Nbtobintegap2W::new(self, 0)
    }
    #[doc = "Bits 8:14 - NON-BACK-TO-BACK INTER-PACKET-GAP PART1. This is a programmable field representing the optional carrierSense window referenced in IEEE 802.3/4.2.3.2.1 'Carrier Deference'. If carrier is detected during the timing of IPGR1, the MAC defers to carrier. If, however, carrier becomes active after IPGR1, the MAC continues timing IPGR2 and transmits, knowingly causing a collision, thus ensuring fair access to medium. Its range of values is 0x0 to IPGR2. The recommended value is 0xC (12d)"]
    #[inline(always)]
    pub fn nbtobintegap1(&mut self) -> Nbtobintegap1W<'_, IpgrSpec> {
        Nbtobintegap1W::new(self, 8)
    }
}
#[doc = "Non Back-to-Back Inter-Packet-Gap register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ipgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IpgrSpec;
impl crate::RegisterSpec for IpgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipgr::R`](R) reader structure"]
impl crate::Readable for IpgrSpec {}
#[doc = "`write(|w| ..)` method takes [`ipgr::W`](W) writer structure"]
impl crate::Writable for IpgrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPGR to value 0"]
impl crate::Resettable for IpgrSpec {}

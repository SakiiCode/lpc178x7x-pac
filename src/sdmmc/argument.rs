#[doc = "Register `ARGUMENT` reader"]
pub type R = crate::R<ArgumentSpec>;
#[doc = "Register `ARGUMENT` writer"]
pub type W = crate::W<ArgumentSpec>;
#[doc = "Field `CmdArg` reader - Command argument"]
pub type CmdArgR = crate::FieldReader<u32>;
#[doc = "Field `CmdArg` writer - Command argument"]
pub type CmdArgW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Command argument"]
    #[inline(always)]
    pub fn cmd_arg(&self) -> CmdArgR {
        CmdArgR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Command argument"]
    #[inline(always)]
    pub fn cmd_arg(&mut self) -> CmdArgW<'_, ArgumentSpec> {
        CmdArgW::new(self, 0)
    }
}
#[doc = "Argument register.\n\nYou can [`read`](crate::Reg::read) this register and get [`argument::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`argument::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ArgumentSpec;
impl crate::RegisterSpec for ArgumentSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`argument::R`](R) reader structure"]
impl crate::Readable for ArgumentSpec {}
#[doc = "`write(|w| ..)` method takes [`argument::W`](W) writer structure"]
impl crate::Writable for ArgumentSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ARGUMENT to value 0"]
impl crate::Resettable for ArgumentSpec {}

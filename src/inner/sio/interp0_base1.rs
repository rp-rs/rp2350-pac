#[doc = "Register `INTERP0_BASE1` reader"]
pub type R = crate::R<INTERP0_BASE1_SPEC>;
#[doc = "Register `INTERP0_BASE1` writer"]
pub type W = crate::W<INTERP0_BASE1_SPEC>;
#[doc = "Field `INTERP0_BASE1` reader - "]
pub type INTERP0_BASE1_R = crate::FieldReader<u32>;
#[doc = "Field `INTERP0_BASE1` writer - "]
pub type INTERP0_BASE1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn interp0_base1(&self) -> INTERP0_BASE1_R {
        INTERP0_BASE1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn interp0_base1(&mut self) -> INTERP0_BASE1_W<INTERP0_BASE1_SPEC> {
        INTERP0_BASE1_W::new(self, 0)
    }
}
#[doc = "Read/write access to BASE1 register.  

You can [`read`](crate::Reg::read) this register and get [`interp0_base1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interp0_base1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTERP0_BASE1_SPEC;
impl crate::RegisterSpec for INTERP0_BASE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interp0_base1::R`](R) reader structure"]
impl crate::Readable for INTERP0_BASE1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`interp0_base1::W`](W) writer structure"]
impl crate::Writable for INTERP0_BASE1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTERP0_BASE1 to value 0"]
impl crate::Resettable for INTERP0_BASE1_SPEC {
    const RESET_VALUE: u32 = 0;
}

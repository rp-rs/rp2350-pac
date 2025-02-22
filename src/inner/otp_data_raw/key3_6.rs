#[doc = "Register `KEY3_6` reader"]
pub type R = crate::R<KEY3_6_SPEC>;
#[doc = "Register `KEY3_6` writer"]
pub type W = crate::W<KEY3_6_SPEC>;
#[doc = "Field `KEY3_6` reader - "]
pub type KEY3_6_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn key3_6(&self) -> KEY3_6_R {
        KEY3_6_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {}
#[doc = "Bits 111:96 of OTP access key 3 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key3_6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key3_6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEY3_6_SPEC;
impl crate::RegisterSpec for KEY3_6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key3_6::R`](R) reader structure"]
impl crate::Readable for KEY3_6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`key3_6::W`](W) writer structure"]
impl crate::Writable for KEY3_6_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEY3_6 to value 0"]
impl crate::Resettable for KEY3_6_SPEC {
    const RESET_VALUE: u32 = 0;
}

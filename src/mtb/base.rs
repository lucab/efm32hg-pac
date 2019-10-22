#[doc = "Reader of register BASE"]
pub type R = crate::R<u32, super::BASE>;
#[doc = "Writer for register BASE"]
pub type W = crate::W<u32, super::BASE>;
#[doc = "Register BASE `reset()`'s with value 0x2000_0000"]
impl crate::ResetValue for super::BASE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x2000_0000
    }
}
#[doc = "Reader of field `BASE`"]
pub type BASE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `BASE`"]
pub struct BASE_W<'a> {
    w: &'a mut W,
}
impl<'a> BASE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - The ram base address."]
    #[inline(always)]
    pub fn base(&self) -> BASE_R {
        BASE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - The ram base address."]
    #[inline(always)]
    pub fn base(&mut self) -> BASE_W {
        BASE_W { w: self }
    }
}

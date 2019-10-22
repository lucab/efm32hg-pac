#[doc = "Reader of register KEYLA"]
pub type R = crate::R<u32, super::KEYLA>;
#[doc = "Writer for register KEYLA"]
pub type W = crate::W<u32, super::KEYLA>;
#[doc = "Register KEYLA `reset()`'s with value 0"]
impl crate::ResetValue for super::KEYLA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `KEYLA`"]
pub type KEYLA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `KEYLA`"]
pub struct KEYLA_W<'a> {
    w: &'a mut W,
}
impl<'a> KEYLA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Key Low Access A"]
    #[inline(always)]
    pub fn keyla(&self) -> KEYLA_R {
        KEYLA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Key Low Access A"]
    #[inline(always)]
    pub fn keyla(&mut self) -> KEYLA_W {
        KEYLA_W { w: self }
    }
}

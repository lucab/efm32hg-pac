#[doc = "Reader of register KEYLC"]
pub type R = crate::R<u32, super::KEYLC>;
#[doc = "Writer for register KEYLC"]
pub type W = crate::W<u32, super::KEYLC>;
#[doc = "Register KEYLC `reset()`'s with value 0"]
impl crate::ResetValue for super::KEYLC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `KEYLC`"]
pub type KEYLC_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `KEYLC`"]
pub struct KEYLC_W<'a> {
    w: &'a mut W,
}
impl<'a> KEYLC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Key Low Access C"]
    #[inline(always)]
    pub fn keylc(&self) -> KEYLC_R {
        KEYLC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Key Low Access C"]
    #[inline(always)]
    pub fn keylc(&mut self) -> KEYLC_W {
        KEYLC_W { w: self }
    }
}

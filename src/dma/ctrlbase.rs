#[doc = "Reader of register CTRLBASE"]
pub type R = crate::R<u32, super::CTRLBASE>;
#[doc = "Writer for register CTRLBASE"]
pub type W = crate::W<u32, super::CTRLBASE>;
#[doc = "Register CTRLBASE `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRLBASE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CTRLBASE`"]
pub type CTRLBASE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CTRLBASE`"]
pub struct CTRLBASE_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRLBASE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Channel Control Data Base Pointer"]
    #[inline(always)]
    pub fn ctrlbase(&self) -> CTRLBASE_R {
        CTRLBASE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel Control Data Base Pointer"]
    #[inline(always)]
    pub fn ctrlbase(&mut self) -> CTRLBASE_W {
        CTRLBASE_W { w: self }
    }
}

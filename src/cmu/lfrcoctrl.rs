#[doc = "Reader of register LFRCOCTRL"]
pub type R = crate::R<u32, super::LFRCOCTRL>;
#[doc = "Writer for register LFRCOCTRL"]
pub type W = crate::W<u32, super::LFRCOCTRL>;
#[doc = "Register LFRCOCTRL `reset()`'s with value 0x40"]
impl crate::ResetValue for super::LFRCOCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x40
    }
}
#[doc = "Reader of field `TUNING`"]
pub type TUNING_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TUNING`"]
pub struct TUNING_W<'a> {
    w: &'a mut W,
}
impl<'a> TUNING_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - LFRCO Tuning Value"]
    #[inline(always)]
    pub fn tuning(&self) -> TUNING_R {
        TUNING_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - LFRCO Tuning Value"]
    #[inline(always)]
    pub fn tuning(&mut self) -> TUNING_W {
        TUNING_W { w: self }
    }
}

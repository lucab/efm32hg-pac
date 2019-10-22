#[doc = "Reader of register USHFRCOTUNE"]
pub type R = crate::R<u32, super::USHFRCOTUNE>;
#[doc = "Writer for register USHFRCOTUNE"]
pub type W = crate::W<u32, super::USHFRCOTUNE>;
#[doc = "Register USHFRCOTUNE `reset()`'s with value 0x20"]
impl crate::ResetValue for super::USHFRCOTUNE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x20
    }
}
#[doc = "Reader of field `FINETUNING`"]
pub type FINETUNING_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FINETUNING`"]
pub struct FINETUNING_W<'a> {
    w: &'a mut W,
}
impl<'a> FINETUNING_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Oscillator fine frequency adjust"]
    #[inline(always)]
    pub fn finetuning(&self) -> FINETUNING_R {
        FINETUNING_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Oscillator fine frequency adjust"]
    #[inline(always)]
    pub fn finetuning(&mut self) -> FINETUNING_W {
        FINETUNING_W { w: self }
    }
}

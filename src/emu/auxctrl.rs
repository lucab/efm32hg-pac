#[doc = "Reader of register AUXCTRL"]
pub type R = crate::R<u32, super::AUXCTRL>;
#[doc = "Writer for register AUXCTRL"]
pub type W = crate::W<u32, super::AUXCTRL>;
#[doc = "Register AUXCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::AUXCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HRCCLR`"]
pub type HRCCLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HRCCLR`"]
pub struct HRCCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> HRCCLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Hard Reset Cause Clear"]
    #[inline(always)]
    pub fn hrcclr(&self) -> HRCCLR_R {
        HRCCLR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Hard Reset Cause Clear"]
    #[inline(always)]
    pub fn hrcclr(&mut self) -> HRCCLR_W {
        HRCCLR_W { w: self }
    }
}

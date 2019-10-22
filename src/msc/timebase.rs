#[doc = "Reader of register TIMEBASE"]
pub type R = crate::R<u32, super::TIMEBASE>;
#[doc = "Writer for register TIMEBASE"]
pub type W = crate::W<u32, super::TIMEBASE>;
#[doc = "Register TIMEBASE `reset()`'s with value 0x10"]
impl crate::ResetValue for super::TIMEBASE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x10
    }
}
#[doc = "Reader of field `BASE`"]
pub type BASE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BASE`"]
pub struct BASE_W<'a> {
    w: &'a mut W,
}
impl<'a> BASE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `PERIOD`"]
pub type PERIOD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PERIOD`"]
pub struct PERIOD_W<'a> {
    w: &'a mut W,
}
impl<'a> PERIOD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Timebase used by MSC to time flash writes and erases"]
    #[inline(always)]
    pub fn base(&self) -> BASE_R {
        BASE_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 16 - Sets the timebase period"]
    #[inline(always)]
    pub fn period(&self) -> PERIOD_R {
        PERIOD_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Timebase used by MSC to time flash writes and erases"]
    #[inline(always)]
    pub fn base(&mut self) -> BASE_W {
        BASE_W { w: self }
    }
    #[doc = "Bit 16 - Sets the timebase period"]
    #[inline(always)]
    pub fn period(&mut self) -> PERIOD_W {
        PERIOD_W { w: self }
    }
}

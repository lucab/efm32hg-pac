#[doc = "Reader of register BIASPROG"]
pub type R = crate::R<u32, super::BIASPROG>;
#[doc = "Writer for register BIASPROG"]
pub type W = crate::W<u32, super::BIASPROG>;
#[doc = "Register BIASPROG `reset()`'s with value 0x0747"]
impl crate::ResetValue for super::BIASPROG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0747
    }
}
#[doc = "Reader of field `BIASPROG`"]
pub type BIASPROG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BIASPROG`"]
pub struct BIASPROG_W<'a> {
    w: &'a mut W,
}
impl<'a> BIASPROG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `HALFBIAS`"]
pub type HALFBIAS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HALFBIAS`"]
pub struct HALFBIAS_W<'a> {
    w: &'a mut W,
}
impl<'a> HALFBIAS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `COMPBIAS`"]
pub type COMPBIAS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `COMPBIAS`"]
pub struct COMPBIAS_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPBIAS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Bias Programming Value"]
    #[inline(always)]
    pub fn biasprog(&self) -> BIASPROG_R {
        BIASPROG_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 6 - Half Bias Current"]
    #[inline(always)]
    pub fn halfbias(&self) -> HALFBIAS_R {
        HALFBIAS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Comparator Bias Value"]
    #[inline(always)]
    pub fn compbias(&self) -> COMPBIAS_R {
        COMPBIAS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Bias Programming Value"]
    #[inline(always)]
    pub fn biasprog(&mut self) -> BIASPROG_W {
        BIASPROG_W { w: self }
    }
    #[doc = "Bit 6 - Half Bias Current"]
    #[inline(always)]
    pub fn halfbias(&mut self) -> HALFBIAS_W {
        HALFBIAS_W { w: self }
    }
    #[doc = "Bits 8:11 - Comparator Bias Value"]
    #[inline(always)]
    pub fn compbias(&mut self) -> COMPBIAS_W {
        COMPBIAS_W { w: self }
    }
}

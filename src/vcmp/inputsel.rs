#[doc = "Reader of register INPUTSEL"]
pub type R = crate::R<u32, super::INPUTSEL>;
#[doc = "Writer for register INPUTSEL"]
pub type W = crate::W<u32, super::INPUTSEL>;
#[doc = "Register INPUTSEL `reset()`'s with value 0"]
impl crate::ResetValue for super::INPUTSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TRIGLEVEL`"]
pub type TRIGLEVEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRIGLEVEL`"]
pub struct TRIGLEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGLEVEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `LPREF`"]
pub type LPREF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPREF`"]
pub struct LPREF_W<'a> {
    w: &'a mut W,
}
impl<'a> LPREF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Trigger Level"]
    #[inline(always)]
    pub fn triglevel(&self) -> TRIGLEVEL_R {
        TRIGLEVEL_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 8 - Low Power Reference"]
    #[inline(always)]
    pub fn lpref(&self) -> LPREF_R {
        LPREF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Trigger Level"]
    #[inline(always)]
    pub fn triglevel(&mut self) -> TRIGLEVEL_W {
        TRIGLEVEL_W { w: self }
    }
    #[doc = "Bit 8 - Low Power Reference"]
    #[inline(always)]
    pub fn lpref(&mut self) -> LPREF_W {
        LPREF_W { w: self }
    }
}

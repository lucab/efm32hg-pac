#[doc = "Reader of register ERRORC"]
pub type R = crate::R<u32, super::ERRORC>;
#[doc = "Writer for register ERRORC"]
pub type W = crate::W<u32, super::ERRORC>;
#[doc = "Register ERRORC `reset()`'s with value 0"]
impl crate::ResetValue for super::ERRORC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ERRORC`"]
pub type ERRORC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERRORC`"]
pub struct ERRORC_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRORC_W<'a> {
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
    #[doc = "Bit 0 - Bus Error Clear"]
    #[inline(always)]
    pub fn errorc(&self) -> ERRORC_R {
        ERRORC_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bus Error Clear"]
    #[inline(always)]
    pub fn errorc(&mut self) -> ERRORC_W {
        ERRORC_W { w: self }
    }
}

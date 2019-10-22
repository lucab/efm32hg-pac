#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0x01"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reader of field `BUSFAULT`"]
pub type BUSFAULT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BUSFAULT`"]
pub struct BUSFAULT_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSFAULT_W<'a> {
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
    #[doc = "Bit 0 - Bus Fault Response Enable"]
    #[inline(always)]
    pub fn busfault(&self) -> BUSFAULT_R {
        BUSFAULT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bus Fault Response Enable"]
    #[inline(always)]
    pub fn busfault(&mut self) -> BUSFAULT_W {
        BUSFAULT_W { w: self }
    }
}

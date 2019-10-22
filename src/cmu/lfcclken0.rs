#[doc = "Reader of register LFCCLKEN0"]
pub type R = crate::R<u32, super::LFCCLKEN0>;
#[doc = "Writer for register LFCCLKEN0"]
pub type W = crate::W<u32, super::LFCCLKEN0>;
#[doc = "Register LFCCLKEN0 `reset()`'s with value 0"]
impl crate::ResetValue for super::LFCCLKEN0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `USBLE`"]
pub type USBLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USBLE`"]
pub struct USBLE_W<'a> {
    w: &'a mut W,
}
impl<'a> USBLE_W<'a> {
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
    #[doc = "Bit 0 - Universal Serial Bus Low Energy Clock Clock Enable"]
    #[inline(always)]
    pub fn usble(&self) -> USBLE_R {
        USBLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Universal Serial Bus Low Energy Clock Clock Enable"]
    #[inline(always)]
    pub fn usble(&mut self) -> USBLE_W {
        USBLE_W { w: self }
    }
}

#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EMVREG`"]
pub type EMVREG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EMVREG`"]
pub struct EMVREG_W<'a> {
    w: &'a mut W,
}
impl<'a> EMVREG_W<'a> {
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
#[doc = "Reader of field `EM2BLOCK`"]
pub type EM2BLOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EM2BLOCK`"]
pub struct EM2BLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> EM2BLOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `EM4CTRL`"]
pub type EM4CTRL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EM4CTRL`"]
pub struct EM4CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> EM4CTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Energy Mode Voltage Regulator Control"]
    #[inline(always)]
    pub fn emvreg(&self) -> EMVREG_R {
        EMVREG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Energy Mode 2 Block"]
    #[inline(always)]
    pub fn em2block(&self) -> EM2BLOCK_R {
        EM2BLOCK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Energy Mode 4 Control"]
    #[inline(always)]
    pub fn em4ctrl(&self) -> EM4CTRL_R {
        EM4CTRL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Energy Mode Voltage Regulator Control"]
    #[inline(always)]
    pub fn emvreg(&mut self) -> EMVREG_W {
        EMVREG_W { w: self }
    }
    #[doc = "Bit 1 - Energy Mode 2 Block"]
    #[inline(always)]
    pub fn em2block(&mut self) -> EM2BLOCK_W {
        EM2BLOCK_W { w: self }
    }
    #[doc = "Bits 2:3 - Energy Mode 4 Control"]
    #[inline(always)]
    pub fn em4ctrl(&mut self) -> EM4CTRL_W {
        EM4CTRL_W { w: self }
    }
}

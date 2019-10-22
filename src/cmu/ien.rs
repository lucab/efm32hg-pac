#[doc = "Reader of register IEN"]
pub type R = crate::R<u32, super::IEN>;
#[doc = "Writer for register IEN"]
pub type W = crate::W<u32, super::IEN>;
#[doc = "Register IEN `reset()`'s with value 0"]
impl crate::ResetValue for super::IEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HFRCORDY`"]
pub type HFRCORDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HFRCORDY`"]
pub struct HFRCORDY_W<'a> {
    w: &'a mut W,
}
impl<'a> HFRCORDY_W<'a> {
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
#[doc = "Reader of field `HFXORDY`"]
pub type HFXORDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HFXORDY`"]
pub struct HFXORDY_W<'a> {
    w: &'a mut W,
}
impl<'a> HFXORDY_W<'a> {
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
#[doc = "Reader of field `LFRCORDY`"]
pub type LFRCORDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LFRCORDY`"]
pub struct LFRCORDY_W<'a> {
    w: &'a mut W,
}
impl<'a> LFRCORDY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `LFXORDY`"]
pub type LFXORDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LFXORDY`"]
pub struct LFXORDY_W<'a> {
    w: &'a mut W,
}
impl<'a> LFXORDY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `AUXHFRCORDY`"]
pub type AUXHFRCORDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUXHFRCORDY`"]
pub struct AUXHFRCORDY_W<'a> {
    w: &'a mut W,
}
impl<'a> AUXHFRCORDY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `CALRDY`"]
pub type CALRDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CALRDY`"]
pub struct CALRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> CALRDY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `CALOF`"]
pub type CALOF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CALOF`"]
pub struct CALOF_W<'a> {
    w: &'a mut W,
}
impl<'a> CALOF_W<'a> {
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
#[doc = "Reader of field `USHFRCORDY`"]
pub type USHFRCORDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USHFRCORDY`"]
pub struct USHFRCORDY_W<'a> {
    w: &'a mut W,
}
impl<'a> USHFRCORDY_W<'a> {
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
#[doc = "Reader of field `USBCHFOSCSEL`"]
pub type USBCHFOSCSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USBCHFOSCSEL`"]
pub struct USBCHFOSCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> USBCHFOSCSEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - HFRCO Ready Interrupt Enable"]
    #[inline(always)]
    pub fn hfrcordy(&self) -> HFRCORDY_R {
        HFRCORDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - HFXO Ready Interrupt Enable"]
    #[inline(always)]
    pub fn hfxordy(&self) -> HFXORDY_R {
        HFXORDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - LFRCO Ready Interrupt Enable"]
    #[inline(always)]
    pub fn lfrcordy(&self) -> LFRCORDY_R {
        LFRCORDY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - LFXO Ready Interrupt Enable"]
    #[inline(always)]
    pub fn lfxordy(&self) -> LFXORDY_R {
        LFXORDY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - AUXHFRCO Ready Interrupt Enable"]
    #[inline(always)]
    pub fn auxhfrcordy(&self) -> AUXHFRCORDY_R {
        AUXHFRCORDY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Calibration Ready Interrupt Enable"]
    #[inline(always)]
    pub fn calrdy(&self) -> CALRDY_R {
        CALRDY_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Calibration Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn calof(&self) -> CALOF_R {
        CALOF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - USHFRCO Ready Interrupt Enable"]
    #[inline(always)]
    pub fn ushfrcordy(&self) -> USHFRCORDY_R {
        USHFRCORDY_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - USBC HF-oscillator Selected Interrupt Flag Clear"]
    #[inline(always)]
    pub fn usbchfoscsel(&self) -> USBCHFOSCSEL_R {
        USBCHFOSCSEL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HFRCO Ready Interrupt Enable"]
    #[inline(always)]
    pub fn hfrcordy(&mut self) -> HFRCORDY_W {
        HFRCORDY_W { w: self }
    }
    #[doc = "Bit 1 - HFXO Ready Interrupt Enable"]
    #[inline(always)]
    pub fn hfxordy(&mut self) -> HFXORDY_W {
        HFXORDY_W { w: self }
    }
    #[doc = "Bit 2 - LFRCO Ready Interrupt Enable"]
    #[inline(always)]
    pub fn lfrcordy(&mut self) -> LFRCORDY_W {
        LFRCORDY_W { w: self }
    }
    #[doc = "Bit 3 - LFXO Ready Interrupt Enable"]
    #[inline(always)]
    pub fn lfxordy(&mut self) -> LFXORDY_W {
        LFXORDY_W { w: self }
    }
    #[doc = "Bit 4 - AUXHFRCO Ready Interrupt Enable"]
    #[inline(always)]
    pub fn auxhfrcordy(&mut self) -> AUXHFRCORDY_W {
        AUXHFRCORDY_W { w: self }
    }
    #[doc = "Bit 5 - Calibration Ready Interrupt Enable"]
    #[inline(always)]
    pub fn calrdy(&mut self) -> CALRDY_W {
        CALRDY_W { w: self }
    }
    #[doc = "Bit 6 - Calibration Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn calof(&mut self) -> CALOF_W {
        CALOF_W { w: self }
    }
    #[doc = "Bit 8 - USHFRCO Ready Interrupt Enable"]
    #[inline(always)]
    pub fn ushfrcordy(&mut self) -> USHFRCORDY_W {
        USHFRCORDY_W { w: self }
    }
    #[doc = "Bit 9 - USBC HF-oscillator Selected Interrupt Flag Clear"]
    #[inline(always)]
    pub fn usbchfoscsel(&mut self) -> USBCHFOSCSEL_W {
        USBCHFOSCSEL_W { w: self }
    }
}

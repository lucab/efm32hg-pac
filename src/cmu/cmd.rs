#[doc = "Writer for register CMD"]
pub type W = crate::W<u32, super::CMD>;
#[doc = "Register CMD `reset()`'s with value 0"]
impl crate::ResetValue for super::CMD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "HFCLK Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HFCLKSEL_AW {
    #[doc = "1: Select HFRCO as HFCLK."]
    HFRCO,
    #[doc = "2: Select HFXO as HFCLK."]
    HFXO,
    #[doc = "3: Select LFRCO as HFCLK."]
    LFRCO,
    #[doc = "4: Select LFXO as HFCLK."]
    LFXO,
    #[doc = "5: Select USHFRCO divided by two as HFCLK."]
    USHFRCODIV2,
}
impl From<HFCLKSEL_AW> for u8 {
    #[inline(always)]
    fn from(variant: HFCLKSEL_AW) -> Self {
        match variant {
            HFCLKSEL_AW::HFRCO => 1,
            HFCLKSEL_AW::HFXO => 2,
            HFCLKSEL_AW::LFRCO => 3,
            HFCLKSEL_AW::LFXO => 4,
            HFCLKSEL_AW::USHFRCODIV2 => 5,
        }
    }
}
#[doc = "Write proxy for field `HFCLKSEL`"]
pub struct HFCLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> HFCLKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HFCLKSEL_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select HFRCO as HFCLK."]
    #[inline(always)]
    pub fn hfrco(self) -> &'a mut W {
        self.variant(HFCLKSEL_AW::HFRCO)
    }
    #[doc = "Select HFXO as HFCLK."]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut W {
        self.variant(HFCLKSEL_AW::HFXO)
    }
    #[doc = "Select LFRCO as HFCLK."]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut W {
        self.variant(HFCLKSEL_AW::LFRCO)
    }
    #[doc = "Select LFXO as HFCLK."]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut W {
        self.variant(HFCLKSEL_AW::LFXO)
    }
    #[doc = "Select USHFRCO divided by two as HFCLK."]
    #[inline(always)]
    pub fn ushfrcodiv2(self) -> &'a mut W {
        self.variant(HFCLKSEL_AW::USHFRCODIV2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Write proxy for field `CALSTART`"]
pub struct CALSTART_W<'a> {
    w: &'a mut W,
}
impl<'a> CALSTART_W<'a> {
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
#[doc = "Write proxy for field `CALSTOP`"]
pub struct CALSTOP_W<'a> {
    w: &'a mut W,
}
impl<'a> CALSTOP_W<'a> {
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
#[doc = "USB Core Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBCCLKSEL_AW {
    #[doc = "2: Select LFXO as HFCORECLKUSBC."]
    LFXO,
    #[doc = "3: Select LFRCO as HFCORECLKUSBC."]
    LFRCO,
    #[doc = "4: Select USHFRCO as HFCORECLKUSBC."]
    USHFRCO,
}
impl From<USBCCLKSEL_AW> for u8 {
    #[inline(always)]
    fn from(variant: USBCCLKSEL_AW) -> Self {
        match variant {
            USBCCLKSEL_AW::LFXO => 2,
            USBCCLKSEL_AW::LFRCO => 3,
            USBCCLKSEL_AW::USHFRCO => 4,
        }
    }
}
#[doc = "Write proxy for field `USBCCLKSEL`"]
pub struct USBCCLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> USBCCLKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBCCLKSEL_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select LFXO as HFCORECLKUSBC."]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut W {
        self.variant(USBCCLKSEL_AW::LFXO)
    }
    #[doc = "Select LFRCO as HFCORECLKUSBC."]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut W {
        self.variant(USBCCLKSEL_AW::LFRCO)
    }
    #[doc = "Select USHFRCO as HFCORECLKUSBC."]
    #[inline(always)]
    pub fn ushfrco(self) -> &'a mut W {
        self.variant(USBCCLKSEL_AW::USHFRCO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | (((value as u32) & 0x07) << 5);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:2 - HFCLK Select"]
    #[inline(always)]
    pub fn hfclksel(&mut self) -> HFCLKSEL_W {
        HFCLKSEL_W { w: self }
    }
    #[doc = "Bit 3 - Calibration Start"]
    #[inline(always)]
    pub fn calstart(&mut self) -> CALSTART_W {
        CALSTART_W { w: self }
    }
    #[doc = "Bit 4 - Calibration Stop"]
    #[inline(always)]
    pub fn calstop(&mut self) -> CALSTOP_W {
        CALSTOP_W { w: self }
    }
    #[doc = "Bits 5:7 - USB Core Clock Select"]
    #[inline(always)]
    pub fn usbcclksel(&mut self) -> USBCCLKSEL_W {
        USBCCLKSEL_W { w: self }
    }
}

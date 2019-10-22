#[doc = "Reader of register USHFRCOCONF"]
pub type R = crate::R<u32, super::USHFRCOCONF>;
#[doc = "Writer for register USHFRCOCONF"]
pub type W = crate::W<u32, super::USHFRCOCONF>;
#[doc = "Register USHFRCOCONF `reset()`'s with value 0x01"]
impl crate::ResetValue for super::USHFRCOCONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "USHFRCO Band Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BAND_A {
    #[doc = "1: 48 MHz band. NOTE: Also set the TUNING and FINETUNING value when changing band."]
    _48MHZ,
    #[doc = "3: 24 MHz band. NOTE: Also set the TUNING and FINETUNING value when changing band."]
    _24MHZ,
}
impl From<BAND_A> for u8 {
    #[inline(always)]
    fn from(variant: BAND_A) -> Self {
        match variant {
            BAND_A::_48MHZ => 1,
            BAND_A::_24MHZ => 3,
        }
    }
}
#[doc = "Reader of field `BAND`"]
pub type BAND_R = crate::R<u8, BAND_A>;
impl BAND_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BAND_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(BAND_A::_48MHZ),
            3 => Val(BAND_A::_24MHZ),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_48MHZ`"]
    #[inline(always)]
    pub fn is_48mhz(&self) -> bool {
        *self == BAND_A::_48MHZ
    }
    #[doc = "Checks if the value of the field is `_24MHZ`"]
    #[inline(always)]
    pub fn is_24mhz(&self) -> bool {
        *self == BAND_A::_24MHZ
    }
}
#[doc = "Write proxy for field `BAND`"]
pub struct BAND_W<'a> {
    w: &'a mut W,
}
impl<'a> BAND_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BAND_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "48 MHz band. NOTE: Also set the TUNING and FINETUNING value when changing band."]
    #[inline(always)]
    pub fn _48mhz(self) -> &'a mut W {
        self.variant(BAND_A::_48MHZ)
    }
    #[doc = "24 MHz band. NOTE: Also set the TUNING and FINETUNING value when changing band."]
    #[inline(always)]
    pub fn _24mhz(self) -> &'a mut W {
        self.variant(BAND_A::_24MHZ)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `USHFRCODIV2DIS`"]
pub type USHFRCODIV2DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USHFRCODIV2DIS`"]
pub struct USHFRCODIV2DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> USHFRCODIV2DIS_W<'a> {
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
impl R {
    #[doc = "Bits 0:2 - USHFRCO Band Select"]
    #[inline(always)]
    pub fn band(&self) -> BAND_R {
        BAND_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 4 - USHFRCO divider for HFCLK disable"]
    #[inline(always)]
    pub fn ushfrcodiv2dis(&self) -> USHFRCODIV2DIS_R {
        USHFRCODIV2DIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - USHFRCO Band Select"]
    #[inline(always)]
    pub fn band(&mut self) -> BAND_W {
        BAND_W { w: self }
    }
    #[doc = "Bit 4 - USHFRCO divider for HFCLK disable"]
    #[inline(always)]
    pub fn ushfrcodiv2dis(&mut self) -> USHFRCODIV2DIS_W {
        USHFRCODIV2DIS_W { w: self }
    }
}

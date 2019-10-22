#[doc = "Reader of register AUXHFRCOCTRL"]
pub type R = crate::R<u32, super::AUXHFRCOCTRL>;
#[doc = "Writer for register AUXHFRCOCTRL"]
pub type W = crate::W<u32, super::AUXHFRCOCTRL>;
#[doc = "Register AUXHFRCOCTRL `reset()`'s with value 0x80"]
impl crate::ResetValue for super::AUXHFRCOCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x80
    }
}
#[doc = "Reader of field `TUNING`"]
pub type TUNING_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TUNING`"]
pub struct TUNING_W<'a> {
    w: &'a mut W,
}
impl<'a> TUNING_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "AUXHFRCO Band Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BAND_A {
    #[doc = "0: 14 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    _14MHZ,
    #[doc = "1: 11 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    _11MHZ,
    #[doc = "2: 7 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    _7MHZ,
    #[doc = "3: 1 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    _1MHZ,
    #[doc = "7: 21 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    _21MHZ,
}
impl From<BAND_A> for u8 {
    #[inline(always)]
    fn from(variant: BAND_A) -> Self {
        match variant {
            BAND_A::_14MHZ => 0,
            BAND_A::_11MHZ => 1,
            BAND_A::_7MHZ => 2,
            BAND_A::_1MHZ => 3,
            BAND_A::_21MHZ => 7,
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
            0 => Val(BAND_A::_14MHZ),
            1 => Val(BAND_A::_11MHZ),
            2 => Val(BAND_A::_7MHZ),
            3 => Val(BAND_A::_1MHZ),
            7 => Val(BAND_A::_21MHZ),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_14MHZ`"]
    #[inline(always)]
    pub fn is_14mhz(&self) -> bool {
        *self == BAND_A::_14MHZ
    }
    #[doc = "Checks if the value of the field is `_11MHZ`"]
    #[inline(always)]
    pub fn is_11mhz(&self) -> bool {
        *self == BAND_A::_11MHZ
    }
    #[doc = "Checks if the value of the field is `_7MHZ`"]
    #[inline(always)]
    pub fn is_7mhz(&self) -> bool {
        *self == BAND_A::_7MHZ
    }
    #[doc = "Checks if the value of the field is `_1MHZ`"]
    #[inline(always)]
    pub fn is_1mhz(&self) -> bool {
        *self == BAND_A::_1MHZ
    }
    #[doc = "Checks if the value of the field is `_21MHZ`"]
    #[inline(always)]
    pub fn is_21mhz(&self) -> bool {
        *self == BAND_A::_21MHZ
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
    #[doc = "14 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    #[inline(always)]
    pub fn _14mhz(self) -> &'a mut W {
        self.variant(BAND_A::_14MHZ)
    }
    #[doc = "11 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    #[inline(always)]
    pub fn _11mhz(self) -> &'a mut W {
        self.variant(BAND_A::_11MHZ)
    }
    #[doc = "7 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    #[inline(always)]
    pub fn _7mhz(self) -> &'a mut W {
        self.variant(BAND_A::_7MHZ)
    }
    #[doc = "1 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    #[inline(always)]
    pub fn _1mhz(self) -> &'a mut W {
        self.variant(BAND_A::_1MHZ)
    }
    #[doc = "21 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    #[inline(always)]
    pub fn _21mhz(self) -> &'a mut W {
        self.variant(BAND_A::_21MHZ)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - AUXHFRCO Tuning Value"]
    #[inline(always)]
    pub fn tuning(&self) -> TUNING_R {
        TUNING_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - AUXHFRCO Band Select"]
    #[inline(always)]
    pub fn band(&self) -> BAND_R {
        BAND_R::new(((self.bits >> 8) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - AUXHFRCO Tuning Value"]
    #[inline(always)]
    pub fn tuning(&mut self) -> TUNING_W {
        TUNING_W { w: self }
    }
    #[doc = "Bits 8:10 - AUXHFRCO Band Select"]
    #[inline(always)]
    pub fn band(&mut self) -> BAND_W {
        BAND_W { w: self }
    }
}

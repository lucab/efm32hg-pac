#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0x000c_262c"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x000c_262c
    }
}
#[doc = "HFXO Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HFXOMODE_A {
    #[doc = "0: 4-25 MHz crystal oscillator."]
    XTAL,
    #[doc = "1: An AC coupled buffer is coupled in series with HFXTAL_N, suitable for external sine wave (4-25 MHz). The sine wave should have a minimum of 200 mV peak to peak."]
    BUFEXTCLK,
    #[doc = "2: Digital external clock on HFXTAL_N pin. Oscillator is effectively bypassed."]
    DIGEXTCLK,
}
impl From<HFXOMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: HFXOMODE_A) -> Self {
        match variant {
            HFXOMODE_A::XTAL => 0,
            HFXOMODE_A::BUFEXTCLK => 1,
            HFXOMODE_A::DIGEXTCLK => 2,
        }
    }
}
#[doc = "Reader of field `HFXOMODE`"]
pub type HFXOMODE_R = crate::R<u8, HFXOMODE_A>;
impl HFXOMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, HFXOMODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(HFXOMODE_A::XTAL),
            1 => Val(HFXOMODE_A::BUFEXTCLK),
            2 => Val(HFXOMODE_A::DIGEXTCLK),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `XTAL`"]
    #[inline(always)]
    pub fn is_xtal(&self) -> bool {
        *self == HFXOMODE_A::XTAL
    }
    #[doc = "Checks if the value of the field is `BUFEXTCLK`"]
    #[inline(always)]
    pub fn is_bufextclk(&self) -> bool {
        *self == HFXOMODE_A::BUFEXTCLK
    }
    #[doc = "Checks if the value of the field is `DIGEXTCLK`"]
    #[inline(always)]
    pub fn is_digextclk(&self) -> bool {
        *self == HFXOMODE_A::DIGEXTCLK
    }
}
#[doc = "Write proxy for field `HFXOMODE`"]
pub struct HFXOMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> HFXOMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HFXOMODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "4-25 MHz crystal oscillator."]
    #[inline(always)]
    pub fn xtal(self) -> &'a mut W {
        self.variant(HFXOMODE_A::XTAL)
    }
    #[doc = "An AC coupled buffer is coupled in series with HFXTAL_N, suitable for external sine wave (4-25 MHz). The sine wave should have a minimum of 200 mV peak to peak."]
    #[inline(always)]
    pub fn bufextclk(self) -> &'a mut W {
        self.variant(HFXOMODE_A::BUFEXTCLK)
    }
    #[doc = "Digital external clock on HFXTAL_N pin. Oscillator is effectively bypassed."]
    #[inline(always)]
    pub fn digextclk(self) -> &'a mut W {
        self.variant(HFXOMODE_A::DIGEXTCLK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "HFXO Start-up Boost Current\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HFXOBOOST_A {
    #[doc = "0: 50 %."]
    _50PCENT,
    #[doc = "1: 70 %."]
    _70PCENT,
    #[doc = "2: 80 %."]
    _80PCENT,
    #[doc = "3: 100 % (default)."]
    _100PCENT,
}
impl From<HFXOBOOST_A> for u8 {
    #[inline(always)]
    fn from(variant: HFXOBOOST_A) -> Self {
        match variant {
            HFXOBOOST_A::_50PCENT => 0,
            HFXOBOOST_A::_70PCENT => 1,
            HFXOBOOST_A::_80PCENT => 2,
            HFXOBOOST_A::_100PCENT => 3,
        }
    }
}
#[doc = "Reader of field `HFXOBOOST`"]
pub type HFXOBOOST_R = crate::R<u8, HFXOBOOST_A>;
impl HFXOBOOST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HFXOBOOST_A {
        match self.bits {
            0 => HFXOBOOST_A::_50PCENT,
            1 => HFXOBOOST_A::_70PCENT,
            2 => HFXOBOOST_A::_80PCENT,
            3 => HFXOBOOST_A::_100PCENT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_50PCENT`"]
    #[inline(always)]
    pub fn is_50pcent(&self) -> bool {
        *self == HFXOBOOST_A::_50PCENT
    }
    #[doc = "Checks if the value of the field is `_70PCENT`"]
    #[inline(always)]
    pub fn is_70pcent(&self) -> bool {
        *self == HFXOBOOST_A::_70PCENT
    }
    #[doc = "Checks if the value of the field is `_80PCENT`"]
    #[inline(always)]
    pub fn is_80pcent(&self) -> bool {
        *self == HFXOBOOST_A::_80PCENT
    }
    #[doc = "Checks if the value of the field is `_100PCENT`"]
    #[inline(always)]
    pub fn is_100pcent(&self) -> bool {
        *self == HFXOBOOST_A::_100PCENT
    }
}
#[doc = "Write proxy for field `HFXOBOOST`"]
pub struct HFXOBOOST_W<'a> {
    w: &'a mut W,
}
impl<'a> HFXOBOOST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HFXOBOOST_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "50 %."]
    #[inline(always)]
    pub fn _50pcent(self) -> &'a mut W {
        self.variant(HFXOBOOST_A::_50PCENT)
    }
    #[doc = "70 %."]
    #[inline(always)]
    pub fn _70pcent(self) -> &'a mut W {
        self.variant(HFXOBOOST_A::_70PCENT)
    }
    #[doc = "80 %."]
    #[inline(always)]
    pub fn _80pcent(self) -> &'a mut W {
        self.variant(HFXOBOOST_A::_80PCENT)
    }
    #[doc = "100 % (default)."]
    #[inline(always)]
    pub fn _100pcent(self) -> &'a mut W {
        self.variant(HFXOBOOST_A::_100PCENT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `HFXOBUFCUR`"]
pub type HFXOBUFCUR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HFXOBUFCUR`"]
pub struct HFXOBUFCUR_W<'a> {
    w: &'a mut W,
}
impl<'a> HFXOBUFCUR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
#[doc = "Reader of field `HFXOGLITCHDETEN`"]
pub type HFXOGLITCHDETEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HFXOGLITCHDETEN`"]
pub struct HFXOGLITCHDETEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HFXOGLITCHDETEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "HFXO Timeout\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HFXOTIMEOUT_A {
    #[doc = "0: Timeout period of 8 cycles."]
    _8CYCLES,
    #[doc = "1: Timeout period of 256 cycles."]
    _256CYCLES,
    #[doc = "2: Timeout period of 1024 cycles."]
    _1KCYCLES,
    #[doc = "3: Timeout period of 16384 cycles."]
    _16KCYCLES,
}
impl From<HFXOTIMEOUT_A> for u8 {
    #[inline(always)]
    fn from(variant: HFXOTIMEOUT_A) -> Self {
        match variant {
            HFXOTIMEOUT_A::_8CYCLES => 0,
            HFXOTIMEOUT_A::_256CYCLES => 1,
            HFXOTIMEOUT_A::_1KCYCLES => 2,
            HFXOTIMEOUT_A::_16KCYCLES => 3,
        }
    }
}
#[doc = "Reader of field `HFXOTIMEOUT`"]
pub type HFXOTIMEOUT_R = crate::R<u8, HFXOTIMEOUT_A>;
impl HFXOTIMEOUT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HFXOTIMEOUT_A {
        match self.bits {
            0 => HFXOTIMEOUT_A::_8CYCLES,
            1 => HFXOTIMEOUT_A::_256CYCLES,
            2 => HFXOTIMEOUT_A::_1KCYCLES,
            3 => HFXOTIMEOUT_A::_16KCYCLES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_8CYCLES`"]
    #[inline(always)]
    pub fn is_8cycles(&self) -> bool {
        *self == HFXOTIMEOUT_A::_8CYCLES
    }
    #[doc = "Checks if the value of the field is `_256CYCLES`"]
    #[inline(always)]
    pub fn is_256cycles(&self) -> bool {
        *self == HFXOTIMEOUT_A::_256CYCLES
    }
    #[doc = "Checks if the value of the field is `_1KCYCLES`"]
    #[inline(always)]
    pub fn is_1kcycles(&self) -> bool {
        *self == HFXOTIMEOUT_A::_1KCYCLES
    }
    #[doc = "Checks if the value of the field is `_16KCYCLES`"]
    #[inline(always)]
    pub fn is_16kcycles(&self) -> bool {
        *self == HFXOTIMEOUT_A::_16KCYCLES
    }
}
#[doc = "Write proxy for field `HFXOTIMEOUT`"]
pub struct HFXOTIMEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> HFXOTIMEOUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HFXOTIMEOUT_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Timeout period of 8 cycles."]
    #[inline(always)]
    pub fn _8cycles(self) -> &'a mut W {
        self.variant(HFXOTIMEOUT_A::_8CYCLES)
    }
    #[doc = "Timeout period of 256 cycles."]
    #[inline(always)]
    pub fn _256cycles(self) -> &'a mut W {
        self.variant(HFXOTIMEOUT_A::_256CYCLES)
    }
    #[doc = "Timeout period of 1024 cycles."]
    #[inline(always)]
    pub fn _1kcycles(self) -> &'a mut W {
        self.variant(HFXOTIMEOUT_A::_1KCYCLES)
    }
    #[doc = "Timeout period of 16384 cycles."]
    #[inline(always)]
    pub fn _16kcycles(self) -> &'a mut W {
        self.variant(HFXOTIMEOUT_A::_16KCYCLES)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | (((value as u32) & 0x03) << 9);
        self.w
    }
}
#[doc = "LFXO Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LFXOMODE_A {
    #[doc = "0: 32.768 kHz crystal oscillator."]
    XTAL,
    #[doc = "1: An AC coupled buffer is coupled in series with LFXTAL_N pin, suitable for external sinus wave (32.768 kHz)."]
    BUFEXTCLK,
    #[doc = "2: Digital external clock on LFXTAL_N pin. Oscillator is effectively bypassed."]
    DIGEXTCLK,
}
impl From<LFXOMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: LFXOMODE_A) -> Self {
        match variant {
            LFXOMODE_A::XTAL => 0,
            LFXOMODE_A::BUFEXTCLK => 1,
            LFXOMODE_A::DIGEXTCLK => 2,
        }
    }
}
#[doc = "Reader of field `LFXOMODE`"]
pub type LFXOMODE_R = crate::R<u8, LFXOMODE_A>;
impl LFXOMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LFXOMODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(LFXOMODE_A::XTAL),
            1 => Val(LFXOMODE_A::BUFEXTCLK),
            2 => Val(LFXOMODE_A::DIGEXTCLK),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `XTAL`"]
    #[inline(always)]
    pub fn is_xtal(&self) -> bool {
        *self == LFXOMODE_A::XTAL
    }
    #[doc = "Checks if the value of the field is `BUFEXTCLK`"]
    #[inline(always)]
    pub fn is_bufextclk(&self) -> bool {
        *self == LFXOMODE_A::BUFEXTCLK
    }
    #[doc = "Checks if the value of the field is `DIGEXTCLK`"]
    #[inline(always)]
    pub fn is_digextclk(&self) -> bool {
        *self == LFXOMODE_A::DIGEXTCLK
    }
}
#[doc = "Write proxy for field `LFXOMODE`"]
pub struct LFXOMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> LFXOMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LFXOMODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "32.768 kHz crystal oscillator."]
    #[inline(always)]
    pub fn xtal(self) -> &'a mut W {
        self.variant(LFXOMODE_A::XTAL)
    }
    #[doc = "An AC coupled buffer is coupled in series with LFXTAL_N pin, suitable for external sinus wave (32.768 kHz)."]
    #[inline(always)]
    pub fn bufextclk(self) -> &'a mut W {
        self.variant(LFXOMODE_A::BUFEXTCLK)
    }
    #[doc = "Digital external clock on LFXTAL_N pin. Oscillator is effectively bypassed."]
    #[inline(always)]
    pub fn digextclk(self) -> &'a mut W {
        self.variant(LFXOMODE_A::DIGEXTCLK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | (((value as u32) & 0x03) << 11);
        self.w
    }
}
#[doc = "Reader of field `LFXOBOOST`"]
pub type LFXOBOOST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LFXOBOOST`"]
pub struct LFXOBOOST_W<'a> {
    w: &'a mut W,
}
impl<'a> LFXOBOOST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `HFCLKDIV`"]
pub type HFCLKDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HFCLKDIV`"]
pub struct HFCLKDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> HFCLKDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 14)) | (((value as u32) & 0x07) << 14);
        self.w
    }
}
#[doc = "Reader of field `LFXOBUFCUR`"]
pub type LFXOBUFCUR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LFXOBUFCUR`"]
pub struct LFXOBUFCUR_W<'a> {
    w: &'a mut W,
}
impl<'a> LFXOBUFCUR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "LFXO Timeout\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LFXOTIMEOUT_A {
    #[doc = "0: Timeout period of 8 cycles."]
    _8CYCLES,
    #[doc = "1: Timeout period of 1024 cycles."]
    _1KCYCLES,
    #[doc = "2: Timeout period of 16384 cycles."]
    _16KCYCLES,
    #[doc = "3: Timeout period of 32768 cycles."]
    _32KCYCLES,
}
impl From<LFXOTIMEOUT_A> for u8 {
    #[inline(always)]
    fn from(variant: LFXOTIMEOUT_A) -> Self {
        match variant {
            LFXOTIMEOUT_A::_8CYCLES => 0,
            LFXOTIMEOUT_A::_1KCYCLES => 1,
            LFXOTIMEOUT_A::_16KCYCLES => 2,
            LFXOTIMEOUT_A::_32KCYCLES => 3,
        }
    }
}
#[doc = "Reader of field `LFXOTIMEOUT`"]
pub type LFXOTIMEOUT_R = crate::R<u8, LFXOTIMEOUT_A>;
impl LFXOTIMEOUT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LFXOTIMEOUT_A {
        match self.bits {
            0 => LFXOTIMEOUT_A::_8CYCLES,
            1 => LFXOTIMEOUT_A::_1KCYCLES,
            2 => LFXOTIMEOUT_A::_16KCYCLES,
            3 => LFXOTIMEOUT_A::_32KCYCLES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_8CYCLES`"]
    #[inline(always)]
    pub fn is_8cycles(&self) -> bool {
        *self == LFXOTIMEOUT_A::_8CYCLES
    }
    #[doc = "Checks if the value of the field is `_1KCYCLES`"]
    #[inline(always)]
    pub fn is_1kcycles(&self) -> bool {
        *self == LFXOTIMEOUT_A::_1KCYCLES
    }
    #[doc = "Checks if the value of the field is `_16KCYCLES`"]
    #[inline(always)]
    pub fn is_16kcycles(&self) -> bool {
        *self == LFXOTIMEOUT_A::_16KCYCLES
    }
    #[doc = "Checks if the value of the field is `_32KCYCLES`"]
    #[inline(always)]
    pub fn is_32kcycles(&self) -> bool {
        *self == LFXOTIMEOUT_A::_32KCYCLES
    }
}
#[doc = "Write proxy for field `LFXOTIMEOUT`"]
pub struct LFXOTIMEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> LFXOTIMEOUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LFXOTIMEOUT_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Timeout period of 8 cycles."]
    #[inline(always)]
    pub fn _8cycles(self) -> &'a mut W {
        self.variant(LFXOTIMEOUT_A::_8CYCLES)
    }
    #[doc = "Timeout period of 1024 cycles."]
    #[inline(always)]
    pub fn _1kcycles(self) -> &'a mut W {
        self.variant(LFXOTIMEOUT_A::_1KCYCLES)
    }
    #[doc = "Timeout period of 16384 cycles."]
    #[inline(always)]
    pub fn _16kcycles(self) -> &'a mut W {
        self.variant(LFXOTIMEOUT_A::_16KCYCLES)
    }
    #[doc = "Timeout period of 32768 cycles."]
    #[inline(always)]
    pub fn _32kcycles(self) -> &'a mut W {
        self.variant(LFXOTIMEOUT_A::_32KCYCLES)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Clock Output Select 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKOUTSEL0_A {
    #[doc = "0: HFRCO (directly from oscillator)."]
    HFRCO,
    #[doc = "1: HFXO (directly from oscillator)."]
    HFXO,
    #[doc = "2: HFCLK/2."]
    HFCLK2,
    #[doc = "3: HFCLK/4."]
    HFCLK4,
    #[doc = "4: HFCLK/8."]
    HFCLK8,
    #[doc = "5: HFCLK/16."]
    HFCLK16,
    #[doc = "6: ULFRCO (directly from oscillator)."]
    ULFRCO,
    #[doc = "7: AUXHFRCO (directly from oscillator)."]
    AUXHFRCO,
}
impl From<CLKOUTSEL0_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKOUTSEL0_A) -> Self {
        match variant {
            CLKOUTSEL0_A::HFRCO => 0,
            CLKOUTSEL0_A::HFXO => 1,
            CLKOUTSEL0_A::HFCLK2 => 2,
            CLKOUTSEL0_A::HFCLK4 => 3,
            CLKOUTSEL0_A::HFCLK8 => 4,
            CLKOUTSEL0_A::HFCLK16 => 5,
            CLKOUTSEL0_A::ULFRCO => 6,
            CLKOUTSEL0_A::AUXHFRCO => 7,
        }
    }
}
#[doc = "Reader of field `CLKOUTSEL0`"]
pub type CLKOUTSEL0_R = crate::R<u8, CLKOUTSEL0_A>;
impl CLKOUTSEL0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKOUTSEL0_A {
        match self.bits {
            0 => CLKOUTSEL0_A::HFRCO,
            1 => CLKOUTSEL0_A::HFXO,
            2 => CLKOUTSEL0_A::HFCLK2,
            3 => CLKOUTSEL0_A::HFCLK4,
            4 => CLKOUTSEL0_A::HFCLK8,
            5 => CLKOUTSEL0_A::HFCLK16,
            6 => CLKOUTSEL0_A::ULFRCO,
            7 => CLKOUTSEL0_A::AUXHFRCO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `HFRCO`"]
    #[inline(always)]
    pub fn is_hfrco(&self) -> bool {
        *self == CLKOUTSEL0_A::HFRCO
    }
    #[doc = "Checks if the value of the field is `HFXO`"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == CLKOUTSEL0_A::HFXO
    }
    #[doc = "Checks if the value of the field is `HFCLK2`"]
    #[inline(always)]
    pub fn is_hfclk2(&self) -> bool {
        *self == CLKOUTSEL0_A::HFCLK2
    }
    #[doc = "Checks if the value of the field is `HFCLK4`"]
    #[inline(always)]
    pub fn is_hfclk4(&self) -> bool {
        *self == CLKOUTSEL0_A::HFCLK4
    }
    #[doc = "Checks if the value of the field is `HFCLK8`"]
    #[inline(always)]
    pub fn is_hfclk8(&self) -> bool {
        *self == CLKOUTSEL0_A::HFCLK8
    }
    #[doc = "Checks if the value of the field is `HFCLK16`"]
    #[inline(always)]
    pub fn is_hfclk16(&self) -> bool {
        *self == CLKOUTSEL0_A::HFCLK16
    }
    #[doc = "Checks if the value of the field is `ULFRCO`"]
    #[inline(always)]
    pub fn is_ulfrco(&self) -> bool {
        *self == CLKOUTSEL0_A::ULFRCO
    }
    #[doc = "Checks if the value of the field is `AUXHFRCO`"]
    #[inline(always)]
    pub fn is_auxhfrco(&self) -> bool {
        *self == CLKOUTSEL0_A::AUXHFRCO
    }
}
#[doc = "Write proxy for field `CLKOUTSEL0`"]
pub struct CLKOUTSEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKOUTSEL0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKOUTSEL0_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "HFRCO (directly from oscillator)."]
    #[inline(always)]
    pub fn hfrco(self) -> &'a mut W {
        self.variant(CLKOUTSEL0_A::HFRCO)
    }
    #[doc = "HFXO (directly from oscillator)."]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut W {
        self.variant(CLKOUTSEL0_A::HFXO)
    }
    #[doc = "HFCLK/2."]
    #[inline(always)]
    pub fn hfclk2(self) -> &'a mut W {
        self.variant(CLKOUTSEL0_A::HFCLK2)
    }
    #[doc = "HFCLK/4."]
    #[inline(always)]
    pub fn hfclk4(self) -> &'a mut W {
        self.variant(CLKOUTSEL0_A::HFCLK4)
    }
    #[doc = "HFCLK/8."]
    #[inline(always)]
    pub fn hfclk8(self) -> &'a mut W {
        self.variant(CLKOUTSEL0_A::HFCLK8)
    }
    #[doc = "HFCLK/16."]
    #[inline(always)]
    pub fn hfclk16(self) -> &'a mut W {
        self.variant(CLKOUTSEL0_A::HFCLK16)
    }
    #[doc = "ULFRCO (directly from oscillator)."]
    #[inline(always)]
    pub fn ulfrco(self) -> &'a mut W {
        self.variant(CLKOUTSEL0_A::ULFRCO)
    }
    #[doc = "AUXHFRCO (directly from oscillator)."]
    #[inline(always)]
    pub fn auxhfrco(self) -> &'a mut W {
        self.variant(CLKOUTSEL0_A::AUXHFRCO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
#[doc = "Clock Output Select 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKOUTSEL1_A {
    #[doc = "0: LFRCO (directly from oscillator)."]
    LFRCO,
    #[doc = "1: LFXO (directly from oscillator)."]
    LFXO,
    #[doc = "2: HFCLK (undivided)."]
    HFCLK,
    #[doc = "3: LFXO (qualified)."]
    LFXOQ,
    #[doc = "4: HFXO (qualified)."]
    HFXOQ,
    #[doc = "5: LFRCO (qualified)."]
    LFRCOQ,
    #[doc = "6: HFRCO (qualified)."]
    HFRCOQ,
    #[doc = "7: AUXHFRCO (qualified)."]
    AUXHFRCOQ,
    #[doc = "8: USHFRCO"]
    USHFRCO,
}
impl From<CLKOUTSEL1_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKOUTSEL1_A) -> Self {
        match variant {
            CLKOUTSEL1_A::LFRCO => 0,
            CLKOUTSEL1_A::LFXO => 1,
            CLKOUTSEL1_A::HFCLK => 2,
            CLKOUTSEL1_A::LFXOQ => 3,
            CLKOUTSEL1_A::HFXOQ => 4,
            CLKOUTSEL1_A::LFRCOQ => 5,
            CLKOUTSEL1_A::HFRCOQ => 6,
            CLKOUTSEL1_A::AUXHFRCOQ => 7,
            CLKOUTSEL1_A::USHFRCO => 8,
        }
    }
}
#[doc = "Reader of field `CLKOUTSEL1`"]
pub type CLKOUTSEL1_R = crate::R<u8, CLKOUTSEL1_A>;
impl CLKOUTSEL1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CLKOUTSEL1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CLKOUTSEL1_A::LFRCO),
            1 => Val(CLKOUTSEL1_A::LFXO),
            2 => Val(CLKOUTSEL1_A::HFCLK),
            3 => Val(CLKOUTSEL1_A::LFXOQ),
            4 => Val(CLKOUTSEL1_A::HFXOQ),
            5 => Val(CLKOUTSEL1_A::LFRCOQ),
            6 => Val(CLKOUTSEL1_A::HFRCOQ),
            7 => Val(CLKOUTSEL1_A::AUXHFRCOQ),
            8 => Val(CLKOUTSEL1_A::USHFRCO),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LFRCO`"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == CLKOUTSEL1_A::LFRCO
    }
    #[doc = "Checks if the value of the field is `LFXO`"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == CLKOUTSEL1_A::LFXO
    }
    #[doc = "Checks if the value of the field is `HFCLK`"]
    #[inline(always)]
    pub fn is_hfclk(&self) -> bool {
        *self == CLKOUTSEL1_A::HFCLK
    }
    #[doc = "Checks if the value of the field is `LFXOQ`"]
    #[inline(always)]
    pub fn is_lfxoq(&self) -> bool {
        *self == CLKOUTSEL1_A::LFXOQ
    }
    #[doc = "Checks if the value of the field is `HFXOQ`"]
    #[inline(always)]
    pub fn is_hfxoq(&self) -> bool {
        *self == CLKOUTSEL1_A::HFXOQ
    }
    #[doc = "Checks if the value of the field is `LFRCOQ`"]
    #[inline(always)]
    pub fn is_lfrcoq(&self) -> bool {
        *self == CLKOUTSEL1_A::LFRCOQ
    }
    #[doc = "Checks if the value of the field is `HFRCOQ`"]
    #[inline(always)]
    pub fn is_hfrcoq(&self) -> bool {
        *self == CLKOUTSEL1_A::HFRCOQ
    }
    #[doc = "Checks if the value of the field is `AUXHFRCOQ`"]
    #[inline(always)]
    pub fn is_auxhfrcoq(&self) -> bool {
        *self == CLKOUTSEL1_A::AUXHFRCOQ
    }
    #[doc = "Checks if the value of the field is `USHFRCO`"]
    #[inline(always)]
    pub fn is_ushfrco(&self) -> bool {
        *self == CLKOUTSEL1_A::USHFRCO
    }
}
#[doc = "Write proxy for field `CLKOUTSEL1`"]
pub struct CLKOUTSEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKOUTSEL1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKOUTSEL1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "LFRCO (directly from oscillator)."]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut W {
        self.variant(CLKOUTSEL1_A::LFRCO)
    }
    #[doc = "LFXO (directly from oscillator)."]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut W {
        self.variant(CLKOUTSEL1_A::LFXO)
    }
    #[doc = "HFCLK (undivided)."]
    #[inline(always)]
    pub fn hfclk(self) -> &'a mut W {
        self.variant(CLKOUTSEL1_A::HFCLK)
    }
    #[doc = "LFXO (qualified)."]
    #[inline(always)]
    pub fn lfxoq(self) -> &'a mut W {
        self.variant(CLKOUTSEL1_A::LFXOQ)
    }
    #[doc = "HFXO (qualified)."]
    #[inline(always)]
    pub fn hfxoq(self) -> &'a mut W {
        self.variant(CLKOUTSEL1_A::HFXOQ)
    }
    #[doc = "LFRCO (qualified)."]
    #[inline(always)]
    pub fn lfrcoq(self) -> &'a mut W {
        self.variant(CLKOUTSEL1_A::LFRCOQ)
    }
    #[doc = "HFRCO (qualified)."]
    #[inline(always)]
    pub fn hfrcoq(self) -> &'a mut W {
        self.variant(CLKOUTSEL1_A::HFRCOQ)
    }
    #[doc = "AUXHFRCO (qualified)."]
    #[inline(always)]
    pub fn auxhfrcoq(self) -> &'a mut W {
        self.variant(CLKOUTSEL1_A::AUXHFRCOQ)
    }
    #[doc = "USHFRCO"]
    #[inline(always)]
    pub fn ushfrco(self) -> &'a mut W {
        self.variant(CLKOUTSEL1_A::USHFRCO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 23)) | (((value as u32) & 0x0f) << 23);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - HFXO Mode"]
    #[inline(always)]
    pub fn hfxomode(&self) -> HFXOMODE_R {
        HFXOMODE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - HFXO Start-up Boost Current"]
    #[inline(always)]
    pub fn hfxoboost(&self) -> HFXOBOOST_R {
        HFXOBOOST_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 5:6 - HFXO Boost Buffer Current"]
    #[inline(always)]
    pub fn hfxobufcur(&self) -> HFXOBUFCUR_R {
        HFXOBUFCUR_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 7 - HFXO Glitch Detector Enable"]
    #[inline(always)]
    pub fn hfxoglitchdeten(&self) -> HFXOGLITCHDETEN_R {
        HFXOGLITCHDETEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 9:10 - HFXO Timeout"]
    #[inline(always)]
    pub fn hfxotimeout(&self) -> HFXOTIMEOUT_R {
        HFXOTIMEOUT_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bits 11:12 - LFXO Mode"]
    #[inline(always)]
    pub fn lfxomode(&self) -> LFXOMODE_R {
        LFXOMODE_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bit 13 - LFXO Start-up Boost Current"]
    #[inline(always)]
    pub fn lfxoboost(&self) -> LFXOBOOST_R {
        LFXOBOOST_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 14:16 - HFCLK Division"]
    #[inline(always)]
    pub fn hfclkdiv(&self) -> HFCLKDIV_R {
        HFCLKDIV_R::new(((self.bits >> 14) & 0x07) as u8)
    }
    #[doc = "Bit 17 - LFXO Boost Buffer Current"]
    #[inline(always)]
    pub fn lfxobufcur(&self) -> LFXOBUFCUR_R {
        LFXOBUFCUR_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 18:19 - LFXO Timeout"]
    #[inline(always)]
    pub fn lfxotimeout(&self) -> LFXOTIMEOUT_R {
        LFXOTIMEOUT_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:22 - Clock Output Select 0"]
    #[inline(always)]
    pub fn clkoutsel0(&self) -> CLKOUTSEL0_R {
        CLKOUTSEL0_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 23:26 - Clock Output Select 1"]
    #[inline(always)]
    pub fn clkoutsel1(&self) -> CLKOUTSEL1_R {
        CLKOUTSEL1_R::new(((self.bits >> 23) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - HFXO Mode"]
    #[inline(always)]
    pub fn hfxomode(&mut self) -> HFXOMODE_W {
        HFXOMODE_W { w: self }
    }
    #[doc = "Bits 2:3 - HFXO Start-up Boost Current"]
    #[inline(always)]
    pub fn hfxoboost(&mut self) -> HFXOBOOST_W {
        HFXOBOOST_W { w: self }
    }
    #[doc = "Bits 5:6 - HFXO Boost Buffer Current"]
    #[inline(always)]
    pub fn hfxobufcur(&mut self) -> HFXOBUFCUR_W {
        HFXOBUFCUR_W { w: self }
    }
    #[doc = "Bit 7 - HFXO Glitch Detector Enable"]
    #[inline(always)]
    pub fn hfxoglitchdeten(&mut self) -> HFXOGLITCHDETEN_W {
        HFXOGLITCHDETEN_W { w: self }
    }
    #[doc = "Bits 9:10 - HFXO Timeout"]
    #[inline(always)]
    pub fn hfxotimeout(&mut self) -> HFXOTIMEOUT_W {
        HFXOTIMEOUT_W { w: self }
    }
    #[doc = "Bits 11:12 - LFXO Mode"]
    #[inline(always)]
    pub fn lfxomode(&mut self) -> LFXOMODE_W {
        LFXOMODE_W { w: self }
    }
    #[doc = "Bit 13 - LFXO Start-up Boost Current"]
    #[inline(always)]
    pub fn lfxoboost(&mut self) -> LFXOBOOST_W {
        LFXOBOOST_W { w: self }
    }
    #[doc = "Bits 14:16 - HFCLK Division"]
    #[inline(always)]
    pub fn hfclkdiv(&mut self) -> HFCLKDIV_W {
        HFCLKDIV_W { w: self }
    }
    #[doc = "Bit 17 - LFXO Boost Buffer Current"]
    #[inline(always)]
    pub fn lfxobufcur(&mut self) -> LFXOBUFCUR_W {
        LFXOBUFCUR_W { w: self }
    }
    #[doc = "Bits 18:19 - LFXO Timeout"]
    #[inline(always)]
    pub fn lfxotimeout(&mut self) -> LFXOTIMEOUT_W {
        LFXOTIMEOUT_W { w: self }
    }
    #[doc = "Bits 20:22 - Clock Output Select 0"]
    #[inline(always)]
    pub fn clkoutsel0(&mut self) -> CLKOUTSEL0_W {
        CLKOUTSEL0_W { w: self }
    }
    #[doc = "Bits 23:26 - Clock Output Select 1"]
    #[inline(always)]
    pub fn clkoutsel1(&mut self) -> CLKOUTSEL1_W {
        CLKOUTSEL1_W { w: self }
    }
}

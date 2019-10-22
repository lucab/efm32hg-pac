#[doc = "Reader of register INPUTSEL"]
pub type R = crate::R<u32, super::INPUTSEL>;
#[doc = "Writer for register INPUTSEL"]
pub type W = crate::W<u32, super::INPUTSEL>;
#[doc = "Register INPUTSEL `reset()`'s with value 0x0001_0080"]
impl crate::ResetValue for super::INPUTSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0001_0080
    }
}
#[doc = "Positive Input Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POSSEL_A {
    #[doc = "0: Channel 0 as positive input."]
    CH0,
    #[doc = "1: Channel 1 as positive input."]
    CH1,
    #[doc = "2: Channel 2 as positive input."]
    CH2,
    #[doc = "3: Channel 3 as positive input."]
    CH3,
    #[doc = "4: Channel 4 as positive input."]
    CH4,
    #[doc = "5: Channel 5 as positive input."]
    CH5,
    #[doc = "6: Channel 6 as positive input."]
    CH6,
    #[doc = "7: Channel 7 as positive input."]
    CH7,
}
impl From<POSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: POSSEL_A) -> Self {
        match variant {
            POSSEL_A::CH0 => 0,
            POSSEL_A::CH1 => 1,
            POSSEL_A::CH2 => 2,
            POSSEL_A::CH3 => 3,
            POSSEL_A::CH4 => 4,
            POSSEL_A::CH5 => 5,
            POSSEL_A::CH6 => 6,
            POSSEL_A::CH7 => 7,
        }
    }
}
#[doc = "Reader of field `POSSEL`"]
pub type POSSEL_R = crate::R<u8, POSSEL_A>;
impl POSSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POSSEL_A {
        match self.bits {
            0 => POSSEL_A::CH0,
            1 => POSSEL_A::CH1,
            2 => POSSEL_A::CH2,
            3 => POSSEL_A::CH3,
            4 => POSSEL_A::CH4,
            5 => POSSEL_A::CH5,
            6 => POSSEL_A::CH6,
            7 => POSSEL_A::CH7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CH0`"]
    #[inline(always)]
    pub fn is_ch0(&self) -> bool {
        *self == POSSEL_A::CH0
    }
    #[doc = "Checks if the value of the field is `CH1`"]
    #[inline(always)]
    pub fn is_ch1(&self) -> bool {
        *self == POSSEL_A::CH1
    }
    #[doc = "Checks if the value of the field is `CH2`"]
    #[inline(always)]
    pub fn is_ch2(&self) -> bool {
        *self == POSSEL_A::CH2
    }
    #[doc = "Checks if the value of the field is `CH3`"]
    #[inline(always)]
    pub fn is_ch3(&self) -> bool {
        *self == POSSEL_A::CH3
    }
    #[doc = "Checks if the value of the field is `CH4`"]
    #[inline(always)]
    pub fn is_ch4(&self) -> bool {
        *self == POSSEL_A::CH4
    }
    #[doc = "Checks if the value of the field is `CH5`"]
    #[inline(always)]
    pub fn is_ch5(&self) -> bool {
        *self == POSSEL_A::CH5
    }
    #[doc = "Checks if the value of the field is `CH6`"]
    #[inline(always)]
    pub fn is_ch6(&self) -> bool {
        *self == POSSEL_A::CH6
    }
    #[doc = "Checks if the value of the field is `CH7`"]
    #[inline(always)]
    pub fn is_ch7(&self) -> bool {
        *self == POSSEL_A::CH7
    }
}
#[doc = "Write proxy for field `POSSEL`"]
pub struct POSSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> POSSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POSSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Channel 0 as positive input."]
    #[inline(always)]
    pub fn ch0(self) -> &'a mut W {
        self.variant(POSSEL_A::CH0)
    }
    #[doc = "Channel 1 as positive input."]
    #[inline(always)]
    pub fn ch1(self) -> &'a mut W {
        self.variant(POSSEL_A::CH1)
    }
    #[doc = "Channel 2 as positive input."]
    #[inline(always)]
    pub fn ch2(self) -> &'a mut W {
        self.variant(POSSEL_A::CH2)
    }
    #[doc = "Channel 3 as positive input."]
    #[inline(always)]
    pub fn ch3(self) -> &'a mut W {
        self.variant(POSSEL_A::CH3)
    }
    #[doc = "Channel 4 as positive input."]
    #[inline(always)]
    pub fn ch4(self) -> &'a mut W {
        self.variant(POSSEL_A::CH4)
    }
    #[doc = "Channel 5 as positive input."]
    #[inline(always)]
    pub fn ch5(self) -> &'a mut W {
        self.variant(POSSEL_A::CH5)
    }
    #[doc = "Channel 6 as positive input."]
    #[inline(always)]
    pub fn ch6(self) -> &'a mut W {
        self.variant(POSSEL_A::CH6)
    }
    #[doc = "Channel 7 as positive input."]
    #[inline(always)]
    pub fn ch7(self) -> &'a mut W {
        self.variant(POSSEL_A::CH7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Negative Input Select\n\nValue on reset: 8"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NEGSEL_A {
    #[doc = "0: Channel 0 as negative input."]
    CH0,
    #[doc = "1: Channel 1 as negative input."]
    CH1,
    #[doc = "2: Channel 2 as negative input."]
    CH2,
    #[doc = "3: Channel 3 as negative input."]
    CH3,
    #[doc = "4: Channel 4 as negative input."]
    CH4,
    #[doc = "5: Channel 5 as negative input."]
    CH5,
    #[doc = "6: Channel 6 as negative input."]
    CH6,
    #[doc = "7: Channel 7 as negative input."]
    CH7,
    #[doc = "8: 1.25 V as negative input."]
    _1V25,
    #[doc = "9: 2.5 V as negative input."]
    _2V5,
    #[doc = "10: Scaled VDD as negative input."]
    VDD,
    #[doc = "11: Capacitive sense mode."]
    CAPSENSE,
}
impl From<NEGSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: NEGSEL_A) -> Self {
        match variant {
            NEGSEL_A::CH0 => 0,
            NEGSEL_A::CH1 => 1,
            NEGSEL_A::CH2 => 2,
            NEGSEL_A::CH3 => 3,
            NEGSEL_A::CH4 => 4,
            NEGSEL_A::CH5 => 5,
            NEGSEL_A::CH6 => 6,
            NEGSEL_A::CH7 => 7,
            NEGSEL_A::_1V25 => 8,
            NEGSEL_A::_2V5 => 9,
            NEGSEL_A::VDD => 10,
            NEGSEL_A::CAPSENSE => 11,
        }
    }
}
#[doc = "Reader of field `NEGSEL`"]
pub type NEGSEL_R = crate::R<u8, NEGSEL_A>;
impl NEGSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, NEGSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(NEGSEL_A::CH0),
            1 => Val(NEGSEL_A::CH1),
            2 => Val(NEGSEL_A::CH2),
            3 => Val(NEGSEL_A::CH3),
            4 => Val(NEGSEL_A::CH4),
            5 => Val(NEGSEL_A::CH5),
            6 => Val(NEGSEL_A::CH6),
            7 => Val(NEGSEL_A::CH7),
            8 => Val(NEGSEL_A::_1V25),
            9 => Val(NEGSEL_A::_2V5),
            10 => Val(NEGSEL_A::VDD),
            11 => Val(NEGSEL_A::CAPSENSE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CH0`"]
    #[inline(always)]
    pub fn is_ch0(&self) -> bool {
        *self == NEGSEL_A::CH0
    }
    #[doc = "Checks if the value of the field is `CH1`"]
    #[inline(always)]
    pub fn is_ch1(&self) -> bool {
        *self == NEGSEL_A::CH1
    }
    #[doc = "Checks if the value of the field is `CH2`"]
    #[inline(always)]
    pub fn is_ch2(&self) -> bool {
        *self == NEGSEL_A::CH2
    }
    #[doc = "Checks if the value of the field is `CH3`"]
    #[inline(always)]
    pub fn is_ch3(&self) -> bool {
        *self == NEGSEL_A::CH3
    }
    #[doc = "Checks if the value of the field is `CH4`"]
    #[inline(always)]
    pub fn is_ch4(&self) -> bool {
        *self == NEGSEL_A::CH4
    }
    #[doc = "Checks if the value of the field is `CH5`"]
    #[inline(always)]
    pub fn is_ch5(&self) -> bool {
        *self == NEGSEL_A::CH5
    }
    #[doc = "Checks if the value of the field is `CH6`"]
    #[inline(always)]
    pub fn is_ch6(&self) -> bool {
        *self == NEGSEL_A::CH6
    }
    #[doc = "Checks if the value of the field is `CH7`"]
    #[inline(always)]
    pub fn is_ch7(&self) -> bool {
        *self == NEGSEL_A::CH7
    }
    #[doc = "Checks if the value of the field is `_1V25`"]
    #[inline(always)]
    pub fn is_1v25(&self) -> bool {
        *self == NEGSEL_A::_1V25
    }
    #[doc = "Checks if the value of the field is `_2V5`"]
    #[inline(always)]
    pub fn is_2v5(&self) -> bool {
        *self == NEGSEL_A::_2V5
    }
    #[doc = "Checks if the value of the field is `VDD`"]
    #[inline(always)]
    pub fn is_vdd(&self) -> bool {
        *self == NEGSEL_A::VDD
    }
    #[doc = "Checks if the value of the field is `CAPSENSE`"]
    #[inline(always)]
    pub fn is_capsense(&self) -> bool {
        *self == NEGSEL_A::CAPSENSE
    }
}
#[doc = "Write proxy for field `NEGSEL`"]
pub struct NEGSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> NEGSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NEGSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Channel 0 as negative input."]
    #[inline(always)]
    pub fn ch0(self) -> &'a mut W {
        self.variant(NEGSEL_A::CH0)
    }
    #[doc = "Channel 1 as negative input."]
    #[inline(always)]
    pub fn ch1(self) -> &'a mut W {
        self.variant(NEGSEL_A::CH1)
    }
    #[doc = "Channel 2 as negative input."]
    #[inline(always)]
    pub fn ch2(self) -> &'a mut W {
        self.variant(NEGSEL_A::CH2)
    }
    #[doc = "Channel 3 as negative input."]
    #[inline(always)]
    pub fn ch3(self) -> &'a mut W {
        self.variant(NEGSEL_A::CH3)
    }
    #[doc = "Channel 4 as negative input."]
    #[inline(always)]
    pub fn ch4(self) -> &'a mut W {
        self.variant(NEGSEL_A::CH4)
    }
    #[doc = "Channel 5 as negative input."]
    #[inline(always)]
    pub fn ch5(self) -> &'a mut W {
        self.variant(NEGSEL_A::CH5)
    }
    #[doc = "Channel 6 as negative input."]
    #[inline(always)]
    pub fn ch6(self) -> &'a mut W {
        self.variant(NEGSEL_A::CH6)
    }
    #[doc = "Channel 7 as negative input."]
    #[inline(always)]
    pub fn ch7(self) -> &'a mut W {
        self.variant(NEGSEL_A::CH7)
    }
    #[doc = "1.25 V as negative input."]
    #[inline(always)]
    pub fn _1v25(self) -> &'a mut W {
        self.variant(NEGSEL_A::_1V25)
    }
    #[doc = "2.5 V as negative input."]
    #[inline(always)]
    pub fn _2v5(self) -> &'a mut W {
        self.variant(NEGSEL_A::_2V5)
    }
    #[doc = "Scaled VDD as negative input."]
    #[inline(always)]
    pub fn vdd(self) -> &'a mut W {
        self.variant(NEGSEL_A::VDD)
    }
    #[doc = "Capacitive sense mode."]
    #[inline(always)]
    pub fn capsense(self) -> &'a mut W {
        self.variant(NEGSEL_A::CAPSENSE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `VDDLEVEL`"]
pub type VDDLEVEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VDDLEVEL`"]
pub struct VDDLEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDLEVEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `CSRESEN`"]
pub type CSRESEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CSRESEN`"]
pub struct CSRESEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CSRESEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Capacitive Sense Mode Internal Resistor Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSRESSEL_A {
    #[doc = "0: Internal capacitive sense resistor value 0."]
    RES0,
    #[doc = "1: Internal capacitive sense resistor value 1."]
    RES1,
    #[doc = "2: Internal capacitive sense resistor value 2."]
    RES2,
    #[doc = "3: Internal capacitive sense resistor value 3."]
    RES3,
}
impl From<CSRESSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CSRESSEL_A) -> Self {
        match variant {
            CSRESSEL_A::RES0 => 0,
            CSRESSEL_A::RES1 => 1,
            CSRESSEL_A::RES2 => 2,
            CSRESSEL_A::RES3 => 3,
        }
    }
}
#[doc = "Reader of field `CSRESSEL`"]
pub type CSRESSEL_R = crate::R<u8, CSRESSEL_A>;
impl CSRESSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSRESSEL_A {
        match self.bits {
            0 => CSRESSEL_A::RES0,
            1 => CSRESSEL_A::RES1,
            2 => CSRESSEL_A::RES2,
            3 => CSRESSEL_A::RES3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RES0`"]
    #[inline(always)]
    pub fn is_res0(&self) -> bool {
        *self == CSRESSEL_A::RES0
    }
    #[doc = "Checks if the value of the field is `RES1`"]
    #[inline(always)]
    pub fn is_res1(&self) -> bool {
        *self == CSRESSEL_A::RES1
    }
    #[doc = "Checks if the value of the field is `RES2`"]
    #[inline(always)]
    pub fn is_res2(&self) -> bool {
        *self == CSRESSEL_A::RES2
    }
    #[doc = "Checks if the value of the field is `RES3`"]
    #[inline(always)]
    pub fn is_res3(&self) -> bool {
        *self == CSRESSEL_A::RES3
    }
}
#[doc = "Write proxy for field `CSRESSEL`"]
pub struct CSRESSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CSRESSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSRESSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Internal capacitive sense resistor value 0."]
    #[inline(always)]
    pub fn res0(self) -> &'a mut W {
        self.variant(CSRESSEL_A::RES0)
    }
    #[doc = "Internal capacitive sense resistor value 1."]
    #[inline(always)]
    pub fn res1(self) -> &'a mut W {
        self.variant(CSRESSEL_A::RES1)
    }
    #[doc = "Internal capacitive sense resistor value 2."]
    #[inline(always)]
    pub fn res2(self) -> &'a mut W {
        self.variant(CSRESSEL_A::RES2)
    }
    #[doc = "Internal capacitive sense resistor value 3."]
    #[inline(always)]
    pub fn res3(self) -> &'a mut W {
        self.variant(CSRESSEL_A::RES3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Positive Input Select"]
    #[inline(always)]
    pub fn possel(&self) -> POSSEL_R {
        POSSEL_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:7 - Negative Input Select"]
    #[inline(always)]
    pub fn negsel(&self) -> NEGSEL_R {
        NEGSEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:13 - VDD Reference Level"]
    #[inline(always)]
    pub fn vddlevel(&self) -> VDDLEVEL_R {
        VDDLEVEL_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - Low Power Reference Mode"]
    #[inline(always)]
    pub fn lpref(&self) -> LPREF_R {
        LPREF_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Capacitive Sense Mode Internal Resistor Enable"]
    #[inline(always)]
    pub fn csresen(&self) -> CSRESEN_R {
        CSRESEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 28:29 - Capacitive Sense Mode Internal Resistor Select"]
    #[inline(always)]
    pub fn csressel(&self) -> CSRESSEL_R {
        CSRESSEL_R::new(((self.bits >> 28) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Positive Input Select"]
    #[inline(always)]
    pub fn possel(&mut self) -> POSSEL_W {
        POSSEL_W { w: self }
    }
    #[doc = "Bits 4:7 - Negative Input Select"]
    #[inline(always)]
    pub fn negsel(&mut self) -> NEGSEL_W {
        NEGSEL_W { w: self }
    }
    #[doc = "Bits 8:13 - VDD Reference Level"]
    #[inline(always)]
    pub fn vddlevel(&mut self) -> VDDLEVEL_W {
        VDDLEVEL_W { w: self }
    }
    #[doc = "Bit 16 - Low Power Reference Mode"]
    #[inline(always)]
    pub fn lpref(&mut self) -> LPREF_W {
        LPREF_W { w: self }
    }
    #[doc = "Bit 24 - Capacitive Sense Mode Internal Resistor Enable"]
    #[inline(always)]
    pub fn csresen(&mut self) -> CSRESEN_W {
        CSRESEN_W { w: self }
    }
    #[doc = "Bits 28:29 - Capacitive Sense Mode Internal Resistor Select"]
    #[inline(always)]
    pub fn csressel(&mut self) -> CSRESSEL_W {
        CSRESSEL_W { w: self }
    }
}

#[doc = "Reader of register TRACECTRL"]
pub type R = crate::R<u32, super::TRACECTRL>;
#[doc = "Writer for register TRACECTRL"]
pub type W = crate::W<u32, super::TRACECTRL>;
#[doc = "Register TRACECTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::TRACECTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TSTARTEN`"]
pub type TSTARTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSTARTEN`"]
pub struct TSTARTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TSTARTEN_W<'a> {
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
#[doc = "MTB TSTART PRS select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSTART_A {
    #[doc = "0: PRS ch 0 is controlling TSTART."]
    PRSCH0,
    #[doc = "1: PRS ch 1 is controlling TSTART."]
    PRSCH1,
    #[doc = "2: PRS ch 2 is controlling TSTART."]
    PRSCH2,
    #[doc = "3: PRS ch 3 is controlling TSTART."]
    PRSCH3,
    #[doc = "4: PRS ch 4 is controlling TSTART."]
    PRSCH4,
    #[doc = "5: PRS ch 5 is controlling TSTART."]
    PRSCH5,
}
impl From<TSTART_A> for u8 {
    #[inline(always)]
    fn from(variant: TSTART_A) -> Self {
        match variant {
            TSTART_A::PRSCH0 => 0,
            TSTART_A::PRSCH1 => 1,
            TSTART_A::PRSCH2 => 2,
            TSTART_A::PRSCH3 => 3,
            TSTART_A::PRSCH4 => 4,
            TSTART_A::PRSCH5 => 5,
        }
    }
}
#[doc = "Reader of field `TSTART`"]
pub type TSTART_R = crate::R<u8, TSTART_A>;
impl TSTART_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TSTART_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TSTART_A::PRSCH0),
            1 => Val(TSTART_A::PRSCH1),
            2 => Val(TSTART_A::PRSCH2),
            3 => Val(TSTART_A::PRSCH3),
            4 => Val(TSTART_A::PRSCH4),
            5 => Val(TSTART_A::PRSCH5),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == TSTART_A::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == TSTART_A::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == TSTART_A::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == TSTART_A::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == TSTART_A::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == TSTART_A::PRSCH5
    }
}
#[doc = "Write proxy for field `TSTART`"]
pub struct TSTART_W<'a> {
    w: &'a mut W,
}
impl<'a> TSTART_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSTART_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PRS ch 0 is controlling TSTART."]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(TSTART_A::PRSCH0)
    }
    #[doc = "PRS ch 1 is controlling TSTART."]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(TSTART_A::PRSCH1)
    }
    #[doc = "PRS ch 2 is controlling TSTART."]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(TSTART_A::PRSCH2)
    }
    #[doc = "PRS ch 3 is controlling TSTART."]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(TSTART_A::PRSCH3)
    }
    #[doc = "PRS ch 4 is controlling TSTART."]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(TSTART_A::PRSCH4)
    }
    #[doc = "PRS ch 5 is controlling TSTART."]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(TSTART_A::PRSCH5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | (((value as u32) & 0x07) << 1);
        self.w
    }
}
#[doc = "Reader of field `TSTOPEN`"]
pub type TSTOPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSTOPEN`"]
pub struct TSTOPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TSTOPEN_W<'a> {
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
#[doc = "MTB TSTOP PRS select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSTOP_A {
    #[doc = "0: PRS ch 0 is controlling TSTOP."]
    PRSCH0,
    #[doc = "1: PRS ch 1 is controlling TSTOP."]
    PRSCH1,
    #[doc = "2: PRS ch 2 is controlling TSTOP."]
    PRSCH2,
    #[doc = "3: PRS ch 3 is controlling TSTOP."]
    PRSCH3,
    #[doc = "4: PRS ch 4 is controlling TSTOP."]
    PRSCH4,
    #[doc = "5: PRS ch 5 is controlling TSTOP."]
    PRSCH5,
}
impl From<TSTOP_A> for u8 {
    #[inline(always)]
    fn from(variant: TSTOP_A) -> Self {
        match variant {
            TSTOP_A::PRSCH0 => 0,
            TSTOP_A::PRSCH1 => 1,
            TSTOP_A::PRSCH2 => 2,
            TSTOP_A::PRSCH3 => 3,
            TSTOP_A::PRSCH4 => 4,
            TSTOP_A::PRSCH5 => 5,
        }
    }
}
#[doc = "Reader of field `TSTOP`"]
pub type TSTOP_R = crate::R<u8, TSTOP_A>;
impl TSTOP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TSTOP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TSTOP_A::PRSCH0),
            1 => Val(TSTOP_A::PRSCH1),
            2 => Val(TSTOP_A::PRSCH2),
            3 => Val(TSTOP_A::PRSCH3),
            4 => Val(TSTOP_A::PRSCH4),
            5 => Val(TSTOP_A::PRSCH5),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == TSTOP_A::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == TSTOP_A::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == TSTOP_A::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == TSTOP_A::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == TSTOP_A::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == TSTOP_A::PRSCH5
    }
}
#[doc = "Write proxy for field `TSTOP`"]
pub struct TSTOP_W<'a> {
    w: &'a mut W,
}
impl<'a> TSTOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSTOP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PRS ch 0 is controlling TSTOP."]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(TSTOP_A::PRSCH0)
    }
    #[doc = "PRS ch 1 is controlling TSTOP."]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(TSTOP_A::PRSCH1)
    }
    #[doc = "PRS ch 2 is controlling TSTOP."]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(TSTOP_A::PRSCH2)
    }
    #[doc = "PRS ch 3 is controlling TSTOP."]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(TSTOP_A::PRSCH3)
    }
    #[doc = "PRS ch 4 is controlling TSTOP."]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(TSTOP_A::PRSCH4)
    }
    #[doc = "PRS ch 5 is controlling TSTOP."]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(TSTOP_A::PRSCH5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 9)) | (((value as u32) & 0x07) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - PRS TSTART Enable"]
    #[inline(always)]
    pub fn tstarten(&self) -> TSTARTEN_R {
        TSTARTEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - MTB TSTART PRS select"]
    #[inline(always)]
    pub fn tstart(&self) -> TSTART_R {
        TSTART_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    #[doc = "Bit 8 - PRS TSTOP Enable"]
    #[inline(always)]
    pub fn tstopen(&self) -> TSTOPEN_R {
        TSTOPEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 9:11 - MTB TSTOP PRS select"]
    #[inline(always)]
    pub fn tstop(&self) -> TSTOP_R {
        TSTOP_R::new(((self.bits >> 9) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - PRS TSTART Enable"]
    #[inline(always)]
    pub fn tstarten(&mut self) -> TSTARTEN_W {
        TSTARTEN_W { w: self }
    }
    #[doc = "Bits 1:3 - MTB TSTART PRS select"]
    #[inline(always)]
    pub fn tstart(&mut self) -> TSTART_W {
        TSTART_W { w: self }
    }
    #[doc = "Bit 8 - PRS TSTOP Enable"]
    #[inline(always)]
    pub fn tstopen(&mut self) -> TSTOPEN_W {
        TSTOPEN_W { w: self }
    }
    #[doc = "Bits 9:11 - MTB TSTOP PRS select"]
    #[inline(always)]
    pub fn tstop(&mut self) -> TSTOP_W {
        TSTOP_W { w: self }
    }
}

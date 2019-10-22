#[doc = "Reader of register INPUT"]
pub type R = crate::R<u32, super::INPUT>;
#[doc = "Writer for register INPUT"]
pub type W = crate::W<u32, super::INPUT>;
#[doc = "Register INPUT `reset()`'s with value 0"]
impl crate::ResetValue for super::INPUT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "S0IN PRS Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S0PRSSEL_A {
    #[doc = "0: PRS Channel 0 selected."]
    PRSCH0,
    #[doc = "1: PRS Channel 1 selected."]
    PRSCH1,
    #[doc = "2: PRS Channel 2 selected."]
    PRSCH2,
    #[doc = "3: PRS Channel 3 selected."]
    PRSCH3,
    #[doc = "4: PRS Channel 4 selected."]
    PRSCH4,
    #[doc = "5: PRS Channel 5 selected."]
    PRSCH5,
}
impl From<S0PRSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: S0PRSSEL_A) -> Self {
        match variant {
            S0PRSSEL_A::PRSCH0 => 0,
            S0PRSSEL_A::PRSCH1 => 1,
            S0PRSSEL_A::PRSCH2 => 2,
            S0PRSSEL_A::PRSCH3 => 3,
            S0PRSSEL_A::PRSCH4 => 4,
            S0PRSSEL_A::PRSCH5 => 5,
        }
    }
}
#[doc = "Reader of field `S0PRSSEL`"]
pub type S0PRSSEL_R = crate::R<u8, S0PRSSEL_A>;
impl S0PRSSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, S0PRSSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(S0PRSSEL_A::PRSCH0),
            1 => Val(S0PRSSEL_A::PRSCH1),
            2 => Val(S0PRSSEL_A::PRSCH2),
            3 => Val(S0PRSSEL_A::PRSCH3),
            4 => Val(S0PRSSEL_A::PRSCH4),
            5 => Val(S0PRSSEL_A::PRSCH5),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == S0PRSSEL_A::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == S0PRSSEL_A::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == S0PRSSEL_A::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == S0PRSSEL_A::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == S0PRSSEL_A::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == S0PRSSEL_A::PRSCH5
    }
}
#[doc = "Write proxy for field `S0PRSSEL`"]
pub struct S0PRSSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> S0PRSSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: S0PRSSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PRS Channel 0 selected."]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(S0PRSSEL_A::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected."]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(S0PRSSEL_A::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected."]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(S0PRSSEL_A::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected."]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(S0PRSSEL_A::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected."]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(S0PRSSEL_A::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected."]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(S0PRSSEL_A::PRSCH5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `S0PRSEN`"]
pub type S0PRSEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S0PRSEN`"]
pub struct S0PRSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> S0PRSEN_W<'a> {
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
#[doc = "S1IN PRS Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S1PRSSEL_A {
    #[doc = "0: PRS Channel 0 selected."]
    PRSCH0,
    #[doc = "1: PRS Channel 1 selected."]
    PRSCH1,
    #[doc = "2: PRS Channel 2 selected."]
    PRSCH2,
    #[doc = "3: PRS Channel 3 selected."]
    PRSCH3,
    #[doc = "4: PRS Channel 4 selected."]
    PRSCH4,
    #[doc = "5: PRS Channel 5 selected."]
    PRSCH5,
}
impl From<S1PRSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: S1PRSSEL_A) -> Self {
        match variant {
            S1PRSSEL_A::PRSCH0 => 0,
            S1PRSSEL_A::PRSCH1 => 1,
            S1PRSSEL_A::PRSCH2 => 2,
            S1PRSSEL_A::PRSCH3 => 3,
            S1PRSSEL_A::PRSCH4 => 4,
            S1PRSSEL_A::PRSCH5 => 5,
        }
    }
}
#[doc = "Reader of field `S1PRSSEL`"]
pub type S1PRSSEL_R = crate::R<u8, S1PRSSEL_A>;
impl S1PRSSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, S1PRSSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(S1PRSSEL_A::PRSCH0),
            1 => Val(S1PRSSEL_A::PRSCH1),
            2 => Val(S1PRSSEL_A::PRSCH2),
            3 => Val(S1PRSSEL_A::PRSCH3),
            4 => Val(S1PRSSEL_A::PRSCH4),
            5 => Val(S1PRSSEL_A::PRSCH5),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == S1PRSSEL_A::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == S1PRSSEL_A::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == S1PRSSEL_A::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == S1PRSSEL_A::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == S1PRSSEL_A::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == S1PRSSEL_A::PRSCH5
    }
}
#[doc = "Write proxy for field `S1PRSSEL`"]
pub struct S1PRSSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> S1PRSSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: S1PRSSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PRS Channel 0 selected."]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(S1PRSSEL_A::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected."]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(S1PRSSEL_A::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected."]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(S1PRSSEL_A::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected."]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(S1PRSSEL_A::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected."]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(S1PRSSEL_A::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected."]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(S1PRSSEL_A::PRSCH5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 6)) | (((value as u32) & 0x07) << 6);
        self.w
    }
}
#[doc = "Reader of field `S1PRSEN`"]
pub type S1PRSEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S1PRSEN`"]
pub struct S1PRSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> S1PRSEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - S0IN PRS Channel Select"]
    #[inline(always)]
    pub fn s0prssel(&self) -> S0PRSSEL_R {
        S0PRSSEL_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 4 - S0IN PRS Enable"]
    #[inline(always)]
    pub fn s0prsen(&self) -> S0PRSEN_R {
        S0PRSEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 6:8 - S1IN PRS Channel Select"]
    #[inline(always)]
    pub fn s1prssel(&self) -> S1PRSSEL_R {
        S1PRSSEL_R::new(((self.bits >> 6) & 0x07) as u8)
    }
    #[doc = "Bit 10 - S1IN PRS Enable"]
    #[inline(always)]
    pub fn s1prsen(&self) -> S1PRSEN_R {
        S1PRSEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - S0IN PRS Channel Select"]
    #[inline(always)]
    pub fn s0prssel(&mut self) -> S0PRSSEL_W {
        S0PRSSEL_W { w: self }
    }
    #[doc = "Bit 4 - S0IN PRS Enable"]
    #[inline(always)]
    pub fn s0prsen(&mut self) -> S0PRSEN_W {
        S0PRSEN_W { w: self }
    }
    #[doc = "Bits 6:8 - S1IN PRS Channel Select"]
    #[inline(always)]
    pub fn s1prssel(&mut self) -> S1PRSSEL_W {
        S1PRSSEL_W { w: self }
    }
    #[doc = "Bit 10 - S1IN PRS Enable"]
    #[inline(always)]
    pub fn s1prsen(&mut self) -> S1PRSEN_W {
        S1PRSEN_W { w: self }
    }
}

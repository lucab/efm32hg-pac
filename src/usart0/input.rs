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
#[doc = "RX PRS Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXPRSSEL_A {
    #[doc = "0: PRS Channel 0 selected"]
    PRSCH0,
    #[doc = "1: PRS Channel 1 selected"]
    PRSCH1,
    #[doc = "2: PRS Channel 2 selected"]
    PRSCH2,
    #[doc = "3: PRS Channel 3 selected"]
    PRSCH3,
    #[doc = "4: PRS Channel 4 selected"]
    PRSCH4,
    #[doc = "5: PRS Channel 5 selected"]
    PRSCH5,
}
impl From<RXPRSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: RXPRSSEL_A) -> Self {
        match variant {
            RXPRSSEL_A::PRSCH0 => 0,
            RXPRSSEL_A::PRSCH1 => 1,
            RXPRSSEL_A::PRSCH2 => 2,
            RXPRSSEL_A::PRSCH3 => 3,
            RXPRSSEL_A::PRSCH4 => 4,
            RXPRSSEL_A::PRSCH5 => 5,
        }
    }
}
#[doc = "Reader of field `RXPRSSEL`"]
pub type RXPRSSEL_R = crate::R<u8, RXPRSSEL_A>;
impl RXPRSSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RXPRSSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RXPRSSEL_A::PRSCH0),
            1 => Val(RXPRSSEL_A::PRSCH1),
            2 => Val(RXPRSSEL_A::PRSCH2),
            3 => Val(RXPRSSEL_A::PRSCH3),
            4 => Val(RXPRSSEL_A::PRSCH4),
            5 => Val(RXPRSSEL_A::PRSCH5),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == RXPRSSEL_A::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == RXPRSSEL_A::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == RXPRSSEL_A::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == RXPRSSEL_A::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == RXPRSSEL_A::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == RXPRSSEL_A::PRSCH5
    }
}
#[doc = "Write proxy for field `RXPRSSEL`"]
pub struct RXPRSSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RXPRSSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXPRSSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PRS Channel 0 selected"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(RXPRSSEL_A::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(RXPRSSEL_A::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(RXPRSSEL_A::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(RXPRSSEL_A::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(RXPRSSEL_A::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(RXPRSSEL_A::PRSCH5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `RXPRS`"]
pub type RXPRS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXPRS`"]
pub struct RXPRS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXPRS_W<'a> {
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
    #[doc = "Bits 0:2 - RX PRS Channel Select"]
    #[inline(always)]
    pub fn rxprssel(&self) -> RXPRSSEL_R {
        RXPRSSEL_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 4 - PRS RX Enable"]
    #[inline(always)]
    pub fn rxprs(&self) -> RXPRS_R {
        RXPRS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - RX PRS Channel Select"]
    #[inline(always)]
    pub fn rxprssel(&mut self) -> RXPRSSEL_W {
        RXPRSSEL_W { w: self }
    }
    #[doc = "Bit 4 - PRS RX Enable"]
    #[inline(always)]
    pub fn rxprs(&mut self) -> RXPRS_W {
        RXPRS_W { w: self }
    }
}

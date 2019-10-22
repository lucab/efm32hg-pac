#[doc = "Reader of register TRIGCTRL"]
pub type R = crate::R<u32, super::TRIGCTRL>;
#[doc = "Writer for register TRIGCTRL"]
pub type W = crate::W<u32, super::TRIGCTRL>;
#[doc = "Register TRIGCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::TRIGCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Trigger PRS Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSEL_A {
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
impl From<TSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TSEL_A) -> Self {
        match variant {
            TSEL_A::PRSCH0 => 0,
            TSEL_A::PRSCH1 => 1,
            TSEL_A::PRSCH2 => 2,
            TSEL_A::PRSCH3 => 3,
            TSEL_A::PRSCH4 => 4,
            TSEL_A::PRSCH5 => 5,
        }
    }
}
#[doc = "Reader of field `TSEL`"]
pub type TSEL_R = crate::R<u8, TSEL_A>;
impl TSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TSEL_A::PRSCH0),
            1 => Val(TSEL_A::PRSCH1),
            2 => Val(TSEL_A::PRSCH2),
            3 => Val(TSEL_A::PRSCH3),
            4 => Val(TSEL_A::PRSCH4),
            5 => Val(TSEL_A::PRSCH5),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == TSEL_A::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == TSEL_A::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == TSEL_A::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == TSEL_A::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == TSEL_A::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == TSEL_A::PRSCH5
    }
}
#[doc = "Write proxy for field `TSEL`"]
pub struct TSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PRS Channel 0 selected"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(TSEL_A::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(TSEL_A::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(TSEL_A::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(TSEL_A::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(TSEL_A::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(TSEL_A::PRSCH5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `RXTEN`"]
pub type RXTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXTEN`"]
pub struct RXTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXTEN_W<'a> {
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
#[doc = "Reader of field `TXTEN`"]
pub type TXTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXTEN`"]
pub struct TXTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXTEN_W<'a> {
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
#[doc = "Reader of field `AUTOTXTEN`"]
pub type AUTOTXTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUTOTXTEN`"]
pub struct AUTOTXTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOTXTEN_W<'a> {
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
impl R {
    #[doc = "Bits 0:2 - Trigger PRS Channel Select"]
    #[inline(always)]
    pub fn tsel(&self) -> TSEL_R {
        TSEL_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 4 - Receive Trigger Enable"]
    #[inline(always)]
    pub fn rxten(&self) -> RXTEN_R {
        RXTEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Transmit Trigger Enable"]
    #[inline(always)]
    pub fn txten(&self) -> TXTEN_R {
        TXTEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - AUTOTX Trigger Enable"]
    #[inline(always)]
    pub fn autotxten(&self) -> AUTOTXTEN_R {
        AUTOTXTEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Trigger PRS Channel Select"]
    #[inline(always)]
    pub fn tsel(&mut self) -> TSEL_W {
        TSEL_W { w: self }
    }
    #[doc = "Bit 4 - Receive Trigger Enable"]
    #[inline(always)]
    pub fn rxten(&mut self) -> RXTEN_W {
        RXTEN_W { w: self }
    }
    #[doc = "Bit 5 - Transmit Trigger Enable"]
    #[inline(always)]
    pub fn txten(&mut self) -> TXTEN_W {
        TXTEN_W { w: self }
    }
    #[doc = "Bit 6 - AUTOTX Trigger Enable"]
    #[inline(always)]
    pub fn autotxten(&mut self) -> AUTOTXTEN_W {
        AUTOTXTEN_W { w: self }
    }
}

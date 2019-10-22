#[doc = "Reader of register ROUTE"]
pub type R = crate::R<u32, super::ROUTE>;
#[doc = "Writer for register ROUTE"]
pub type W = crate::W<u32, super::ROUTE>;
#[doc = "Register ROUTE `reset()`'s with value 0"]
impl crate::ResetValue for super::ROUTE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RXPEN`"]
pub type RXPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXPEN`"]
pub struct RXPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXPEN_W<'a> {
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
#[doc = "Reader of field `TXPEN`"]
pub type TXPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXPEN`"]
pub struct TXPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXPEN_W<'a> {
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
#[doc = "Reader of field `CSPEN`"]
pub type CSPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CSPEN`"]
pub struct CSPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CSPEN_W<'a> {
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
#[doc = "Reader of field `CLKPEN`"]
pub type CLKPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKPEN`"]
pub struct CLKPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPEN_W<'a> {
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
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCATION_A {
    #[doc = "0: Location 0"]
    LOC0,
    #[doc = "1: Location 1"]
    LOC1,
    #[doc = "2: Location 2"]
    LOC2,
    #[doc = "3: Location 3"]
    LOC3,
    #[doc = "4: Location 4"]
    LOC4,
    #[doc = "5: Location 5"]
    LOC5,
    #[doc = "6: Location 6"]
    LOC6,
}
impl From<LOCATION_A> for u8 {
    #[inline(always)]
    fn from(variant: LOCATION_A) -> Self {
        match variant {
            LOCATION_A::LOC0 => 0,
            LOCATION_A::LOC1 => 1,
            LOCATION_A::LOC2 => 2,
            LOCATION_A::LOC3 => 3,
            LOCATION_A::LOC4 => 4,
            LOCATION_A::LOC5 => 5,
            LOCATION_A::LOC6 => 6,
        }
    }
}
#[doc = "Reader of field `LOCATION`"]
pub type LOCATION_R = crate::R<u8, LOCATION_A>;
impl LOCATION_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LOCATION_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(LOCATION_A::LOC0),
            1 => Val(LOCATION_A::LOC1),
            2 => Val(LOCATION_A::LOC2),
            3 => Val(LOCATION_A::LOC3),
            4 => Val(LOCATION_A::LOC4),
            5 => Val(LOCATION_A::LOC5),
            6 => Val(LOCATION_A::LOC6),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == LOCATION_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == LOCATION_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == LOCATION_A::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == LOCATION_A::LOC3
    }
    #[doc = "Checks if the value of the field is `LOC4`"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == LOCATION_A::LOC4
    }
    #[doc = "Checks if the value of the field is `LOC5`"]
    #[inline(always)]
    pub fn is_loc5(&self) -> bool {
        *self == LOCATION_A::LOC5
    }
    #[doc = "Checks if the value of the field is `LOC6`"]
    #[inline(always)]
    pub fn is_loc6(&self) -> bool {
        *self == LOCATION_A::LOC6
    }
}
#[doc = "Write proxy for field `LOCATION`"]
pub struct LOCATION_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCATION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCATION_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(LOCATION_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(LOCATION_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(LOCATION_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut W {
        self.variant(LOCATION_A::LOC3)
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut W {
        self.variant(LOCATION_A::LOC4)
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn loc5(self) -> &'a mut W {
        self.variant(LOCATION_A::LOC5)
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn loc6(self) -> &'a mut W {
        self.variant(LOCATION_A::LOC6)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - RX Pin Enable"]
    #[inline(always)]
    pub fn rxpen(&self) -> RXPEN_R {
        RXPEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TX Pin Enable"]
    #[inline(always)]
    pub fn txpen(&self) -> TXPEN_R {
        TXPEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CS Pin Enable"]
    #[inline(always)]
    pub fn cspen(&self) -> CSPEN_R {
        CSPEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - CLK Pin Enable"]
    #[inline(always)]
    pub fn clkpen(&self) -> CLKPEN_R {
        CLKPEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - I/O Location"]
    #[inline(always)]
    pub fn location(&self) -> LOCATION_R {
        LOCATION_R::new(((self.bits >> 8) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - RX Pin Enable"]
    #[inline(always)]
    pub fn rxpen(&mut self) -> RXPEN_W {
        RXPEN_W { w: self }
    }
    #[doc = "Bit 1 - TX Pin Enable"]
    #[inline(always)]
    pub fn txpen(&mut self) -> TXPEN_W {
        TXPEN_W { w: self }
    }
    #[doc = "Bit 2 - CS Pin Enable"]
    #[inline(always)]
    pub fn cspen(&mut self) -> CSPEN_W {
        CSPEN_W { w: self }
    }
    #[doc = "Bit 3 - CLK Pin Enable"]
    #[inline(always)]
    pub fn clkpen(&mut self) -> CLKPEN_W {
        CLKPEN_W { w: self }
    }
    #[doc = "Bits 8:10 - I/O Location"]
    #[inline(always)]
    pub fn location(&mut self) -> LOCATION_W {
        LOCATION_W { w: self }
    }
}

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
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:10 - I/O Location"]
    #[inline(always)]
    pub fn location(&self) -> LOCATION_R {
        LOCATION_R::new(((self.bits >> 8) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 8:10 - I/O Location"]
    #[inline(always)]
    pub fn location(&mut self) -> LOCATION_W {
        LOCATION_W { w: self }
    }
}

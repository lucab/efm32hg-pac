#[doc = "Reader of register PB_CTRL"]
pub type R = crate::R<u32, super::PB_CTRL>;
#[doc = "Writer for register PB_CTRL"]
pub type W = crate::W<u32, super::PB_CTRL>;
#[doc = "Register PB_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::PB_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Drive Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DRIVEMODE_A {
    #[doc = "0: 6 mA drive current"]
    STANDARD,
    #[doc = "1: 0.1 mA drive current"]
    LOWEST,
    #[doc = "2: 20 mA drive current"]
    HIGH,
    #[doc = "3: 1 mA drive current"]
    LOW,
}
impl From<DRIVEMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: DRIVEMODE_A) -> Self {
        match variant {
            DRIVEMODE_A::STANDARD => 0,
            DRIVEMODE_A::LOWEST => 1,
            DRIVEMODE_A::HIGH => 2,
            DRIVEMODE_A::LOW => 3,
        }
    }
}
#[doc = "Reader of field `DRIVEMODE`"]
pub type DRIVEMODE_R = crate::R<u8, DRIVEMODE_A>;
impl DRIVEMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRIVEMODE_A {
        match self.bits {
            0 => DRIVEMODE_A::STANDARD,
            1 => DRIVEMODE_A::LOWEST,
            2 => DRIVEMODE_A::HIGH,
            3 => DRIVEMODE_A::LOW,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `STANDARD`"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == DRIVEMODE_A::STANDARD
    }
    #[doc = "Checks if the value of the field is `LOWEST`"]
    #[inline(always)]
    pub fn is_lowest(&self) -> bool {
        *self == DRIVEMODE_A::LOWEST
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == DRIVEMODE_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == DRIVEMODE_A::LOW
    }
}
#[doc = "Write proxy for field `DRIVEMODE`"]
pub struct DRIVEMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DRIVEMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DRIVEMODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "6 mA drive current"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(DRIVEMODE_A::STANDARD)
    }
    #[doc = "0.1 mA drive current"]
    #[inline(always)]
    pub fn lowest(self) -> &'a mut W {
        self.variant(DRIVEMODE_A::LOWEST)
    }
    #[doc = "20 mA drive current"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(DRIVEMODE_A::HIGH)
    }
    #[doc = "1 mA drive current"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(DRIVEMODE_A::LOW)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Drive Mode Select"]
    #[inline(always)]
    pub fn drivemode(&self) -> DRIVEMODE_R {
        DRIVEMODE_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Drive Mode Select"]
    #[inline(always)]
    pub fn drivemode(&mut self) -> DRIVEMODE_W {
        DRIVEMODE_W { w: self }
    }
}

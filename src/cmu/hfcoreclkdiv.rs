#[doc = "Reader of register HFCORECLKDIV"]
pub type R = crate::R<u32, super::HFCORECLKDIV>;
#[doc = "Writer for register HFCORECLKDIV"]
pub type W = crate::W<u32, super::HFCORECLKDIV>;
#[doc = "Register HFCORECLKDIV `reset()`'s with value 0"]
impl crate::ResetValue for super::HFCORECLKDIV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "HFCORECLK Divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HFCORECLKDIV_A {
    #[doc = "0: HFCORECLK = HFCLK."]
    HFCLK,
    #[doc = "1: HFCORECLK = HFCLK/2."]
    HFCLK2,
    #[doc = "2: HFCORECLK = HFCLK/4."]
    HFCLK4,
    #[doc = "3: HFCORECLK = HFCLK/8."]
    HFCLK8,
    #[doc = "4: HFCORECLK = HFCLK/16."]
    HFCLK16,
    #[doc = "5: HFCORECLK = HFCLK/32."]
    HFCLK32,
    #[doc = "6: HFCORECLK = HFCLK/64."]
    HFCLK64,
    #[doc = "7: HFCORECLK = HFCLK/128."]
    HFCLK128,
    #[doc = "8: HFCORECLK = HFCLK/256."]
    HFCLK256,
    #[doc = "9: HFCORECLK = HFCLK/512."]
    HFCLK512,
}
impl From<HFCORECLKDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: HFCORECLKDIV_A) -> Self {
        match variant {
            HFCORECLKDIV_A::HFCLK => 0,
            HFCORECLKDIV_A::HFCLK2 => 1,
            HFCORECLKDIV_A::HFCLK4 => 2,
            HFCORECLKDIV_A::HFCLK8 => 3,
            HFCORECLKDIV_A::HFCLK16 => 4,
            HFCORECLKDIV_A::HFCLK32 => 5,
            HFCORECLKDIV_A::HFCLK64 => 6,
            HFCORECLKDIV_A::HFCLK128 => 7,
            HFCORECLKDIV_A::HFCLK256 => 8,
            HFCORECLKDIV_A::HFCLK512 => 9,
        }
    }
}
#[doc = "Reader of field `HFCORECLKDIV`"]
pub type HFCORECLKDIV_R = crate::R<u8, HFCORECLKDIV_A>;
impl HFCORECLKDIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, HFCORECLKDIV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(HFCORECLKDIV_A::HFCLK),
            1 => Val(HFCORECLKDIV_A::HFCLK2),
            2 => Val(HFCORECLKDIV_A::HFCLK4),
            3 => Val(HFCORECLKDIV_A::HFCLK8),
            4 => Val(HFCORECLKDIV_A::HFCLK16),
            5 => Val(HFCORECLKDIV_A::HFCLK32),
            6 => Val(HFCORECLKDIV_A::HFCLK64),
            7 => Val(HFCORECLKDIV_A::HFCLK128),
            8 => Val(HFCORECLKDIV_A::HFCLK256),
            9 => Val(HFCORECLKDIV_A::HFCLK512),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `HFCLK`"]
    #[inline(always)]
    pub fn is_hfclk(&self) -> bool {
        *self == HFCORECLKDIV_A::HFCLK
    }
    #[doc = "Checks if the value of the field is `HFCLK2`"]
    #[inline(always)]
    pub fn is_hfclk2(&self) -> bool {
        *self == HFCORECLKDIV_A::HFCLK2
    }
    #[doc = "Checks if the value of the field is `HFCLK4`"]
    #[inline(always)]
    pub fn is_hfclk4(&self) -> bool {
        *self == HFCORECLKDIV_A::HFCLK4
    }
    #[doc = "Checks if the value of the field is `HFCLK8`"]
    #[inline(always)]
    pub fn is_hfclk8(&self) -> bool {
        *self == HFCORECLKDIV_A::HFCLK8
    }
    #[doc = "Checks if the value of the field is `HFCLK16`"]
    #[inline(always)]
    pub fn is_hfclk16(&self) -> bool {
        *self == HFCORECLKDIV_A::HFCLK16
    }
    #[doc = "Checks if the value of the field is `HFCLK32`"]
    #[inline(always)]
    pub fn is_hfclk32(&self) -> bool {
        *self == HFCORECLKDIV_A::HFCLK32
    }
    #[doc = "Checks if the value of the field is `HFCLK64`"]
    #[inline(always)]
    pub fn is_hfclk64(&self) -> bool {
        *self == HFCORECLKDIV_A::HFCLK64
    }
    #[doc = "Checks if the value of the field is `HFCLK128`"]
    #[inline(always)]
    pub fn is_hfclk128(&self) -> bool {
        *self == HFCORECLKDIV_A::HFCLK128
    }
    #[doc = "Checks if the value of the field is `HFCLK256`"]
    #[inline(always)]
    pub fn is_hfclk256(&self) -> bool {
        *self == HFCORECLKDIV_A::HFCLK256
    }
    #[doc = "Checks if the value of the field is `HFCLK512`"]
    #[inline(always)]
    pub fn is_hfclk512(&self) -> bool {
        *self == HFCORECLKDIV_A::HFCLK512
    }
}
#[doc = "Write proxy for field `HFCORECLKDIV`"]
pub struct HFCORECLKDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> HFCORECLKDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HFCORECLKDIV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "HFCORECLK = HFCLK."]
    #[inline(always)]
    pub fn hfclk(self) -> &'a mut W {
        self.variant(HFCORECLKDIV_A::HFCLK)
    }
    #[doc = "HFCORECLK = HFCLK/2."]
    #[inline(always)]
    pub fn hfclk2(self) -> &'a mut W {
        self.variant(HFCORECLKDIV_A::HFCLK2)
    }
    #[doc = "HFCORECLK = HFCLK/4."]
    #[inline(always)]
    pub fn hfclk4(self) -> &'a mut W {
        self.variant(HFCORECLKDIV_A::HFCLK4)
    }
    #[doc = "HFCORECLK = HFCLK/8."]
    #[inline(always)]
    pub fn hfclk8(self) -> &'a mut W {
        self.variant(HFCORECLKDIV_A::HFCLK8)
    }
    #[doc = "HFCORECLK = HFCLK/16."]
    #[inline(always)]
    pub fn hfclk16(self) -> &'a mut W {
        self.variant(HFCORECLKDIV_A::HFCLK16)
    }
    #[doc = "HFCORECLK = HFCLK/32."]
    #[inline(always)]
    pub fn hfclk32(self) -> &'a mut W {
        self.variant(HFCORECLKDIV_A::HFCLK32)
    }
    #[doc = "HFCORECLK = HFCLK/64."]
    #[inline(always)]
    pub fn hfclk64(self) -> &'a mut W {
        self.variant(HFCORECLKDIV_A::HFCLK64)
    }
    #[doc = "HFCORECLK = HFCLK/128."]
    #[inline(always)]
    pub fn hfclk128(self) -> &'a mut W {
        self.variant(HFCORECLKDIV_A::HFCLK128)
    }
    #[doc = "HFCORECLK = HFCLK/256."]
    #[inline(always)]
    pub fn hfclk256(self) -> &'a mut W {
        self.variant(HFCORECLKDIV_A::HFCLK256)
    }
    #[doc = "HFCORECLK = HFCLK/512."]
    #[inline(always)]
    pub fn hfclk512(self) -> &'a mut W {
        self.variant(HFCORECLKDIV_A::HFCLK512)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `HFCORECLKLEDIV`"]
pub type HFCORECLKLEDIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HFCORECLKLEDIV`"]
pub struct HFCORECLKLEDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> HFCORECLKLEDIV_W<'a> {
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
impl R {
    #[doc = "Bits 0:3 - HFCORECLK Divider"]
    #[inline(always)]
    pub fn hfcoreclkdiv(&self) -> HFCORECLKDIV_R {
        HFCORECLKDIV_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Additional Division Factor For HFCORECLKLE"]
    #[inline(always)]
    pub fn hfcoreclklediv(&self) -> HFCORECLKLEDIV_R {
        HFCORECLKLEDIV_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - HFCORECLK Divider"]
    #[inline(always)]
    pub fn hfcoreclkdiv(&mut self) -> HFCORECLKDIV_W {
        HFCORECLKDIV_W { w: self }
    }
    #[doc = "Bit 8 - Additional Division Factor For HFCORECLKLE"]
    #[inline(always)]
    pub fn hfcoreclklediv(&mut self) -> HFCORECLKLEDIV_W {
        HFCORECLKLEDIV_W { w: self }
    }
}

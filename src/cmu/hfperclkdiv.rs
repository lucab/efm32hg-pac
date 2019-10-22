#[doc = "Reader of register HFPERCLKDIV"]
pub type R = crate::R<u32, super::HFPERCLKDIV>;
#[doc = "Writer for register HFPERCLKDIV"]
pub type W = crate::W<u32, super::HFPERCLKDIV>;
#[doc = "Register HFPERCLKDIV `reset()`'s with value 0x0100"]
impl crate::ResetValue for super::HFPERCLKDIV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0100
    }
}
#[doc = "HFPERCLK Divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HFPERCLKDIV_A {
    #[doc = "0: HFPERCLK = HFCLK."]
    HFCLK,
    #[doc = "1: HFPERCLK = HFCLK/2."]
    HFCLK2,
    #[doc = "2: HFPERCLK = HFCLK/4."]
    HFCLK4,
    #[doc = "3: HFPERCLK = HFCLK/8."]
    HFCLK8,
    #[doc = "4: HFPERCLK = HFCLK/16."]
    HFCLK16,
    #[doc = "5: HFPERCLK = HFCLK/32."]
    HFCLK32,
    #[doc = "6: HFPERCLK = HFCLK/64."]
    HFCLK64,
    #[doc = "7: HFPERCLK = HFCLK/128."]
    HFCLK128,
    #[doc = "8: HFPERCLK = HFCLK/256."]
    HFCLK256,
    #[doc = "9: HFPERCLK = HFCLK/512."]
    HFCLK512,
}
impl From<HFPERCLKDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: HFPERCLKDIV_A) -> Self {
        match variant {
            HFPERCLKDIV_A::HFCLK => 0,
            HFPERCLKDIV_A::HFCLK2 => 1,
            HFPERCLKDIV_A::HFCLK4 => 2,
            HFPERCLKDIV_A::HFCLK8 => 3,
            HFPERCLKDIV_A::HFCLK16 => 4,
            HFPERCLKDIV_A::HFCLK32 => 5,
            HFPERCLKDIV_A::HFCLK64 => 6,
            HFPERCLKDIV_A::HFCLK128 => 7,
            HFPERCLKDIV_A::HFCLK256 => 8,
            HFPERCLKDIV_A::HFCLK512 => 9,
        }
    }
}
#[doc = "Reader of field `HFPERCLKDIV`"]
pub type HFPERCLKDIV_R = crate::R<u8, HFPERCLKDIV_A>;
impl HFPERCLKDIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, HFPERCLKDIV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(HFPERCLKDIV_A::HFCLK),
            1 => Val(HFPERCLKDIV_A::HFCLK2),
            2 => Val(HFPERCLKDIV_A::HFCLK4),
            3 => Val(HFPERCLKDIV_A::HFCLK8),
            4 => Val(HFPERCLKDIV_A::HFCLK16),
            5 => Val(HFPERCLKDIV_A::HFCLK32),
            6 => Val(HFPERCLKDIV_A::HFCLK64),
            7 => Val(HFPERCLKDIV_A::HFCLK128),
            8 => Val(HFPERCLKDIV_A::HFCLK256),
            9 => Val(HFPERCLKDIV_A::HFCLK512),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `HFCLK`"]
    #[inline(always)]
    pub fn is_hfclk(&self) -> bool {
        *self == HFPERCLKDIV_A::HFCLK
    }
    #[doc = "Checks if the value of the field is `HFCLK2`"]
    #[inline(always)]
    pub fn is_hfclk2(&self) -> bool {
        *self == HFPERCLKDIV_A::HFCLK2
    }
    #[doc = "Checks if the value of the field is `HFCLK4`"]
    #[inline(always)]
    pub fn is_hfclk4(&self) -> bool {
        *self == HFPERCLKDIV_A::HFCLK4
    }
    #[doc = "Checks if the value of the field is `HFCLK8`"]
    #[inline(always)]
    pub fn is_hfclk8(&self) -> bool {
        *self == HFPERCLKDIV_A::HFCLK8
    }
    #[doc = "Checks if the value of the field is `HFCLK16`"]
    #[inline(always)]
    pub fn is_hfclk16(&self) -> bool {
        *self == HFPERCLKDIV_A::HFCLK16
    }
    #[doc = "Checks if the value of the field is `HFCLK32`"]
    #[inline(always)]
    pub fn is_hfclk32(&self) -> bool {
        *self == HFPERCLKDIV_A::HFCLK32
    }
    #[doc = "Checks if the value of the field is `HFCLK64`"]
    #[inline(always)]
    pub fn is_hfclk64(&self) -> bool {
        *self == HFPERCLKDIV_A::HFCLK64
    }
    #[doc = "Checks if the value of the field is `HFCLK128`"]
    #[inline(always)]
    pub fn is_hfclk128(&self) -> bool {
        *self == HFPERCLKDIV_A::HFCLK128
    }
    #[doc = "Checks if the value of the field is `HFCLK256`"]
    #[inline(always)]
    pub fn is_hfclk256(&self) -> bool {
        *self == HFPERCLKDIV_A::HFCLK256
    }
    #[doc = "Checks if the value of the field is `HFCLK512`"]
    #[inline(always)]
    pub fn is_hfclk512(&self) -> bool {
        *self == HFPERCLKDIV_A::HFCLK512
    }
}
#[doc = "Write proxy for field `HFPERCLKDIV`"]
pub struct HFPERCLKDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> HFPERCLKDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HFPERCLKDIV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "HFPERCLK = HFCLK."]
    #[inline(always)]
    pub fn hfclk(self) -> &'a mut W {
        self.variant(HFPERCLKDIV_A::HFCLK)
    }
    #[doc = "HFPERCLK = HFCLK/2."]
    #[inline(always)]
    pub fn hfclk2(self) -> &'a mut W {
        self.variant(HFPERCLKDIV_A::HFCLK2)
    }
    #[doc = "HFPERCLK = HFCLK/4."]
    #[inline(always)]
    pub fn hfclk4(self) -> &'a mut W {
        self.variant(HFPERCLKDIV_A::HFCLK4)
    }
    #[doc = "HFPERCLK = HFCLK/8."]
    #[inline(always)]
    pub fn hfclk8(self) -> &'a mut W {
        self.variant(HFPERCLKDIV_A::HFCLK8)
    }
    #[doc = "HFPERCLK = HFCLK/16."]
    #[inline(always)]
    pub fn hfclk16(self) -> &'a mut W {
        self.variant(HFPERCLKDIV_A::HFCLK16)
    }
    #[doc = "HFPERCLK = HFCLK/32."]
    #[inline(always)]
    pub fn hfclk32(self) -> &'a mut W {
        self.variant(HFPERCLKDIV_A::HFCLK32)
    }
    #[doc = "HFPERCLK = HFCLK/64."]
    #[inline(always)]
    pub fn hfclk64(self) -> &'a mut W {
        self.variant(HFPERCLKDIV_A::HFCLK64)
    }
    #[doc = "HFPERCLK = HFCLK/128."]
    #[inline(always)]
    pub fn hfclk128(self) -> &'a mut W {
        self.variant(HFPERCLKDIV_A::HFCLK128)
    }
    #[doc = "HFPERCLK = HFCLK/256."]
    #[inline(always)]
    pub fn hfclk256(self) -> &'a mut W {
        self.variant(HFPERCLKDIV_A::HFCLK256)
    }
    #[doc = "HFPERCLK = HFCLK/512."]
    #[inline(always)]
    pub fn hfclk512(self) -> &'a mut W {
        self.variant(HFPERCLKDIV_A::HFCLK512)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `HFPERCLKEN`"]
pub type HFPERCLKEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HFPERCLKEN`"]
pub struct HFPERCLKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HFPERCLKEN_W<'a> {
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
    #[doc = "Bits 0:3 - HFPERCLK Divider"]
    #[inline(always)]
    pub fn hfperclkdiv(&self) -> HFPERCLKDIV_R {
        HFPERCLKDIV_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - HFPERCLK Enable"]
    #[inline(always)]
    pub fn hfperclken(&self) -> HFPERCLKEN_R {
        HFPERCLKEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - HFPERCLK Divider"]
    #[inline(always)]
    pub fn hfperclkdiv(&mut self) -> HFPERCLKDIV_W {
        HFPERCLKDIV_W { w: self }
    }
    #[doc = "Bit 8 - HFPERCLK Enable"]
    #[inline(always)]
    pub fn hfperclken(&mut self) -> HFPERCLKEN_W {
        HFPERCLKEN_W { w: self }
    }
}

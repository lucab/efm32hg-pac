#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0x4700_0000"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x4700_0000
    }
}
#[doc = "Reader of field `EN`"]
pub type EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN`"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
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
#[doc = "Reader of field `INACTVAL`"]
pub type INACTVAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INACTVAL`"]
pub struct INACTVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> INACTVAL_W<'a> {
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
#[doc = "Reader of field `HYSTEN`"]
pub type HYSTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HYSTEN`"]
pub struct HYSTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HYSTEN_W<'a> {
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
#[doc = "Warm-Up Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WARMTIME_A {
    #[doc = "0: 4 HFPERCLK cycles"]
    _4CYCLES,
    #[doc = "1: 8 HFPERCLK cycles"]
    _8CYCLES,
    #[doc = "2: 16 HFPERCLK cycles"]
    _16CYCLES,
    #[doc = "3: 32 HFPERCLK cycles"]
    _32CYCLES,
    #[doc = "4: 64 HFPERCLK cycles"]
    _64CYCLES,
    #[doc = "5: 128 HFPERCLK cycles"]
    _128CYCLES,
    #[doc = "6: 256 HFPERCLK cycles"]
    _256CYCLES,
    #[doc = "7: 512 HFPERCLK cycles"]
    _512CYCLES,
}
impl From<WARMTIME_A> for u8 {
    #[inline(always)]
    fn from(variant: WARMTIME_A) -> Self {
        match variant {
            WARMTIME_A::_4CYCLES => 0,
            WARMTIME_A::_8CYCLES => 1,
            WARMTIME_A::_16CYCLES => 2,
            WARMTIME_A::_32CYCLES => 3,
            WARMTIME_A::_64CYCLES => 4,
            WARMTIME_A::_128CYCLES => 5,
            WARMTIME_A::_256CYCLES => 6,
            WARMTIME_A::_512CYCLES => 7,
        }
    }
}
#[doc = "Reader of field `WARMTIME`"]
pub type WARMTIME_R = crate::R<u8, WARMTIME_A>;
impl WARMTIME_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WARMTIME_A {
        match self.bits {
            0 => WARMTIME_A::_4CYCLES,
            1 => WARMTIME_A::_8CYCLES,
            2 => WARMTIME_A::_16CYCLES,
            3 => WARMTIME_A::_32CYCLES,
            4 => WARMTIME_A::_64CYCLES,
            5 => WARMTIME_A::_128CYCLES,
            6 => WARMTIME_A::_256CYCLES,
            7 => WARMTIME_A::_512CYCLES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_4CYCLES`"]
    #[inline(always)]
    pub fn is_4cycles(&self) -> bool {
        *self == WARMTIME_A::_4CYCLES
    }
    #[doc = "Checks if the value of the field is `_8CYCLES`"]
    #[inline(always)]
    pub fn is_8cycles(&self) -> bool {
        *self == WARMTIME_A::_8CYCLES
    }
    #[doc = "Checks if the value of the field is `_16CYCLES`"]
    #[inline(always)]
    pub fn is_16cycles(&self) -> bool {
        *self == WARMTIME_A::_16CYCLES
    }
    #[doc = "Checks if the value of the field is `_32CYCLES`"]
    #[inline(always)]
    pub fn is_32cycles(&self) -> bool {
        *self == WARMTIME_A::_32CYCLES
    }
    #[doc = "Checks if the value of the field is `_64CYCLES`"]
    #[inline(always)]
    pub fn is_64cycles(&self) -> bool {
        *self == WARMTIME_A::_64CYCLES
    }
    #[doc = "Checks if the value of the field is `_128CYCLES`"]
    #[inline(always)]
    pub fn is_128cycles(&self) -> bool {
        *self == WARMTIME_A::_128CYCLES
    }
    #[doc = "Checks if the value of the field is `_256CYCLES`"]
    #[inline(always)]
    pub fn is_256cycles(&self) -> bool {
        *self == WARMTIME_A::_256CYCLES
    }
    #[doc = "Checks if the value of the field is `_512CYCLES`"]
    #[inline(always)]
    pub fn is_512cycles(&self) -> bool {
        *self == WARMTIME_A::_512CYCLES
    }
}
#[doc = "Write proxy for field `WARMTIME`"]
pub struct WARMTIME_W<'a> {
    w: &'a mut W,
}
impl<'a> WARMTIME_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WARMTIME_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "4 HFPERCLK cycles"]
    #[inline(always)]
    pub fn _4cycles(self) -> &'a mut W {
        self.variant(WARMTIME_A::_4CYCLES)
    }
    #[doc = "8 HFPERCLK cycles"]
    #[inline(always)]
    pub fn _8cycles(self) -> &'a mut W {
        self.variant(WARMTIME_A::_8CYCLES)
    }
    #[doc = "16 HFPERCLK cycles"]
    #[inline(always)]
    pub fn _16cycles(self) -> &'a mut W {
        self.variant(WARMTIME_A::_16CYCLES)
    }
    #[doc = "32 HFPERCLK cycles"]
    #[inline(always)]
    pub fn _32cycles(self) -> &'a mut W {
        self.variant(WARMTIME_A::_32CYCLES)
    }
    #[doc = "64 HFPERCLK cycles"]
    #[inline(always)]
    pub fn _64cycles(self) -> &'a mut W {
        self.variant(WARMTIME_A::_64CYCLES)
    }
    #[doc = "128 HFPERCLK cycles"]
    #[inline(always)]
    pub fn _128cycles(self) -> &'a mut W {
        self.variant(WARMTIME_A::_128CYCLES)
    }
    #[doc = "256 HFPERCLK cycles"]
    #[inline(always)]
    pub fn _256cycles(self) -> &'a mut W {
        self.variant(WARMTIME_A::_256CYCLES)
    }
    #[doc = "512 HFPERCLK cycles"]
    #[inline(always)]
    pub fn _512cycles(self) -> &'a mut W {
        self.variant(WARMTIME_A::_512CYCLES)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `IRISE`"]
pub type IRISE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IRISE`"]
pub struct IRISE_W<'a> {
    w: &'a mut W,
}
impl<'a> IRISE_W<'a> {
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
#[doc = "Reader of field `IFALL`"]
pub type IFALL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IFALL`"]
pub struct IFALL_W<'a> {
    w: &'a mut W,
}
impl<'a> IFALL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `BIASPROG`"]
pub type BIASPROG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BIASPROG`"]
pub struct BIASPROG_W<'a> {
    w: &'a mut W,
}
impl<'a> BIASPROG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `HALFBIAS`"]
pub type HALFBIAS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HALFBIAS`"]
pub struct HALFBIAS_W<'a> {
    w: &'a mut W,
}
impl<'a> HALFBIAS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Voltage Supply Comparator Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Inactive Value"]
    #[inline(always)]
    pub fn inactval(&self) -> INACTVAL_R {
        INACTVAL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Hysteresis Enable"]
    #[inline(always)]
    pub fn hysten(&self) -> HYSTEN_R {
        HYSTEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Warm-Up Time"]
    #[inline(always)]
    pub fn warmtime(&self) -> WARMTIME_R {
        WARMTIME_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 16 - Rising Edge Interrupt Sense"]
    #[inline(always)]
    pub fn irise(&self) -> IRISE_R {
        IRISE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Falling Edge Interrupt Sense"]
    #[inline(always)]
    pub fn ifall(&self) -> IFALL_R {
        IFALL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 24:27 - VCMP Bias Programming Value"]
    #[inline(always)]
    pub fn biasprog(&self) -> BIASPROG_R {
        BIASPROG_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 30 - Half Bias Current"]
    #[inline(always)]
    pub fn halfbias(&self) -> HALFBIAS_R {
        HALFBIAS_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Voltage Supply Comparator Enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Bit 2 - Inactive Value"]
    #[inline(always)]
    pub fn inactval(&mut self) -> INACTVAL_W {
        INACTVAL_W { w: self }
    }
    #[doc = "Bit 4 - Hysteresis Enable"]
    #[inline(always)]
    pub fn hysten(&mut self) -> HYSTEN_W {
        HYSTEN_W { w: self }
    }
    #[doc = "Bits 8:10 - Warm-Up Time"]
    #[inline(always)]
    pub fn warmtime(&mut self) -> WARMTIME_W {
        WARMTIME_W { w: self }
    }
    #[doc = "Bit 16 - Rising Edge Interrupt Sense"]
    #[inline(always)]
    pub fn irise(&mut self) -> IRISE_W {
        IRISE_W { w: self }
    }
    #[doc = "Bit 17 - Falling Edge Interrupt Sense"]
    #[inline(always)]
    pub fn ifall(&mut self) -> IFALL_W {
        IFALL_W { w: self }
    }
    #[doc = "Bits 24:27 - VCMP Bias Programming Value"]
    #[inline(always)]
    pub fn biasprog(&mut self) -> BIASPROG_W {
        BIASPROG_W { w: self }
    }
    #[doc = "Bit 30 - Half Bias Current"]
    #[inline(always)]
    pub fn halfbias(&mut self) -> HALFBIAS_W {
        HALFBIAS_W { w: self }
    }
}

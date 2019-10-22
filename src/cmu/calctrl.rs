#[doc = "Reader of register CALCTRL"]
pub type R = crate::R<u32, super::CALCTRL>;
#[doc = "Writer for register CALCTRL"]
pub type W = crate::W<u32, super::CALCTRL>;
#[doc = "Register CALCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CALCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Calibration Up-counter Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UPSEL_A {
    #[doc = "0: Select HFXO as up-counter."]
    HFXO,
    #[doc = "1: Select LFXO as up-counter."]
    LFXO,
    #[doc = "2: Select HFRCO as up-counter."]
    HFRCO,
    #[doc = "3: Select LFRCO as up-counter."]
    LFRCO,
    #[doc = "4: Select AUXHFRCO as up-counter."]
    AUXHFRCO,
    #[doc = "5: Select USHFRCO as up-counter."]
    USHFRCO,
}
impl From<UPSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: UPSEL_A) -> Self {
        match variant {
            UPSEL_A::HFXO => 0,
            UPSEL_A::LFXO => 1,
            UPSEL_A::HFRCO => 2,
            UPSEL_A::LFRCO => 3,
            UPSEL_A::AUXHFRCO => 4,
            UPSEL_A::USHFRCO => 5,
        }
    }
}
#[doc = "Reader of field `UPSEL`"]
pub type UPSEL_R = crate::R<u8, UPSEL_A>;
impl UPSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, UPSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(UPSEL_A::HFXO),
            1 => Val(UPSEL_A::LFXO),
            2 => Val(UPSEL_A::HFRCO),
            3 => Val(UPSEL_A::LFRCO),
            4 => Val(UPSEL_A::AUXHFRCO),
            5 => Val(UPSEL_A::USHFRCO),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `HFXO`"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == UPSEL_A::HFXO
    }
    #[doc = "Checks if the value of the field is `LFXO`"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == UPSEL_A::LFXO
    }
    #[doc = "Checks if the value of the field is `HFRCO`"]
    #[inline(always)]
    pub fn is_hfrco(&self) -> bool {
        *self == UPSEL_A::HFRCO
    }
    #[doc = "Checks if the value of the field is `LFRCO`"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == UPSEL_A::LFRCO
    }
    #[doc = "Checks if the value of the field is `AUXHFRCO`"]
    #[inline(always)]
    pub fn is_auxhfrco(&self) -> bool {
        *self == UPSEL_A::AUXHFRCO
    }
    #[doc = "Checks if the value of the field is `USHFRCO`"]
    #[inline(always)]
    pub fn is_ushfrco(&self) -> bool {
        *self == UPSEL_A::USHFRCO
    }
}
#[doc = "Write proxy for field `UPSEL`"]
pub struct UPSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> UPSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UPSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select HFXO as up-counter."]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut W {
        self.variant(UPSEL_A::HFXO)
    }
    #[doc = "Select LFXO as up-counter."]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut W {
        self.variant(UPSEL_A::LFXO)
    }
    #[doc = "Select HFRCO as up-counter."]
    #[inline(always)]
    pub fn hfrco(self) -> &'a mut W {
        self.variant(UPSEL_A::HFRCO)
    }
    #[doc = "Select LFRCO as up-counter."]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut W {
        self.variant(UPSEL_A::LFRCO)
    }
    #[doc = "Select AUXHFRCO as up-counter."]
    #[inline(always)]
    pub fn auxhfrco(self) -> &'a mut W {
        self.variant(UPSEL_A::AUXHFRCO)
    }
    #[doc = "Select USHFRCO as up-counter."]
    #[inline(always)]
    pub fn ushfrco(self) -> &'a mut W {
        self.variant(UPSEL_A::USHFRCO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Calibration Down-counter Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOWNSEL_A {
    #[doc = "0: Select HFCLK for down-counter."]
    HFCLK,
    #[doc = "1: Select HFXO for down-counter."]
    HFXO,
    #[doc = "2: Select LFXO for down-counter."]
    LFXO,
    #[doc = "3: Select HFRCO for down-counter."]
    HFRCO,
    #[doc = "4: Select LFRCO for down-counter."]
    LFRCO,
    #[doc = "5: Select AUXHFRCO for down-counter."]
    AUXHFRCO,
    #[doc = "6: Select USHFRCO for down-counter."]
    USHFRCO,
}
impl From<DOWNSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DOWNSEL_A) -> Self {
        match variant {
            DOWNSEL_A::HFCLK => 0,
            DOWNSEL_A::HFXO => 1,
            DOWNSEL_A::LFXO => 2,
            DOWNSEL_A::HFRCO => 3,
            DOWNSEL_A::LFRCO => 4,
            DOWNSEL_A::AUXHFRCO => 5,
            DOWNSEL_A::USHFRCO => 6,
        }
    }
}
#[doc = "Reader of field `DOWNSEL`"]
pub type DOWNSEL_R = crate::R<u8, DOWNSEL_A>;
impl DOWNSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DOWNSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DOWNSEL_A::HFCLK),
            1 => Val(DOWNSEL_A::HFXO),
            2 => Val(DOWNSEL_A::LFXO),
            3 => Val(DOWNSEL_A::HFRCO),
            4 => Val(DOWNSEL_A::LFRCO),
            5 => Val(DOWNSEL_A::AUXHFRCO),
            6 => Val(DOWNSEL_A::USHFRCO),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `HFCLK`"]
    #[inline(always)]
    pub fn is_hfclk(&self) -> bool {
        *self == DOWNSEL_A::HFCLK
    }
    #[doc = "Checks if the value of the field is `HFXO`"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == DOWNSEL_A::HFXO
    }
    #[doc = "Checks if the value of the field is `LFXO`"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == DOWNSEL_A::LFXO
    }
    #[doc = "Checks if the value of the field is `HFRCO`"]
    #[inline(always)]
    pub fn is_hfrco(&self) -> bool {
        *self == DOWNSEL_A::HFRCO
    }
    #[doc = "Checks if the value of the field is `LFRCO`"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == DOWNSEL_A::LFRCO
    }
    #[doc = "Checks if the value of the field is `AUXHFRCO`"]
    #[inline(always)]
    pub fn is_auxhfrco(&self) -> bool {
        *self == DOWNSEL_A::AUXHFRCO
    }
    #[doc = "Checks if the value of the field is `USHFRCO`"]
    #[inline(always)]
    pub fn is_ushfrco(&self) -> bool {
        *self == DOWNSEL_A::USHFRCO
    }
}
#[doc = "Write proxy for field `DOWNSEL`"]
pub struct DOWNSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DOWNSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DOWNSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select HFCLK for down-counter."]
    #[inline(always)]
    pub fn hfclk(self) -> &'a mut W {
        self.variant(DOWNSEL_A::HFCLK)
    }
    #[doc = "Select HFXO for down-counter."]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut W {
        self.variant(DOWNSEL_A::HFXO)
    }
    #[doc = "Select LFXO for down-counter."]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut W {
        self.variant(DOWNSEL_A::LFXO)
    }
    #[doc = "Select HFRCO for down-counter."]
    #[inline(always)]
    pub fn hfrco(self) -> &'a mut W {
        self.variant(DOWNSEL_A::HFRCO)
    }
    #[doc = "Select LFRCO for down-counter."]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut W {
        self.variant(DOWNSEL_A::LFRCO)
    }
    #[doc = "Select AUXHFRCO for down-counter."]
    #[inline(always)]
    pub fn auxhfrco(self) -> &'a mut W {
        self.variant(DOWNSEL_A::AUXHFRCO)
    }
    #[doc = "Select USHFRCO for down-counter."]
    #[inline(always)]
    pub fn ushfrco(self) -> &'a mut W {
        self.variant(DOWNSEL_A::USHFRCO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
        self.w
    }
}
#[doc = "Reader of field `CONT`"]
pub type CONT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CONT`"]
pub struct CONT_W<'a> {
    w: &'a mut W,
}
impl<'a> CONT_W<'a> {
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
    #[doc = "Bits 0:2 - Calibration Up-counter Select"]
    #[inline(always)]
    pub fn upsel(&self) -> UPSEL_R {
        UPSEL_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:5 - Calibration Down-counter Select"]
    #[inline(always)]
    pub fn downsel(&self) -> DOWNSEL_R {
        DOWNSEL_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bit 6 - Continuous Calibration"]
    #[inline(always)]
    pub fn cont(&self) -> CONT_R {
        CONT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Calibration Up-counter Select"]
    #[inline(always)]
    pub fn upsel(&mut self) -> UPSEL_W {
        UPSEL_W { w: self }
    }
    #[doc = "Bits 3:5 - Calibration Down-counter Select"]
    #[inline(always)]
    pub fn downsel(&mut self) -> DOWNSEL_W {
        DOWNSEL_W { w: self }
    }
    #[doc = "Bit 6 - Continuous Calibration"]
    #[inline(always)]
    pub fn cont(&mut self) -> CONT_W {
        CONT_W { w: self }
    }
}

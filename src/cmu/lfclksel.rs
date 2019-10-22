#[doc = "Reader of register LFCLKSEL"]
pub type R = crate::R<u32, super::LFCLKSEL>;
#[doc = "Writer for register LFCLKSEL"]
pub type W = crate::W<u32, super::LFCLKSEL>;
#[doc = "Register LFCLKSEL `reset()`'s with value 0x15"]
impl crate::ResetValue for super::LFCLKSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x15
    }
}
#[doc = "Clock Select for LFA\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LFA_A {
    #[doc = "0: LFACLK is disabled"]
    DISABLED,
    #[doc = "1: LFRCO selected as LFACLK"]
    LFRCO,
    #[doc = "2: LFXO selected as LFACLK"]
    LFXO,
    #[doc = "3: HFCORECLKLE divided by two or four is selected as LFACLK. The division factor is determined by CMU_CTRL_HFLE and CMU_HFCORECLKDIV_HFCORECLKLEDIV."]
    HFCORECLKLEDIV2,
}
impl From<LFA_A> for u8 {
    #[inline(always)]
    fn from(variant: LFA_A) -> Self {
        match variant {
            LFA_A::DISABLED => 0,
            LFA_A::LFRCO => 1,
            LFA_A::LFXO => 2,
            LFA_A::HFCORECLKLEDIV2 => 3,
        }
    }
}
#[doc = "Reader of field `LFA`"]
pub type LFA_R = crate::R<u8, LFA_A>;
impl LFA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LFA_A {
        match self.bits {
            0 => LFA_A::DISABLED,
            1 => LFA_A::LFRCO,
            2 => LFA_A::LFXO,
            3 => LFA_A::HFCORECLKLEDIV2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LFA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `LFRCO`"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == LFA_A::LFRCO
    }
    #[doc = "Checks if the value of the field is `LFXO`"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == LFA_A::LFXO
    }
    #[doc = "Checks if the value of the field is `HFCORECLKLEDIV2`"]
    #[inline(always)]
    pub fn is_hfcoreclklediv2(&self) -> bool {
        *self == LFA_A::HFCORECLKLEDIV2
    }
}
#[doc = "Write proxy for field `LFA`"]
pub struct LFA_W<'a> {
    w: &'a mut W,
}
impl<'a> LFA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LFA_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "LFACLK is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LFA_A::DISABLED)
    }
    #[doc = "LFRCO selected as LFACLK"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut W {
        self.variant(LFA_A::LFRCO)
    }
    #[doc = "LFXO selected as LFACLK"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut W {
        self.variant(LFA_A::LFXO)
    }
    #[doc = "HFCORECLKLE divided by two or four is selected as LFACLK. The division factor is determined by CMU_CTRL_HFLE and CMU_HFCORECLKDIV_HFCORECLKLEDIV."]
    #[inline(always)]
    pub fn hfcoreclklediv2(self) -> &'a mut W {
        self.variant(LFA_A::HFCORECLKLEDIV2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Clock Select for LFB\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LFB_A {
    #[doc = "0: LFBCLK is disabled"]
    DISABLED,
    #[doc = "1: LFRCO selected as LFBCLK"]
    LFRCO,
    #[doc = "2: LFXO selected as LFBCLK"]
    LFXO,
    #[doc = "3: HFCORECLKLE divided by two or four is selected as LFACLK. The division factor is determined by CMU_CTRL_HFLE and CMU_HFCORECLKDIV_HFCORECLKLEDIV."]
    HFCORECLKLEDIV2,
}
impl From<LFB_A> for u8 {
    #[inline(always)]
    fn from(variant: LFB_A) -> Self {
        match variant {
            LFB_A::DISABLED => 0,
            LFB_A::LFRCO => 1,
            LFB_A::LFXO => 2,
            LFB_A::HFCORECLKLEDIV2 => 3,
        }
    }
}
#[doc = "Reader of field `LFB`"]
pub type LFB_R = crate::R<u8, LFB_A>;
impl LFB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LFB_A {
        match self.bits {
            0 => LFB_A::DISABLED,
            1 => LFB_A::LFRCO,
            2 => LFB_A::LFXO,
            3 => LFB_A::HFCORECLKLEDIV2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LFB_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `LFRCO`"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == LFB_A::LFRCO
    }
    #[doc = "Checks if the value of the field is `LFXO`"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == LFB_A::LFXO
    }
    #[doc = "Checks if the value of the field is `HFCORECLKLEDIV2`"]
    #[inline(always)]
    pub fn is_hfcoreclklediv2(&self) -> bool {
        *self == LFB_A::HFCORECLKLEDIV2
    }
}
#[doc = "Write proxy for field `LFB`"]
pub struct LFB_W<'a> {
    w: &'a mut W,
}
impl<'a> LFB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LFB_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "LFBCLK is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LFB_A::DISABLED)
    }
    #[doc = "LFRCO selected as LFBCLK"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut W {
        self.variant(LFB_A::LFRCO)
    }
    #[doc = "LFXO selected as LFBCLK"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut W {
        self.variant(LFB_A::LFXO)
    }
    #[doc = "HFCORECLKLE divided by two or four is selected as LFACLK. The division factor is determined by CMU_CTRL_HFLE and CMU_HFCORECLKDIV_HFCORECLKLEDIV."]
    #[inline(always)]
    pub fn hfcoreclklediv2(self) -> &'a mut W {
        self.variant(LFB_A::HFCORECLKLEDIV2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Clock Select for LFC\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LFC_A {
    #[doc = "0: LFCCLK clock disabled."]
    DISABLED,
    #[doc = "1: LFRCO selected as LFCCLK clock"]
    LFRCO,
    #[doc = "2: LFXO selected as LFCCLK clock"]
    LFXO,
}
impl From<LFC_A> for u8 {
    #[inline(always)]
    fn from(variant: LFC_A) -> Self {
        match variant {
            LFC_A::DISABLED => 0,
            LFC_A::LFRCO => 1,
            LFC_A::LFXO => 2,
        }
    }
}
#[doc = "Reader of field `LFC`"]
pub type LFC_R = crate::R<u8, LFC_A>;
impl LFC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LFC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(LFC_A::DISABLED),
            1 => Val(LFC_A::LFRCO),
            2 => Val(LFC_A::LFXO),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LFC_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `LFRCO`"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == LFC_A::LFRCO
    }
    #[doc = "Checks if the value of the field is `LFXO`"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == LFC_A::LFXO
    }
}
#[doc = "Write proxy for field `LFC`"]
pub struct LFC_W<'a> {
    w: &'a mut W,
}
impl<'a> LFC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LFC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "LFCCLK clock disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LFC_A::DISABLED)
    }
    #[doc = "LFRCO selected as LFCCLK clock"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut W {
        self.variant(LFC_A::LFRCO)
    }
    #[doc = "LFXO selected as LFCCLK clock"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut W {
        self.variant(LFC_A::LFXO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `LFAE`"]
pub type LFAE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LFAE`"]
pub struct LFAE_W<'a> {
    w: &'a mut W,
}
impl<'a> LFAE_W<'a> {
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
#[doc = "Reader of field `LFBE`"]
pub type LFBE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LFBE`"]
pub struct LFBE_W<'a> {
    w: &'a mut W,
}
impl<'a> LFBE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Clock Select for LFA"]
    #[inline(always)]
    pub fn lfa(&self) -> LFA_R {
        LFA_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Clock Select for LFB"]
    #[inline(always)]
    pub fn lfb(&self) -> LFB_R {
        LFB_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Clock Select for LFC"]
    #[inline(always)]
    pub fn lfc(&self) -> LFC_R {
        LFC_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 16 - Clock Select for LFA Extended"]
    #[inline(always)]
    pub fn lfae(&self) -> LFAE_R {
        LFAE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Clock Select for LFB Extended"]
    #[inline(always)]
    pub fn lfbe(&self) -> LFBE_R {
        LFBE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Clock Select for LFA"]
    #[inline(always)]
    pub fn lfa(&mut self) -> LFA_W {
        LFA_W { w: self }
    }
    #[doc = "Bits 2:3 - Clock Select for LFB"]
    #[inline(always)]
    pub fn lfb(&mut self) -> LFB_W {
        LFB_W { w: self }
    }
    #[doc = "Bits 4:5 - Clock Select for LFC"]
    #[inline(always)]
    pub fn lfc(&mut self) -> LFC_W {
        LFC_W { w: self }
    }
    #[doc = "Bit 16 - Clock Select for LFA Extended"]
    #[inline(always)]
    pub fn lfae(&mut self) -> LFAE_W {
        LFAE_W { w: self }
    }
    #[doc = "Bit 20 - Clock Select for LFB Extended"]
    #[inline(always)]
    pub fn lfbe(&mut self) -> LFBE_W {
        LFBE_W { w: self }
    }
}

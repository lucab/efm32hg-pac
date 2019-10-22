#[doc = "Reader of register DCFG"]
pub type R = crate::R<u32, super::DCFG>;
#[doc = "Writer for register DCFG"]
pub type W = crate::W<u32, super::DCFG>;
#[doc = "Register DCFG `reset()`'s with value 0x0800_0000"]
impl crate::ResetValue for super::DCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0800_0000
    }
}
#[doc = "Device Speed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEVSPD_A {
    #[doc = "2: Low speed (PHY clock is 6 MHz). If you select 6 MHz LS mode, you must do a soft reset."]
    LS,
    #[doc = "3: Full speed (PHY clock is 48 MHz)."]
    FS,
}
impl From<DEVSPD_A> for u8 {
    #[inline(always)]
    fn from(variant: DEVSPD_A) -> Self {
        match variant {
            DEVSPD_A::LS => 2,
            DEVSPD_A::FS => 3,
        }
    }
}
#[doc = "Reader of field `DEVSPD`"]
pub type DEVSPD_R = crate::R<u8, DEVSPD_A>;
impl DEVSPD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DEVSPD_A> {
        use crate::Variant::*;
        match self.bits {
            2 => Val(DEVSPD_A::LS),
            3 => Val(DEVSPD_A::FS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LS`"]
    #[inline(always)]
    pub fn is_ls(&self) -> bool {
        *self == DEVSPD_A::LS
    }
    #[doc = "Checks if the value of the field is `FS`"]
    #[inline(always)]
    pub fn is_fs(&self) -> bool {
        *self == DEVSPD_A::FS
    }
}
#[doc = "Write proxy for field `DEVSPD`"]
pub struct DEVSPD_W<'a> {
    w: &'a mut W,
}
impl<'a> DEVSPD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DEVSPD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Low speed (PHY clock is 6 MHz). If you select 6 MHz LS mode, you must do a soft reset."]
    #[inline(always)]
    pub fn ls(self) -> &'a mut W {
        self.variant(DEVSPD_A::LS)
    }
    #[doc = "Full speed (PHY clock is 48 MHz)."]
    #[inline(always)]
    pub fn fs(self) -> &'a mut W {
        self.variant(DEVSPD_A::FS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `NZSTSOUTHSHK`"]
pub type NZSTSOUTHSHK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NZSTSOUTHSHK`"]
pub struct NZSTSOUTHSHK_W<'a> {
    w: &'a mut W,
}
impl<'a> NZSTSOUTHSHK_W<'a> {
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
#[doc = "Reader of field `ENA32KHZSUSP`"]
pub type ENA32KHZSUSP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENA32KHZSUSP`"]
pub struct ENA32KHZSUSP_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA32KHZSUSP_W<'a> {
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
#[doc = "Reader of field `DEVADDR`"]
pub type DEVADDR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DEVADDR`"]
pub struct DEVADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> DEVADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 4)) | (((value as u32) & 0x7f) << 4);
        self.w
    }
}
#[doc = "Periodic Frame Interval\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PERFRINT_A {
    #[doc = "0: 80% of the frame interval."]
    _80PCNT,
    #[doc = "1: 85% of the frame interval."]
    _85PCNT,
    #[doc = "2: 90% of the frame interval."]
    _90PCNT,
    #[doc = "3: 95% of the frame interval."]
    _95PCNT,
}
impl From<PERFRINT_A> for u8 {
    #[inline(always)]
    fn from(variant: PERFRINT_A) -> Self {
        match variant {
            PERFRINT_A::_80PCNT => 0,
            PERFRINT_A::_85PCNT => 1,
            PERFRINT_A::_90PCNT => 2,
            PERFRINT_A::_95PCNT => 3,
        }
    }
}
#[doc = "Reader of field `PERFRINT`"]
pub type PERFRINT_R = crate::R<u8, PERFRINT_A>;
impl PERFRINT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PERFRINT_A {
        match self.bits {
            0 => PERFRINT_A::_80PCNT,
            1 => PERFRINT_A::_85PCNT,
            2 => PERFRINT_A::_90PCNT,
            3 => PERFRINT_A::_95PCNT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_80PCNT`"]
    #[inline(always)]
    pub fn is_80pcnt(&self) -> bool {
        *self == PERFRINT_A::_80PCNT
    }
    #[doc = "Checks if the value of the field is `_85PCNT`"]
    #[inline(always)]
    pub fn is_85pcnt(&self) -> bool {
        *self == PERFRINT_A::_85PCNT
    }
    #[doc = "Checks if the value of the field is `_90PCNT`"]
    #[inline(always)]
    pub fn is_90pcnt(&self) -> bool {
        *self == PERFRINT_A::_90PCNT
    }
    #[doc = "Checks if the value of the field is `_95PCNT`"]
    #[inline(always)]
    pub fn is_95pcnt(&self) -> bool {
        *self == PERFRINT_A::_95PCNT
    }
}
#[doc = "Write proxy for field `PERFRINT`"]
pub struct PERFRINT_W<'a> {
    w: &'a mut W,
}
impl<'a> PERFRINT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PERFRINT_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "80% of the frame interval."]
    #[inline(always)]
    pub fn _80pcnt(self) -> &'a mut W {
        self.variant(PERFRINT_A::_80PCNT)
    }
    #[doc = "85% of the frame interval."]
    #[inline(always)]
    pub fn _85pcnt(self) -> &'a mut W {
        self.variant(PERFRINT_A::_85PCNT)
    }
    #[doc = "90% of the frame interval."]
    #[inline(always)]
    pub fn _90pcnt(self) -> &'a mut W {
        self.variant(PERFRINT_A::_90PCNT)
    }
    #[doc = "95% of the frame interval."]
    #[inline(always)]
    pub fn _95pcnt(self) -> &'a mut W {
        self.variant(PERFRINT_A::_95PCNT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | (((value as u32) & 0x03) << 11);
        self.w
    }
}
#[doc = "Reader of field `ERRATICINTMSK`"]
pub type ERRATICINTMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERRATICINTMSK`"]
pub struct ERRATICINTMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRATICINTMSK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `RESVALID`"]
pub type RESVALID_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESVALID`"]
pub struct RESVALID_W<'a> {
    w: &'a mut W,
}
impl<'a> RESVALID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 26)) | (((value as u32) & 0x3f) << 26);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Device Speed"]
    #[inline(always)]
    pub fn devspd(&self) -> DEVSPD_R {
        DEVSPD_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Non-Zero-Length Status OUT Handshake"]
    #[inline(always)]
    pub fn nzstsouthshk(&self) -> NZSTSOUTHSHK_R {
        NZSTSOUTHSHK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable 32 KHz Suspend mode"]
    #[inline(always)]
    pub fn ena32khzsusp(&self) -> ENA32KHZSUSP_R {
        ENA32KHZSUSP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:10 - Device Address"]
    #[inline(always)]
    pub fn devaddr(&self) -> DEVADDR_R {
        DEVADDR_R::new(((self.bits >> 4) & 0x7f) as u8)
    }
    #[doc = "Bits 11:12 - Periodic Frame Interval"]
    #[inline(always)]
    pub fn perfrint(&self) -> PERFRINT_R {
        PERFRINT_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bit 15 - \"\""]
    #[inline(always)]
    pub fn erraticintmsk(&self) -> ERRATICINTMSK_R {
        ERRATICINTMSK_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 26:31 - Resume Validation Period"]
    #[inline(always)]
    pub fn resvalid(&self) -> RESVALID_R {
        RESVALID_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Device Speed"]
    #[inline(always)]
    pub fn devspd(&mut self) -> DEVSPD_W {
        DEVSPD_W { w: self }
    }
    #[doc = "Bit 2 - Non-Zero-Length Status OUT Handshake"]
    #[inline(always)]
    pub fn nzstsouthshk(&mut self) -> NZSTSOUTHSHK_W {
        NZSTSOUTHSHK_W { w: self }
    }
    #[doc = "Bit 3 - Enable 32 KHz Suspend mode"]
    #[inline(always)]
    pub fn ena32khzsusp(&mut self) -> ENA32KHZSUSP_W {
        ENA32KHZSUSP_W { w: self }
    }
    #[doc = "Bits 4:10 - Device Address"]
    #[inline(always)]
    pub fn devaddr(&mut self) -> DEVADDR_W {
        DEVADDR_W { w: self }
    }
    #[doc = "Bits 11:12 - Periodic Frame Interval"]
    #[inline(always)]
    pub fn perfrint(&mut self) -> PERFRINT_W {
        PERFRINT_W { w: self }
    }
    #[doc = "Bit 15 - \"\""]
    #[inline(always)]
    pub fn erraticintmsk(&mut self) -> ERRATICINTMSK_W {
        ERRATICINTMSK_W { w: self }
    }
    #[doc = "Bits 26:31 - Resume Validation Period"]
    #[inline(always)]
    pub fn resvalid(&mut self) -> RESVALID_W {
        RESVALID_W { w: self }
    }
}

#[doc = "Reader of register CAL"]
pub type R = crate::R<u32, super::CAL>;
#[doc = "Writer for register CAL"]
pub type W = crate::W<u32, super::CAL>;
#[doc = "Register CAL `reset()`'s with value 0x3f00_3f00"]
impl crate::ResetValue for super::CAL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x3f00_3f00
    }
}
#[doc = "Reader of field `SINGLEOFFSET`"]
pub type SINGLEOFFSET_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SINGLEOFFSET`"]
pub struct SINGLEOFFSET_W<'a> {
    w: &'a mut W,
}
impl<'a> SINGLEOFFSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `SINGLEGAIN`"]
pub type SINGLEGAIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SINGLEGAIN`"]
pub struct SINGLEGAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> SINGLEGAIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u32) & 0x7f) << 8);
        self.w
    }
}
#[doc = "Reader of field `SCANOFFSET`"]
pub type SCANOFFSET_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCANOFFSET`"]
pub struct SCANOFFSET_W<'a> {
    w: &'a mut W,
}
impl<'a> SCANOFFSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | (((value as u32) & 0x7f) << 16);
        self.w
    }
}
#[doc = "Reader of field `SCANGAIN`"]
pub type SCANGAIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCANGAIN`"]
pub struct SCANGAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> SCANGAIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 24)) | (((value as u32) & 0x7f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Single Mode Offset Calibration Value"]
    #[inline(always)]
    pub fn singleoffset(&self) -> SINGLEOFFSET_R {
        SINGLEOFFSET_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Single Mode Gain Calibration Value"]
    #[inline(always)]
    pub fn singlegain(&self) -> SINGLEGAIN_R {
        SINGLEGAIN_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - Scan Mode Offset Calibration Value"]
    #[inline(always)]
    pub fn scanoffset(&self) -> SCANOFFSET_R {
        SCANOFFSET_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - Scan Mode Gain Calibration Value"]
    #[inline(always)]
    pub fn scangain(&self) -> SCANGAIN_R {
        SCANGAIN_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Single Mode Offset Calibration Value"]
    #[inline(always)]
    pub fn singleoffset(&mut self) -> SINGLEOFFSET_W {
        SINGLEOFFSET_W { w: self }
    }
    #[doc = "Bits 8:14 - Single Mode Gain Calibration Value"]
    #[inline(always)]
    pub fn singlegain(&mut self) -> SINGLEGAIN_W {
        SINGLEGAIN_W { w: self }
    }
    #[doc = "Bits 16:22 - Scan Mode Offset Calibration Value"]
    #[inline(always)]
    pub fn scanoffset(&mut self) -> SCANOFFSET_W {
        SCANOFFSET_W { w: self }
    }
    #[doc = "Bits 24:30 - Scan Mode Gain Calibration Value"]
    #[inline(always)]
    pub fn scangain(&mut self) -> SCANGAIN_W {
        SCANGAIN_W { w: self }
    }
}

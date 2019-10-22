#[doc = "Reader of register CLKDIV"]
pub type R = crate::R<u32, super::CLKDIV>;
#[doc = "Writer for register CLKDIV"]
pub type W = crate::W<u32, super::CLKDIV>;
#[doc = "Register CLKDIV `reset()`'s with value 0"]
impl crate::ResetValue for super::CLKDIV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DIVEXT`"]
pub type DIVEXT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIVEXT`"]
pub struct DIVEXT_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVEXT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
        self.w
    }
}
#[doc = "Reader of field `DIV`"]
pub type DIV_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DIV`"]
pub struct DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff << 6)) | (((value as u32) & 0x7fff) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:5 - Extended Fractional Clock Divider"]
    #[inline(always)]
    pub fn divext(&self) -> DIVEXT_R {
        DIVEXT_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bits 6:20 - Fractional Clock Divider"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new(((self.bits >> 6) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 3:5 - Extended Fractional Clock Divider"]
    #[inline(always)]
    pub fn divext(&mut self) -> DIVEXT_W {
        DIVEXT_W { w: self }
    }
    #[doc = "Bits 6:20 - Fractional Clock Divider"]
    #[inline(always)]
    pub fn div(&mut self) -> DIV_W {
        DIV_W { w: self }
    }
}

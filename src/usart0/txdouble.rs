#[doc = "Writer for register TXDOUBLE"]
pub type W = crate::W<u32, super::TXDOUBLE>;
#[doc = "Register TXDOUBLE `reset()`'s with value 0"]
impl crate::ResetValue for super::TXDOUBLE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `TXDATA0`"]
pub struct TXDATA0_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDATA0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Write proxy for field `TXDATA1`"]
pub struct TXDATA1_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDATA1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:7 - TX Data"]
    #[inline(always)]
    pub fn txdata0(&mut self) -> TXDATA0_W {
        TXDATA0_W { w: self }
    }
    #[doc = "Bits 8:15 - TX Data"]
    #[inline(always)]
    pub fn txdata1(&mut self) -> TXDATA1_W {
        TXDATA1_W { w: self }
    }
}

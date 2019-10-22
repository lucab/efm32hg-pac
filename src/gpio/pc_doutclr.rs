#[doc = "Writer for register PC_DOUTCLR"]
pub type W = crate::W<u32, super::PC_DOUTCLR>;
#[doc = "Register PC_DOUTCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::PC_DOUTCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `DOUTCLR`"]
pub struct DOUTCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUTCLR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:15 - Data Out Clear"]
    #[inline(always)]
    pub fn doutclr(&mut self) -> DOUTCLR_W {
        DOUTCLR_W { w: self }
    }
}

#[doc = "Writer for register PE_DOUTSET"]
pub type W = crate::W<u32, super::PE_DOUTSET>;
#[doc = "Register PE_DOUTSET `reset()`'s with value 0"]
impl crate::ResetValue for super::PE_DOUTSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `DOUTSET`"]
pub struct DOUTSET_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUTSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:15 - Data Out Set"]
    #[inline(always)]
    pub fn doutset(&mut self) -> DOUTSET_W {
        DOUTSET_W { w: self }
    }
}

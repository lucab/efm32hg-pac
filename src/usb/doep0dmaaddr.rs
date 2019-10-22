#[doc = "Reader of register DOEP0DMAADDR"]
pub type R = crate::R<u32, super::DOEP0DMAADDR>;
#[doc = "Writer for register DOEP0DMAADDR"]
pub type W = crate::W<u32, super::DOEP0DMAADDR>;
#[doc = "Register DOEP0DMAADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::DOEP0DMAADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DOEP0DMAADDR`"]
pub type DOEP0DMAADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DOEP0DMAADDR`"]
pub struct DOEP0DMAADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> DOEP0DMAADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - DMA Address"]
    #[inline(always)]
    pub fn doep0dmaaddr(&self) -> DOEP0DMAADDR_R {
        DOEP0DMAADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - DMA Address"]
    #[inline(always)]
    pub fn doep0dmaaddr(&mut self) -> DOEP0DMAADDR_W {
        DOEP0DMAADDR_W { w: self }
    }
}

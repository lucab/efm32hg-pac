#[doc = "Reader of register DIEP0DMAADDR"]
pub type R = crate::R<u32, super::DIEP0DMAADDR>;
#[doc = "Writer for register DIEP0DMAADDR"]
pub type W = crate::W<u32, super::DIEP0DMAADDR>;
#[doc = "Register DIEP0DMAADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::DIEP0DMAADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DIEP0DMAADDR`"]
pub type DIEP0DMAADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DIEP0DMAADDR`"]
pub struct DIEP0DMAADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> DIEP0DMAADDR_W<'a> {
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
    pub fn diep0dmaaddr(&self) -> DIEP0DMAADDR_R {
        DIEP0DMAADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - DMA Address"]
    #[inline(always)]
    pub fn diep0dmaaddr(&mut self) -> DIEP0DMAADDR_W {
        DIEP0DMAADDR_W { w: self }
    }
}

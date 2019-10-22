#[doc = "Reader of register IRQLATENCY"]
pub type R = crate::R<u32, super::IRQLATENCY>;
#[doc = "Writer for register IRQLATENCY"]
pub type W = crate::W<u32, super::IRQLATENCY>;
#[doc = "Register IRQLATENCY `reset()`'s with value 0"]
impl crate::ResetValue for super::IRQLATENCY {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IRQLATENCY`"]
pub type IRQLATENCY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IRQLATENCY`"]
pub struct IRQLATENCY_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQLATENCY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Irq Latency Register"]
    #[inline(always)]
    pub fn irqlatency(&self) -> IRQLATENCY_R {
        IRQLATENCY_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Irq Latency Register"]
    #[inline(always)]
    pub fn irqlatency(&mut self) -> IRQLATENCY_W {
        IRQLATENCY_W { w: self }
    }
}

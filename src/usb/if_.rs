#[doc = "Reader of register IF"]
pub type R = crate::R<u32, super::IF>;
#[doc = "Reader of field `VREGOSH`"]
pub type VREGOSH_R = crate::R<bool, bool>;
#[doc = "Reader of field `VREGOSL`"]
pub type VREGOSL_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - VREGO Sense High Interrupt Flag"]
    #[inline(always)]
    pub fn vregosh(&self) -> VREGOSH_R {
        VREGOSH_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - VREGO Sense Low Interrupt Flag"]
    #[inline(always)]
    pub fn vregosl(&self) -> VREGOSL_R {
        VREGOSL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}

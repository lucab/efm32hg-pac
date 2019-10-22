#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Reader of field `VCMPACT`"]
pub type VCMPACT_R = crate::R<bool, bool>;
#[doc = "Reader of field `VCMPOUT`"]
pub type VCMPOUT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Voltage Supply Comparator Active"]
    #[inline(always)]
    pub fn vcmpact(&self) -> VCMPACT_R {
        VCMPACT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Voltage Supply Comparator Output"]
    #[inline(always)]
    pub fn vcmpout(&self) -> VCMPOUT_R {
        VCMPOUT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}

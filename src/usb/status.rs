#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Reader of field `VREGOS`"]
pub type VREGOS_R = crate::R<bool, bool>;
#[doc = "Reader of field `LEMACTIVE`"]
pub type LEMACTIVE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - VREGO Sense Output"]
    #[inline(always)]
    pub fn vregos(&self) -> VREGOS_R {
        VREGOS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Low Energy Mode Active"]
    #[inline(always)]
    pub fn lemactive(&self) -> LEMACTIVE_R {
        LEMACTIVE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}

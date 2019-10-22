#[doc = "Reader of register IDAC0CAL0"]
pub type R = crate::R<u32, super::IDAC0CAL0>;
#[doc = "Reader of field `RANGE0`"]
pub type RANGE0_R = crate::R<u8, u8>;
#[doc = "Reader of field `RANGE1`"]
pub type RANGE1_R = crate::R<u8, u8>;
#[doc = "Reader of field `RANGE2`"]
pub type RANGE2_R = crate::R<u8, u8>;
#[doc = "Reader of field `RANGE3`"]
pub type RANGE3_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Current range 0 tuning value for IDAC0"]
    #[inline(always)]
    pub fn range0(&self) -> RANGE0_R {
        RANGE0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Current range 1 tuning value for IDAC0"]
    #[inline(always)]
    pub fn range1(&self) -> RANGE1_R {
        RANGE1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Current range 2 tuning value for IDAC0"]
    #[inline(always)]
    pub fn range2(&self) -> RANGE2_R {
        RANGE2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Current range 3 tuning value for IDAC0"]
    #[inline(always)]
    pub fn range3(&self) -> RANGE3_R {
        RANGE3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}

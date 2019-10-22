#[doc = "Reader of register SYNCBUSY"]
pub type R = crate::R<u32, super::SYNCBUSY>;
#[doc = "Reader of field `LFACLKEN0`"]
pub type LFACLKEN0_R = crate::R<bool, bool>;
#[doc = "Reader of field `LFAPRESC0`"]
pub type LFAPRESC0_R = crate::R<bool, bool>;
#[doc = "Reader of field `LFBCLKEN0`"]
pub type LFBCLKEN0_R = crate::R<bool, bool>;
#[doc = "Reader of field `LFBPRESC0`"]
pub type LFBPRESC0_R = crate::R<bool, bool>;
#[doc = "Reader of field `LFCCLKEN0`"]
pub type LFCCLKEN0_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Low Frequency A Clock Enable 0 Busy"]
    #[inline(always)]
    pub fn lfaclken0(&self) -> LFACLKEN0_R {
        LFACLKEN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Low Frequency A Prescaler 0 Busy"]
    #[inline(always)]
    pub fn lfapresc0(&self) -> LFAPRESC0_R {
        LFAPRESC0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Low Frequency B Clock Enable 0 Busy"]
    #[inline(always)]
    pub fn lfbclken0(&self) -> LFBCLKEN0_R {
        LFBCLKEN0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Low Frequency B Prescaler 0 Busy"]
    #[inline(always)]
    pub fn lfbpresc0(&self) -> LFBPRESC0_R {
        LFBPRESC0_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Low Frequency C Clock Enable 0 Busy"]
    #[inline(always)]
    pub fn lfcclken0(&self) -> LFCCLKEN0_R {
        LFCCLKEN0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}

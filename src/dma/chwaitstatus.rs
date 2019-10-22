#[doc = "Reader of register CHWAITSTATUS"]
pub type R = crate::R<u32, super::CHWAITSTATUS>;
#[doc = "Reader of field `CH0WAITSTATUS`"]
pub type CH0WAITSTATUS_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH1WAITSTATUS`"]
pub type CH1WAITSTATUS_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH2WAITSTATUS`"]
pub type CH2WAITSTATUS_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH3WAITSTATUS`"]
pub type CH3WAITSTATUS_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH4WAITSTATUS`"]
pub type CH4WAITSTATUS_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH5WAITSTATUS`"]
pub type CH5WAITSTATUS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Channel 0 Wait on Request Status"]
    #[inline(always)]
    pub fn ch0waitstatus(&self) -> CH0WAITSTATUS_R {
        CH0WAITSTATUS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Wait on Request Status"]
    #[inline(always)]
    pub fn ch1waitstatus(&self) -> CH1WAITSTATUS_R {
        CH1WAITSTATUS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Wait on Request Status"]
    #[inline(always)]
    pub fn ch2waitstatus(&self) -> CH2WAITSTATUS_R {
        CH2WAITSTATUS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Wait on Request Status"]
    #[inline(always)]
    pub fn ch3waitstatus(&self) -> CH3WAITSTATUS_R {
        CH3WAITSTATUS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Channel 4 Wait on Request Status"]
    #[inline(always)]
    pub fn ch4waitstatus(&self) -> CH4WAITSTATUS_R {
        CH4WAITSTATUS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Channel 5 Wait on Request Status"]
    #[inline(always)]
    pub fn ch5waitstatus(&self) -> CH5WAITSTATUS_R {
        CH5WAITSTATUS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}

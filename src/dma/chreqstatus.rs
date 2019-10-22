#[doc = "Reader of register CHREQSTATUS"]
pub type R = crate::R<u32, super::CHREQSTATUS>;
#[doc = "Reader of field `CH0REQSTATUS`"]
pub type CH0REQSTATUS_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH1REQSTATUS`"]
pub type CH1REQSTATUS_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH2REQSTATUS`"]
pub type CH2REQSTATUS_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH3REQSTATUS`"]
pub type CH3REQSTATUS_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH4REQSTATUS`"]
pub type CH4REQSTATUS_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH5REQSTATUS`"]
pub type CH5REQSTATUS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Channel 0 Request Status"]
    #[inline(always)]
    pub fn ch0reqstatus(&self) -> CH0REQSTATUS_R {
        CH0REQSTATUS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Request Status"]
    #[inline(always)]
    pub fn ch1reqstatus(&self) -> CH1REQSTATUS_R {
        CH1REQSTATUS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Request Status"]
    #[inline(always)]
    pub fn ch2reqstatus(&self) -> CH2REQSTATUS_R {
        CH2REQSTATUS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Request Status"]
    #[inline(always)]
    pub fn ch3reqstatus(&self) -> CH3REQSTATUS_R {
        CH3REQSTATUS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Channel 4 Request Status"]
    #[inline(always)]
    pub fn ch4reqstatus(&self) -> CH4REQSTATUS_R {
        CH4REQSTATUS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Channel 5 Request Status"]
    #[inline(always)]
    pub fn ch5reqstatus(&self) -> CH5REQSTATUS_R {
        CH5REQSTATUS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}

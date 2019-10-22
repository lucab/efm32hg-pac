#[doc = "Reader of register CHSREQSTATUS"]
pub type R = crate::R<u32, super::CHSREQSTATUS>;
#[doc = "Reader of field `CH0SREQSTATUS`"]
pub type CH0SREQSTATUS_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH1SREQSTATUS`"]
pub type CH1SREQSTATUS_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH2SREQSTATUS`"]
pub type CH2SREQSTATUS_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH3SREQSTATUS`"]
pub type CH3SREQSTATUS_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH4SREQSTATUS`"]
pub type CH4SREQSTATUS_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH5SREQSTATUS`"]
pub type CH5SREQSTATUS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Channel 0 Single Request Status"]
    #[inline(always)]
    pub fn ch0sreqstatus(&self) -> CH0SREQSTATUS_R {
        CH0SREQSTATUS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Single Request Status"]
    #[inline(always)]
    pub fn ch1sreqstatus(&self) -> CH1SREQSTATUS_R {
        CH1SREQSTATUS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Single Request Status"]
    #[inline(always)]
    pub fn ch2sreqstatus(&self) -> CH2SREQSTATUS_R {
        CH2SREQSTATUS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Single Request Status"]
    #[inline(always)]
    pub fn ch3sreqstatus(&self) -> CH3SREQSTATUS_R {
        CH3SREQSTATUS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Channel 4 Single Request Status"]
    #[inline(always)]
    pub fn ch4sreqstatus(&self) -> CH4SREQSTATUS_R {
        CH4SREQSTATUS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Channel 5 Single Request Status"]
    #[inline(always)]
    pub fn ch5sreqstatus(&self) -> CH5SREQSTATUS_R {
        CH5SREQSTATUS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}

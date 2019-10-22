#[doc = "Reader of register PART"]
pub type R = crate::R<u32, super::PART>;
#[doc = "Reader of field `DEVICE_FAMILY`"]
pub type DEVICE_FAMILY_R = crate::R<u8, u8>;
#[doc = "Reader of field `PROD_REV`"]
pub type PROD_REV_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 16:23 - Device Family, 0x47 for Gecko"]
    #[inline(always)]
    pub fn device_family(&self) -> DEVICE_FAMILY_R {
        DEVICE_FAMILY_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Production revision"]
    #[inline(always)]
    pub fn prod_rev(&self) -> PROD_REV_R {
        PROD_REV_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}

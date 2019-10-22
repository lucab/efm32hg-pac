#[doc = "Reader of register CAL"]
pub type R = crate::R<u32, super::CAL>;
#[doc = "Reader of field `CRC`"]
pub type CRC_R = crate::R<u16, u16>;
#[doc = "Reader of field `TEMP`"]
pub type TEMP_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:15 - Integrity CRC checksum"]
    #[inline(always)]
    pub fn crc(&self) -> CRC_R {
        CRC_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Calibration temperature, DegC"]
    #[inline(always)]
    pub fn temp(&self) -> TEMP_R {
        TEMP_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}

#[doc = "Reader of register MSIZE"]
pub type R = crate::R<u32, super::MSIZE>;
#[doc = "Reader of field `FLASH`"]
pub type FLASH_R = crate::R<u16, u16>;
#[doc = "Reader of field `SRAM`"]
pub type SRAM_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - SRAM size in kilobytes"]
    #[inline(always)]
    pub fn flash(&self) -> FLASH_R {
        FLASH_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Flash size in kilobytes"]
    #[inline(always)]
    pub fn sram(&self) -> SRAM_R {
        SRAM_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}

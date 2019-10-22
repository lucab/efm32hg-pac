#[doc = "Reader of register MEMINFO"]
pub type R = crate::R<u32, super::MEMINFO>;
#[doc = "Reader of field `FLASH_PAGE_SIZE`"]
pub type FLASH_PAGE_SIZE_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 24:31 - Flash page size (refer to ref.man for encoding)"]
    #[inline(always)]
    pub fn flash_page_size(&self) -> FLASH_PAGE_SIZE_R {
        FLASH_PAGE_SIZE_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}

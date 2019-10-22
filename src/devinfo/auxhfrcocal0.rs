#[doc = "Reader of register AUXHFRCOCAL0"]
pub type R = crate::R<u32, super::AUXHFRCOCAL0>;
#[doc = "Reader of field `BAND1`"]
pub type BAND1_R = crate::R<u8, u8>;
#[doc = "Reader of field `BAND7`"]
pub type BAND7_R = crate::R<u8, u8>;
#[doc = "Reader of field `BAND11`"]
pub type BAND11_R = crate::R<u8, u8>;
#[doc = "Reader of field `BAND14`"]
pub type BAND14_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - 1MHz tuning value for AUXHFRCO"]
    #[inline(always)]
    pub fn band1(&self) -> BAND1_R {
        BAND1_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 7MHz tuning value for AUXHFRCO"]
    #[inline(always)]
    pub fn band7(&self) -> BAND7_R {
        BAND7_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 11MHz tuning value for AUXHFRCO"]
    #[inline(always)]
    pub fn band11(&self) -> BAND11_R {
        BAND11_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 14MHz tuning value for AUXHFRCO"]
    #[inline(always)]
    pub fn band14(&self) -> BAND14_R {
        BAND14_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}

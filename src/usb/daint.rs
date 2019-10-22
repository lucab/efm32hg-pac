#[doc = "Reader of register DAINT"]
pub type R = crate::R<u32, super::DAINT>;
#[doc = "Reader of field `INEPINT0`"]
pub type INEPINT0_R = crate::R<bool, bool>;
#[doc = "Reader of field `INEPINT1`"]
pub type INEPINT1_R = crate::R<bool, bool>;
#[doc = "Reader of field `INEPINT2`"]
pub type INEPINT2_R = crate::R<bool, bool>;
#[doc = "Reader of field `INEPINT3`"]
pub type INEPINT3_R = crate::R<bool, bool>;
#[doc = "Reader of field `OUTEPINT0`"]
pub type OUTEPINT0_R = crate::R<bool, bool>;
#[doc = "Reader of field `OUTEPINT1`"]
pub type OUTEPINT1_R = crate::R<bool, bool>;
#[doc = "Reader of field `OUTEPINT2`"]
pub type OUTEPINT2_R = crate::R<bool, bool>;
#[doc = "Reader of field `OUTEPINT3`"]
pub type OUTEPINT3_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - IN Endpoint 0 Interrupt Bit"]
    #[inline(always)]
    pub fn inepint0(&self) -> INEPINT0_R {
        INEPINT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - IN Endpoint 1 Interrupt Bit"]
    #[inline(always)]
    pub fn inepint1(&self) -> INEPINT1_R {
        INEPINT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - IN Endpoint 2 Interrupt Bit"]
    #[inline(always)]
    pub fn inepint2(&self) -> INEPINT2_R {
        INEPINT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - IN Endpoint 3 Interrupt Bit"]
    #[inline(always)]
    pub fn inepint3(&self) -> INEPINT3_R {
        INEPINT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 16 - OUT Endpoint 0 Interrupt Bit"]
    #[inline(always)]
    pub fn outepint0(&self) -> OUTEPINT0_R {
        OUTEPINT0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - OUT Endpoint 1 Interrupt Bit"]
    #[inline(always)]
    pub fn outepint1(&self) -> OUTEPINT1_R {
        OUTEPINT1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - OUT Endpoint 2 Interrupt Bit"]
    #[inline(always)]
    pub fn outepint2(&self) -> OUTEPINT2_R {
        OUTEPINT2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - OUT Endpoint 3 Interrupt Bit"]
    #[inline(always)]
    pub fn outepint3(&self) -> OUTEPINT3_R {
        OUTEPINT3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}

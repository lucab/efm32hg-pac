#[doc = "Reader of register RSTCAUSE"]
pub type R = crate::R<u32, super::RSTCAUSE>;
#[doc = "Reader of field `PORST`"]
pub type PORST_R = crate::R<bool, bool>;
#[doc = "Reader of field `BODUNREGRST`"]
pub type BODUNREGRST_R = crate::R<bool, bool>;
#[doc = "Reader of field `BODREGRST`"]
pub type BODREGRST_R = crate::R<bool, bool>;
#[doc = "Reader of field `EXTRST`"]
pub type EXTRST_R = crate::R<bool, bool>;
#[doc = "Reader of field `WDOGRST`"]
pub type WDOGRST_R = crate::R<bool, bool>;
#[doc = "Reader of field `LOCKUPRST`"]
pub type LOCKUPRST_R = crate::R<bool, bool>;
#[doc = "Reader of field `SYSREQRST`"]
pub type SYSREQRST_R = crate::R<bool, bool>;
#[doc = "Reader of field `EM4RST`"]
pub type EM4RST_R = crate::R<bool, bool>;
#[doc = "Reader of field `EM4WURST`"]
pub type EM4WURST_R = crate::R<bool, bool>;
#[doc = "Reader of field `BODAVDD0`"]
pub type BODAVDD0_R = crate::R<bool, bool>;
#[doc = "Reader of field `BODAVDD1`"]
pub type BODAVDD1_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Power On Reset"]
    #[inline(always)]
    pub fn porst(&self) -> PORST_R {
        PORST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Brown Out Detector Unregulated Domain Reset"]
    #[inline(always)]
    pub fn bodunregrst(&self) -> BODUNREGRST_R {
        BODUNREGRST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Brown Out Detector Regulated Domain Reset"]
    #[inline(always)]
    pub fn bodregrst(&self) -> BODREGRST_R {
        BODREGRST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - External Pin Reset"]
    #[inline(always)]
    pub fn extrst(&self) -> EXTRST_R {
        EXTRST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Watchdog Reset"]
    #[inline(always)]
    pub fn wdogrst(&self) -> WDOGRST_R {
        WDOGRST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - LOCKUP Reset"]
    #[inline(always)]
    pub fn lockuprst(&self) -> LOCKUPRST_R {
        LOCKUPRST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - System Request Reset"]
    #[inline(always)]
    pub fn sysreqrst(&self) -> SYSREQRST_R {
        SYSREQRST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - EM4 Reset"]
    #[inline(always)]
    pub fn em4rst(&self) -> EM4RST_R {
        EM4RST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - EM4 Wake-up Reset"]
    #[inline(always)]
    pub fn em4wurst(&self) -> EM4WURST_R {
        EM4WURST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - AVDD0 Bod Reset"]
    #[inline(always)]
    pub fn bodavdd0(&self) -> BODAVDD0_R {
        BODAVDD0_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - AVDD1 Bod Reset"]
    #[inline(always)]
    pub fn bodavdd1(&self) -> BODAVDD1_R {
        BODAVDD1_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}

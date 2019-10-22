#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Reader of field `HFRCOENS`"]
pub type HFRCOENS_R = crate::R<bool, bool>;
#[doc = "Reader of field `HFRCORDY`"]
pub type HFRCORDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `HFXOENS`"]
pub type HFXOENS_R = crate::R<bool, bool>;
#[doc = "Reader of field `HFXORDY`"]
pub type HFXORDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `AUXHFRCOENS`"]
pub type AUXHFRCOENS_R = crate::R<bool, bool>;
#[doc = "Reader of field `AUXHFRCORDY`"]
pub type AUXHFRCORDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `LFRCOENS`"]
pub type LFRCOENS_R = crate::R<bool, bool>;
#[doc = "Reader of field `LFRCORDY`"]
pub type LFRCORDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `LFXOENS`"]
pub type LFXOENS_R = crate::R<bool, bool>;
#[doc = "Reader of field `LFXORDY`"]
pub type LFXORDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `HFRCOSEL`"]
pub type HFRCOSEL_R = crate::R<bool, bool>;
#[doc = "Reader of field `HFXOSEL`"]
pub type HFXOSEL_R = crate::R<bool, bool>;
#[doc = "Reader of field `LFRCOSEL`"]
pub type LFRCOSEL_R = crate::R<bool, bool>;
#[doc = "Reader of field `LFXOSEL`"]
pub type LFXOSEL_R = crate::R<bool, bool>;
#[doc = "Reader of field `CALBSY`"]
pub type CALBSY_R = crate::R<bool, bool>;
#[doc = "Reader of field `USBCLFXOSEL`"]
pub type USBCLFXOSEL_R = crate::R<bool, bool>;
#[doc = "Reader of field `USBCLFRCOSEL`"]
pub type USBCLFRCOSEL_R = crate::R<bool, bool>;
#[doc = "Reader of field `USBCUSHFRCOSEL`"]
pub type USBCUSHFRCOSEL_R = crate::R<bool, bool>;
#[doc = "Reader of field `USBCHFCLKSYNC`"]
pub type USBCHFCLKSYNC_R = crate::R<bool, bool>;
#[doc = "Reader of field `USHFRCOENS`"]
pub type USHFRCOENS_R = crate::R<bool, bool>;
#[doc = "Reader of field `USHFRCORDY`"]
pub type USHFRCORDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `USHFRCOSUSPEND`"]
pub type USHFRCOSUSPEND_R = crate::R<bool, bool>;
#[doc = "Reader of field `USHFRCODIV2SEL`"]
pub type USHFRCODIV2SEL_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - HFRCO Enable Status"]
    #[inline(always)]
    pub fn hfrcoens(&self) -> HFRCOENS_R {
        HFRCOENS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - HFRCO Ready"]
    #[inline(always)]
    pub fn hfrcordy(&self) -> HFRCORDY_R {
        HFRCORDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - HFXO Enable Status"]
    #[inline(always)]
    pub fn hfxoens(&self) -> HFXOENS_R {
        HFXOENS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - HFXO Ready"]
    #[inline(always)]
    pub fn hfxordy(&self) -> HFXORDY_R {
        HFXORDY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - AUXHFRCO Enable Status"]
    #[inline(always)]
    pub fn auxhfrcoens(&self) -> AUXHFRCOENS_R {
        AUXHFRCOENS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - AUXHFRCO Ready"]
    #[inline(always)]
    pub fn auxhfrcordy(&self) -> AUXHFRCORDY_R {
        AUXHFRCORDY_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - LFRCO Enable Status"]
    #[inline(always)]
    pub fn lfrcoens(&self) -> LFRCOENS_R {
        LFRCOENS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - LFRCO Ready"]
    #[inline(always)]
    pub fn lfrcordy(&self) -> LFRCORDY_R {
        LFRCORDY_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - LFXO Enable Status"]
    #[inline(always)]
    pub fn lfxoens(&self) -> LFXOENS_R {
        LFXOENS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - LFXO Ready"]
    #[inline(always)]
    pub fn lfxordy(&self) -> LFXORDY_R {
        LFXORDY_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - HFRCO Selected"]
    #[inline(always)]
    pub fn hfrcosel(&self) -> HFRCOSEL_R {
        HFRCOSEL_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - HFXO Selected"]
    #[inline(always)]
    pub fn hfxosel(&self) -> HFXOSEL_R {
        HFXOSEL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - LFRCO Selected"]
    #[inline(always)]
    pub fn lfrcosel(&self) -> LFRCOSEL_R {
        LFRCOSEL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - LFXO Selected"]
    #[inline(always)]
    pub fn lfxosel(&self) -> LFXOSEL_R {
        LFXOSEL_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Calibration Busy"]
    #[inline(always)]
    pub fn calbsy(&self) -> CALBSY_R {
        CALBSY_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - USBC LFXO Selected"]
    #[inline(always)]
    pub fn usbclfxosel(&self) -> USBCLFXOSEL_R {
        USBCLFXOSEL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - USBC LFRCO Selected"]
    #[inline(always)]
    pub fn usbclfrcosel(&self) -> USBCLFRCOSEL_R {
        USBCLFRCOSEL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - USBC USHFRCO Selected"]
    #[inline(always)]
    pub fn usbcushfrcosel(&self) -> USBCUSHFRCOSEL_R {
        USBCUSHFRCOSEL_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 20 - USBC is synchronous to HFCLK"]
    #[inline(always)]
    pub fn usbchfclksync(&self) -> USBCHFCLKSYNC_R {
        USBCHFCLKSYNC_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - USHFRCO Enable Status"]
    #[inline(always)]
    pub fn ushfrcoens(&self) -> USHFRCOENS_R {
        USHFRCOENS_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - USHFRCO Ready"]
    #[inline(always)]
    pub fn ushfrcordy(&self) -> USHFRCORDY_R {
        USHFRCORDY_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - USHFRCO is suspended"]
    #[inline(always)]
    pub fn ushfrcosuspend(&self) -> USHFRCOSUSPEND_R {
        USHFRCOSUSPEND_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 26 - USHFRCODIV2 Selected"]
    #[inline(always)]
    pub fn ushfrcodiv2sel(&self) -> USHFRCODIV2SEL_R {
        USHFRCODIV2SEL_R::new(((self.bits >> 26) & 0x01) != 0)
    }
}

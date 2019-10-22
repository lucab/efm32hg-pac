#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CMU Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - High Frequency Core Clock Division Register"]
    pub hfcoreclkdiv: HFCORECLKDIV,
    #[doc = "0x08 - High Frequency Peripheral Clock Division Register"]
    pub hfperclkdiv: HFPERCLKDIV,
    #[doc = "0x0c - HFRCO Control Register"]
    pub hfrcoctrl: HFRCOCTRL,
    #[doc = "0x10 - LFRCO Control Register"]
    pub lfrcoctrl: LFRCOCTRL,
    #[doc = "0x14 - AUXHFRCO Control Register"]
    pub auxhfrcoctrl: AUXHFRCOCTRL,
    #[doc = "0x18 - Calibration Control Register"]
    pub calctrl: CALCTRL,
    #[doc = "0x1c - Calibration Counter Register"]
    pub calcnt: CALCNT,
    #[doc = "0x20 - Oscillator Enable/Disable Command Register"]
    pub oscencmd: OSCENCMD,
    #[doc = "0x24 - Command Register"]
    pub cmd: CMD,
    #[doc = "0x28 - Low Frequency Clock Select Register"]
    pub lfclksel: LFCLKSEL,
    #[doc = "0x2c - Status Register"]
    pub status: STATUS,
    #[doc = "0x30 - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x34 - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x38 - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x3c - Interrupt Enable Register"]
    pub ien: IEN,
    #[doc = "0x40 - High Frequency Core Clock Enable Register 0"]
    pub hfcoreclken0: HFCORECLKEN0,
    #[doc = "0x44 - High Frequency Peripheral Clock Enable Register 0"]
    pub hfperclken0: HFPERCLKEN0,
    _reserved18: [u8; 8usize],
    #[doc = "0x50 - Synchronization Busy Register"]
    pub syncbusy: SYNCBUSY,
    #[doc = "0x54 - Freeze Register"]
    pub freeze: FREEZE,
    #[doc = "0x58 - Low Frequency A Clock Enable Register 0 (Async Reg)"]
    pub lfaclken0: LFACLKEN0,
    _reserved21: [u8; 4usize],
    #[doc = "0x60 - Low Frequency B Clock Enable Register 0 (Async Reg)"]
    pub lfbclken0: LFBCLKEN0,
    #[doc = "0x64 - Low Frequency C Clock Enable Register 0 (Async Reg)"]
    pub lfcclken0: LFCCLKEN0,
    #[doc = "0x68 - Low Frequency A Prescaler Register 0 (Async Reg)"]
    pub lfapresc0: LFAPRESC0,
    _reserved24: [u8; 4usize],
    #[doc = "0x70 - Low Frequency B Prescaler Register 0 (Async Reg)"]
    pub lfbpresc0: LFBPRESC0,
    _reserved25: [u8; 4usize],
    #[doc = "0x78 - PCNT Control Register"]
    pub pcntctrl: PCNTCTRL,
    _reserved26: [u8; 4usize],
    #[doc = "0x80 - I/O Routing Register"]
    pub route: ROUTE,
    #[doc = "0x84 - Configuration Lock Register"]
    pub lock: LOCK,
    _reserved28: [u8; 72usize],
    #[doc = "0xd0 - USB Clock Recovery Control"]
    pub usbcrctrl: USBCRCTRL,
    #[doc = "0xd4 - USHFRCO Control"]
    pub ushfrcoctrl: USHFRCOCTRL,
    #[doc = "0xd8 - USHFRCO Frequency Tune"]
    pub ushfrcotune: USHFRCOTUNE,
    #[doc = "0xdc - USHFRCO Configuration"]
    pub ushfrcoconf: USHFRCOCONF,
}
#[doc = "CMU Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "CMU Control Register"]
pub mod ctrl;
#[doc = "High Frequency Core Clock Division Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hfcoreclkdiv](hfcoreclkdiv) module"]
pub type HFCORECLKDIV = crate::Reg<u32, _HFCORECLKDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HFCORECLKDIV;
#[doc = "`read()` method returns [hfcoreclkdiv::R](hfcoreclkdiv::R) reader structure"]
impl crate::Readable for HFCORECLKDIV {}
#[doc = "`write(|w| ..)` method takes [hfcoreclkdiv::W](hfcoreclkdiv::W) writer structure"]
impl crate::Writable for HFCORECLKDIV {}
#[doc = "High Frequency Core Clock Division Register"]
pub mod hfcoreclkdiv;
#[doc = "High Frequency Peripheral Clock Division Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hfperclkdiv](hfperclkdiv) module"]
pub type HFPERCLKDIV = crate::Reg<u32, _HFPERCLKDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HFPERCLKDIV;
#[doc = "`read()` method returns [hfperclkdiv::R](hfperclkdiv::R) reader structure"]
impl crate::Readable for HFPERCLKDIV {}
#[doc = "`write(|w| ..)` method takes [hfperclkdiv::W](hfperclkdiv::W) writer structure"]
impl crate::Writable for HFPERCLKDIV {}
#[doc = "High Frequency Peripheral Clock Division Register"]
pub mod hfperclkdiv;
#[doc = "HFRCO Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hfrcoctrl](hfrcoctrl) module"]
pub type HFRCOCTRL = crate::Reg<u32, _HFRCOCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HFRCOCTRL;
#[doc = "`read()` method returns [hfrcoctrl::R](hfrcoctrl::R) reader structure"]
impl crate::Readable for HFRCOCTRL {}
#[doc = "`write(|w| ..)` method takes [hfrcoctrl::W](hfrcoctrl::W) writer structure"]
impl crate::Writable for HFRCOCTRL {}
#[doc = "HFRCO Control Register"]
pub mod hfrcoctrl;
#[doc = "LFRCO Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lfrcoctrl](lfrcoctrl) module"]
pub type LFRCOCTRL = crate::Reg<u32, _LFRCOCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LFRCOCTRL;
#[doc = "`read()` method returns [lfrcoctrl::R](lfrcoctrl::R) reader structure"]
impl crate::Readable for LFRCOCTRL {}
#[doc = "`write(|w| ..)` method takes [lfrcoctrl::W](lfrcoctrl::W) writer structure"]
impl crate::Writable for LFRCOCTRL {}
#[doc = "LFRCO Control Register"]
pub mod lfrcoctrl;
#[doc = "AUXHFRCO Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [auxhfrcoctrl](auxhfrcoctrl) module"]
pub type AUXHFRCOCTRL = crate::Reg<u32, _AUXHFRCOCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AUXHFRCOCTRL;
#[doc = "`read()` method returns [auxhfrcoctrl::R](auxhfrcoctrl::R) reader structure"]
impl crate::Readable for AUXHFRCOCTRL {}
#[doc = "`write(|w| ..)` method takes [auxhfrcoctrl::W](auxhfrcoctrl::W) writer structure"]
impl crate::Writable for AUXHFRCOCTRL {}
#[doc = "AUXHFRCO Control Register"]
pub mod auxhfrcoctrl;
#[doc = "Calibration Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [calctrl](calctrl) module"]
pub type CALCTRL = crate::Reg<u32, _CALCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CALCTRL;
#[doc = "`read()` method returns [calctrl::R](calctrl::R) reader structure"]
impl crate::Readable for CALCTRL {}
#[doc = "`write(|w| ..)` method takes [calctrl::W](calctrl::W) writer structure"]
impl crate::Writable for CALCTRL {}
#[doc = "Calibration Control Register"]
pub mod calctrl;
#[doc = "Calibration Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [calcnt](calcnt) module"]
pub type CALCNT = crate::Reg<u32, _CALCNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CALCNT;
#[doc = "`read()` method returns [calcnt::R](calcnt::R) reader structure"]
impl crate::Readable for CALCNT {}
#[doc = "`write(|w| ..)` method takes [calcnt::W](calcnt::W) writer structure"]
impl crate::Writable for CALCNT {}
#[doc = "Calibration Counter Register"]
pub mod calcnt;
#[doc = "Oscillator Enable/Disable Command Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [oscencmd](oscencmd) module"]
pub type OSCENCMD = crate::Reg<u32, _OSCENCMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSCENCMD;
#[doc = "`write(|w| ..)` method takes [oscencmd::W](oscencmd::W) writer structure"]
impl crate::Writable for OSCENCMD {}
#[doc = "Oscillator Enable/Disable Command Register"]
pub mod oscencmd;
#[doc = "Command Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cmd](cmd) module"]
pub type CMD = crate::Reg<u32, _CMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMD;
#[doc = "`write(|w| ..)` method takes [cmd::W](cmd::W) writer structure"]
impl crate::Writable for CMD {}
#[doc = "Command Register"]
pub mod cmd;
#[doc = "Low Frequency Clock Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lfclksel](lfclksel) module"]
pub type LFCLKSEL = crate::Reg<u32, _LFCLKSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LFCLKSEL;
#[doc = "`read()` method returns [lfclksel::R](lfclksel::R) reader structure"]
impl crate::Readable for LFCLKSEL {}
#[doc = "`write(|w| ..)` method takes [lfclksel::W](lfclksel::W) writer structure"]
impl crate::Writable for LFCLKSEL {}
#[doc = "Low Frequency Clock Select Register"]
pub mod lfclksel;
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "Status Register"]
pub mod status;
#[doc = "Interrupt Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [if_](if_) module"]
pub type IF = crate::Reg<u32, _IF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IF;
#[doc = "`read()` method returns [if_::R](if_::R) reader structure"]
impl crate::Readable for IF {}
#[doc = "Interrupt Flag Register"]
pub mod if_;
#[doc = "Interrupt Flag Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ifs](ifs) module"]
pub type IFS = crate::Reg<u32, _IFS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IFS;
#[doc = "`write(|w| ..)` method takes [ifs::W](ifs::W) writer structure"]
impl crate::Writable for IFS {}
#[doc = "Interrupt Flag Set Register"]
pub mod ifs;
#[doc = "Interrupt Flag Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ifc](ifc) module"]
pub type IFC = crate::Reg<u32, _IFC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IFC;
#[doc = "`write(|w| ..)` method takes [ifc::W](ifc::W) writer structure"]
impl crate::Writable for IFC {}
#[doc = "Interrupt Flag Clear Register"]
pub mod ifc;
#[doc = "Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ien](ien) module"]
pub type IEN = crate::Reg<u32, _IEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IEN;
#[doc = "`read()` method returns [ien::R](ien::R) reader structure"]
impl crate::Readable for IEN {}
#[doc = "`write(|w| ..)` method takes [ien::W](ien::W) writer structure"]
impl crate::Writable for IEN {}
#[doc = "Interrupt Enable Register"]
pub mod ien;
#[doc = "High Frequency Core Clock Enable Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hfcoreclken0](hfcoreclken0) module"]
pub type HFCORECLKEN0 = crate::Reg<u32, _HFCORECLKEN0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HFCORECLKEN0;
#[doc = "`read()` method returns [hfcoreclken0::R](hfcoreclken0::R) reader structure"]
impl crate::Readable for HFCORECLKEN0 {}
#[doc = "`write(|w| ..)` method takes [hfcoreclken0::W](hfcoreclken0::W) writer structure"]
impl crate::Writable for HFCORECLKEN0 {}
#[doc = "High Frequency Core Clock Enable Register 0"]
pub mod hfcoreclken0;
#[doc = "High Frequency Peripheral Clock Enable Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hfperclken0](hfperclken0) module"]
pub type HFPERCLKEN0 = crate::Reg<u32, _HFPERCLKEN0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HFPERCLKEN0;
#[doc = "`read()` method returns [hfperclken0::R](hfperclken0::R) reader structure"]
impl crate::Readable for HFPERCLKEN0 {}
#[doc = "`write(|w| ..)` method takes [hfperclken0::W](hfperclken0::W) writer structure"]
impl crate::Writable for HFPERCLKEN0 {}
#[doc = "High Frequency Peripheral Clock Enable Register 0"]
pub mod hfperclken0;
#[doc = "Synchronization Busy Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [syncbusy](syncbusy) module"]
pub type SYNCBUSY = crate::Reg<u32, _SYNCBUSY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYNCBUSY;
#[doc = "`read()` method returns [syncbusy::R](syncbusy::R) reader structure"]
impl crate::Readable for SYNCBUSY {}
#[doc = "Synchronization Busy Register"]
pub mod syncbusy;
#[doc = "Freeze Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [freeze](freeze) module"]
pub type FREEZE = crate::Reg<u32, _FREEZE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FREEZE;
#[doc = "`read()` method returns [freeze::R](freeze::R) reader structure"]
impl crate::Readable for FREEZE {}
#[doc = "`write(|w| ..)` method takes [freeze::W](freeze::W) writer structure"]
impl crate::Writable for FREEZE {}
#[doc = "Freeze Register"]
pub mod freeze;
#[doc = "Low Frequency A Clock Enable Register 0 (Async Reg)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lfaclken0](lfaclken0) module"]
pub type LFACLKEN0 = crate::Reg<u32, _LFACLKEN0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LFACLKEN0;
#[doc = "`read()` method returns [lfaclken0::R](lfaclken0::R) reader structure"]
impl crate::Readable for LFACLKEN0 {}
#[doc = "`write(|w| ..)` method takes [lfaclken0::W](lfaclken0::W) writer structure"]
impl crate::Writable for LFACLKEN0 {}
#[doc = "Low Frequency A Clock Enable Register 0 (Async Reg)"]
pub mod lfaclken0;
#[doc = "Low Frequency B Clock Enable Register 0 (Async Reg)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lfbclken0](lfbclken0) module"]
pub type LFBCLKEN0 = crate::Reg<u32, _LFBCLKEN0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LFBCLKEN0;
#[doc = "`read()` method returns [lfbclken0::R](lfbclken0::R) reader structure"]
impl crate::Readable for LFBCLKEN0 {}
#[doc = "`write(|w| ..)` method takes [lfbclken0::W](lfbclken0::W) writer structure"]
impl crate::Writable for LFBCLKEN0 {}
#[doc = "Low Frequency B Clock Enable Register 0 (Async Reg)"]
pub mod lfbclken0;
#[doc = "Low Frequency C Clock Enable Register 0 (Async Reg)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lfcclken0](lfcclken0) module"]
pub type LFCCLKEN0 = crate::Reg<u32, _LFCCLKEN0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LFCCLKEN0;
#[doc = "`read()` method returns [lfcclken0::R](lfcclken0::R) reader structure"]
impl crate::Readable for LFCCLKEN0 {}
#[doc = "`write(|w| ..)` method takes [lfcclken0::W](lfcclken0::W) writer structure"]
impl crate::Writable for LFCCLKEN0 {}
#[doc = "Low Frequency C Clock Enable Register 0 (Async Reg)"]
pub mod lfcclken0;
#[doc = "Low Frequency A Prescaler Register 0 (Async Reg)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lfapresc0](lfapresc0) module"]
pub type LFAPRESC0 = crate::Reg<u32, _LFAPRESC0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LFAPRESC0;
#[doc = "`read()` method returns [lfapresc0::R](lfapresc0::R) reader structure"]
impl crate::Readable for LFAPRESC0 {}
#[doc = "`write(|w| ..)` method takes [lfapresc0::W](lfapresc0::W) writer structure"]
impl crate::Writable for LFAPRESC0 {}
#[doc = "Low Frequency A Prescaler Register 0 (Async Reg)"]
pub mod lfapresc0;
#[doc = "Low Frequency B Prescaler Register 0 (Async Reg)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lfbpresc0](lfbpresc0) module"]
pub type LFBPRESC0 = crate::Reg<u32, _LFBPRESC0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LFBPRESC0;
#[doc = "`read()` method returns [lfbpresc0::R](lfbpresc0::R) reader structure"]
impl crate::Readable for LFBPRESC0 {}
#[doc = "`write(|w| ..)` method takes [lfbpresc0::W](lfbpresc0::W) writer structure"]
impl crate::Writable for LFBPRESC0 {}
#[doc = "Low Frequency B Prescaler Register 0 (Async Reg)"]
pub mod lfbpresc0;
#[doc = "PCNT Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcntctrl](pcntctrl) module"]
pub type PCNTCTRL = crate::Reg<u32, _PCNTCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCNTCTRL;
#[doc = "`read()` method returns [pcntctrl::R](pcntctrl::R) reader structure"]
impl crate::Readable for PCNTCTRL {}
#[doc = "`write(|w| ..)` method takes [pcntctrl::W](pcntctrl::W) writer structure"]
impl crate::Writable for PCNTCTRL {}
#[doc = "PCNT Control Register"]
pub mod pcntctrl;
#[doc = "I/O Routing Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [route](route) module"]
pub type ROUTE = crate::Reg<u32, _ROUTE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ROUTE;
#[doc = "`read()` method returns [route::R](route::R) reader structure"]
impl crate::Readable for ROUTE {}
#[doc = "`write(|w| ..)` method takes [route::W](route::W) writer structure"]
impl crate::Writable for ROUTE {}
#[doc = "I/O Routing Register"]
pub mod route;
#[doc = "Configuration Lock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lock](lock) module"]
pub type LOCK = crate::Reg<u32, _LOCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LOCK;
#[doc = "`read()` method returns [lock::R](lock::R) reader structure"]
impl crate::Readable for LOCK {}
#[doc = "`write(|w| ..)` method takes [lock::W](lock::W) writer structure"]
impl crate::Writable for LOCK {}
#[doc = "Configuration Lock Register"]
pub mod lock;
#[doc = "USB Clock Recovery Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [usbcrctrl](usbcrctrl) module"]
pub type USBCRCTRL = crate::Reg<u32, _USBCRCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBCRCTRL;
#[doc = "`read()` method returns [usbcrctrl::R](usbcrctrl::R) reader structure"]
impl crate::Readable for USBCRCTRL {}
#[doc = "`write(|w| ..)` method takes [usbcrctrl::W](usbcrctrl::W) writer structure"]
impl crate::Writable for USBCRCTRL {}
#[doc = "USB Clock Recovery Control"]
pub mod usbcrctrl;
#[doc = "USHFRCO Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ushfrcoctrl](ushfrcoctrl) module"]
pub type USHFRCOCTRL = crate::Reg<u32, _USHFRCOCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USHFRCOCTRL;
#[doc = "`read()` method returns [ushfrcoctrl::R](ushfrcoctrl::R) reader structure"]
impl crate::Readable for USHFRCOCTRL {}
#[doc = "`write(|w| ..)` method takes [ushfrcoctrl::W](ushfrcoctrl::W) writer structure"]
impl crate::Writable for USHFRCOCTRL {}
#[doc = "USHFRCO Control"]
pub mod ushfrcoctrl;
#[doc = "USHFRCO Frequency Tune\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ushfrcotune](ushfrcotune) module"]
pub type USHFRCOTUNE = crate::Reg<u32, _USHFRCOTUNE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USHFRCOTUNE;
#[doc = "`read()` method returns [ushfrcotune::R](ushfrcotune::R) reader structure"]
impl crate::Readable for USHFRCOTUNE {}
#[doc = "`write(|w| ..)` method takes [ushfrcotune::W](ushfrcotune::W) writer structure"]
impl crate::Writable for USHFRCOTUNE {}
#[doc = "USHFRCO Frequency Tune"]
pub mod ushfrcotune;
#[doc = "USHFRCO Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ushfrcoconf](ushfrcoconf) module"]
pub type USHFRCOCONF = crate::Reg<u32, _USHFRCOCONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USHFRCOCONF;
#[doc = "`read()` method returns [ushfrcoconf::R](ushfrcoconf::R) reader structure"]
impl crate::Readable for USHFRCOCONF {}
#[doc = "`write(|w| ..)` method takes [ushfrcoconf::W](ushfrcoconf::W) writer structure"]
impl crate::Writable for USHFRCOCONF {}
#[doc = "USHFRCO Configuration"]
pub mod ushfrcoconf;

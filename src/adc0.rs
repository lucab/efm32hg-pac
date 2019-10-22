#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Command Register"]
    pub cmd: CMD,
    #[doc = "0x08 - Status Register"]
    pub status: STATUS,
    #[doc = "0x0c - Single Sample Control Register"]
    pub singlectrl: SINGLECTRL,
    #[doc = "0x10 - Scan Control Register"]
    pub scanctrl: SCANCTRL,
    #[doc = "0x14 - Interrupt Enable Register"]
    pub ien: IEN,
    #[doc = "0x18 - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x1c - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x20 - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x24 - Single Conversion Result Data"]
    pub singledata: SINGLEDATA,
    #[doc = "0x28 - Scan Conversion Result Data"]
    pub scandata: SCANDATA,
    #[doc = "0x2c - Single Conversion Result Data Peek Register"]
    pub singledatap: SINGLEDATAP,
    #[doc = "0x30 - Scan Sequence Result Data Peek Register"]
    pub scandatap: SCANDATAP,
    #[doc = "0x34 - Calibration Register"]
    pub cal: CAL,
    _reserved14: [u8; 4usize],
    #[doc = "0x3c - Bias Programming Register"]
    pub biasprog: BIASPROG,
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "Command Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cmd](cmd) module"]
pub type CMD = crate::Reg<u32, _CMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMD;
#[doc = "`write(|w| ..)` method takes [cmd::W](cmd::W) writer structure"]
impl crate::Writable for CMD {}
#[doc = "Command Register"]
pub mod cmd;
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "Status Register"]
pub mod status;
#[doc = "Single Sample Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [singlectrl](singlectrl) module"]
pub type SINGLECTRL = crate::Reg<u32, _SINGLECTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SINGLECTRL;
#[doc = "`read()` method returns [singlectrl::R](singlectrl::R) reader structure"]
impl crate::Readable for SINGLECTRL {}
#[doc = "`write(|w| ..)` method takes [singlectrl::W](singlectrl::W) writer structure"]
impl crate::Writable for SINGLECTRL {}
#[doc = "Single Sample Control Register"]
pub mod singlectrl;
#[doc = "Scan Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [scanctrl](scanctrl) module"]
pub type SCANCTRL = crate::Reg<u32, _SCANCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCANCTRL;
#[doc = "`read()` method returns [scanctrl::R](scanctrl::R) reader structure"]
impl crate::Readable for SCANCTRL {}
#[doc = "`write(|w| ..)` method takes [scanctrl::W](scanctrl::W) writer structure"]
impl crate::Writable for SCANCTRL {}
#[doc = "Scan Control Register"]
pub mod scanctrl;
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
#[doc = "Single Conversion Result Data\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [singledata](singledata) module"]
pub type SINGLEDATA = crate::Reg<u32, _SINGLEDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SINGLEDATA;
#[doc = "`read()` method returns [singledata::R](singledata::R) reader structure"]
impl crate::Readable for SINGLEDATA {}
#[doc = "Single Conversion Result Data"]
pub mod singledata;
#[doc = "Scan Conversion Result Data\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [scandata](scandata) module"]
pub type SCANDATA = crate::Reg<u32, _SCANDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCANDATA;
#[doc = "`read()` method returns [scandata::R](scandata::R) reader structure"]
impl crate::Readable for SCANDATA {}
#[doc = "Scan Conversion Result Data"]
pub mod scandata;
#[doc = "Single Conversion Result Data Peek Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [singledatap](singledatap) module"]
pub type SINGLEDATAP = crate::Reg<u32, _SINGLEDATAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SINGLEDATAP;
#[doc = "`read()` method returns [singledatap::R](singledatap::R) reader structure"]
impl crate::Readable for SINGLEDATAP {}
#[doc = "Single Conversion Result Data Peek Register"]
pub mod singledatap;
#[doc = "Scan Sequence Result Data Peek Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [scandatap](scandatap) module"]
pub type SCANDATAP = crate::Reg<u32, _SCANDATAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCANDATAP;
#[doc = "`read()` method returns [scandatap::R](scandatap::R) reader structure"]
impl crate::Readable for SCANDATAP {}
#[doc = "Scan Sequence Result Data Peek Register"]
pub mod scandatap;
#[doc = "Calibration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cal](cal) module"]
pub type CAL = crate::Reg<u32, _CAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAL;
#[doc = "`read()` method returns [cal::R](cal::R) reader structure"]
impl crate::Readable for CAL {}
#[doc = "`write(|w| ..)` method takes [cal::W](cal::W) writer structure"]
impl crate::Writable for CAL {}
#[doc = "Calibration Register"]
pub mod cal;
#[doc = "Bias Programming Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [biasprog](biasprog) module"]
pub type BIASPROG = crate::Reg<u32, _BIASPROG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BIASPROG;
#[doc = "`read()` method returns [biasprog::R](biasprog::R) reader structure"]
impl crate::Readable for BIASPROG {}
#[doc = "`write(|w| ..)` method takes [biasprog::W](biasprog::W) writer structure"]
impl crate::Writable for BIASPROG {}
#[doc = "Bias Programming Register"]
pub mod biasprog;

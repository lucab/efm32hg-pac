#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA Status Registers"]
    pub status: STATUS,
    #[doc = "0x04 - DMA Configuration Register"]
    pub config: CONFIG,
    #[doc = "0x08 - Channel Control Data Base Pointer Register"]
    pub ctrlbase: CTRLBASE,
    #[doc = "0x0c - Channel Alternate Control Data Base Pointer Register"]
    pub altctrlbase: ALTCTRLBASE,
    #[doc = "0x10 - Channel Wait on Request Status Register"]
    pub chwaitstatus: CHWAITSTATUS,
    #[doc = "0x14 - Channel Software Request Register"]
    pub chswreq: CHSWREQ,
    #[doc = "0x18 - Channel Useburst Set Register"]
    pub chusebursts: CHUSEBURSTS,
    #[doc = "0x1c - Channel Useburst Clear Register"]
    pub chuseburstc: CHUSEBURSTC,
    #[doc = "0x20 - Channel Request Mask Set Register"]
    pub chreqmasks: CHREQMASKS,
    #[doc = "0x24 - Channel Request Mask Clear Register"]
    pub chreqmaskc: CHREQMASKC,
    #[doc = "0x28 - Channel Enable Set Register"]
    pub chens: CHENS,
    #[doc = "0x2c - Channel Enable Clear Register"]
    pub chenc: CHENC,
    #[doc = "0x30 - Channel Alternate Set Register"]
    pub chalts: CHALTS,
    #[doc = "0x34 - Channel Alternate Clear Register"]
    pub chaltc: CHALTC,
    #[doc = "0x38 - Channel Priority Set Register"]
    pub chpris: CHPRIS,
    #[doc = "0x3c - Channel Priority Clear Register"]
    pub chpric: CHPRIC,
    _reserved16: [u8; 12usize],
    #[doc = "0x4c - Bus Error Clear Register"]
    pub errorc: ERRORC,
    _reserved17: [u8; 3520usize],
    #[doc = "0xe10 - Channel Request Status"]
    pub chreqstatus: CHREQSTATUS,
    _reserved18: [u8; 4usize],
    #[doc = "0xe18 - Channel Single Request Status"]
    pub chsreqstatus: CHSREQSTATUS,
    _reserved19: [u8; 484usize],
    #[doc = "0x1000 - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x1004 - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x1008 - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x100c - Interrupt Enable register"]
    pub ien: IEN,
    _reserved23: [u8; 240usize],
    #[doc = "0x1100 - Channel Control Register"]
    pub ch0_ctrl: CH0_CTRL,
    #[doc = "0x1104 - Channel Control Register"]
    pub ch1_ctrl: CH1_CTRL,
    #[doc = "0x1108 - Channel Control Register"]
    pub ch2_ctrl: CH2_CTRL,
    #[doc = "0x110c - Channel Control Register"]
    pub ch3_ctrl: CH3_CTRL,
    #[doc = "0x1110 - Channel Control Register"]
    pub ch4_ctrl: CH4_CTRL,
    #[doc = "0x1114 - Channel Control Register"]
    pub ch5_ctrl: CH5_CTRL,
}
#[doc = "DMA Status Registers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "DMA Status Registers"]
pub mod status;
#[doc = "DMA Configuration Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [config](config) module"]
pub type CONFIG = crate::Reg<u32, _CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIG;
#[doc = "`write(|w| ..)` method takes [config::W](config::W) writer structure"]
impl crate::Writable for CONFIG {}
#[doc = "DMA Configuration Register"]
pub mod config;
#[doc = "Channel Control Data Base Pointer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrlbase](ctrlbase) module"]
pub type CTRLBASE = crate::Reg<u32, _CTRLBASE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRLBASE;
#[doc = "`read()` method returns [ctrlbase::R](ctrlbase::R) reader structure"]
impl crate::Readable for CTRLBASE {}
#[doc = "`write(|w| ..)` method takes [ctrlbase::W](ctrlbase::W) writer structure"]
impl crate::Writable for CTRLBASE {}
#[doc = "Channel Control Data Base Pointer Register"]
pub mod ctrlbase;
#[doc = "Channel Alternate Control Data Base Pointer Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [altctrlbase](altctrlbase) module"]
pub type ALTCTRLBASE = crate::Reg<u32, _ALTCTRLBASE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALTCTRLBASE;
#[doc = "`read()` method returns [altctrlbase::R](altctrlbase::R) reader structure"]
impl crate::Readable for ALTCTRLBASE {}
#[doc = "Channel Alternate Control Data Base Pointer Register"]
pub mod altctrlbase;
#[doc = "Channel Wait on Request Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [chwaitstatus](chwaitstatus) module"]
pub type CHWAITSTATUS = crate::Reg<u32, _CHWAITSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHWAITSTATUS;
#[doc = "`read()` method returns [chwaitstatus::R](chwaitstatus::R) reader structure"]
impl crate::Readable for CHWAITSTATUS {}
#[doc = "Channel Wait on Request Status Register"]
pub mod chwaitstatus;
#[doc = "Channel Software Request Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [chswreq](chswreq) module"]
pub type CHSWREQ = crate::Reg<u32, _CHSWREQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHSWREQ;
#[doc = "`write(|w| ..)` method takes [chswreq::W](chswreq::W) writer structure"]
impl crate::Writable for CHSWREQ {}
#[doc = "Channel Software Request Register"]
pub mod chswreq;
#[doc = "Channel Useburst Set Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [chusebursts](chusebursts) module"]
pub type CHUSEBURSTS = crate::Reg<u32, _CHUSEBURSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHUSEBURSTS;
#[doc = "`read()` method returns [chusebursts::R](chusebursts::R) reader structure"]
impl crate::Readable for CHUSEBURSTS {}
#[doc = "`write(|w| ..)` method takes [chusebursts::W](chusebursts::W) writer structure"]
impl crate::Writable for CHUSEBURSTS {}
#[doc = "Channel Useburst Set Register"]
pub mod chusebursts;
#[doc = "Channel Useburst Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [chuseburstc](chuseburstc) module"]
pub type CHUSEBURSTC = crate::Reg<u32, _CHUSEBURSTC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHUSEBURSTC;
#[doc = "`write(|w| ..)` method takes [chuseburstc::W](chuseburstc::W) writer structure"]
impl crate::Writable for CHUSEBURSTC {}
#[doc = "Channel Useburst Clear Register"]
pub mod chuseburstc;
#[doc = "Channel Request Mask Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [chreqmasks](chreqmasks) module"]
pub type CHREQMASKS = crate::Reg<u32, _CHREQMASKS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHREQMASKS;
#[doc = "`write(|w| ..)` method takes [chreqmasks::W](chreqmasks::W) writer structure"]
impl crate::Writable for CHREQMASKS {}
#[doc = "Channel Request Mask Set Register"]
pub mod chreqmasks;
#[doc = "Channel Request Mask Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [chreqmaskc](chreqmaskc) module"]
pub type CHREQMASKC = crate::Reg<u32, _CHREQMASKC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHREQMASKC;
#[doc = "`write(|w| ..)` method takes [chreqmaskc::W](chreqmaskc::W) writer structure"]
impl crate::Writable for CHREQMASKC {}
#[doc = "Channel Request Mask Clear Register"]
pub mod chreqmaskc;
#[doc = "Channel Enable Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [chens](chens) module"]
pub type CHENS = crate::Reg<u32, _CHENS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHENS;
#[doc = "`write(|w| ..)` method takes [chens::W](chens::W) writer structure"]
impl crate::Writable for CHENS {}
#[doc = "Channel Enable Set Register"]
pub mod chens;
#[doc = "Channel Enable Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [chenc](chenc) module"]
pub type CHENC = crate::Reg<u32, _CHENC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHENC;
#[doc = "`write(|w| ..)` method takes [chenc::W](chenc::W) writer structure"]
impl crate::Writable for CHENC {}
#[doc = "Channel Enable Clear Register"]
pub mod chenc;
#[doc = "Channel Alternate Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [chalts](chalts) module"]
pub type CHALTS = crate::Reg<u32, _CHALTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHALTS;
#[doc = "`write(|w| ..)` method takes [chalts::W](chalts::W) writer structure"]
impl crate::Writable for CHALTS {}
#[doc = "Channel Alternate Set Register"]
pub mod chalts;
#[doc = "Channel Alternate Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [chaltc](chaltc) module"]
pub type CHALTC = crate::Reg<u32, _CHALTC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHALTC;
#[doc = "`write(|w| ..)` method takes [chaltc::W](chaltc::W) writer structure"]
impl crate::Writable for CHALTC {}
#[doc = "Channel Alternate Clear Register"]
pub mod chaltc;
#[doc = "Channel Priority Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [chpris](chpris) module"]
pub type CHPRIS = crate::Reg<u32, _CHPRIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHPRIS;
#[doc = "`write(|w| ..)` method takes [chpris::W](chpris::W) writer structure"]
impl crate::Writable for CHPRIS {}
#[doc = "Channel Priority Set Register"]
pub mod chpris;
#[doc = "Channel Priority Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [chpric](chpric) module"]
pub type CHPRIC = crate::Reg<u32, _CHPRIC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHPRIC;
#[doc = "`write(|w| ..)` method takes [chpric::W](chpric::W) writer structure"]
impl crate::Writable for CHPRIC {}
#[doc = "Channel Priority Clear Register"]
pub mod chpric;
#[doc = "Bus Error Clear Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [errorc](errorc) module"]
pub type ERRORC = crate::Reg<u32, _ERRORC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ERRORC;
#[doc = "`read()` method returns [errorc::R](errorc::R) reader structure"]
impl crate::Readable for ERRORC {}
#[doc = "`write(|w| ..)` method takes [errorc::W](errorc::W) writer structure"]
impl crate::Writable for ERRORC {}
#[doc = "Bus Error Clear Register"]
pub mod errorc;
#[doc = "Channel Request Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [chreqstatus](chreqstatus) module"]
pub type CHREQSTATUS = crate::Reg<u32, _CHREQSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHREQSTATUS;
#[doc = "`read()` method returns [chreqstatus::R](chreqstatus::R) reader structure"]
impl crate::Readable for CHREQSTATUS {}
#[doc = "Channel Request Status"]
pub mod chreqstatus;
#[doc = "Channel Single Request Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [chsreqstatus](chsreqstatus) module"]
pub type CHSREQSTATUS = crate::Reg<u32, _CHSREQSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHSREQSTATUS;
#[doc = "`read()` method returns [chsreqstatus::R](chsreqstatus::R) reader structure"]
impl crate::Readable for CHSREQSTATUS {}
#[doc = "Channel Single Request Status"]
pub mod chsreqstatus;
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
#[doc = "Interrupt Enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ien](ien) module"]
pub type IEN = crate::Reg<u32, _IEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IEN;
#[doc = "`read()` method returns [ien::R](ien::R) reader structure"]
impl crate::Readable for IEN {}
#[doc = "`write(|w| ..)` method takes [ien::W](ien::W) writer structure"]
impl crate::Writable for IEN {}
#[doc = "Interrupt Enable register"]
pub mod ien;
#[doc = "Channel Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ch0_ctrl](ch0_ctrl) module"]
pub type CH0_CTRL = crate::Reg<u32, _CH0_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0_CTRL;
#[doc = "`read()` method returns [ch0_ctrl::R](ch0_ctrl::R) reader structure"]
impl crate::Readable for CH0_CTRL {}
#[doc = "`write(|w| ..)` method takes [ch0_ctrl::W](ch0_ctrl::W) writer structure"]
impl crate::Writable for CH0_CTRL {}
#[doc = "Channel Control Register"]
pub mod ch0_ctrl;
#[doc = "Channel Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ch1_ctrl](ch1_ctrl) module"]
pub type CH1_CTRL = crate::Reg<u32, _CH1_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1_CTRL;
#[doc = "`read()` method returns [ch1_ctrl::R](ch1_ctrl::R) reader structure"]
impl crate::Readable for CH1_CTRL {}
#[doc = "`write(|w| ..)` method takes [ch1_ctrl::W](ch1_ctrl::W) writer structure"]
impl crate::Writable for CH1_CTRL {}
#[doc = "Channel Control Register"]
pub mod ch1_ctrl;
#[doc = "Channel Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ch2_ctrl](ch2_ctrl) module"]
pub type CH2_CTRL = crate::Reg<u32, _CH2_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2_CTRL;
#[doc = "`read()` method returns [ch2_ctrl::R](ch2_ctrl::R) reader structure"]
impl crate::Readable for CH2_CTRL {}
#[doc = "`write(|w| ..)` method takes [ch2_ctrl::W](ch2_ctrl::W) writer structure"]
impl crate::Writable for CH2_CTRL {}
#[doc = "Channel Control Register"]
pub mod ch2_ctrl;
#[doc = "Channel Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ch3_ctrl](ch3_ctrl) module"]
pub type CH3_CTRL = crate::Reg<u32, _CH3_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3_CTRL;
#[doc = "`read()` method returns [ch3_ctrl::R](ch3_ctrl::R) reader structure"]
impl crate::Readable for CH3_CTRL {}
#[doc = "`write(|w| ..)` method takes [ch3_ctrl::W](ch3_ctrl::W) writer structure"]
impl crate::Writable for CH3_CTRL {}
#[doc = "Channel Control Register"]
pub mod ch3_ctrl;
#[doc = "Channel Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ch4_ctrl](ch4_ctrl) module"]
pub type CH4_CTRL = crate::Reg<u32, _CH4_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH4_CTRL;
#[doc = "`read()` method returns [ch4_ctrl::R](ch4_ctrl::R) reader structure"]
impl crate::Readable for CH4_CTRL {}
#[doc = "`write(|w| ..)` method takes [ch4_ctrl::W](ch4_ctrl::W) writer structure"]
impl crate::Writable for CH4_CTRL {}
#[doc = "Channel Control Register"]
pub mod ch4_ctrl;
#[doc = "Channel Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ch5_ctrl](ch5_ctrl) module"]
pub type CH5_CTRL = crate::Reg<u32, _CH5_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH5_CTRL;
#[doc = "`read()` method returns [ch5_ctrl::R](ch5_ctrl::R) reader structure"]
impl crate::Readable for CH5_CTRL {}
#[doc = "`write(|w| ..)` method takes [ch5_ctrl::W](ch5_ctrl::W) writer structure"]
impl crate::Writable for CH5_CTRL {}
#[doc = "Channel Control Register"]
pub mod ch5_ctrl;

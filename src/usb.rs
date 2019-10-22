#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - System Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - System Status Register"]
    pub status: STATUS,
    #[doc = "0x08 - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x0c - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x10 - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x14 - Interrupt Enable Register"]
    pub ien: IEN,
    #[doc = "0x18 - I/O Routing Register"]
    pub route: ROUTE,
    _reserved7: [u8; 245740usize],
    #[doc = "0x3c008 - AHB Configuration Register"]
    pub gahbcfg: GAHBCFG,
    #[doc = "0x3c00c - USB Configuration Register"]
    pub gusbcfg: GUSBCFG,
    #[doc = "0x3c010 - Reset Register"]
    pub grstctl: GRSTCTL,
    #[doc = "0x3c014 - Interrupt Register"]
    pub gintsts: GINTSTS,
    #[doc = "0x3c018 - Interrupt Mask Register"]
    pub gintmsk: GINTMSK,
    #[doc = "0x3c01c - Receive Status Debug Read Register"]
    pub grxstsr: GRXSTSR,
    #[doc = "0x3c020 - Receive Status Read and Pop Register"]
    pub grxstsp: GRXSTSP,
    #[doc = "0x3c024 - Receive FIFO Size Register"]
    pub grxfsiz: GRXFSIZ,
    #[doc = "0x3c028 - Non-periodic Transmit FIFO Size Register"]
    pub gnptxfsiz: GNPTXFSIZ,
    _reserved16: [u8; 48usize],
    #[doc = "0x3c05c - Global DFIFO Configuration Register"]
    pub gdfifocfg: GDFIFOCFG,
    _reserved17: [u8; 164usize],
    #[doc = "0x3c104 - Device IN Endpoint Transmit FIFO 1 Size Register"]
    pub dieptxf1: DIEPTXF1,
    #[doc = "0x3c108 - Device IN Endpoint Transmit FIFO 2 Size Register"]
    pub dieptxf2: DIEPTXF2,
    #[doc = "0x3c10c - Device IN Endpoint Transmit FIFO 3 Size Register"]
    pub dieptxf3: DIEPTXF3,
    _reserved20: [u8; 1776usize],
    #[doc = "0x3c800 - Device Configuration Register"]
    pub dcfg: DCFG,
    #[doc = "0x3c804 - Device Control Register"]
    pub dctl: DCTL,
    #[doc = "0x3c808 - Device Status Register"]
    pub dsts: DSTS,
    _reserved23: [u8; 4usize],
    #[doc = "0x3c810 - Device IN Endpoint Common Interrupt Mask Register"]
    pub diepmsk: DIEPMSK,
    #[doc = "0x3c814 - Device OUT Endpoint Common Interrupt Mask Register"]
    pub doepmsk: DOEPMSK,
    #[doc = "0x3c818 - Device All Endpoints Interrupt Register"]
    pub daint: DAINT,
    #[doc = "0x3c81c - Device All Endpoints Interrupt Mask Register"]
    pub daintmsk: DAINTMSK,
    _reserved27: [u8; 20usize],
    #[doc = "0x3c834 - Device IN Endpoint FIFO Empty Interrupt Mask Register"]
    pub diepempmsk: DIEPEMPMSK,
    _reserved28: [u8; 200usize],
    #[doc = "0x3c900 - Device IN Endpoint 0 Control Register"]
    pub diep0ctl: DIEP0CTL,
    _reserved29: [u8; 4usize],
    #[doc = "0x3c908 - Device IN Endpoint 0 Interrupt Register"]
    pub diep0int: DIEP0INT,
    _reserved30: [u8; 4usize],
    #[doc = "0x3c910 - Device IN Endpoint 0 Transfer Size Register"]
    pub diep0tsiz: DIEP0TSIZ,
    #[doc = "0x3c914 - Device IN Endpoint 0 DMA Address Register"]
    pub diep0dmaaddr: DIEP0DMAADDR,
    #[doc = "0x3c918 - Device IN Endpoint 0 Transmit FIFO Status Register"]
    pub diep0txfsts: DIEP0TXFSTS,
    _reserved33: [u8; 4usize],
    #[doc = "0x3c920 - Device IN Endpoint x+1 Control Register"]
    pub diep0_ctl: DIEP0_CTL,
    _reserved34: [u8; 4usize],
    #[doc = "0x3c928 - Device IN Endpoint x+1 Interrupt Register"]
    pub diep0_int: DIEP0_INT,
    _reserved35: [u8; 4usize],
    #[doc = "0x3c930 - Device IN Endpoint x+1 Transfer Size Register"]
    pub diep0_tsiz: DIEP0_TSIZ,
    #[doc = "0x3c934 - Device IN Endpoint x+1 DMA Address Register"]
    pub diep0_dmaaddr: DIEP0_DMAADDR,
    #[doc = "0x3c938 - Device IN Endpoint x+1 Transmit FIFO Status Register"]
    pub diep0_txfsts: DIEP0_TXFSTS,
    _reserved38: [u8; 4usize],
    #[doc = "0x3c940 - Device IN Endpoint x+1 Control Register"]
    pub diep1_ctl: DIEP1_CTL,
    _reserved39: [u8; 4usize],
    #[doc = "0x3c948 - Device IN Endpoint x+1 Interrupt Register"]
    pub diep1_int: DIEP1_INT,
    _reserved40: [u8; 4usize],
    #[doc = "0x3c950 - Device IN Endpoint x+1 Transfer Size Register"]
    pub diep1_tsiz: DIEP1_TSIZ,
    #[doc = "0x3c954 - Device IN Endpoint x+1 DMA Address Register"]
    pub diep1_dmaaddr: DIEP1_DMAADDR,
    #[doc = "0x3c958 - Device IN Endpoint x+1 Transmit FIFO Status Register"]
    pub diep1_txfsts: DIEP1_TXFSTS,
    _reserved43: [u8; 4usize],
    #[doc = "0x3c960 - Device IN Endpoint x+1 Control Register"]
    pub diep2_ctl: DIEP2_CTL,
    _reserved44: [u8; 4usize],
    #[doc = "0x3c968 - Device IN Endpoint x+1 Interrupt Register"]
    pub diep2_int: DIEP2_INT,
    _reserved45: [u8; 4usize],
    #[doc = "0x3c970 - Device IN Endpoint x+1 Transfer Size Register"]
    pub diep2_tsiz: DIEP2_TSIZ,
    #[doc = "0x3c974 - Device IN Endpoint x+1 DMA Address Register"]
    pub diep2_dmaaddr: DIEP2_DMAADDR,
    #[doc = "0x3c978 - Device IN Endpoint x+1 Transmit FIFO Status Register"]
    pub diep2_txfsts: DIEP2_TXFSTS,
    _reserved48: [u8; 388usize],
    #[doc = "0x3cb00 - Device OUT Endpoint 0 Control Register"]
    pub doep0ctl: DOEP0CTL,
    _reserved49: [u8; 4usize],
    #[doc = "0x3cb08 - Device OUT Endpoint 0 Interrupt Register"]
    pub doep0int: DOEP0INT,
    _reserved50: [u8; 4usize],
    #[doc = "0x3cb10 - Device OUT Endpoint 0 Transfer Size Register"]
    pub doep0tsiz: DOEP0TSIZ,
    #[doc = "0x3cb14 - Device OUT Endpoint 0 DMA Address Register"]
    pub doep0dmaaddr: DOEP0DMAADDR,
    _reserved52: [u8; 8usize],
    #[doc = "0x3cb20 - Device OUT Endpoint x+1 Control Register"]
    pub doep0_ctl: DOEP0_CTL,
    _reserved53: [u8; 4usize],
    #[doc = "0x3cb28 - Device OUT Endpoint x+1 Interrupt Register"]
    pub doep0_int: DOEP0_INT,
    _reserved54: [u8; 4usize],
    #[doc = "0x3cb30 - Device OUT Endpoint x+1 Transfer Size Register"]
    pub doep0_tsiz: DOEP0_TSIZ,
    #[doc = "0x3cb34 - Device OUT Endpoint x+1 DMA Address Register"]
    pub doep0_dmaaddr: DOEP0_DMAADDR,
    _reserved56: [u8; 8usize],
    #[doc = "0x3cb40 - Device OUT Endpoint x+1 Control Register"]
    pub doep1_ctl: DOEP1_CTL,
    _reserved57: [u8; 4usize],
    #[doc = "0x3cb48 - Device OUT Endpoint x+1 Interrupt Register"]
    pub doep1_int: DOEP1_INT,
    _reserved58: [u8; 4usize],
    #[doc = "0x3cb50 - Device OUT Endpoint x+1 Transfer Size Register"]
    pub doep1_tsiz: DOEP1_TSIZ,
    #[doc = "0x3cb54 - Device OUT Endpoint x+1 DMA Address Register"]
    pub doep1_dmaaddr: DOEP1_DMAADDR,
    _reserved60: [u8; 8usize],
    #[doc = "0x3cb60 - Device OUT Endpoint x+1 Control Register"]
    pub doep2_ctl: DOEP2_CTL,
    _reserved61: [u8; 4usize],
    #[doc = "0x3cb68 - Device OUT Endpoint x+1 Interrupt Register"]
    pub doep2_int: DOEP2_INT,
    _reserved62: [u8; 4usize],
    #[doc = "0x3cb70 - Device OUT Endpoint x+1 Transfer Size Register"]
    pub doep2_tsiz: DOEP2_TSIZ,
    #[doc = "0x3cb74 - Device OUT Endpoint x+1 DMA Address Register"]
    pub doep2_dmaaddr: DOEP2_DMAADDR,
    _reserved64: [u8; 648usize],
    #[doc = "0x3ce00 - Power and Clock Gating Control Register"]
    pub pcgcctl: PCGCCTL,
}
#[doc = "System Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "System Control Register"]
pub mod ctrl;
#[doc = "System Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "System Status Register"]
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
#[doc = "AHB Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gahbcfg](gahbcfg) module"]
pub type GAHBCFG = crate::Reg<u32, _GAHBCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GAHBCFG;
#[doc = "`read()` method returns [gahbcfg::R](gahbcfg::R) reader structure"]
impl crate::Readable for GAHBCFG {}
#[doc = "`write(|w| ..)` method takes [gahbcfg::W](gahbcfg::W) writer structure"]
impl crate::Writable for GAHBCFG {}
#[doc = "AHB Configuration Register"]
pub mod gahbcfg;
#[doc = "USB Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gusbcfg](gusbcfg) module"]
pub type GUSBCFG = crate::Reg<u32, _GUSBCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GUSBCFG;
#[doc = "`read()` method returns [gusbcfg::R](gusbcfg::R) reader structure"]
impl crate::Readable for GUSBCFG {}
#[doc = "`write(|w| ..)` method takes [gusbcfg::W](gusbcfg::W) writer structure"]
impl crate::Writable for GUSBCFG {}
#[doc = "USB Configuration Register"]
pub mod gusbcfg;
#[doc = "Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [grstctl](grstctl) module"]
pub type GRSTCTL = crate::Reg<u32, _GRSTCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GRSTCTL;
#[doc = "`read()` method returns [grstctl::R](grstctl::R) reader structure"]
impl crate::Readable for GRSTCTL {}
#[doc = "`write(|w| ..)` method takes [grstctl::W](grstctl::W) writer structure"]
impl crate::Writable for GRSTCTL {}
#[doc = "Reset Register"]
pub mod grstctl;
#[doc = "Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gintsts](gintsts) module"]
pub type GINTSTS = crate::Reg<u32, _GINTSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GINTSTS;
#[doc = "`read()` method returns [gintsts::R](gintsts::R) reader structure"]
impl crate::Readable for GINTSTS {}
#[doc = "`write(|w| ..)` method takes [gintsts::W](gintsts::W) writer structure"]
impl crate::Writable for GINTSTS {}
#[doc = "Interrupt Register"]
pub mod gintsts;
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gintmsk](gintmsk) module"]
pub type GINTMSK = crate::Reg<u32, _GINTMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GINTMSK;
#[doc = "`read()` method returns [gintmsk::R](gintmsk::R) reader structure"]
impl crate::Readable for GINTMSK {}
#[doc = "`write(|w| ..)` method takes [gintmsk::W](gintmsk::W) writer structure"]
impl crate::Writable for GINTMSK {}
#[doc = "Interrupt Mask Register"]
pub mod gintmsk;
#[doc = "Receive Status Debug Read Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [grxstsr](grxstsr) module"]
pub type GRXSTSR = crate::Reg<u32, _GRXSTSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GRXSTSR;
#[doc = "`read()` method returns [grxstsr::R](grxstsr::R) reader structure"]
impl crate::Readable for GRXSTSR {}
#[doc = "Receive Status Debug Read Register"]
pub mod grxstsr;
#[doc = "Receive Status Read and Pop Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [grxstsp](grxstsp) module"]
pub type GRXSTSP = crate::Reg<u32, _GRXSTSP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GRXSTSP;
#[doc = "`read()` method returns [grxstsp::R](grxstsp::R) reader structure"]
impl crate::Readable for GRXSTSP {}
#[doc = "Receive Status Read and Pop Register"]
pub mod grxstsp;
#[doc = "Receive FIFO Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [grxfsiz](grxfsiz) module"]
pub type GRXFSIZ = crate::Reg<u32, _GRXFSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GRXFSIZ;
#[doc = "`read()` method returns [grxfsiz::R](grxfsiz::R) reader structure"]
impl crate::Readable for GRXFSIZ {}
#[doc = "`write(|w| ..)` method takes [grxfsiz::W](grxfsiz::W) writer structure"]
impl crate::Writable for GRXFSIZ {}
#[doc = "Receive FIFO Size Register"]
pub mod grxfsiz;
#[doc = "Non-periodic Transmit FIFO Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gnptxfsiz](gnptxfsiz) module"]
pub type GNPTXFSIZ = crate::Reg<u32, _GNPTXFSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GNPTXFSIZ;
#[doc = "`read()` method returns [gnptxfsiz::R](gnptxfsiz::R) reader structure"]
impl crate::Readable for GNPTXFSIZ {}
#[doc = "`write(|w| ..)` method takes [gnptxfsiz::W](gnptxfsiz::W) writer structure"]
impl crate::Writable for GNPTXFSIZ {}
#[doc = "Non-periodic Transmit FIFO Size Register"]
pub mod gnptxfsiz;
#[doc = "Global DFIFO Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gdfifocfg](gdfifocfg) module"]
pub type GDFIFOCFG = crate::Reg<u32, _GDFIFOCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GDFIFOCFG;
#[doc = "`read()` method returns [gdfifocfg::R](gdfifocfg::R) reader structure"]
impl crate::Readable for GDFIFOCFG {}
#[doc = "`write(|w| ..)` method takes [gdfifocfg::W](gdfifocfg::W) writer structure"]
impl crate::Writable for GDFIFOCFG {}
#[doc = "Global DFIFO Configuration Register"]
pub mod gdfifocfg;
#[doc = "Device IN Endpoint Transmit FIFO 1 Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dieptxf1](dieptxf1) module"]
pub type DIEPTXF1 = crate::Reg<u32, _DIEPTXF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPTXF1;
#[doc = "`read()` method returns [dieptxf1::R](dieptxf1::R) reader structure"]
impl crate::Readable for DIEPTXF1 {}
#[doc = "`write(|w| ..)` method takes [dieptxf1::W](dieptxf1::W) writer structure"]
impl crate::Writable for DIEPTXF1 {}
#[doc = "Device IN Endpoint Transmit FIFO 1 Size Register"]
pub mod dieptxf1;
#[doc = "Device IN Endpoint Transmit FIFO 2 Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dieptxf2](dieptxf2) module"]
pub type DIEPTXF2 = crate::Reg<u32, _DIEPTXF2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPTXF2;
#[doc = "`read()` method returns [dieptxf2::R](dieptxf2::R) reader structure"]
impl crate::Readable for DIEPTXF2 {}
#[doc = "`write(|w| ..)` method takes [dieptxf2::W](dieptxf2::W) writer structure"]
impl crate::Writable for DIEPTXF2 {}
#[doc = "Device IN Endpoint Transmit FIFO 2 Size Register"]
pub mod dieptxf2;
#[doc = "Device IN Endpoint Transmit FIFO 3 Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dieptxf3](dieptxf3) module"]
pub type DIEPTXF3 = crate::Reg<u32, _DIEPTXF3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPTXF3;
#[doc = "`read()` method returns [dieptxf3::R](dieptxf3::R) reader structure"]
impl crate::Readable for DIEPTXF3 {}
#[doc = "`write(|w| ..)` method takes [dieptxf3::W](dieptxf3::W) writer structure"]
impl crate::Writable for DIEPTXF3 {}
#[doc = "Device IN Endpoint Transmit FIFO 3 Size Register"]
pub mod dieptxf3;
#[doc = "Device Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dcfg](dcfg) module"]
pub type DCFG = crate::Reg<u32, _DCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCFG;
#[doc = "`read()` method returns [dcfg::R](dcfg::R) reader structure"]
impl crate::Readable for DCFG {}
#[doc = "`write(|w| ..)` method takes [dcfg::W](dcfg::W) writer structure"]
impl crate::Writable for DCFG {}
#[doc = "Device Configuration Register"]
pub mod dcfg;
#[doc = "Device Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dctl](dctl) module"]
pub type DCTL = crate::Reg<u32, _DCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCTL;
#[doc = "`read()` method returns [dctl::R](dctl::R) reader structure"]
impl crate::Readable for DCTL {}
#[doc = "`write(|w| ..)` method takes [dctl::W](dctl::W) writer structure"]
impl crate::Writable for DCTL {}
#[doc = "Device Control Register"]
pub mod dctl;
#[doc = "Device Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dsts](dsts) module"]
pub type DSTS = crate::Reg<u32, _DSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSTS;
#[doc = "`read()` method returns [dsts::R](dsts::R) reader structure"]
impl crate::Readable for DSTS {}
#[doc = "Device Status Register"]
pub mod dsts;
#[doc = "Device IN Endpoint Common Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [diepmsk](diepmsk) module"]
pub type DIEPMSK = crate::Reg<u32, _DIEPMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPMSK;
#[doc = "`read()` method returns [diepmsk::R](diepmsk::R) reader structure"]
impl crate::Readable for DIEPMSK {}
#[doc = "`write(|w| ..)` method takes [diepmsk::W](diepmsk::W) writer structure"]
impl crate::Writable for DIEPMSK {}
#[doc = "Device IN Endpoint Common Interrupt Mask Register"]
pub mod diepmsk;
#[doc = "Device OUT Endpoint Common Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [doepmsk](doepmsk) module"]
pub type DOEPMSK = crate::Reg<u32, _DOEPMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEPMSK;
#[doc = "`read()` method returns [doepmsk::R](doepmsk::R) reader structure"]
impl crate::Readable for DOEPMSK {}
#[doc = "`write(|w| ..)` method takes [doepmsk::W](doepmsk::W) writer structure"]
impl crate::Writable for DOEPMSK {}
#[doc = "Device OUT Endpoint Common Interrupt Mask Register"]
pub mod doepmsk;
#[doc = "Device All Endpoints Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [daint](daint) module"]
pub type DAINT = crate::Reg<u32, _DAINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAINT;
#[doc = "`read()` method returns [daint::R](daint::R) reader structure"]
impl crate::Readable for DAINT {}
#[doc = "Device All Endpoints Interrupt Register"]
pub mod daint;
#[doc = "Device All Endpoints Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [daintmsk](daintmsk) module"]
pub type DAINTMSK = crate::Reg<u32, _DAINTMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAINTMSK;
#[doc = "`read()` method returns [daintmsk::R](daintmsk::R) reader structure"]
impl crate::Readable for DAINTMSK {}
#[doc = "`write(|w| ..)` method takes [daintmsk::W](daintmsk::W) writer structure"]
impl crate::Writable for DAINTMSK {}
#[doc = "Device All Endpoints Interrupt Mask Register"]
pub mod daintmsk;
#[doc = "Device IN Endpoint FIFO Empty Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [diepempmsk](diepempmsk) module"]
pub type DIEPEMPMSK = crate::Reg<u32, _DIEPEMPMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPEMPMSK;
#[doc = "`read()` method returns [diepempmsk::R](diepempmsk::R) reader structure"]
impl crate::Readable for DIEPEMPMSK {}
#[doc = "`write(|w| ..)` method takes [diepempmsk::W](diepempmsk::W) writer structure"]
impl crate::Writable for DIEPEMPMSK {}
#[doc = "Device IN Endpoint FIFO Empty Interrupt Mask Register"]
pub mod diepempmsk;
#[doc = "Device IN Endpoint 0 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [diep0ctl](diep0ctl) module"]
pub type DIEP0CTL = crate::Reg<u32, _DIEP0CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEP0CTL;
#[doc = "`read()` method returns [diep0ctl::R](diep0ctl::R) reader structure"]
impl crate::Readable for DIEP0CTL {}
#[doc = "`write(|w| ..)` method takes [diep0ctl::W](diep0ctl::W) writer structure"]
impl crate::Writable for DIEP0CTL {}
#[doc = "Device IN Endpoint 0 Control Register"]
pub mod diep0ctl;
#[doc = "Device IN Endpoint 0 Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [diep0int](diep0int) module"]
pub type DIEP0INT = crate::Reg<u32, _DIEP0INT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEP0INT;
#[doc = "`read()` method returns [diep0int::R](diep0int::R) reader structure"]
impl crate::Readable for DIEP0INT {}
#[doc = "`write(|w| ..)` method takes [diep0int::W](diep0int::W) writer structure"]
impl crate::Writable for DIEP0INT {}
#[doc = "Device IN Endpoint 0 Interrupt Register"]
pub mod diep0int;
#[doc = "Device IN Endpoint 0 Transfer Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [diep0tsiz](diep0tsiz) module"]
pub type DIEP0TSIZ = crate::Reg<u32, _DIEP0TSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEP0TSIZ;
#[doc = "`read()` method returns [diep0tsiz::R](diep0tsiz::R) reader structure"]
impl crate::Readable for DIEP0TSIZ {}
#[doc = "`write(|w| ..)` method takes [diep0tsiz::W](diep0tsiz::W) writer structure"]
impl crate::Writable for DIEP0TSIZ {}
#[doc = "Device IN Endpoint 0 Transfer Size Register"]
pub mod diep0tsiz;
#[doc = "Device IN Endpoint 0 DMA Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [diep0dmaaddr](diep0dmaaddr) module"]
pub type DIEP0DMAADDR = crate::Reg<u32, _DIEP0DMAADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEP0DMAADDR;
#[doc = "`read()` method returns [diep0dmaaddr::R](diep0dmaaddr::R) reader structure"]
impl crate::Readable for DIEP0DMAADDR {}
#[doc = "`write(|w| ..)` method takes [diep0dmaaddr::W](diep0dmaaddr::W) writer structure"]
impl crate::Writable for DIEP0DMAADDR {}
#[doc = "Device IN Endpoint 0 DMA Address Register"]
pub mod diep0dmaaddr;
#[doc = "Device IN Endpoint 0 Transmit FIFO Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [diep0txfsts](diep0txfsts) module"]
pub type DIEP0TXFSTS = crate::Reg<u32, _DIEP0TXFSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEP0TXFSTS;
#[doc = "`read()` method returns [diep0txfsts::R](diep0txfsts::R) reader structure"]
impl crate::Readable for DIEP0TXFSTS {}
#[doc = "Device IN Endpoint 0 Transmit FIFO Status Register"]
pub mod diep0txfsts;
#[doc = "Device IN Endpoint x+1 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [diep0_ctl](diep0_ctl) module"]
pub type DIEP0_CTL = crate::Reg<u32, _DIEP0_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEP0_CTL;
#[doc = "`read()` method returns [diep0_ctl::R](diep0_ctl::R) reader structure"]
impl crate::Readable for DIEP0_CTL {}
#[doc = "`write(|w| ..)` method takes [diep0_ctl::W](diep0_ctl::W) writer structure"]
impl crate::Writable for DIEP0_CTL {}
#[doc = "Device IN Endpoint x+1 Control Register"]
pub mod diep0_ctl;
#[doc = "Device IN Endpoint x+1 Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [diep0_int](diep0_int) module"]
pub type DIEP0_INT = crate::Reg<u32, _DIEP0_INT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEP0_INT;
#[doc = "`read()` method returns [diep0_int::R](diep0_int::R) reader structure"]
impl crate::Readable for DIEP0_INT {}
#[doc = "`write(|w| ..)` method takes [diep0_int::W](diep0_int::W) writer structure"]
impl crate::Writable for DIEP0_INT {}
#[doc = "Device IN Endpoint x+1 Interrupt Register"]
pub mod diep0_int;
#[doc = "Device IN Endpoint x+1 Transfer Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [diep0_tsiz](diep0_tsiz) module"]
pub type DIEP0_TSIZ = crate::Reg<u32, _DIEP0_TSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEP0_TSIZ;
#[doc = "`read()` method returns [diep0_tsiz::R](diep0_tsiz::R) reader structure"]
impl crate::Readable for DIEP0_TSIZ {}
#[doc = "`write(|w| ..)` method takes [diep0_tsiz::W](diep0_tsiz::W) writer structure"]
impl crate::Writable for DIEP0_TSIZ {}
#[doc = "Device IN Endpoint x+1 Transfer Size Register"]
pub mod diep0_tsiz;
#[doc = "Device IN Endpoint x+1 DMA Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [diep0_dmaaddr](diep0_dmaaddr) module"]
pub type DIEP0_DMAADDR = crate::Reg<u32, _DIEP0_DMAADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEP0_DMAADDR;
#[doc = "`read()` method returns [diep0_dmaaddr::R](diep0_dmaaddr::R) reader structure"]
impl crate::Readable for DIEP0_DMAADDR {}
#[doc = "`write(|w| ..)` method takes [diep0_dmaaddr::W](diep0_dmaaddr::W) writer structure"]
impl crate::Writable for DIEP0_DMAADDR {}
#[doc = "Device IN Endpoint x+1 DMA Address Register"]
pub mod diep0_dmaaddr;
#[doc = "Device IN Endpoint x+1 Transmit FIFO Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [diep0_txfsts](diep0_txfsts) module"]
pub type DIEP0_TXFSTS = crate::Reg<u32, _DIEP0_TXFSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEP0_TXFSTS;
#[doc = "`read()` method returns [diep0_txfsts::R](diep0_txfsts::R) reader structure"]
impl crate::Readable for DIEP0_TXFSTS {}
#[doc = "Device IN Endpoint x+1 Transmit FIFO Status Register"]
pub mod diep0_txfsts;
#[doc = "Device IN Endpoint x+1 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [diep1_ctl](diep1_ctl) module"]
pub type DIEP1_CTL = crate::Reg<u32, _DIEP1_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEP1_CTL;
#[doc = "`read()` method returns [diep1_ctl::R](diep1_ctl::R) reader structure"]
impl crate::Readable for DIEP1_CTL {}
#[doc = "`write(|w| ..)` method takes [diep1_ctl::W](diep1_ctl::W) writer structure"]
impl crate::Writable for DIEP1_CTL {}
#[doc = "Device IN Endpoint x+1 Control Register"]
pub mod diep1_ctl;
#[doc = "Device IN Endpoint x+1 Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [diep1_int](diep1_int) module"]
pub type DIEP1_INT = crate::Reg<u32, _DIEP1_INT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEP1_INT;
#[doc = "`read()` method returns [diep1_int::R](diep1_int::R) reader structure"]
impl crate::Readable for DIEP1_INT {}
#[doc = "`write(|w| ..)` method takes [diep1_int::W](diep1_int::W) writer structure"]
impl crate::Writable for DIEP1_INT {}
#[doc = "Device IN Endpoint x+1 Interrupt Register"]
pub mod diep1_int;
#[doc = "Device IN Endpoint x+1 Transfer Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [diep1_tsiz](diep1_tsiz) module"]
pub type DIEP1_TSIZ = crate::Reg<u32, _DIEP1_TSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEP1_TSIZ;
#[doc = "`read()` method returns [diep1_tsiz::R](diep1_tsiz::R) reader structure"]
impl crate::Readable for DIEP1_TSIZ {}
#[doc = "`write(|w| ..)` method takes [diep1_tsiz::W](diep1_tsiz::W) writer structure"]
impl crate::Writable for DIEP1_TSIZ {}
#[doc = "Device IN Endpoint x+1 Transfer Size Register"]
pub mod diep1_tsiz;
#[doc = "Device IN Endpoint x+1 DMA Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [diep1_dmaaddr](diep1_dmaaddr) module"]
pub type DIEP1_DMAADDR = crate::Reg<u32, _DIEP1_DMAADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEP1_DMAADDR;
#[doc = "`read()` method returns [diep1_dmaaddr::R](diep1_dmaaddr::R) reader structure"]
impl crate::Readable for DIEP1_DMAADDR {}
#[doc = "`write(|w| ..)` method takes [diep1_dmaaddr::W](diep1_dmaaddr::W) writer structure"]
impl crate::Writable for DIEP1_DMAADDR {}
#[doc = "Device IN Endpoint x+1 DMA Address Register"]
pub mod diep1_dmaaddr;
#[doc = "Device IN Endpoint x+1 Transmit FIFO Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [diep1_txfsts](diep1_txfsts) module"]
pub type DIEP1_TXFSTS = crate::Reg<u32, _DIEP1_TXFSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEP1_TXFSTS;
#[doc = "`read()` method returns [diep1_txfsts::R](diep1_txfsts::R) reader structure"]
impl crate::Readable for DIEP1_TXFSTS {}
#[doc = "Device IN Endpoint x+1 Transmit FIFO Status Register"]
pub mod diep1_txfsts;
#[doc = "Device IN Endpoint x+1 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [diep2_ctl](diep2_ctl) module"]
pub type DIEP2_CTL = crate::Reg<u32, _DIEP2_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEP2_CTL;
#[doc = "`read()` method returns [diep2_ctl::R](diep2_ctl::R) reader structure"]
impl crate::Readable for DIEP2_CTL {}
#[doc = "`write(|w| ..)` method takes [diep2_ctl::W](diep2_ctl::W) writer structure"]
impl crate::Writable for DIEP2_CTL {}
#[doc = "Device IN Endpoint x+1 Control Register"]
pub mod diep2_ctl;
#[doc = "Device IN Endpoint x+1 Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [diep2_int](diep2_int) module"]
pub type DIEP2_INT = crate::Reg<u32, _DIEP2_INT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEP2_INT;
#[doc = "`read()` method returns [diep2_int::R](diep2_int::R) reader structure"]
impl crate::Readable for DIEP2_INT {}
#[doc = "`write(|w| ..)` method takes [diep2_int::W](diep2_int::W) writer structure"]
impl crate::Writable for DIEP2_INT {}
#[doc = "Device IN Endpoint x+1 Interrupt Register"]
pub mod diep2_int;
#[doc = "Device IN Endpoint x+1 Transfer Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [diep2_tsiz](diep2_tsiz) module"]
pub type DIEP2_TSIZ = crate::Reg<u32, _DIEP2_TSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEP2_TSIZ;
#[doc = "`read()` method returns [diep2_tsiz::R](diep2_tsiz::R) reader structure"]
impl crate::Readable for DIEP2_TSIZ {}
#[doc = "`write(|w| ..)` method takes [diep2_tsiz::W](diep2_tsiz::W) writer structure"]
impl crate::Writable for DIEP2_TSIZ {}
#[doc = "Device IN Endpoint x+1 Transfer Size Register"]
pub mod diep2_tsiz;
#[doc = "Device IN Endpoint x+1 DMA Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [diep2_dmaaddr](diep2_dmaaddr) module"]
pub type DIEP2_DMAADDR = crate::Reg<u32, _DIEP2_DMAADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEP2_DMAADDR;
#[doc = "`read()` method returns [diep2_dmaaddr::R](diep2_dmaaddr::R) reader structure"]
impl crate::Readable for DIEP2_DMAADDR {}
#[doc = "`write(|w| ..)` method takes [diep2_dmaaddr::W](diep2_dmaaddr::W) writer structure"]
impl crate::Writable for DIEP2_DMAADDR {}
#[doc = "Device IN Endpoint x+1 DMA Address Register"]
pub mod diep2_dmaaddr;
#[doc = "Device IN Endpoint x+1 Transmit FIFO Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [diep2_txfsts](diep2_txfsts) module"]
pub type DIEP2_TXFSTS = crate::Reg<u32, _DIEP2_TXFSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEP2_TXFSTS;
#[doc = "`read()` method returns [diep2_txfsts::R](diep2_txfsts::R) reader structure"]
impl crate::Readable for DIEP2_TXFSTS {}
#[doc = "Device IN Endpoint x+1 Transmit FIFO Status Register"]
pub mod diep2_txfsts;
#[doc = "Device OUT Endpoint 0 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [doep0ctl](doep0ctl) module"]
pub type DOEP0CTL = crate::Reg<u32, _DOEP0CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEP0CTL;
#[doc = "`read()` method returns [doep0ctl::R](doep0ctl::R) reader structure"]
impl crate::Readable for DOEP0CTL {}
#[doc = "`write(|w| ..)` method takes [doep0ctl::W](doep0ctl::W) writer structure"]
impl crate::Writable for DOEP0CTL {}
#[doc = "Device OUT Endpoint 0 Control Register"]
pub mod doep0ctl;
#[doc = "Device OUT Endpoint 0 Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [doep0int](doep0int) module"]
pub type DOEP0INT = crate::Reg<u32, _DOEP0INT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEP0INT;
#[doc = "`read()` method returns [doep0int::R](doep0int::R) reader structure"]
impl crate::Readable for DOEP0INT {}
#[doc = "`write(|w| ..)` method takes [doep0int::W](doep0int::W) writer structure"]
impl crate::Writable for DOEP0INT {}
#[doc = "Device OUT Endpoint 0 Interrupt Register"]
pub mod doep0int;
#[doc = "Device OUT Endpoint 0 Transfer Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [doep0tsiz](doep0tsiz) module"]
pub type DOEP0TSIZ = crate::Reg<u32, _DOEP0TSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEP0TSIZ;
#[doc = "`read()` method returns [doep0tsiz::R](doep0tsiz::R) reader structure"]
impl crate::Readable for DOEP0TSIZ {}
#[doc = "`write(|w| ..)` method takes [doep0tsiz::W](doep0tsiz::W) writer structure"]
impl crate::Writable for DOEP0TSIZ {}
#[doc = "Device OUT Endpoint 0 Transfer Size Register"]
pub mod doep0tsiz;
#[doc = "Device OUT Endpoint 0 DMA Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [doep0dmaaddr](doep0dmaaddr) module"]
pub type DOEP0DMAADDR = crate::Reg<u32, _DOEP0DMAADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEP0DMAADDR;
#[doc = "`read()` method returns [doep0dmaaddr::R](doep0dmaaddr::R) reader structure"]
impl crate::Readable for DOEP0DMAADDR {}
#[doc = "`write(|w| ..)` method takes [doep0dmaaddr::W](doep0dmaaddr::W) writer structure"]
impl crate::Writable for DOEP0DMAADDR {}
#[doc = "Device OUT Endpoint 0 DMA Address Register"]
pub mod doep0dmaaddr;
#[doc = "Device OUT Endpoint x+1 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [doep0_ctl](doep0_ctl) module"]
pub type DOEP0_CTL = crate::Reg<u32, _DOEP0_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEP0_CTL;
#[doc = "`read()` method returns [doep0_ctl::R](doep0_ctl::R) reader structure"]
impl crate::Readable for DOEP0_CTL {}
#[doc = "`write(|w| ..)` method takes [doep0_ctl::W](doep0_ctl::W) writer structure"]
impl crate::Writable for DOEP0_CTL {}
#[doc = "Device OUT Endpoint x+1 Control Register"]
pub mod doep0_ctl;
#[doc = "Device OUT Endpoint x+1 Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [doep0_int](doep0_int) module"]
pub type DOEP0_INT = crate::Reg<u32, _DOEP0_INT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEP0_INT;
#[doc = "`read()` method returns [doep0_int::R](doep0_int::R) reader structure"]
impl crate::Readable for DOEP0_INT {}
#[doc = "`write(|w| ..)` method takes [doep0_int::W](doep0_int::W) writer structure"]
impl crate::Writable for DOEP0_INT {}
#[doc = "Device OUT Endpoint x+1 Interrupt Register"]
pub mod doep0_int;
#[doc = "Device OUT Endpoint x+1 Transfer Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [doep0_tsiz](doep0_tsiz) module"]
pub type DOEP0_TSIZ = crate::Reg<u32, _DOEP0_TSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEP0_TSIZ;
#[doc = "`read()` method returns [doep0_tsiz::R](doep0_tsiz::R) reader structure"]
impl crate::Readable for DOEP0_TSIZ {}
#[doc = "`write(|w| ..)` method takes [doep0_tsiz::W](doep0_tsiz::W) writer structure"]
impl crate::Writable for DOEP0_TSIZ {}
#[doc = "Device OUT Endpoint x+1 Transfer Size Register"]
pub mod doep0_tsiz;
#[doc = "Device OUT Endpoint x+1 DMA Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [doep0_dmaaddr](doep0_dmaaddr) module"]
pub type DOEP0_DMAADDR = crate::Reg<u32, _DOEP0_DMAADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEP0_DMAADDR;
#[doc = "`read()` method returns [doep0_dmaaddr::R](doep0_dmaaddr::R) reader structure"]
impl crate::Readable for DOEP0_DMAADDR {}
#[doc = "`write(|w| ..)` method takes [doep0_dmaaddr::W](doep0_dmaaddr::W) writer structure"]
impl crate::Writable for DOEP0_DMAADDR {}
#[doc = "Device OUT Endpoint x+1 DMA Address Register"]
pub mod doep0_dmaaddr;
#[doc = "Device OUT Endpoint x+1 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [doep1_ctl](doep1_ctl) module"]
pub type DOEP1_CTL = crate::Reg<u32, _DOEP1_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEP1_CTL;
#[doc = "`read()` method returns [doep1_ctl::R](doep1_ctl::R) reader structure"]
impl crate::Readable for DOEP1_CTL {}
#[doc = "`write(|w| ..)` method takes [doep1_ctl::W](doep1_ctl::W) writer structure"]
impl crate::Writable for DOEP1_CTL {}
#[doc = "Device OUT Endpoint x+1 Control Register"]
pub mod doep1_ctl;
#[doc = "Device OUT Endpoint x+1 Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [doep1_int](doep1_int) module"]
pub type DOEP1_INT = crate::Reg<u32, _DOEP1_INT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEP1_INT;
#[doc = "`read()` method returns [doep1_int::R](doep1_int::R) reader structure"]
impl crate::Readable for DOEP1_INT {}
#[doc = "`write(|w| ..)` method takes [doep1_int::W](doep1_int::W) writer structure"]
impl crate::Writable for DOEP1_INT {}
#[doc = "Device OUT Endpoint x+1 Interrupt Register"]
pub mod doep1_int;
#[doc = "Device OUT Endpoint x+1 Transfer Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [doep1_tsiz](doep1_tsiz) module"]
pub type DOEP1_TSIZ = crate::Reg<u32, _DOEP1_TSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEP1_TSIZ;
#[doc = "`read()` method returns [doep1_tsiz::R](doep1_tsiz::R) reader structure"]
impl crate::Readable for DOEP1_TSIZ {}
#[doc = "`write(|w| ..)` method takes [doep1_tsiz::W](doep1_tsiz::W) writer structure"]
impl crate::Writable for DOEP1_TSIZ {}
#[doc = "Device OUT Endpoint x+1 Transfer Size Register"]
pub mod doep1_tsiz;
#[doc = "Device OUT Endpoint x+1 DMA Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [doep1_dmaaddr](doep1_dmaaddr) module"]
pub type DOEP1_DMAADDR = crate::Reg<u32, _DOEP1_DMAADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEP1_DMAADDR;
#[doc = "`read()` method returns [doep1_dmaaddr::R](doep1_dmaaddr::R) reader structure"]
impl crate::Readable for DOEP1_DMAADDR {}
#[doc = "`write(|w| ..)` method takes [doep1_dmaaddr::W](doep1_dmaaddr::W) writer structure"]
impl crate::Writable for DOEP1_DMAADDR {}
#[doc = "Device OUT Endpoint x+1 DMA Address Register"]
pub mod doep1_dmaaddr;
#[doc = "Device OUT Endpoint x+1 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [doep2_ctl](doep2_ctl) module"]
pub type DOEP2_CTL = crate::Reg<u32, _DOEP2_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEP2_CTL;
#[doc = "`read()` method returns [doep2_ctl::R](doep2_ctl::R) reader structure"]
impl crate::Readable for DOEP2_CTL {}
#[doc = "`write(|w| ..)` method takes [doep2_ctl::W](doep2_ctl::W) writer structure"]
impl crate::Writable for DOEP2_CTL {}
#[doc = "Device OUT Endpoint x+1 Control Register"]
pub mod doep2_ctl;
#[doc = "Device OUT Endpoint x+1 Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [doep2_int](doep2_int) module"]
pub type DOEP2_INT = crate::Reg<u32, _DOEP2_INT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEP2_INT;
#[doc = "`read()` method returns [doep2_int::R](doep2_int::R) reader structure"]
impl crate::Readable for DOEP2_INT {}
#[doc = "`write(|w| ..)` method takes [doep2_int::W](doep2_int::W) writer structure"]
impl crate::Writable for DOEP2_INT {}
#[doc = "Device OUT Endpoint x+1 Interrupt Register"]
pub mod doep2_int;
#[doc = "Device OUT Endpoint x+1 Transfer Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [doep2_tsiz](doep2_tsiz) module"]
pub type DOEP2_TSIZ = crate::Reg<u32, _DOEP2_TSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEP2_TSIZ;
#[doc = "`read()` method returns [doep2_tsiz::R](doep2_tsiz::R) reader structure"]
impl crate::Readable for DOEP2_TSIZ {}
#[doc = "`write(|w| ..)` method takes [doep2_tsiz::W](doep2_tsiz::W) writer structure"]
impl crate::Writable for DOEP2_TSIZ {}
#[doc = "Device OUT Endpoint x+1 Transfer Size Register"]
pub mod doep2_tsiz;
#[doc = "Device OUT Endpoint x+1 DMA Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [doep2_dmaaddr](doep2_dmaaddr) module"]
pub type DOEP2_DMAADDR = crate::Reg<u32, _DOEP2_DMAADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEP2_DMAADDR;
#[doc = "`read()` method returns [doep2_dmaaddr::R](doep2_dmaaddr::R) reader structure"]
impl crate::Readable for DOEP2_DMAADDR {}
#[doc = "`write(|w| ..)` method takes [doep2_dmaaddr::W](doep2_dmaaddr::W) writer structure"]
impl crate::Writable for DOEP2_DMAADDR {}
#[doc = "Device OUT Endpoint x+1 DMA Address Register"]
pub mod doep2_dmaaddr;
#[doc = "Power and Clock Gating Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcgcctl](pcgcctl) module"]
pub type PCGCCTL = crate::Reg<u32, _PCGCCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCGCCTL;
#[doc = "`read()` method returns [pcgcctl::R](pcgcctl::R) reader structure"]
impl crate::Readable for PCGCCTL {}
#[doc = "`write(|w| ..)` method takes [pcgcctl::W](pcgcctl::W) writer structure"]
impl crate::Writable for PCGCCTL {}
#[doc = "Power and Clock Gating Control Register"]
pub mod pcgcctl;

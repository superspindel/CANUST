/*
use stm32f0x::{CAN};
use canust::CAN_MESSAGE;

impl can_hal_trait for CAN_HAL {
    //Write to the mcr register
    fn write_mcr(&self, can_mcr: CAN_MCR) {
        let can_reg = self.0;
        if can_mcr.DBF.is_some(){ can_reg.can_mcr.modify(|_, w| w.dbf().bit(can_mcr.DBF)) };
        if can_mcr.RESET.is_some(){ can_reg.can_mcr.modify(|_, w| w.reset().bit(can_mcr.RESET)) };
        if can_mcr.TTCM.is_some(){ can_reg.can_mcr.modify(|_, w| w.ttcm().bit(can_mcr.TTCM)) };
        if can_mcr.ABOM.is_some(){ can_reg.can_mcr.modify(|_, w| w.abom().bit(can_mcr.ABOM)) };
        if can_mcr.AWUM.is_some(){ can_reg.can_mcr.modify(|_, w| w.awum().bit(can_mcr.AWUM)) };
        if can_mcr.NART.is_some(){ can_reg.can_mcr.modify(|_, w| w.nart().bit(can_mcr.NART)) };
        if can_mcr.RFLM.is_some(){ can_reg.can_mcr.modify(|_, w| w.rflm().bit(can_mcr.RFLM)) };
        if can_mcr.TXFP.is_some(){ can_reg.can_mcr.modify(|_, w| w.txfp().bit(can_mcr.TXFP)) };
        if can_mcr.SLEEP.is_some(){ can_reg.can_mcr.modify(|_, w| w.sleep().bit(can_mcr.SLEEP)) };
        if can_mcr.INRQ.is_some(){ can_reg.can_mcr.modify(|_, w| w.inrq().bit(can_mcr.INRQ)) };
    }

    //Read from mcr register
    fn read_mcr(&self) -> CAN_MCR {
        let can_reg = self.0;
        CAN_MCR {
            DBF: can_reg.can_mcr.read().dbf().bit(),
            RESET: can_reg.can_mcr.read().reset().bit(),
            TTCM: can_reg.can_mcr.read().ttcm().bit(),
            ABOM: can_reg.can_mcr.read().abom().bit(),
            AWUM: can_reg.can_mcr.read().awum().bit(),
            NART: can_reg.can_mcr.read().nart().bit(),
            RFLM: can_reg.can_mcr.read().rflm().bit(),
            TXFP: can_reg.can_mcr.read().txfp().bit(),
            SLEEP: can_reg.can_mcr.read().sleep().bit(),
            INRQ: can_reg.can_mcr.read().inrq().bit(),
        }
    }

    fn write_msr(&self, can_msr: CAN_MSR) {
        can_reg = self.0;
        if can_msr.SLAKI.is_some(){ can_reg.can_msr.modify(|_, w| w.slaki().bit(can_msr.SLAKI)) };
        if can_msr.WKUI.is_some(){ can_reg.can_msr.modify(|_, w| w.wkui().bit(can_msr.WKUI)) };
        if can_msr.ERRI.is_some(){ can_reg.can_msr.modify(|_, w| w.erri().bit(can_msr.ERRI)) };
    }

    fn read_msr(&self) -> CAN_MSR {
        let can_reg = self.0;
        can_msr = CAN_MSR {
            RX: can_reg.can_msr.read().rx().bit(),
            SAMP: can_reg.can_msr.read().samp().bit(),
            RXM: can_reg.can_msr.read().rxm().bit(),
            TXM: can_reg.can_msr.read().txm().bit(),
            SLAKI: can_reg.can_msr.read().slaki().bit(),
            WKUI: can_reg.can_msr.read().wkui().bit(),
            ERRI: can_reg.can_msr.read().erri().bit(),
            SLAK: can_reg.can_msr.read().slak().bit(),
            INAK: can_reg.can_msr.read().inak().bit(),
        }
    }

    fn write_tsr(&self, can_tsr: CAN_TSR) {
        let can_reg = self.0;
        if can_tsr.ABRQ2.is_some(){ can_reg.can_tsr.modify(|_, w| w.abrq2().bit(can_tsr.ABRQ2)) };
        if can_tsr.TERR2.is_some(){ can_reg.can_tsr.modify(|_, w| w.terr2().bit(can_tsr.TERR2)) };
        if can_tsr.ALST2.is_some(){ can_reg.can_tsr.modify(|_, w| w.alst2().bit(can_tsr.ALST2)) };
        if can_tsr.TXOK2.is_some(){ can_reg.can_tsr.modify(|_, w| w.txok2().bit(can_tsr.TXOK2)) };
        if can_tsr.RQCP2.is_some(){ can_reg.can_tsr.modify(|_, w| w.rqcp2().bit(can_tsr.RQCP2)) };
  
        if can_tsr.ABRQ1.is_some(){ can_reg.can_tsr.modify(|_, w| w.abrq1().bit(can_tsr.ABRQ1)) };
        if can_tsr.TERR1.is_some(){ can_reg.can_tsr.modify(|_, w| w.terr1().bit(can_tsr.TERR1)) };
        if can_tsr.ALST1.is_some(){ can_reg.can_tsr.modify(|_, w| w.alst1().bit(can_tsr.ALST1)) };
        if can_tsr.TXOK1.is_some(){ can_reg.can_tsr.modify(|_, w| w.txok1().bit(can_tsr.TXOK1)) };
        if can_tsr.RQCP1.is_some(){ can_reg.can_tsr.modify(|_, w| w.rqcp1().bit(can_tsr.RQCP1)) };
  
        if can_tsr.ABRQ0.is_some(){ can_reg.can_tsr.modify(|_, w| w.abrq0().bit(can_tsr.ABRQ0)) };
        if can_tsr.TERR0.is_some(){ can_reg.can_tsr.modify(|_, w| w.terr0().bit(can_tsr.TERR0)) };
        if can_tsr.ALST0.is_some(){ can_reg.can_tsr.modify(|_, w| w.alst0().bit(can_tsr.ALST0)) };
        if can_tsr.TXOK0.is_some(){ can_reg.can_tsr.modify(|_, w| w.txok0().bit(can_tsr.TXOK0)) };
        if can_tsr.RQCP0.is_some(){ can_reg.can_tsr.modify(|_, w| w.rqcp0().bit(can_tsr.RQCP0)) };
    }

    fn read_tsr(&self) -> CAN_TSR {
        let can_reg = self.0;
        can_tsr = CAN_TSR {
            LOW2: can_reg.can_tsr.read().low2().bit(),
            LOW1: can_reg.can_tsr.read().low1().bit(),
            LOW0: can_reg.can_tsr.read().low0().bit(),

            TME2: can_reg.can_tsr.read().tme2().bit(),
            TME1: can_reg.can_tsr.read().tme1().bit(),
            TME0: can_reg.can_tsr.read().tme0().bit(),
        
            CODE: can_reg.can_tsr.read().code().bit(),

            ABRQ2: can_reg.can_tsr.read().abrq2().bit(),
            TERR2: can_reg.can_tsr.read().terr2().bit(),
            ALST2: can_reg.can_tsr.read().alst2().bit(),
            TXOK2: can_reg.can_tsr.read().txok2().bit(),
            RQCP2: can_reg.can_tsr.read().rqcp2().bit(),

            ABRQ1: can_reg.can_tsr.read().abrq1().bit(),
            TERR1: can_reg.can_tsr.read().terr1().bit(),
            ALST1: can_reg.can_tsr.read().alst1().bit(),
            TXOK1: can_reg.can_tsr.read().txok1().bit(),
            RQCP1: can_reg.can_tsr.read().rqcp1().bit(),

            ABRQ0: can_reg.can_tsr.read().abrq0().bit(),
            TERR0: can_reg.can_tsr.read().terr0().bit(),
            ALST0: can_reg.can_tsr.read().alst0().bit(),
            TXOK0: can_reg.can_tsr.read().txok0().bit(),
            RQCP0: can_reg.can_tsr.read().rqcp0().bit(),
        }
    }

    fn write_rf0r(&self, can_rf0r: CAN_RF0R) {
        let can_reg = self.0;
        if can_rf0r.RFOM0.is_some(){ can_reg.can_rf0r.modify(|_, w| w.rfom0().bit(can_rf0r.RFOM0)) };
        if can_rf0r.FOVR0.is_some(){ can_reg.can_rf0r.modify(|_, w| w.fovr0().bit(can_rf0r.FOVR0)) };
        if can_rf0r.FULL0.is_some(){ can_reg.can_rf0r.modify(|_, w| w.full0().bit(can_rf0r.FULL0)) };
    }

    fn read_rf0r(&self) -> CAN_RF0R {
        let can_reg= self.0;
        CAN_RF0R {
            rfom0: can_reg.can_rf0r.read().rfom0().bit(),
            fovr0: can_reg.can_rf0r.read().fovr0().bit(),
            full0: can_reg.can_rf0r.read().full0().bit(),
            fmp0: can_reg.can_rf0r.read().fmp0().bits(),
        }
    }

    fn write_rf1r(&self, can_rf1r: CAN_RF1R) {
        let can_reg = self.0;
        if can_rf1r.RFOM1.is_some(){ can_reg.can_rf1r.modify(|_, w| w.rfom1().bit(can_rf1r.RFOM1)) };
        if can_rf1r.FOVR1.is_some(){ can_reg.can_rf1r.modify(|_, w| w.fovr1().bit(can_rf1r.FOVR1)) };
        if can_rf1r.FULL1.is_some(){ can_reg.can_rf1r.modify(|_, w| w.full1().bit(can_rf1r.FULL1)) };
    }

    fn read_rf1r(&self) -> CAN_RF1R {
        let can_reg= self.0;

        can_rf1r = CAN_RF1R {
            rfom1: can_reg.can_rf1r.read().rfom1().bit(),
            fovr1: can_reg.can_rf1r.read().fovr1().bit(),
            full1: can_reg.can_rf1r.read().full1().bit(),
            fmp1: can_reg.can_rf1r.read().fmp1().bits(),
        }
    }

    fn write_ier(&self, can_ier: CAN_IER) {
        let can_reg = self.0;
        if can_ier.SLKIE.is_some(){ can_reg.can_ier.modify(|_, w| w.slkie().bit(can_rf1r.SLKIE)) };
        if can_ier.WKUIE.is_some(){ can_reg.can_ier.modify(|_, w| w.wkuie().bit(can_rf1r.WKUIE)) };
        if can_ier.ERRIE.is_some(){ can_reg.can_ier.modify(|_, w| w.errie().bit(can_rf1r.ERRIE)) };
        if can_ier.LECIE.is_some(){ can_reg.can_ier.modify(|_, w| w.lecie().bit(can_rf1r.LECIE)) };
        if can_ier.BOFIE.is_some(){ can_reg.can_ier.modify(|_, w| w.bofie().bit(can_rf1r.BOFIE)) };
        if can_ier.EPVIE.is_some(){ can_reg.can_ier.modify(|_, w| w.epvie().bit(can_rf1r.EPVIE)) };
        if can_ier.EWGIE.is_some(){ can_reg.can_ier.modify(|_, w| w.ewgie().bit(can_rf1r.EWGIE)) };
        if can_ier.FOVIE1.is_some(){ can_reg.can_ier.modify(|_, w| w.fovie1().bit(can_rf1r.FOVIE1)) };
        if can_ier.FFIE1.is_some(){ can_reg.can_ier.modify(|_, w| w.ffie1().bit(can_rf1r.FFIE1)) };
        if can_ier.FMPIE1.is_some(){ can_reg.can_ier.modify(|_, w| w.fmpie1().bit(can_rf1r.FMPIE1)) };
        if can_ier.FOVIE0.is_some(){ can_reg.can_ier.modify(|_, w| w.fovie0().bit(can_rf1r.FOVIE0)) };
        if can_ier.FFIE0.is_some(){ can_reg.can_ier.modify(|_, w| w.ffie0().bit(can_rf1r.FFIE0)) };
        if can_ier.FMPIE0.is_some(){ can_reg.can_ier.modify(|_, w| w.fmpie0().bit(can_rf1r.FMPIE0)) };
        if can_ier.FOVIE1.is_some(){ can_reg.can_ier.modify(|_, w| w.fovie1().bit(can_rf1r.FOVIE1)) };
        if can_ier.FFIE1.is_some(){ can_reg.can_ier.modify(|_, w| w.ffie1().bit(can_rf1r.FFIE1)) };
        if can_ier.TMEIE.is_some(){ can_reg.can_ier.modify(|_, w| w.tmeie().bit(can_rf1r.TMEIE)) };
    }

    fn read_ier(&self) -> CAN_IER {
        let can_reg = self.0;
        CAN_IER {
            SLKIE: can_reg.can_ier.read().slkie().bit(),
            WKUIE: can_reg.can_ier.read().wkuie().bit(),
            ERRIE: can_reg.can_ier.read().errie().bit(),
            LECIE: can_reg.can_ier.read().lecie().bit(),
            BOFIE: can_reg.can_ier.read().bofie().bit(),
            EPVIE: can_reg.can_ier.read().epvie().bit(),
            EWGIE: can_reg.can_ier.read().ewgie().bit(),

            FOVIE1: can_reg.can_ier.read().fovie1().bit(),
            FFIE1: can_reg.can_ier.read().ffie1().bit(),
            FMPIE1: can_reg.can_ier.read().fmpie1().bit(),

            FOVIE0: can_reg.can_ier.read().fovie0().bit(),
            FFIE0: can_reg.can_ier.read().ffie0().bit(),
            FMPIE0: can_reg.can_ier.read().fmpie0().bit(),

            TMEIE: can_reg.can_ier.read().tmeie().bit(),
        }
    }

    fn write_esr(&self, can_esr: CAN_ESR) {
        let can_reg = self.0;
        if can_esr.LEC.is_some(){ can_reg.can_esr.modify(|_, w| w.lec().bit(can_esr.LEC)) };
    }

    fn read_esr(&self) -> CAN_ESR {
        let can_reg = self.0;

        CAN_ESR {
            REC: can_reg.can_esr.read().rec().bits(),
            TEC: can_reg.can_esr.read().tec().bits(),
            LEC: can_reg.can_esr.read().lec().bits(),
            BOFF: can_reg.can_esr.read().boff().bit(),
            EPVF: can_reg.can_esr.read().epvf().bit(),
            EWGF: can_reg.can_esr.read().ewgf().bit(),
        }
    }

    fn write_btr(&self, can_btr: CAN_BTR) {
        let can_reg = self.0;
        if can_btr.SILM.is_some(){ can_reg.can_btr.modify(|_, w| w.silm().bit(can_btr.SILM)) };
        if can_btr.LBKM.is_some(){ can_reg.can_btr.modify(|_, w| w.lbkm().bit(can_btr.LBKM)) };
        if can_btr.SJW.is_some(){ can_reg.can_btr.modify(|_, w| w.sjw().bits(can_btr.SJW)) };
        if can_btr.TS2.is_some(){ can_reg.can_btr.modify(|_, w| w.ts2().bits(can_btr.TS2)) };
        if can_btr.TS1.is_some(){ can_reg.can_btr.modify(|_, w| w.ts1().bits(can_btr.TS1)) };
        if can_btr.BRP.is_some(){ can_reg.can_btr.modify(|_, w| w.brp().bits(can_btr.BRP)) };
    }

    fn read_btr(&self) -> CAN_BTR {
        let can_reg = self.0;

        CAN_BTR {
            SILM: can_reg.can_btr.read().silm().bit(),
            LBKM: can_reg.can_btr.read().lbkm().bit(),
            SJW: can_reg.can_btr.read().sjw().bits(),
            TS2: can_reg.can_btr.read().ts2().bits(),
            TS1: can_reg.can_btr.read().ts1().bits(),
            BRP: can_reg.can_btr.read().brp().bits(),
        }
    }

    fn write_ti2r(&self, can_tixr: CAN_TIxR) {
        let can_reg = self.0;
        if can_tixr.STID.is_some(){ can_reg.can_ti2r.modify(|_, w| w.stid().bits(can_tixr.STID)) };
        if can_tixr.EXID.is_some(){ can_reg.can_ti2r.modify(|_, w| w.exid().bits(can_tixr.EXID)) };
        if can_tixr.IDE.is_some(){ can_reg.can_ti2r.modify(|_, w| w.ide().bit(can_tixr.IDE)) };
        if can_tixr.RTR.is_some(){ can_reg.can_ti2r.modify(|_, w| w.rtr().bit(can_tixr.RTR)) };
        if can_tixr.TXRQ.is_some(){ can_reg.can_ti2r.modify(|_, w| w.txrq().bit(can_tixr.TXRQ)) };
    }

    fn read_ti2r(&self) -> CAN_TIxR {
        let can_reg = self.0;

        CAN_TIxR {
            STID: can_reg.can_ti2r.read().stid().bits(),
            EXID: can_reg.can_ti2r.read().exid().bits(),
            IDE: can_reg.can_ti2r.read().ide().bit(),
            RTR: can_reg.can_ti2r.read().rtr().bit(),
            TXRQ: can_reg.can_ti2r.read().txrq().bit(),
        }
    }

    fn write_ti1r(&self, can_tixr: CAN_TIxR) {
        let can_reg = self.0;
        if can_tixr.STID.is_some(){ can_reg.can_ti1r.modify(|_, w| w.stid().bits(can_tixr.STID)) };
        if can_tixr.EXID.is_some(){ can_reg.can_ti1r.modify(|_, w| w.exid().bits(can_tixr.EXID)) };
        if can_tixr.IDE.is_some(){ can_reg.can_ti1r.modify(|_, w| w.ide().bit(can_tixr.IDE)) };
        if can_tixr.RTR.is_some(){ can_reg.can_ti1r.modify(|_, w| w.rtr().bit(can_tixr.RTR)) };
        if can_tixr.TXRQ.is_some(){ can_reg.can_ti1r.modify(|_, w| w.txrq().bit(can_tixr.TXRQ)) };
    }

    fn read_ti1r(&self) -> CAN_TIxR {
        let can_reg = self.0;

        CAN_TIxR {
            STID: can_reg.can_ti1r.read().stid().bits(),
            EXID: can_reg.can_ti1r.read().exid().bits(),
            IDE: can_reg.can_ti1r.read().ide().bit(),
            RTR: can_reg.can_ti1r.read().rtr().bit(),
            TXRQ: can_reg.can_ti1r.read().txrq().bit(),
        }
    }

    fn write_ti0r(&self, can_tixr: CAN_TIxR) {
        let can_reg = self.0;
        if can_tixr.STID.is_some(){ can_reg.can_ti0r.modify(|_, w| w.stid().bits(can_tixr.STID)) };
        if can_tixr.EXID.is_some(){ can_reg.can_ti0r.modify(|_, w| w.exid().bits(can_tixr.EXID)) };
        if can_tixr.IDE.is_some(){ can_reg.can_ti0r.modify(|_, w| w.ide().bit(can_tixr.IDE)) };
        if can_tixr.RTR.is_some(){ can_reg.can_ti0r.modify(|_, w| w.rtr().bit(can_tixr.RTR)) };
        if can_tixr.TXRQ.is_some(){ can_reg.can_ti0r.modify(|_, w| w.txrq().bit(can_tixr.TXRQ)) };
    }

    fn read_ti0r(&self) -> CAN_TIxR {
        let can_reg = self.0;

        CAN_TIxR {
            STID: can_reg.can_ti0r.read().stid().bits(),
            EXID: can_reg.can_ti0r.read().exid().bits(),
            IDE: can_reg.can_ti0r.read().ide().bit(),
            RTR: can_reg.can_ti0r.read().rtr().bit(),
            TXRQ: can_reg.can_ti0r.read().txrq().bit(),
        }
    }

    fn write_tdt0r(&self, can_tdtxr: CAN_TDTxR) {
        let can_reg = self.0;
        if can_tdtxr.TIME.is_some(){ can_reg.can_tdt0r.modify(|_, w| w.time().bits(can_tdtxr.TIME)) };
        if can_tdtxr.TGT.is_some(){ can_reg.can_tdt0r.modify(|_, w| w.tgt().bit(can_tdtxr.TGT)) };
        if can_tdtxr.DLC.is_some(){ can_reg.can_tdt0r.modify(|_, w| w.dlc().bits(can_tdtxr.DLC)) };
    }

    fn read_tdt0r(&self) -> CAN_TDTxR {
        let can_reg = self.0;

        CAN_TDTxR {
            TIME: can_reg.can_tdt0r.read().time().bits(),
            TGT: can_reg.can_tdt0r.read().tgt().bit(),
            DLC: can_reg.can_tdt0r.read().dlc().bits(),
        }
    }

    fn write_tdt1r(&self, can_tdtxr: CAN_TDTxR) {
        let can_reg = self.0;
        if can_tdtxr.TIME.is_some(){ can_reg.can_tdt1r.modify(|_, w| w.time().bits(can_tdtxr.TIME)) };
        if can_tdtxr.TGT.is_some(){ can_reg.can_tdt1r.modify(|_, w| w.tgt().bit(can_tdtxr.TGT)) };
        if can_tdtxr.DLC.is_some(){ can_reg.can_tdt1r.modify(|_, w| w.dlc().bits(can_tdtxr.DLC)) };
    }

    fn read_tdt1r(&self) -> CAN_TDTxR {
        let can_reg = self.0;

        CAN_TDTxR {
            TIME: can_reg.can_tdt1r.read().time().bits(),
            TGT: can_reg.can_tdt1r.read().tgt().bit(),
            DLC: can_reg.can_tdt1r.read().dlc().bits(),
        }
    }

    fn write_tdt2r(&self, can_tdtxr: CAN_TDTxR) {
        let can_reg = self.0;
        if can_tdtxr.TIME.is_some(){ can_reg.can_tdt2r.modify(|_, w| w.time().bits(can_tdtxr.TIME)) };
        if can_tdtxr.TGT.is_some(){ can_reg.can_tdt2r.modify(|_, w| w.tgt().bit(can_tdtxr.TGT)) };
        if can_tdtxr.DLC.is_some(){ can_reg.can_tdt2r.modify(|_, w| w.dlc().bits(can_tdtxr.DLC)) };
    }

    fn read_tdt2r(&self) -> CAN_TDTxR {
        let can_reg = self.0;

        CAN_TDTxR {
            TIME: can_reg.can_tdt0r.read().time().bits(),
            TGT: can_reg.can_tdt0r.read().tgt().bit(),
            DLC: can_reg.can_tdt0r.read().dlc().bits(),
        }
    }

    fn write_tdl0r(&self, can_tdlxr: CAN_TDLxR) {
        let can_reg = self.0;
        if can_tdlxr.DATA0.is_some(){ can_reg.can_tdl0r.modify(|_, w| w.data0().bits(can_tdlxr.DATA0)) };
        if can_tdlxr.DATA1.is_some(){ can_reg.can_tdl0r.modify(|_, w| w.data1().bits(can_tdlxr.DATA1)) };
        if can_tdlxr.DATA2.is_some(){ can_reg.can_tdl0r.modify(|_, w| w.data2().bits(can_tdlxr.DATA2)) };
        if can_tdlxr.DATA3.is_some(){ can_reg.can_tdl0r.modify(|_, w| w.data3().bits(can_tdlxr.DATA3)) };
    }

    fn read_tdl0r(&self) -> CAN_TDLxR {
        let can_reg = self.0;
        
        CAN_TDLxR {
            DATA0: can_reg.can_tdl0r.read().data0().bits(),
            DATA1: can_reg.can_tdl0r.read().data1().bits(),
            DATA2: can_reg.can_tdl0r.read().data2().bits(),
            DATA3: can_reg.can_tdl0r.read().data3().bits(),
        }
    }

    fn write_tdl1r(&self, can_tdlxr: CAN_TDLxR) {
        let can_reg = self.0;
        if can_tdlxr.DATA0.is_some(){ can_reg.can_tdl1r.modify(|_, w| w.data0().bits(can_tdlxr.DATA0)) };
        if can_tdlxr.DATA1.is_some(){ can_reg.can_tdl1r.modify(|_, w| w.data1().bits(can_tdlxr.DATA1)) };
        if can_tdlxr.DATA2.is_some(){ can_reg.can_tdl1r.modify(|_, w| w.data2().bits(can_tdlxr.DATA2)) };
        if can_tdlxr.DATA3.is_some(){ can_reg.can_tdl1r.modify(|_, w| w.data3().bits(can_tdlxr.DATA3)) };
    }

    fn read_tdl1r(&self) -> CAN_TDLxR {
        let can_reg = self.0;
        
        CAN_TDLxR {
            DATA0: can_reg.can_tdl1r.read().data0().bits(),
            DATA1: can_reg.can_tdl1r.read().data1().bits(),
            DATA2: can_reg.can_tdl1r.read().data2().bits(),
            DATA3: can_reg.can_tdl1r.read().data3().bits(),
        }
    }

    fn write_tdl2r(&self, can_tdlxr: CAN_TDLxR) {
        let can_reg = self.0;
        if can_tdlxr.DATA0.is_some(){ can_reg.can_tdl2r.modify(|_, w| w.data0().bits(can_tdlxr.DATA0)) };
        if can_tdlxr.DATA1.is_some(){ can_reg.can_tdl2r.modify(|_, w| w.data1().bits(can_tdlxr.DATA1)) };
        if can_tdlxr.DATA2.is_some(){ can_reg.can_tdl2r.modify(|_, w| w.data2().bits(can_tdlxr.DATA2)) };
        if can_tdlxr.DATA3.is_some(){ can_reg.can_tdl2r.modify(|_, w| w.data3().bits(can_tdlxr.DATA3)) };
    }

    fn read_tdl2r(&self) -> CAN_TDLxR {
        let can_reg = self.0;
        
        CAN_TDLxR {
            DATA0: can_reg.can_tdl2r.read().data0().bits(),
            DATA1: can_reg.can_tdl2r.read().data1().bits(),
            DATA2: can_reg.can_tdl2r.read().data2().bits(),
            DATA3: can_reg.can_tdl2r.read().data3().bits(),
        }
    }

    fn write_tdh0r(&self, can_tdhxr: CAN_TDHxR) {
        let can_reg = self.0;
        if can_tdhxr.DATA4.is_some(){ can_reg.can_tdh0r.modify(|_, w| w.data4().bits(can_tdhxr.DATA4)) };
        if can_tdhxr.DATA5.is_some(){ can_reg.can_tdh0r.modify(|_, w| w.data5().bits(can_tdhxr.DATA5)) };
        if can_tdhxr.DATA6.is_some(){ can_reg.can_tdh0r.modify(|_, w| w.data6().bits(can_tdhxr.DATA6)) };
        if can_tdhxr.DATA7.is_some(){ can_reg.can_tdh0r.modify(|_, w| w.data7().bits(can_tdhxr.DATA7)) };
    }

    fn read_tdh0r(&self) -> CAN_TDHxR {
        let can_reg = self.0;
        
        CAN_TDHxR {
            DATA4: can_reg.can_tdh0r.read().data4().bits(),
            DATA5: can_reg.can_tdh0r.read().data5().bits(),
            DATA6: can_reg.can_tdh0r.read().data6().bits(),
            DATA7: can_reg.can_tdh0r.read().data7().bits(),
        }
    }

    fn write_tdh1r(&self, can_tdhxr: CAN_TDHxR) {
        let can_reg = self.0;
        if can_tdhxr.DATA4.is_some(){ can_reg.can_tdh1r.modify(|_, w| w.data4().bits(can_tdhxr.DATA4)) };
        if can_tdhxr.DATA5.is_some(){ can_reg.can_tdh1r.modify(|_, w| w.data5().bits(can_tdhxr.DATA5)) };
        if can_tdhxr.DATA6.is_some(){ can_reg.can_tdh1r.modify(|_, w| w.data6().bits(can_tdhxr.DATA6)) };
        if can_tdhxr.DATA7.is_some(){ can_reg.can_tdh1r.modify(|_, w| w.data7().bits(can_tdhxr.DATA7)) };
    }

    fn read_tdh1r(&self) -> CAN_TDHxR {
        let can_reg = self.0;
        
        CAN_TDHxR {
            DATA4: can_reg.can_tdh1r.read().data4().bits(),
            DATA5: can_reg.can_tdh1r.read().data5().bits(),
            DATA6: can_reg.can_tdh1r.read().data6().bits(),
            DATA7: can_reg.can_tdh1r.read().data7().bits(),
        }
    }

    fn write_tdh2r(&self, can_tdhxr: CAN_TDHxR) {
        let can_reg = self.0;
        if can_tdhxr.DATA4.is_some(){ can_reg.can_tdh2r.modify(|_, w| w.data4().bits(can_tdhxr.DATA4)) };
        if can_tdhxr.DATA5.is_some(){ can_reg.can_tdh2r.modify(|_, w| w.data5().bits(can_tdhxr.DATA5)) };
        if can_tdhxr.DATA6.is_some(){ can_reg.can_tdh2r.modify(|_, w| w.data6().bits(can_tdhxr.DATA6)) };
        if can_tdhxr.DATA7.is_some(){ can_reg.can_tdh2r.modify(|_, w| w.data7().bits(can_tdhxr.DATA7)) };
    }

    fn read_tdh2r(&self) -> CAN_TDHxR {
        let can_reg = self.0;
        
        CAN_TDHxR {
            DATA4: can_reg.can_tdh2r.read().data4().bits(),
            DATA5: can_reg.can_tdh2r.read().data5().bits(),
            DATA6: can_reg.can_tdh2r.read().data6().bits(),
            DATA7: can_reg.can_tdh2r.read().data7().bits(),
        }
    }

    fn write_ri0r(&self, can_rixr: CAN_RIxR) {
        let can_reg = self.0;
        if can_rixr.STID.is_some(){ can_reg.can_ri0r.modify(|_, w| w.stid().bits(can_rixr.STID)) };
        if can_rixr.EXID.is_some(){ can_reg.can_ri0r.modify(|_, w| w.exid().bits(can_rixr.EXID)) };
        if can_rixr.IDE.is_some(){ can_reg.can_ri0r.modify(|_, w| w.ide().bits(can_rixr.IDE)) };
        if can_rixr.RTR.is_some(){ can_reg.can_ri0r.modify(|_, w| w.rtr().bits(can_rixr.RTR)) };
    }

    fn read_ri0r(&self) -> CAN_RIxR {
        let can_reg = self.0;

        CAN_RIxR {
            STID: can_reg.can_ri0r.read().stid().bits(),
            EXID: can_reg.can_ri0r.read().exid().bits(),
            IDE: can_reg.can_ri0r.read().ide().bits(),
            RTR: can_reg.can_ri0r.read().rtr().bits(),
        }
    }

    fn write_ri1r(&self, can_rixr: CAN_RIxR) {
        let can_reg = self.0;
        if can_rixr.STID.is_some(){ can_reg.can_ri1r.modify(|_, w| w.stid().bits(can_rixr.STID)) };
        if can_rixr.EXID.is_some(){ can_reg.can_ri1r.modify(|_, w| w.exid().bits(can_rixr.EXID)) };
        if can_rixr.IDE.is_some(){ can_reg.can_ri1r.modify(|_, w| w.ide().bits(can_rixr.IDE)) };
        if can_rixr.RTR.is_some(){ can_reg.can_ri1r.modify(|_, w| w.rtr().bits(can_rixr.RTR)) };
    }
    
    fn read_ri1r(&self) -> CAN_RIxR {
        let can_reg = self.0;

        CAN_RIxR {
            STID: can_reg.can_ri1r.read().stid().bits(),
            EXID: can_reg.can_ri1r.read().exid().bits(),
            IDE: can_reg.can_ri1r.read().ide().bits(),
            RTR: can_reg.can_ri1r.read().rtr().bits(),
        }
    }

    fn write_rdt0r(&self, can_rdtxr: CAN_RDTxR) {
        let can_reg = self.0;
        if can_rdtxr.TIME.is_some(){ can_reg.can_rdt0r.modify(|_, w| w.time().bits(can_rdtxr.TIME)) };
        if can_rdtxr.FMI.is_some(){ can_reg.can_rdt0r.modify(|_, w| w.fmi().bits(can_rdtxr.FMI)) };
        if can_rdtxr.DLC.is_some(){ can_reg.can_rdt0r.modify(|_, w| w.dlc().bits(can_rdtxr.DLC)) };
    }

    fn read_rdt0r(&self) -> CAN_RDTxR {
        let can_reg = self.0;

        CAN_RDTxR {
            TIME: can_reg.can_rdt0r.read().time().bits(),
            FMI: can_reg.can_rdt0r.read().fmi().bits(),
            DLC: can_reg.can_rdt0r.read().dlc().bits(),
        }
    }

    fn write_rdt1r(&self, can_rdtxr: CAN_RDTxR) {
        let can_reg = self.0;
        if can_rdtxr.TIME.is_some(){ can_reg.can_rdt1r.modify(|_, w| w.time().bits(can_rdtxr.TIME)) };
        if can_rdtxr.FMI.is_some(){ can_reg.can_rdt1r.modify(|_, w| w.fmi().bits(can_rdtxr.FMI)) };
        if can_rdtxr.DLC.is_some(){ can_reg.can_rdt1r.modify(|_, w| w.dlc().bits(can_rdtxr.DLC)) };
    }

    fn read_rdt1r(&self) -> CAN_RDTxR {
        let can_reg = self.0;

        CAN_RDTxR {
            TIME: can_reg.can_rdt1r.read().time().bits(),
            FMI: can_reg.can_rdt1r.read().fmi().bits(),
            DLC: can_reg.can_rdt1r.read().dlc().bits(),
        }
    }

    fn write_rdl0r(&self, can_rdlxr: CAN_RDLxR) {
        let can_reg = self.0;

        if can_rdlxr.DATA0.is_some(){ can_reg.can_rdl0r.modify(|_, w| w.data0().bits(can_rdlxr.DATA0)) };
        if can_rdlxr.DATA1.is_some(){ can_reg.can_rdl0r.modify(|_, w| w.data1().bits(can_rdlxr.DATA1)) };
        if can_rdlxr.DATA2.is_some(){ can_reg.can_rdl0r.modify(|_, w| w.data2().bits(can_rdlxr.DATA2)) };
        if can_rdlxr.DATA3.is_some(){ can_reg.can_rdl0r.modify(|_, w| w.data3().bits(can_rdlxr.DATA3)) };
    }

    fn read_rdl0r(&self) -> CAN_RDLxR {
        let can_reg = self.0;

        CAN_RDLxR {
            DATA0: can_reg.can_rdl0r.read().data0().bits(),
            DATA1: can_reg.can_rdl0r.read().data1().bits(),
            DATA2: can_reg.can_rdl0r.read().data2().bits(),
            DATA3: can_reg.can_rdl0r.read().data3().bits(),
        }
    }

    fn write_rdl1r(&self, can_rdlxr: CAN_RDLxR) {
        let can_reg = self.0;

        if can_rdlxr.DATA0.is_some(){ can_reg.can_rdl1r.modify(|_, w| w.data0().bits(can_rdlxr.DATA0)) };
        if can_rdlxr.DATA1.is_some(){ can_reg.can_rdl1r.modify(|_, w| w.data1().bits(can_rdlxr.DATA1)) };
        if can_rdlxr.DATA2.is_some(){ can_reg.can_rdl1r.modify(|_, w| w.data2().bits(can_rdlxr.DATA2)) };
        if can_rdlxr.DATA3.is_some(){ can_reg.can_rdl1r.modify(|_, w| w.data3().bits(can_rdlxr.DATA3)) };
    }

    fn read_rdl1r(&self) -> CAN_RDLxR {
        let can_reg = self.0;

        CAN_RDLxR {
            DATA0: can_reg.can_rdl1r.read().data0().bits(),
            DATA1: can_reg.can_rdl1r.read().data1().bits(),
            DATA2: can_reg.can_rdl1r.read().data2().bits(),
            DATA3: can_reg.can_rdl1r.read().data3().bits(),
        }
    }

    fn write_rdh0r(&self, can_rdhxr: CAN_RDHxR) {
        let can_reg = self.0;

        if can_rdhxr.DATA4.is_some(){ can_reg.can_rdh0r.modify(|_, w| w.data0().bits(can_rdhxr.DATA4)) };
        if can_rdhxr.DATA5.is_some(){ can_reg.can_rdh0r.modify(|_, w| w.data1().bits(can_rdhxr.DATA5)) };
        if can_rdhxr.DATA6.is_some(){ can_reg.can_rdh0r.modify(|_, w| w.data2().bits(can_rdhxr.DATA6)) };
        if can_rdhxr.DATA7.is_some(){ can_reg.can_rdh0r.modify(|_, w| w.data3().bits(can_rdhxr.DATA7)) };
    }

    fn read_rdh0r(&self) -> CAN_RDHxR {
        let can_reg = self.0;

        CAN_RDHxR {
            DATA4: can_reg.can_rdh0r.read().data4().bits(),
            DATA5: can_reg.can_rdh0r.read().data5().bits(),
            DATA6: can_reg.can_rdh0r.read().data6().bits(),
            DATA7: can_reg.can_rdh0r.read().data7().bits(),
        }
    }

    fn write_rdh1r(&self, can_rdhxr: CAN_RDHxR) {
        let can_reg = self.0;

        if can_rdhxr.DATA4.is_some(){ can_reg.can_rdh1r.modify(|_, w| w.data0().bits(can_rdhxr.DATA4)) };
        if can_rdhxr.DATA5.is_some(){ can_reg.can_rdh1r.modify(|_, w| w.data1().bits(can_rdhxr.DATA5)) };
        if can_rdhxr.DATA6.is_some(){ can_reg.can_rdh1r.modify(|_, w| w.data2().bits(can_rdhxr.DATA6)) };
        if can_rdhxr.DATA7.is_some(){ can_reg.can_rdh1r.modify(|_, w| w.data3().bits(can_rdhxr.DATA7)) };
    }

    fn read_rdh1r(&self) -> CAN_RDHxR {
        let can_reg = self.0;

        CAN_RDHxR {
            DATA4: can_reg.can_rdh1r.read().data4().bits(),
            DATA5: can_reg.can_rdh1r.read().data5().bits(),
            DATA6: can_reg.can_rdh1r.read().data6().bits(),
            DATA7: can_reg.can_rdh1r.read().data7().bits(),
        }
    }

    fn write_fmr(&self, can_fmr: CAN_FMR) {
        let can_reg = self.0;
        if can_fmr.CAN2SB.is_some(){ can_reg.can_fmr.modify(|_, w| w.can2sb().bits(can_fmr.CAN2SB)) };
        if can_fmr.FINIT.is_some(){ can_reg.can_fmr.modify(|_, w| w.finit().bit(can_fmr.FINIT)) };
    }

    fn read_fmr(&self) -> CAN_FMR {
        let can_reg = self.0;
        
        CAN_FMR {
            CAN2SB: can_reg.can_fmr.read().can2sb().bits(),
            FINIT: can_reg.can_fmr.read().finit().bit(),
        }
    }
}

pub trait can_hal_trait{
    fn write_mcr(&self, CAN_MCR);
    fn read_mcr(&self) -> CAN_MCR;

    fn write_msr(&self, CAN_MSR);
    fn read_msr(&self) -> CAN_MSR;

    fn write_tsr(&self, CAN_TSR);
    fn read_tsr(&self) -> CAN_TSR;

    fn write_rf0r(&self, CAN_RF0R);
    fn read_rf0r(&self) -> CAN_RF0R;

    fn write_rf1r(&self, CAN_RF1R);
    fn read_rf1r(&self) -> CAN_RF1R;

    fn write_ier(&self, CAN_IER);
    fn read_ier(&self) -> CAN_IER;

    fn write_esr(&self, CAN_ESR);
    fn read_esr(&self) -> CAN_ESR;

    fn write_btr(&self, CAN_BTR);
    fn read_btr(&self) -> CAN_BTR;

    fn write_ti0r(&self, CAN_TIxR);
    fn read_ti0r(&self) -> CAN_TIxR;

    fn write_ti1r(&self, CAN_TIxR);
    fn read_ti1r(&self) -> CAN_TIxR;

    fn write_ti2r(&self, CAN_TIxR);
    fn read_ti2r(&self) -> CAN_TIxR;

    fn write_tdl0r(&self, CAN_TDLxR);
    fn read_tdl0r(&self) -> CAN_TDLxR;

    fn write_tdl1r(&self, CAN_TDLxR);
    fn read_tdl1r(&self) -> CAN_TDLxR;

    fn write_tdl2r(&self, CAN_TDLxR);
    fn read_tdl2r(&self) -> CAN_TDLxR;

    fn write_tdh0r(&self, CAN_TDHxR);
    fn read_tdh0r(&self) -> CAN_TDHxR;
    
    fn write_tdh1r(&self, CAN_TDHxR);
    fn read_tdh1r(&self) -> CAN_TDHxR;

    fn write_tdh2r(&self, CAN_TDHxR);
    fn read_tdh2r(&self) -> CAN_TDHxR;

    fn write_ri0r(&self, CAN_RIxR);
    fn read_ri0r(&self) -> CAN_RIxR;

    fn write_ri1r(&self, CAN_RIxR);
    fn read_ri1r(&self) -> CAN_RIxR;

    fn write_rdt0r(&self, CAN_RDTxR);
    fn read_rdt0r(&self) -> CAN_RDTxR;

    fn write_rdt1r(&self, CAN_RDTxR);
    fn read_rdt1r(&self) -> CAN_RDTxR;

    fn write_rdl0r(&self, CAN_RDLxR);
    fn read_rdl0r(&self) -> CAN_RDLxR;

    fn write_rdl1r(&self, CAN_RDLxR);
    fn read_rdl1r(&self) -> CAN_RDLxR;

    fn write_rdh0r(&self, CAN_RDHxR);
    fn read_rdh0r(&self) -> CAN_RDHxR;

    fn write_rdh1r(&self, CAN_RDHxR);
    fn read_rdh1r(&self) -> CAN_RDHxR;
}

pub struct CAN_HAL;

pub struct CAN_MCR{
    pub DBF: Option<bool>,
    pub RESET: Option<bool>,
    pub TTCM: Option<bool>,
    pub ABOM: Option<bool>,
    pub AWUM: Option<bool>,
    pub NART: Option<bool>,
    pub RFLM: Option<bool>,
    pub TXFP: Option<bool>,
    pub SLEEP: Option<bool>,
    pub INRQ: Option<bool>,
}

pub struct CAN_MSR{
    pub RX: Option<bool>,
    pub SAMP: Option<bool>,
    pub RXM: Option<bool>,
    pub TXM: Option<bool>,
    pub SLAKI: Option<bool>,
    pub WKUI: Option<bool>,
    pub ERRI: Option<bool>,
    pub SLAK: Option<bool>,
    pub INAK: Option<bool>,
}

pub struct CAN_TSR{
    pub LOW2: Option<bool>,
    pub LOW1: Option<bool>,
    pub LOW0: Option<bool>,
    pub TME2: Option<bool>,
    pub TME1: Option<bool>,
    pub TME0: Option<bool>,
    pub CODE: Option<u8>,
    pub ABRQ2: Option<bool>,
    pub TERR2: Option<bool>,
    pub ALST2: Option<bool>,
    pub TXOK2: Option<bool>,
    pub RQCP2: Option<bool>,
    pub ABRQ1: Option<bool>,
    pub TERR1: Option<bool>,
    pub ALST1: Option<bool>,
    pub TXOK1: Option<bool>,
    pub RQCP1: Option<bool>,
    pub ABRQ0: Option<bool>,
    pub TERR0: Option<bool>,
    pub ALST0: Option<bool>,
    pub TXOK0: Option<bool>,
    pub RQCP0: Option<bool>,
}

pub struct CAN_RF0R{
    pub RFOM0: Option<bool>,
    pub FOVR0: Option<bool>,
    pub FULL0: Option<bool>,
    pub FMP0: Option<u8>,
}

pub struct CAN_RF1R{
    pub RFOM1: Option<bool>,
    pub FOVR1: Option<bool>,
    pub FULL1: Option<bool>,
    pub FMP1: Option<u8>,
}

pub struct CAN_IER{
    pub SLKIE: Option<bool>,
    pub WKUIE: Option<bool>,
    pub ERRIE: Option<bool>,
    pub LECIE: Option<bool>,
    pub BOFIE: Option<bool>,
    pub EPVIE: Option<bool>,
    pub EWGIE: Option<bool>,
    pub FOVIE1: Option<bool>,
    pub FFIE1: Option<bool>,
    pub FMPIE1: Option<bool>,
    pub FOVIE0: Option<bool>,
    pub FFIE0: Option<bool>,
    pub FMPIE0: Option<bool>,
    pub TMEIE: Option<bool>,
}

pub struct CAN_ESR{
    pub REC: Option<u8>,
    pub TEC: Option<u8>,
    pub LEC: Option<u8>,
    pub BOFF: Option<bool>,
    pub EPVF: Option<bool>,
    pub EWGF: Option<bool>,
}

pub struct CAN_BTR{
    pub SILM: Option<bool>,
    pub LBKM: Option<bool>,
    pub SJW: Option<u8>,
    pub TS2: Option<u8>,
    pub TS1: Option<u8>,
    pub BRP: Option<u16>,
}


pub struct CAN_TIxR{
    pub STID: Option<u16>,
    pub EXID: Option<u32>,
    pub IDE: Option<bool>,
    pub RTR: Option<bool>,
    pub TXRQ: Option<bool>,
}

pub struct CAN_TDTxR{
    pub TIME: Option<u16>,
    pub TGT: Option<bool>,
    pub DLC: Option<u8>,
}

pub struct CAN_TDLxR{
    pub DATA3: Option<u8>,
    pub DATA2: Option<u8>,
    pub DATA1: Option<u8>,
    pub DATA0: Option<u8>,
}

pub struct CAN_TDHxR{
    pub DATA7: Option<u8>,
    pub DATA6: Option<u8>,
    pub DATA5: Option<u8>,
    pub DATA4: Option<u8>,
}

pub struct CAN_RIxR{
    pub STID: Option<u16>,
    pub EXID: Option<u32>,
    pub IDE: Option<bool>,
    pub RTR: Option<bool>,
}

pub struct CAN_RDTxR{
    pub TIME: Option<u16>,
    pub FMI: Option<u8>,
    pub DLC: Option<u8>,
}

pub struct CAN_RDLxR{
    pub DATA3: Option<u8>,
    pub DATA2: Option<u8>,
    pub DATA1: Option<u8>,
    pub DATA0: Option<u8>,
}

pub struct CAN_RDHxR{
    pub DATA7: Option<u8>,
    pub DATA6: Option<u8>,
    pub DATA5: Option<u8>,
    pub DATA4: Option<u8>,
}

*/
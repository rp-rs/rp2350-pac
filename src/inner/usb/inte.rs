#[doc = "Register `INTE` reader"]
pub type R = crate::R<INTE_SPEC>;
#[doc = "Register `INTE` writer"]
pub type W = crate::W<INTE_SPEC>;
#[doc = "Field `HOST_CONN_DIS` reader - Host: raised when a device is connected or disconnected (i.e. when SIE_STATUS.SPEED changes). Cleared by writing to SIE_STATUS.SPEED"]
pub type HOST_CONN_DIS_R = crate::BitReader;
#[doc = "Field `HOST_CONN_DIS` writer - Host: raised when a device is connected or disconnected (i.e. when SIE_STATUS.SPEED changes). Cleared by writing to SIE_STATUS.SPEED"]
pub type HOST_CONN_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOST_RESUME` reader - Host: raised when a device wakes up the host. Cleared by writing to SIE_STATUS.RESUME"]
pub type HOST_RESUME_R = crate::BitReader;
#[doc = "Field `HOST_RESUME` writer - Host: raised when a device wakes up the host. Cleared by writing to SIE_STATUS.RESUME"]
pub type HOST_RESUME_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOST_SOF` reader - Host: raised every time the host sends a SOF (Start of Frame). Cleared by reading SOF_RD"]
pub type HOST_SOF_R = crate::BitReader;
#[doc = "Field `HOST_SOF` writer - Host: raised every time the host sends a SOF (Start of Frame). Cleared by reading SOF_RD"]
pub type HOST_SOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANS_COMPLETE` reader - Raised every time SIE_STATUS.TRANS_COMPLETE is set. Clear by writing to this bit."]
pub type TRANS_COMPLETE_R = crate::BitReader;
#[doc = "Field `TRANS_COMPLETE` writer - Raised every time SIE_STATUS.TRANS_COMPLETE is set. Clear by writing to this bit."]
pub type TRANS_COMPLETE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUFF_STATUS` reader - Raised when any bit in BUFF_STATUS is set. Clear by clearing all bits in BUFF_STATUS."]
pub type BUFF_STATUS_R = crate::BitReader;
#[doc = "Field `BUFF_STATUS` writer - Raised when any bit in BUFF_STATUS is set. Clear by clearing all bits in BUFF_STATUS."]
pub type BUFF_STATUS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERROR_DATA_SEQ` reader - Source: SIE_STATUS.DATA_SEQ_ERROR"]
pub type ERROR_DATA_SEQ_R = crate::BitReader;
#[doc = "Field `ERROR_DATA_SEQ` writer - Source: SIE_STATUS.DATA_SEQ_ERROR"]
pub type ERROR_DATA_SEQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERROR_RX_TIMEOUT` reader - Source: SIE_STATUS.RX_TIMEOUT"]
pub type ERROR_RX_TIMEOUT_R = crate::BitReader;
#[doc = "Field `ERROR_RX_TIMEOUT` writer - Source: SIE_STATUS.RX_TIMEOUT"]
pub type ERROR_RX_TIMEOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERROR_RX_OVERFLOW` reader - Source: SIE_STATUS.RX_OVERFLOW"]
pub type ERROR_RX_OVERFLOW_R = crate::BitReader;
#[doc = "Field `ERROR_RX_OVERFLOW` writer - Source: SIE_STATUS.RX_OVERFLOW"]
pub type ERROR_RX_OVERFLOW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERROR_BIT_STUFF` reader - Source: SIE_STATUS.BIT_STUFF_ERROR"]
pub type ERROR_BIT_STUFF_R = crate::BitReader;
#[doc = "Field `ERROR_BIT_STUFF` writer - Source: SIE_STATUS.BIT_STUFF_ERROR"]
pub type ERROR_BIT_STUFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERROR_CRC` reader - Source: SIE_STATUS.CRC_ERROR"]
pub type ERROR_CRC_R = crate::BitReader;
#[doc = "Field `ERROR_CRC` writer - Source: SIE_STATUS.CRC_ERROR"]
pub type ERROR_CRC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALL` reader - Source: SIE_STATUS.STALL_REC"]
pub type STALL_R = crate::BitReader;
#[doc = "Field `STALL` writer - Source: SIE_STATUS.STALL_REC"]
pub type STALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBUS_DETECT` reader - Source: SIE_STATUS.VBUS_DETECTED"]
pub type VBUS_DETECT_R = crate::BitReader;
#[doc = "Field `VBUS_DETECT` writer - Source: SIE_STATUS.VBUS_DETECTED"]
pub type VBUS_DETECT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUS_RESET` reader - Source: SIE_STATUS.BUS_RESET"]
pub type BUS_RESET_R = crate::BitReader;
#[doc = "Field `BUS_RESET` writer - Source: SIE_STATUS.BUS_RESET"]
pub type BUS_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEV_CONN_DIS` reader - Set when the device connection state changes. Cleared by writing to SIE_STATUS.CONNECTED"]
pub type DEV_CONN_DIS_R = crate::BitReader;
#[doc = "Field `DEV_CONN_DIS` writer - Set when the device connection state changes. Cleared by writing to SIE_STATUS.CONNECTED"]
pub type DEV_CONN_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEV_SUSPEND` reader - Set when the device suspend state changes. Cleared by writing to SIE_STATUS.SUSPENDED"]
pub type DEV_SUSPEND_R = crate::BitReader;
#[doc = "Field `DEV_SUSPEND` writer - Set when the device suspend state changes. Cleared by writing to SIE_STATUS.SUSPENDED"]
pub type DEV_SUSPEND_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEV_RESUME_FROM_HOST` reader - Set when the device receives a resume from the host. Cleared by writing to SIE_STATUS.RESUME"]
pub type DEV_RESUME_FROM_HOST_R = crate::BitReader;
#[doc = "Field `DEV_RESUME_FROM_HOST` writer - Set when the device receives a resume from the host. Cleared by writing to SIE_STATUS.RESUME"]
pub type DEV_RESUME_FROM_HOST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETUP_REQ` reader - Device. Source: SIE_STATUS.SETUP_REC"]
pub type SETUP_REQ_R = crate::BitReader;
#[doc = "Field `SETUP_REQ` writer - Device. Source: SIE_STATUS.SETUP_REC"]
pub type SETUP_REQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEV_SOF` reader - Set every time the device receives a SOF (Start of Frame) packet. Cleared by reading SOF_RD"]
pub type DEV_SOF_R = crate::BitReader;
#[doc = "Field `DEV_SOF` writer - Set every time the device receives a SOF (Start of Frame) packet. Cleared by reading SOF_RD"]
pub type DEV_SOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ABORT_DONE` reader - Raised when any bit in ABORT_DONE is set. Clear by clearing all bits in ABORT_DONE."]
pub type ABORT_DONE_R = crate::BitReader;
#[doc = "Field `ABORT_DONE` writer - Raised when any bit in ABORT_DONE is set. Clear by clearing all bits in ABORT_DONE."]
pub type ABORT_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP_STALL_NAK` reader - Raised when any bit in EP_STATUS_STALL_NAK is set. Clear by clearing all bits in EP_STATUS_STALL_NAK."]
pub type EP_STALL_NAK_R = crate::BitReader;
#[doc = "Field `EP_STALL_NAK` writer - Raised when any bit in EP_STATUS_STALL_NAK is set. Clear by clearing all bits in EP_STATUS_STALL_NAK."]
pub type EP_STALL_NAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_SHORT_PACKET` reader - Source: SIE_STATUS.RX_SHORT_PACKET"]
pub type RX_SHORT_PACKET_R = crate::BitReader;
#[doc = "Field `RX_SHORT_PACKET` writer - Source: SIE_STATUS.RX_SHORT_PACKET"]
pub type RX_SHORT_PACKET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDPOINT_ERROR` reader - Source: SIE_STATUS.ENDPOINT_ERROR"]
pub type ENDPOINT_ERROR_R = crate::BitReader;
#[doc = "Field `ENDPOINT_ERROR` writer - Source: SIE_STATUS.ENDPOINT_ERROR"]
pub type ENDPOINT_ERROR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEV_SM_WATCHDOG_FIRED` reader - Source: DEV_SM_WATCHDOG.FIRED"]
pub type DEV_SM_WATCHDOG_FIRED_R = crate::BitReader;
#[doc = "Field `DEV_SM_WATCHDOG_FIRED` writer - Source: DEV_SM_WATCHDOG.FIRED"]
pub type DEV_SM_WATCHDOG_FIRED_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPX_STOPPED_ON_NAK` reader - Source: NAK_POLL.EPX_STOPPED_ON_NAK"]
pub type EPX_STOPPED_ON_NAK_R = crate::BitReader;
#[doc = "Field `EPX_STOPPED_ON_NAK` writer - Source: NAK_POLL.EPX_STOPPED_ON_NAK"]
pub type EPX_STOPPED_ON_NAK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Host: raised when a device is connected or disconnected (i.e. when SIE_STATUS.SPEED changes). Cleared by writing to SIE_STATUS.SPEED"]
    #[inline(always)]
    pub fn host_conn_dis(&self) -> HOST_CONN_DIS_R {
        HOST_CONN_DIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Host: raised when a device wakes up the host. Cleared by writing to SIE_STATUS.RESUME"]
    #[inline(always)]
    pub fn host_resume(&self) -> HOST_RESUME_R {
        HOST_RESUME_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Host: raised every time the host sends a SOF (Start of Frame). Cleared by reading SOF_RD"]
    #[inline(always)]
    pub fn host_sof(&self) -> HOST_SOF_R {
        HOST_SOF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Raised every time SIE_STATUS.TRANS_COMPLETE is set. Clear by writing to this bit."]
    #[inline(always)]
    pub fn trans_complete(&self) -> TRANS_COMPLETE_R {
        TRANS_COMPLETE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Raised when any bit in BUFF_STATUS is set. Clear by clearing all bits in BUFF_STATUS."]
    #[inline(always)]
    pub fn buff_status(&self) -> BUFF_STATUS_R {
        BUFF_STATUS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Source: SIE_STATUS.DATA_SEQ_ERROR"]
    #[inline(always)]
    pub fn error_data_seq(&self) -> ERROR_DATA_SEQ_R {
        ERROR_DATA_SEQ_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Source: SIE_STATUS.RX_TIMEOUT"]
    #[inline(always)]
    pub fn error_rx_timeout(&self) -> ERROR_RX_TIMEOUT_R {
        ERROR_RX_TIMEOUT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Source: SIE_STATUS.RX_OVERFLOW"]
    #[inline(always)]
    pub fn error_rx_overflow(&self) -> ERROR_RX_OVERFLOW_R {
        ERROR_RX_OVERFLOW_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Source: SIE_STATUS.BIT_STUFF_ERROR"]
    #[inline(always)]
    pub fn error_bit_stuff(&self) -> ERROR_BIT_STUFF_R {
        ERROR_BIT_STUFF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Source: SIE_STATUS.CRC_ERROR"]
    #[inline(always)]
    pub fn error_crc(&self) -> ERROR_CRC_R {
        ERROR_CRC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Source: SIE_STATUS.STALL_REC"]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Source: SIE_STATUS.VBUS_DETECTED"]
    #[inline(always)]
    pub fn vbus_detect(&self) -> VBUS_DETECT_R {
        VBUS_DETECT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Source: SIE_STATUS.BUS_RESET"]
    #[inline(always)]
    pub fn bus_reset(&self) -> BUS_RESET_R {
        BUS_RESET_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Set when the device connection state changes. Cleared by writing to SIE_STATUS.CONNECTED"]
    #[inline(always)]
    pub fn dev_conn_dis(&self) -> DEV_CONN_DIS_R {
        DEV_CONN_DIS_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Set when the device suspend state changes. Cleared by writing to SIE_STATUS.SUSPENDED"]
    #[inline(always)]
    pub fn dev_suspend(&self) -> DEV_SUSPEND_R {
        DEV_SUSPEND_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Set when the device receives a resume from the host. Cleared by writing to SIE_STATUS.RESUME"]
    #[inline(always)]
    pub fn dev_resume_from_host(&self) -> DEV_RESUME_FROM_HOST_R {
        DEV_RESUME_FROM_HOST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Device. Source: SIE_STATUS.SETUP_REC"]
    #[inline(always)]
    pub fn setup_req(&self) -> SETUP_REQ_R {
        SETUP_REQ_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Set every time the device receives a SOF (Start of Frame) packet. Cleared by reading SOF_RD"]
    #[inline(always)]
    pub fn dev_sof(&self) -> DEV_SOF_R {
        DEV_SOF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Raised when any bit in ABORT_DONE is set. Clear by clearing all bits in ABORT_DONE."]
    #[inline(always)]
    pub fn abort_done(&self) -> ABORT_DONE_R {
        ABORT_DONE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Raised when any bit in EP_STATUS_STALL_NAK is set. Clear by clearing all bits in EP_STATUS_STALL_NAK."]
    #[inline(always)]
    pub fn ep_stall_nak(&self) -> EP_STALL_NAK_R {
        EP_STALL_NAK_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Source: SIE_STATUS.RX_SHORT_PACKET"]
    #[inline(always)]
    pub fn rx_short_packet(&self) -> RX_SHORT_PACKET_R {
        RX_SHORT_PACKET_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Source: SIE_STATUS.ENDPOINT_ERROR"]
    #[inline(always)]
    pub fn endpoint_error(&self) -> ENDPOINT_ERROR_R {
        ENDPOINT_ERROR_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Source: DEV_SM_WATCHDOG.FIRED"]
    #[inline(always)]
    pub fn dev_sm_watchdog_fired(&self) -> DEV_SM_WATCHDOG_FIRED_R {
        DEV_SM_WATCHDOG_FIRED_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Source: NAK_POLL.EPX_STOPPED_ON_NAK"]
    #[inline(always)]
    pub fn epx_stopped_on_nak(&self) -> EPX_STOPPED_ON_NAK_R {
        EPX_STOPPED_ON_NAK_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Host: raised when a device is connected or disconnected (i.e. when SIE_STATUS.SPEED changes). Cleared by writing to SIE_STATUS.SPEED"]
    #[inline(always)]
    #[must_use]
    pub fn host_conn_dis(&mut self) -> HOST_CONN_DIS_W<INTE_SPEC> {
        HOST_CONN_DIS_W::new(self, 0)
    }
    #[doc = "Bit 1 - Host: raised when a device wakes up the host. Cleared by writing to SIE_STATUS.RESUME"]
    #[inline(always)]
    #[must_use]
    pub fn host_resume(&mut self) -> HOST_RESUME_W<INTE_SPEC> {
        HOST_RESUME_W::new(self, 1)
    }
    #[doc = "Bit 2 - Host: raised every time the host sends a SOF (Start of Frame). Cleared by reading SOF_RD"]
    #[inline(always)]
    #[must_use]
    pub fn host_sof(&mut self) -> HOST_SOF_W<INTE_SPEC> {
        HOST_SOF_W::new(self, 2)
    }
    #[doc = "Bit 3 - Raised every time SIE_STATUS.TRANS_COMPLETE is set. Clear by writing to this bit."]
    #[inline(always)]
    #[must_use]
    pub fn trans_complete(&mut self) -> TRANS_COMPLETE_W<INTE_SPEC> {
        TRANS_COMPLETE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Raised when any bit in BUFF_STATUS is set. Clear by clearing all bits in BUFF_STATUS."]
    #[inline(always)]
    #[must_use]
    pub fn buff_status(&mut self) -> BUFF_STATUS_W<INTE_SPEC> {
        BUFF_STATUS_W::new(self, 4)
    }
    #[doc = "Bit 5 - Source: SIE_STATUS.DATA_SEQ_ERROR"]
    #[inline(always)]
    #[must_use]
    pub fn error_data_seq(&mut self) -> ERROR_DATA_SEQ_W<INTE_SPEC> {
        ERROR_DATA_SEQ_W::new(self, 5)
    }
    #[doc = "Bit 6 - Source: SIE_STATUS.RX_TIMEOUT"]
    #[inline(always)]
    #[must_use]
    pub fn error_rx_timeout(&mut self) -> ERROR_RX_TIMEOUT_W<INTE_SPEC> {
        ERROR_RX_TIMEOUT_W::new(self, 6)
    }
    #[doc = "Bit 7 - Source: SIE_STATUS.RX_OVERFLOW"]
    #[inline(always)]
    #[must_use]
    pub fn error_rx_overflow(&mut self) -> ERROR_RX_OVERFLOW_W<INTE_SPEC> {
        ERROR_RX_OVERFLOW_W::new(self, 7)
    }
    #[doc = "Bit 8 - Source: SIE_STATUS.BIT_STUFF_ERROR"]
    #[inline(always)]
    #[must_use]
    pub fn error_bit_stuff(&mut self) -> ERROR_BIT_STUFF_W<INTE_SPEC> {
        ERROR_BIT_STUFF_W::new(self, 8)
    }
    #[doc = "Bit 9 - Source: SIE_STATUS.CRC_ERROR"]
    #[inline(always)]
    #[must_use]
    pub fn error_crc(&mut self) -> ERROR_CRC_W<INTE_SPEC> {
        ERROR_CRC_W::new(self, 9)
    }
    #[doc = "Bit 10 - Source: SIE_STATUS.STALL_REC"]
    #[inline(always)]
    #[must_use]
    pub fn stall(&mut self) -> STALL_W<INTE_SPEC> {
        STALL_W::new(self, 10)
    }
    #[doc = "Bit 11 - Source: SIE_STATUS.VBUS_DETECTED"]
    #[inline(always)]
    #[must_use]
    pub fn vbus_detect(&mut self) -> VBUS_DETECT_W<INTE_SPEC> {
        VBUS_DETECT_W::new(self, 11)
    }
    #[doc = "Bit 12 - Source: SIE_STATUS.BUS_RESET"]
    #[inline(always)]
    #[must_use]
    pub fn bus_reset(&mut self) -> BUS_RESET_W<INTE_SPEC> {
        BUS_RESET_W::new(self, 12)
    }
    #[doc = "Bit 13 - Set when the device connection state changes. Cleared by writing to SIE_STATUS.CONNECTED"]
    #[inline(always)]
    #[must_use]
    pub fn dev_conn_dis(&mut self) -> DEV_CONN_DIS_W<INTE_SPEC> {
        DEV_CONN_DIS_W::new(self, 13)
    }
    #[doc = "Bit 14 - Set when the device suspend state changes. Cleared by writing to SIE_STATUS.SUSPENDED"]
    #[inline(always)]
    #[must_use]
    pub fn dev_suspend(&mut self) -> DEV_SUSPEND_W<INTE_SPEC> {
        DEV_SUSPEND_W::new(self, 14)
    }
    #[doc = "Bit 15 - Set when the device receives a resume from the host. Cleared by writing to SIE_STATUS.RESUME"]
    #[inline(always)]
    #[must_use]
    pub fn dev_resume_from_host(&mut self) -> DEV_RESUME_FROM_HOST_W<INTE_SPEC> {
        DEV_RESUME_FROM_HOST_W::new(self, 15)
    }
    #[doc = "Bit 16 - Device. Source: SIE_STATUS.SETUP_REC"]
    #[inline(always)]
    #[must_use]
    pub fn setup_req(&mut self) -> SETUP_REQ_W<INTE_SPEC> {
        SETUP_REQ_W::new(self, 16)
    }
    #[doc = "Bit 17 - Set every time the device receives a SOF (Start of Frame) packet. Cleared by reading SOF_RD"]
    #[inline(always)]
    #[must_use]
    pub fn dev_sof(&mut self) -> DEV_SOF_W<INTE_SPEC> {
        DEV_SOF_W::new(self, 17)
    }
    #[doc = "Bit 18 - Raised when any bit in ABORT_DONE is set. Clear by clearing all bits in ABORT_DONE."]
    #[inline(always)]
    #[must_use]
    pub fn abort_done(&mut self) -> ABORT_DONE_W<INTE_SPEC> {
        ABORT_DONE_W::new(self, 18)
    }
    #[doc = "Bit 19 - Raised when any bit in EP_STATUS_STALL_NAK is set. Clear by clearing all bits in EP_STATUS_STALL_NAK."]
    #[inline(always)]
    #[must_use]
    pub fn ep_stall_nak(&mut self) -> EP_STALL_NAK_W<INTE_SPEC> {
        EP_STALL_NAK_W::new(self, 19)
    }
    #[doc = "Bit 20 - Source: SIE_STATUS.RX_SHORT_PACKET"]
    #[inline(always)]
    #[must_use]
    pub fn rx_short_packet(&mut self) -> RX_SHORT_PACKET_W<INTE_SPEC> {
        RX_SHORT_PACKET_W::new(self, 20)
    }
    #[doc = "Bit 21 - Source: SIE_STATUS.ENDPOINT_ERROR"]
    #[inline(always)]
    #[must_use]
    pub fn endpoint_error(&mut self) -> ENDPOINT_ERROR_W<INTE_SPEC> {
        ENDPOINT_ERROR_W::new(self, 21)
    }
    #[doc = "Bit 22 - Source: DEV_SM_WATCHDOG.FIRED"]
    #[inline(always)]
    #[must_use]
    pub fn dev_sm_watchdog_fired(&mut self) -> DEV_SM_WATCHDOG_FIRED_W<INTE_SPEC> {
        DEV_SM_WATCHDOG_FIRED_W::new(self, 22)
    }
    #[doc = "Bit 23 - Source: NAK_POLL.EPX_STOPPED_ON_NAK"]
    #[inline(always)]
    #[must_use]
    pub fn epx_stopped_on_nak(&mut self) -> EPX_STOPPED_ON_NAK_W<INTE_SPEC> {
        EPX_STOPPED_ON_NAK_W::new(self, 23)
    }
}
#[doc = "Interrupt Enable  

You can [`read`](crate::Reg::read) this register and get [`inte::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inte::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTE_SPEC;
impl crate::RegisterSpec for INTE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inte::R`](R) reader structure"]
impl crate::Readable for INTE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`inte::W`](W) writer structure"]
impl crate::Writable for INTE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTE to value 0"]
impl crate::Resettable for INTE_SPEC {
    const RESET_VALUE: u32 = 0;
}

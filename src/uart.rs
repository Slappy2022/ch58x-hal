use ch58x::ch58x as pac;
use core::ops::Deref;

pub trait Instance: Deref<Target = pac::uart3::RegisterBlock> {
    fn register_block(&self) -> &pac::uart3::RegisterBlock;
}
impl Instance for pac::UART3 {
    fn register_block(&self) -> &pac::uart3::RegisterBlock {
        self
    }
}

pub struct Serial<T, TxPin, RxPin>
where
    T: Instance,
{
    pub uart: T,
    pub tx: TxPin,
    pub rx: RxPin,
}
impl<T, TxPin, RxPin> Serial<T, TxPin, RxPin>
where
    T: Instance,
{
    pub fn new(uart: T, tx: TxPin, rx: RxPin) -> Self {
        Self { uart, tx, rx }.init()
        // TODO: Tie tx and rx to uart
    }
    fn set_baud(self, baud: u32) -> Self {
        let x = 10 * crate::system_clock_hz() / 8 / baud;
        let x = (x + 5) / 10;
        self.uart.dl.write(|w| w.dl().variant(x as u16));
        self
    }
    fn init(self) -> Self {
        self.uart.fcr.write(|w| {
            w.fcr_fifo_en().variant(true);
            w.fcr_tx_fifo_clr().variant(true);
            w.fcr_rx_fifo_clr().variant(true);
            w.fcr_fifo_trig().variant(0x02)
        });
        self.uart.lcr.write(|w| w.lcr_word_sz().variant(0x03));
        self.uart.ier.write(|w| w.ier_txd_en().variant(true));
        self.uart.div.write(|w| w.div().variant(1));
        self.set_baud(115_200)
    }
    fn write_byte(&self, b: u8) -> nb::Result<(), core::convert::Infallible> {
        match self.uart.tfc.read().tfc().bits() {
            8 => Err(nb::Error::WouldBlock),
            _ => Ok(self.uart.thr().write(|w| w.thr().variant(b))),
        }
    }
    fn write_bytes(&self, data: &[u8]) -> Result<(), core::convert::Infallible> {
        for &b in data {
            nb::block!(self.write_byte(b))?;
        }
        Ok(())
    }
}
impl<T, TxPin, RxPin> core::fmt::Write for Serial<T, TxPin, RxPin>
where
    T: Instance,
{
    #[inline]
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write_bytes(s.as_bytes()).map_err(|_| core::fmt::Error)
    }
}

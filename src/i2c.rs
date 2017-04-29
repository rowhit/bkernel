//! I2C module adapter for use with futures.

use ::core::cell::UnsafeCell;

use stm32f4::i2c::{self, I2c};

use futures::{Async, AsyncSink, Future, Poll, Sink, StartSend};
use futures::future::IntoFuture;

pub struct I2cBus<'a> {
    i2c: &'a I2c,
    current_transaction: UnsafeCell<Option<I2cTransaction>>,
}

pub enum I2cTransaction {
    MasterTransmitter {
        slave_address: u8,
        buffer: *const u8,
        bytes_left: usize,
    },
}

pub static I2C1_BUS: I2cBus = I2cBus::new(unsafe{&i2c::I2C1});
pub static I2C2_BUS: I2cBus = I2cBus::new(unsafe{&i2c::I2C2});
pub static I2C3_BUS: I2cBus = I2cBus::new(unsafe{&i2c::I2C3});

unsafe impl Sync for I2cBus<'static> {
}

impl<'a> I2cBus<'a> {
    const fn new(i2c: &'a I2c) -> Self {
        I2cBus {
            i2c,
            current_transaction: UnsafeCell::new(None),
        }
    }

    pub fn master_transmit<'b>(&self, slave_address: u8, buffer: *const u8, buffer_size: usize) -> I2cTransaction {
        I2cTransaction::MasterTransmitter {
            slave_address,
            buffer,
            bytes_left: buffer_size,
        }
    }
}

impl<'a, 'b> Sink for &'b I2cBus<'a> {
    type SinkItem = I2cTransaction;
    type SinkError = ();

    fn start_send(&mut self, item: I2cTransaction) -> StartSend<Self::SinkItem, Self::SinkError> {
        unsafe {
            // TODO(ashmalko): race condition
            let current_transaction = self.current_transaction.get();
            if (*current_transaction).is_none() {
                *current_transaction = Some(item);
                Ok(AsyncSink::Ready)
            } else {
                Ok(AsyncSink::NotReady(item))
            }
        }
    }

    fn poll_complete(&mut self) -> Poll<(), Self::SinkError> {
        unsafe {
            if (*self.current_transaction.get()).is_none() {
                Ok(Async::Ready(()))
            } else {
                Ok(Async::NotReady)
            }
        }
    }

    fn close(&mut self) -> Poll<(), Self::SinkError> {
        self.poll_complete()
    }
}

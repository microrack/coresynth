use crate::os::{Mutex, Semaphore};
use crate::peripheral::{Static};

use super::bindings::{
    HAL_StatusTypeDef,
    SPI_HandleTypeDef,
    hspi1,
    HAL_SPI_TransmitReceive_IT,
    HAL_SPI_Transmit_IT,
    HAL_SPI_Receive_IT
};

use super::release::{checked_release, Release};

use crate::hal::traits::{SPI as SPITrait, Error, Result};

pub struct SPI {
    pub spi: *mut SPI_HandleTypeDef,
    pub semaphore : Semaphore,
}

impl SPI {
    const MAX_CHUNK_SIZE : usize = core::u16::MAX as usize;

    fn new(spi: *mut SPI_HandleTypeDef) -> Result<SPI>  {
        Ok(SPI {
            spi,
            semaphore: Semaphore::empty(1).map_err(|_| Error {
                call: "SPI::new Semaphore::empty",
            })?,
        })
    }

    fn exchange_unsafe(&self, tx_bytes:&[u8], rx_bytes:&mut [u8]) -> Result<()> {
        assert_eq!(tx_bytes.len(), rx_bytes.len(), "Exchanging TX and RX buffer length mismatch");
        let len = tx_bytes.len();
        assert!(len <= SPI::MAX_CHUNK_SIZE, "Can't exchange more than 64kB at once");
        let status = unsafe { HAL_SPI_TransmitReceive_IT(
            self.spi,
            // for some reason, HAL_SPI_TransmitReceive_IT accept both buffers as mutable
            tx_bytes.as_ptr() as *mut u8,
            rx_bytes.as_mut_ptr(),
            len as u16
        ) };
        match status {
            HAL_StatusTypeDef::HAL_OK => Ok(()),
            _ => Err(Error { call: "SPI::exchange HAL_SPI_TransmitReceive_IT" })
        }
    }
    fn write_unsafe(&self, bytes:&[u8]) -> Result<()> {
        let len = bytes.len();
        assert!(len <= SPI::MAX_CHUNK_SIZE, "Can't write more than 64kB at once");
        let status = unsafe { HAL_SPI_Transmit_IT(
            self.spi,
            // for some reason, HAL_SPI_Transmit_IT accept buffer as mutable
            bytes.as_ptr() as *mut u8,
            len as u16
        ) };
        match status {
            HAL_StatusTypeDef::HAL_OK => Ok(()),
            _ => Err(Error { call: "SPI::write HAL_SPI_Transmit_IT" })
        }
    }
    fn read_unsafe(&self, bytes:&mut[u8]) -> Result<()> {
        let len = bytes.len();
        assert!(len <= SPI::MAX_CHUNK_SIZE, "Can't read more than 64kB at once");
        let status = unsafe { HAL_SPI_Receive_IT(
            self.spi,
            bytes.as_mut_ptr(),
            len as u16
        ) };
        match status {
            HAL_StatusTypeDef::HAL_OK => Ok(()),
            _ => Err(Error { call: "SPI::read HAL_SPI_Receive_IT" })
        }
    }

    fn write_slice_blocking(&self, bytes:&[u8]) -> Result<()> {
        self.write_unsafe(bytes)?;
        self.semaphore.acquire().map_err(|_| Error {
            call: "SPI::write Semaphore::acquire",
        })?;
        Ok(())
    }
    fn read_slice_blocking(&self, bytes:&mut[u8]) -> Result<()> {
        self.read_unsafe(bytes)?;
        self.semaphore.acquire().map_err(|_| Error {
            call: "SPI::read Semaphore::acquire",
        })?;
        Ok(())
    }
    fn exchange_slice_blocking(&self, tx_bytes:&[u8], rx_bytes:&mut [u8]) -> Result<()> {
        self.exchange_unsafe(tx_bytes, rx_bytes)?;
        self.semaphore.acquire().map_err(|_| Error {
            call: "SPI::exchange Semaphore::acquire",
        })?;
        Ok(())
    }
}

impl SPITrait for SPI {
    fn write(&self, bytes: &[u8]) -> Result<()> {
        for chunk in bytes.chunks(SPI::MAX_CHUNK_SIZE) {
            self.write_slice_blocking(chunk)?;
        }

        Ok(())
    }
    fn read(&self, bytes: &mut [u8]) -> Result<()> {
        for chunk in bytes.chunks_mut(SPI::MAX_CHUNK_SIZE) {
            self.read_slice_blocking(chunk)?
        }

        Ok(())
    }
    fn exchange(&self, tx_bytes: &[u8], rx_bytes: &mut [u8]) -> Result<()> {
        assert_eq!(tx_bytes.len(), rx_bytes.len(), "Exchanging TX and RX buffer length mismatch");

        let tx_chunks = tx_bytes.chunks(SPI::MAX_CHUNK_SIZE);
        let rx_chunks = rx_bytes.chunks_mut(SPI::MAX_CHUNK_SIZE);

        for (tx_chunk, rx_chunk) in tx_chunks.zip(rx_chunks) {
            self.exchange_slice_blocking(tx_chunk, rx_chunk)?
        }

        Ok(())
    }
}

impl Release<SPI_HandleTypeDef> for SPI {
    fn checked_release(&self, ptr:*mut SPI_HandleTypeDef) -> Result<()> {
        if self.spi == ptr {
            self.semaphore.release().map_err(|_| Error {
                call: "SPI::checked_release Semaphore::release",
            })?;
        }
        Ok(())
    }
}

// pub static _SPI: Static<Mutex<SPI>> = Static::new();
pub static ALL_SPIS: [&Static<Mutex<SPI>>;1] = [&_SPI]; // TODO

pub fn spi_init_static() -> Result<()> {
    /*
    let spi1 = SPI::new(unsafe { &mut hspi1 })?;
    let mutex = Mutex::new(spi1)
        .map_err(|_| Error {
            call: "SPI init static Mutex::new",
        })?;

    _SPI.init(mutex);
    */
    
    Ok(())
}

#[no_mangle]
pub extern "C" fn HAL_SPI_TxCpltCallback(hspi: *mut SPI_HandleTypeDef) {
    checked_release(&ALL_SPIS, hspi)
        .expect("HAL_SPI_TxCpltCallback checked_release");
}

#[no_mangle]
pub extern "C" fn HAL_SPI_TxRxCpltCallback(hspi: *mut SPI_HandleTypeDef) {
    checked_release(&ALL_SPIS, hspi)
        .expect("HAL_SPI_TxRxCpltCallback checked_release");
}

#[no_mangle]
pub extern "C" fn HAL_SPI_RxCpltCallback(hspi: *mut SPI_HandleTypeDef) {
    checked_release(&ALL_SPIS, hspi)
        .expect("HAL_SPI_RxCpltCallback checked_release");
}

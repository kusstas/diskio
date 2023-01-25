//! Diskio library provides trait for handling disk IO devices.

#![no_std]

use flagset::{flags, FlagSet};

flags! {
    /// Flags of [`Status`].
    pub enum StatusFlag: u8 {
        /// Drive not initialized.
        NotInitialized = 0x01,
        /// Drive is write protected.
        WriteProtected = 0x02,
        /// Error occured.
        ErrorOccured = 0x04,
    }
}

/// Flagset of [`DiskioDevice`] status.
pub type Status = FlagSet<StatusFlag>;

/// Memory sector address.
pub type Lba = u64;

/// Sector size of a drive.
pub type SectorSize = usize;

/// Block size of a drive.
pub type BlockSize = usize;

/// Data area.
/// `0` - start of data area.
/// `1` - end of data area.
pub type DataArea = (Lba, Lba);

/// [`DiskioDevice`] error type.
///
/// `T` - Device error type.
#[derive(Debug, Clone, Copy)]
pub enum Error<T> {
    /// Device isn't initialized.
    NotInitialized,
    /// Device is already initialized.
    AlreadyInitialized,
    /// The feature isn't supported by this device.
    NotSupported,
    /// Can't write to write protected device.
    WriteProtected,
    /// Invalid argument passed to device methods.
    InvalidArgument,
    /// Hardware error occurred.
    Hardware(T),
}

/// Ioctl commands.
pub enum IoctlCmd<'a> {
    /// Complete pending write process.
    CtrlSync,
    /// Get media size.
    GetSectorCount(&'a mut Lba),
    /// Get sector size.
    GetSectorSize(&'a mut SectorSize),
    /// Get erase block size.
    GetBlockSize(&'a mut BlockSize),
    /// Inform device that the data on the block of sectors is no longer used.
    CtrlTrim(&'a DataArea),
}

/// Represents disk IO device.
pub trait DiskioDevice {
    /// Device error type.
    type HardwareError;

    /// Get status of device.
    fn status(&self) -> Status;

    /// Reset device (optional).
    fn reset(&mut self) {}

    /// Initialize device.
    fn initialize(&mut self) -> Result<(), Error<Self::HardwareError>>;

    /// Read data blocks from device by address.
    fn read(&self, buf: &mut [u8], lba: Lba) -> Result<(), Error<Self::HardwareError>>;

    /// Write data blocks to device by address.
    fn write(&self, buf: &[u8], lba: Lba) -> Result<(), Error<Self::HardwareError>>;

    /// Make ioctl query to device.
    fn ioctl(&self, cmd: IoctlCmd) -> Result<(), Error<Self::HardwareError>>;
}

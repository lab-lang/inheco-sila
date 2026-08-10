//! Typed protocol building blocks for Inheco's SiLA-controlled
//! instruments: SiLA 1.x SOAP over plain HTTP.
//!
//! - **Protocol** ([`soap`], [`methodset`]) — pure, no I/O: SOAP 1.1
//!   envelope encoding and decoding for the command vocabulary, the
//!   response and return-code model, device-event parsing with the
//!   canned acknowledgements, and a validated MethodSet builder
//!   rendering the vendor's thermal-profile XML dialect. Timestamps and
//!   method names are caller inputs, so every document renders to the
//!   same bytes and is testable as a string with no hardware.
//!
//! The protocol knowledge here derives from PyLabRobot's ODTC backend
//! and SiLA interface (MIT licensed) and from the Inheco ODTC user
//! manual (document 900584); the device's own `odtc.wsdl` is the
//! authority on the full return-code table. No vendor library is
//! involved: control is HTTP and XML end to end.

pub mod methodset;
pub mod soap;

pub use methodset::{
    BLOCK_MAX_CELSIUS, BLOCK_MIN_CELSIUS, LID_MAX_CELSIUS, LID_MIN_CELSIUS, MAX_SLOPE_C_PER_S,
    MethodSetError, MethodSettings, ProgramStage, ProgramStep, ThermalProgram,
};
pub use soap::{
    Command, DataEvent, DataSeries, DeviceState, IncomingEvent, ResponseEvent, SoapError,
    StatusEvent, SyncResponse,
};

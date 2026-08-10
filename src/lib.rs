//! A typed implementation of the SiLA 1.x SOAP protocol Inheco's
//! SiLA-controlled instruments speak: envelope encoding and decoding for
//! the command vocabulary, the response and return-code model, and
//! device-event parsing with the canned acknowledgements. Everything is
//! pure, and every envelope is pinned byte for byte by golden tests.
//!
//! The protocol knowledge here derives from PyLabRobot's ODTC backend
//! and SiLA interface (MIT licensed) and from the Inheco ODTC user
//! manual (document 900584); the device's own `odtc.wsdl` is the
//! authority on the full return-code table. No vendor library is
//! involved: control is HTTP and XML end to end.

pub mod soap;

pub use soap::{
    Command, DataEvent, DataSeries, DeviceState, IncomingEvent, ResponseEvent, SoapError,
    StatusEvent, SyncResponse,
};

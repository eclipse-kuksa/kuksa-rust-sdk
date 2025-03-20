#[cfg(feature = "v1")]
pub use kuksa as v1;

#[cfg(feature = "v1")]
pub use databroker_proto::kuksa::val::v1 as v1_proto;

#[cfg(any(feature = "v2", feature = "default"))]
pub use kuksa_val_v2 as v2;

#[cfg(any(feature = "v2", feature = "default"))]
pub use databroker_proto::kuksa::val::v2 as v2_proto;

#[cfg(feature = "sdv")]
pub use kuksa_sdv as sdv;

#[cfg(feature = "sdv")]
pub use databroker_proto::sdv::databroker::v1 as sdv_proto;

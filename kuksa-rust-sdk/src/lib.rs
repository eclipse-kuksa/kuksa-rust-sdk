pub use databroker_proto::kuksa::val::v1 as v1_proto;

pub use databroker_proto::kuksa::val::v2 as v2_proto;

pub use databroker_proto::sdv::databroker::v1 as sdv_proto;

pub mod kuksa {
    pub mod common;
    pub mod val {
        pub mod v1;
        pub mod v2;
    }
}

pub mod sdv {
    pub mod databroker {
        pub mod v1;
    }
}

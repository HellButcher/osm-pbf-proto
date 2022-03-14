// re-export of protobuf
pub use quick_protobuf;

include!(concat!(env!("OUT_DIR"), "/protos-gen/mod.rs"));

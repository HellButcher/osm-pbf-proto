// re-export of protobuf
pub use prost;

pub mod fileformat {
    include!(concat!(env!("OUT_DIR"), "/fileformat.rs"));
}
pub mod osmformat {
    include!(concat!(env!("OUT_DIR"), "/osmformat.rs"));
}

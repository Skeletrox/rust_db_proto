pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/messages.rs"));
    include!(concat!(env!("OUT_DIR"), "/datatypes.rs"));
}

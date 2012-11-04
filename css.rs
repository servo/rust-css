// Shortcuts to the netsurfcss types
mod n {
    pub mod ll {
        pub use p = netsurfcss::ll::properties;
        pub use s = netsurfcss::ll::select;
        pub use t = netsurfcss::ll::types;
        pub use c = netsurfcss::ll::computed;
    }

    pub use p = netsurfcss::properties;
    pub use s = netsurfcss::select;
    pub use t = netsurfcss::types;
    pub use c = netsurfcss::computed;
    pub use v = netsurfcss::values;
    pub use h = netsurfcss::hint;
    pub use u = netsurfcss::util;
}
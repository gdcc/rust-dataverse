pub mod client;
pub mod response;

pub mod native_api {
    pub mod collection {
        pub mod create;
    }
    pub mod info {
        pub mod version;
    }
}

pub mod cli {
    pub mod base;
    pub mod collection;
    pub mod info;
}

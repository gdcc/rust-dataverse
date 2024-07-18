pub mod client;
pub mod identifier;
pub mod filewrapper;
pub mod request;
pub mod response;
pub mod utils;
pub mod callback;

pub mod native_api {
    pub mod collection {
        pub mod content;
        pub mod create;
        pub mod delete;
        pub mod publish;
    }
    pub mod info {
        pub mod version;
    }
    pub mod dataset {
        pub mod create;
        pub mod delete;
        pub mod edit;
        pub mod get;
        pub mod link;
        pub mod publish;
        pub mod upload;
    }
    pub mod file {
        pub mod replace;
    }
}

pub mod cli {
    pub mod base;
    pub mod collection;
    pub mod dataset;
    pub mod file;
    pub mod info;
}

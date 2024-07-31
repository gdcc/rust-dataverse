#![warn(unused_crate_dependencies)]
pub mod client;
pub mod identifier;
pub mod filewrapper;
pub mod request;
pub mod response;
pub mod utils;
pub mod callback;

pub mod native_api {
    pub mod collection {
        // Re-export the collection API modules
        pub use content::get_content;
        pub use create::create_collection;
        pub use delete::delete_collection;
        pub use publish::publish_collection;

        pub mod content;
        pub mod create;
        pub mod delete;
        pub mod publish;
    }
    pub mod info {
        // Re-export the info API modules
        pub use version::get_version;

        pub mod version;
    }
    pub mod dataset {
        // Re-export the dataset API modules
        pub use create::create_dataset;
        pub use delete::delete_dataset;
        pub use edit::edit_dataset_metadata;
        pub use get::get_dataset_meta;
        pub use link::link_dataset;
        pub use upload::upload_file_to_dataset;

        pub mod create;
        pub mod delete;
        pub mod edit;
        pub mod get;
        pub mod link;
        pub mod publish;
        pub mod upload;
    }
    pub mod file {
        pub use replace::replace_file;

        pub mod replace;
    }
}

pub mod prelude {
    pub use super::callback::CallbackFun;
    pub use super::client::BaseClient;
    pub use super::identifier::Identifier;
    pub use super::native_api::collection;
    pub use super::native_api::dataset;
    pub use super::native_api::file;
    pub use super::native_api::info;
}

pub mod cli {
    pub mod base;
    pub mod collection;
    pub mod dataset;
    pub mod file;
    pub mod info;
}

#[cfg(test)]
mod test_utils;

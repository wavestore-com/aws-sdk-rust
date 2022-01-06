#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::blacklisted_name)]
#![allow(clippy::vec_init_then_push)]
#![allow(rustdoc::bare_urls)]
#![warn(missing_docs)]
//! <fullname>Amazon Neptune</fullname>
//! <p>Amazon Neptune is a fast, reliable, fully-managed graph database service that makes it
//! easy to build and run applications that work with highly connected datasets. The core of
//! Amazon Neptune is a purpose-built, high-performance graph database engine optimized for
//! storing billions of relationships and querying the graph with milliseconds latency. Amazon
//! Neptune supports popular graph models Property Graph and W3C's RDF, and their respective query
//! languages Apache TinkerPop Gremlin and SPARQL, allowing you to easily build queries that
//! efficiently navigate highly connected datasets. Neptune powers graph use cases such as
//! recommendation engines, fraud detection, knowledge graphs, drug discovery, and network
//! security.</p>
//!
//! <p>This interface reference for Amazon Neptune contains documentation for a programming or
//! command line interface you can use to manage Amazon Neptune. Note that Amazon Neptune is
//! asynchronous, which means that some interfaces might require techniques such as polling or
//! callback functions to determine when a command has been applied. In this reference, the
//! parameter descriptions indicate whether a command is applied immediately, on the next instance
//! reboot, or during the maintenance window. The reference structure is as follows, and we list
//! following some related topics from the user guide.</p>
//!
//! # Crate Organization
//!
//! The entry point for most customers will be [`Client`]. [`Client`] exposes one method for each API offered
//! by the service.
//!
//! Some APIs require complex or nested arguments. These exist in [`model`].
//!
//! Lastly, errors that can be returned by the service are contained within [`error`]. [`Error`] defines a meta
//! error encompassing all possible errors that can be returned by the service.
//!
//! The other modules within this crate are not required for normal usage.

// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use error_meta::Error;

pub use config::Config;

mod aws_endpoint;
/// Client and fluent builders for calling the service.
pub mod client;
/// Configuration for the service.
pub mod config;
/// Errors that can occur when calling the service.
pub mod error;
mod error_meta;
/// Input structures for operations.
pub mod input;
/// Generated accessors for nested fields
mod lens;
pub mod middleware;
/// Data structures used by operation inputs/outputs.
pub mod model;
mod no_credentials;
/// All operations that this crate can perform.
pub mod operation;
mod operation_deser;
mod operation_ser;
/// Output structures for operations.
pub mod output;
/// Paginators for the service
pub mod paginator;
mod query_ser;
mod rest_xml_wrapped_errors;
mod xml_deser;
/// Crate version number.
pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
pub use aws_smithy_http::byte_stream::ByteStream;
pub use aws_smithy_http::result::SdkError;
pub use aws_smithy_types::Blob;
pub use aws_smithy_types::DateTime;
static API_METADATA: aws_http::user_agent::ApiMetadata =
    aws_http::user_agent::ApiMetadata::new("neptune", PKG_VERSION);
pub use aws_smithy_http::endpoint::Endpoint;
pub use aws_smithy_types::retry::RetryConfig;
pub use aws_types::app_name::AppName;
pub use aws_types::region::Region;
pub use aws_types::Credentials;
pub use client::Client;

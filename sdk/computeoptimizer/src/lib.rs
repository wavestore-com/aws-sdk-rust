#![allow(deprecated)]
#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::blacklisted_name)]
#![allow(clippy::vec_init_then_push)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_return)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(rustdoc::bare_urls)]

#![warn(missing_docs)]
//! <p>Compute Optimizer is a service that analyzes the configuration and utilization
//! metrics of your Amazon Web Services compute resources, such as Amazon EC2
//! instances, Amazon EC2 Auto Scaling groups, Lambda functions, Amazon EBS volumes, and Amazon ECS services on Fargate. 
//! It reports whether your resources are optimal, and generates
//! optimization recommendations to reduce the cost and improve the performance of your
//! workloads. Compute Optimizer also provides recent utilization metric data, in addition
//! to projected utilization metric data for the recommendations, which you can use to
//! evaluate which recommendation provides the best price-performance trade-off. The
//! analysis of your usage patterns can help you decide when to move or resize your running
//! resources, and still meet your performance and capacity requirements. For more
//! information about Compute Optimizer, including the required permissions to use the
//! service, see the <a href="https://docs.aws.amazon.com/compute-optimizer/latest/ug/">Compute Optimizer User Guide</a>.</p>
//! 
//! # Crate Organization
//! 
//! The entry point for most customers will be [`Client`]. [`Client`] exposes one method for each API offered
//! by the service.
//! 
//! Some APIs require complex or nested arguments. These exist in [`model`](crate::model).
//! 
//! Lastly, errors that can be returned by the service are contained within [`error`]. [`Error`] defines a meta
//! error encompassing all possible errors that can be returned by the service.
//! 
//! The other modules within this crate are not required for normal usage.


// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use error_meta::Error;

#[doc(inline)]
pub use config::Config;

/// Client and fluent builders for calling the service.
pub mod client;

/// Configuration for the service.
pub mod config;

/// Endpoint resolution functionality
pub mod endpoint;

/// All error types that operations can return. Documentation on these types is copied from the model.
pub mod error;

mod error_meta;

/// Input structures for operations. Documentation on these types is copied from the model.
pub mod input;

/// Data structures used by operation inputs/outputs. Documentation on these types is copied from the model.
pub mod model;

/// All operations that this crate can perform.
pub mod operation;

/// Output structures for operations. Documentation on these types is copied from the model.
pub mod output;

/// Data primitives referenced by other data types.
pub mod types;

pub mod middleware;

mod no_credentials;

mod operation_deser;

mod operation_ser;

/// Paginators for the service
pub mod paginator;

mod json_deser;

mod json_ser;

/// Generated accessors for nested fields
mod lens;

/// Endpoints standard library functions
mod endpoint_lib;

mod json_errors;

/// Crate version number.
                    pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
pub use aws_smithy_http::endpoint::Endpoint;
static API_METADATA: aws_http::user_agent::ApiMetadata = aws_http::user_agent::ApiMetadata::new("computeoptimizer", PKG_VERSION);
pub use aws_types::app_name::AppName;
#[doc(inline)]
pub use client::Client;
pub use aws_types::region::Region;
pub use aws_credential_types::Credentials;


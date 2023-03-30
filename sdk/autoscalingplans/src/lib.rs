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
//! <fullname>AWS Auto Scaling</fullname>
//! 
//! 
//! <p>Use AWS Auto Scaling to create scaling plans for your applications to
//! automatically scale your scalable AWS resources. </p>
//! <p>
//! <b>API Summary</b>
//! </p>
//! <p>You can use the AWS Auto Scaling service API to accomplish the following tasks:</p>
//! <ul>
//! <li>
//! <p>Create and manage scaling plans</p>
//! </li>
//! <li>
//! <p>Define target tracking scaling policies to dynamically scale your resources based
//! on utilization</p>
//! </li>
//! <li>
//! <p>Scale Amazon EC2 Auto Scaling groups using predictive scaling and dynamic scaling to scale your
//! Amazon EC2 capacity faster</p>
//! </li>
//! <li>
//! <p>Set minimum and maximum capacity limits</p>
//! </li>
//! <li>
//! <p>Retrieve information on existing scaling plans</p>
//! </li>
//! <li>
//! <p>Access current forecast data and historical forecast data for up to 56 days
//! previous</p>
//! </li>
//! </ul>
//! 
//! <p>To learn more about AWS Auto Scaling, including information about granting IAM users required
//! permissions for AWS Auto Scaling actions, see the <a href="https://docs.aws.amazon.com/autoscaling/plans/userguide/what-is-aws-auto-scaling.html">AWS Auto Scaling User Guide</a>. </p>
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
//! 
//! # Examples
//! Examples can be found [here](https://github.com/awslabs/aws-sdk-rust/tree/main/examples/autoscalingplans).


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

mod json_deser;

mod json_ser;

/// Endpoints standard library functions
mod endpoint_lib;

mod json_errors;

/// Crate version number.
                    pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
pub use aws_smithy_http::endpoint::Endpoint;
static API_METADATA: aws_http::user_agent::ApiMetadata = aws_http::user_agent::ApiMetadata::new("autoscalingplans", PKG_VERSION);
pub use aws_types::app_name::AppName;
#[doc(inline)]
pub use client::Client;
pub use aws_types::region::Region;
pub use aws_credential_types::Credentials;


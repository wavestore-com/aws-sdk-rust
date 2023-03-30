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
//! <p>Organizations is a web service that enables you to consolidate your multiple
//! Amazon Web Services accounts into an <i>organization</i> and centrally manage your
//! accounts and their resources.</p>
//! <p>This guide provides descriptions of the Organizations operations. For more
//! information about using this service, see the <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_introduction.html">Organizations User Guide</a>.</p>
//! <p>
//! <b>Support and feedback for Organizations</b>
//! </p>
//! <p>We welcome your feedback. Send your comments to <a href="mailto:feedback-awsorganizations@amazon.com">feedback-awsorganizations@amazon.com</a> or post your feedback and questions in
//! the <a href="http://forums.aws.amazon.com/forum.jspa?forumID=219">Organizations support forum</a>. For
//! more information about the Amazon Web Services support forums, see <a href="http://forums.aws.amazon.com/help.jspa">Forums Help</a>.</p>
//! <p>
//! <b>Endpoint to call When using the CLI or the Amazon Web Services
//! SDK</b>
//! </p>
//! <p>For the current release of Organizations, specify the <code>us-east-1</code> region
//! for all Amazon Web Services API and CLI calls made from the commercial Amazon Web Services Regions outside of
//! China. If calling from one of the Amazon Web Services Regions in China, then specify
//! <code>cn-northwest-1</code>. You can do this in the CLI by using these parameters
//! and commands:</p>
//! <ul>
//! <li>
//! <p>Use the following parameter with each command to specify both the endpoint and
//! its region:</p>
//! <p>
//! <code>--endpoint-url https://organizations.us-east-1.amazonaws.com</code>
//! <i>(from commercial Amazon Web Services Regions outside of China)</i>
//! </p>
//! <p>or</p>
//! <p>
//! <code>--endpoint-url
//! https://organizations.cn-northwest-1.amazonaws.com.cn</code>
//! <i>(from Amazon Web Services Regions in China)</i>
//! </p>
//! </li>
//! <li>
//! <p>Use the default endpoint, but configure your default region with this
//! command:</p>
//! <p>
//! <code>aws configure set default.region us-east-1</code>
//! <i>(from commercial Amazon Web Services Regions outside of China)</i>
//! </p>
//! <p>or</p>
//! <p>
//! <code>aws configure set default.region cn-northwest-1</code>
//! <i>(from Amazon Web Services Regions in China)</i>
//! </p>
//! </li>
//! <li>
//! <p>Use the following parameter with each command to specify the endpoint:</p>
//! <p>
//! <code>--region us-east-1</code>
//! <i>(from commercial Amazon Web Services Regions outside of China)</i>
//! </p>
//! <p>or</p>
//! <p>
//! <code>--region cn-northwest-1</code>
//! <i>(from Amazon Web Services Regions in China)</i>
//! </p>
//! </li>
//! </ul>
//! <p>
//! <b>Recording API Requests</b>
//! </p>
//! <p>Organizations supports CloudTrail, a service that records Amazon Web Services API calls for your
//! Amazon Web Services account and delivers log files to an Amazon S3 bucket. By using information collected
//! by CloudTrail, you can determine which requests the Organizations service received, who made the
//! request and when, and so on. For more about Organizations and its support for CloudTrail, see
//! <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_incident-response.html#orgs_cloudtrail-integration">Logging
//! Organizations Events with CloudTrail</a> in the <i>Organizations User Guide</i>.
//! To learn more about CloudTrail, including how to turn it on and find your log files, see the
//! <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/what_is_cloud_trail_top_level.html">CloudTrail User Guide</a>.</p>
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
static API_METADATA: aws_http::user_agent::ApiMetadata = aws_http::user_agent::ApiMetadata::new("organizations", PKG_VERSION);
pub use aws_types::app_name::AppName;
#[doc(inline)]
pub use client::Client;
pub use aws_types::region::Region;
pub use aws_credential_types::Credentials;


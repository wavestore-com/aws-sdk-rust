// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A summary of enabled controls.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct EnabledControlSummary  {
    /// <p>The ARN of the control. Only <b>Strongly recommended</b> and <b>Elective</b> controls are permitted, with the exception of the <b>Region deny</b> guardrail.</p>
    #[doc(hidden)]
    pub control_identifier: std::option::Option<std::string::String>,
}
impl EnabledControlSummary {
    /// <p>The ARN of the control. Only <b>Strongly recommended</b> and <b>Elective</b> controls are permitted, with the exception of the <b>Region deny</b> guardrail.</p>
    pub fn control_identifier(&self) -> std::option::Option<& str> {
        self.control_identifier.as_deref()
    }
}
/// See [`EnabledControlSummary`](crate::model::EnabledControlSummary).
pub mod enabled_control_summary {
    
    /// A builder for [`EnabledControlSummary`](crate::model::EnabledControlSummary).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) control_identifier: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The ARN of the control. Only <b>Strongly recommended</b> and <b>Elective</b> controls are permitted, with the exception of the <b>Region deny</b> guardrail.</p>
        pub fn control_identifier(mut self, input: impl Into<std::string::String>) -> Self {
            self.control_identifier = Some(input.into());
            self
        }
        /// <p>The ARN of the control. Only <b>Strongly recommended</b> and <b>Elective</b> controls are permitted, with the exception of the <b>Region deny</b> guardrail.</p>
        pub fn set_control_identifier(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.control_identifier = input; self
        }
        /// Consumes the builder and constructs a [`EnabledControlSummary`](crate::model::EnabledControlSummary).
        pub fn build(self) -> crate::model::EnabledControlSummary {
            crate::model::EnabledControlSummary {
                control_identifier: self.control_identifier
                ,
            }
        }
    }
    
    
}
impl EnabledControlSummary {
    /// Creates a new builder-style object to manufacture [`EnabledControlSummary`](crate::model::EnabledControlSummary).
    pub fn builder() -> crate::model::enabled_control_summary::Builder {
        crate::model::enabled_control_summary::Builder::default()
    }
}

/// <p>An operation performed by the control.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ControlOperation  {
    /// <p>One of <code>ENABLE_CONTROL</code> or <code>DISABLE_CONTROL</code>.</p>
    #[doc(hidden)]
    pub operation_type: std::option::Option<crate::model::ControlOperationType>,
    /// <p>The time that the operation began.</p>
    #[doc(hidden)]
    pub start_time: std::option::Option<aws_smithy_types::DateTime>,
    /// <p>The time that the operation finished.</p>
    #[doc(hidden)]
    pub end_time: std::option::Option<aws_smithy_types::DateTime>,
    /// <p>One of <code>IN_PROGRESS</code>, <code>SUCEEDED</code>, or <code>FAILED</code>.</p>
    #[doc(hidden)]
    pub status: std::option::Option<crate::model::ControlOperationStatus>,
    /// <p>If the operation result is <code>FAILED</code>, this string contains a message explaining why the operation failed.</p>
    #[doc(hidden)]
    pub status_message: std::option::Option<std::string::String>,
}
impl ControlOperation {
    /// <p>One of <code>ENABLE_CONTROL</code> or <code>DISABLE_CONTROL</code>.</p>
    pub fn operation_type(&self) -> std::option::Option<& crate::model::ControlOperationType> {
        self.operation_type.as_ref()
    }
    /// <p>The time that the operation began.</p>
    pub fn start_time(&self) -> std::option::Option<& aws_smithy_types::DateTime> {
        self.start_time.as_ref()
    }
    /// <p>The time that the operation finished.</p>
    pub fn end_time(&self) -> std::option::Option<& aws_smithy_types::DateTime> {
        self.end_time.as_ref()
    }
    /// <p>One of <code>IN_PROGRESS</code>, <code>SUCEEDED</code>, or <code>FAILED</code>.</p>
    pub fn status(&self) -> std::option::Option<& crate::model::ControlOperationStatus> {
        self.status.as_ref()
    }
    /// <p>If the operation result is <code>FAILED</code>, this string contains a message explaining why the operation failed.</p>
    pub fn status_message(&self) -> std::option::Option<& str> {
        self.status_message.as_deref()
    }
}
/// See [`ControlOperation`](crate::model::ControlOperation).
pub mod control_operation {
    
    /// A builder for [`ControlOperation`](crate::model::ControlOperation).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) operation_type: std::option::Option<crate::model::ControlOperationType>,
        pub(crate) start_time: std::option::Option<aws_smithy_types::DateTime>,
        pub(crate) end_time: std::option::Option<aws_smithy_types::DateTime>,
        pub(crate) status: std::option::Option<crate::model::ControlOperationStatus>,
        pub(crate) status_message: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>One of <code>ENABLE_CONTROL</code> or <code>DISABLE_CONTROL</code>.</p>
        pub fn operation_type(mut self, input: crate::model::ControlOperationType) -> Self {
            self.operation_type = Some(input);
            self
        }
        /// <p>One of <code>ENABLE_CONTROL</code> or <code>DISABLE_CONTROL</code>.</p>
        pub fn set_operation_type(mut self, input: std::option::Option<crate::model::ControlOperationType>) -> Self {
            self.operation_type = input; self
        }
        /// <p>The time that the operation began.</p>
        pub fn start_time(mut self, input: aws_smithy_types::DateTime) -> Self {
            self.start_time = Some(input);
            self
        }
        /// <p>The time that the operation began.</p>
        pub fn set_start_time(mut self, input: std::option::Option<aws_smithy_types::DateTime>) -> Self {
            self.start_time = input; self
        }
        /// <p>The time that the operation finished.</p>
        pub fn end_time(mut self, input: aws_smithy_types::DateTime) -> Self {
            self.end_time = Some(input);
            self
        }
        /// <p>The time that the operation finished.</p>
        pub fn set_end_time(mut self, input: std::option::Option<aws_smithy_types::DateTime>) -> Self {
            self.end_time = input; self
        }
        /// <p>One of <code>IN_PROGRESS</code>, <code>SUCEEDED</code>, or <code>FAILED</code>.</p>
        pub fn status(mut self, input: crate::model::ControlOperationStatus) -> Self {
            self.status = Some(input);
            self
        }
        /// <p>One of <code>IN_PROGRESS</code>, <code>SUCEEDED</code>, or <code>FAILED</code>.</p>
        pub fn set_status(mut self, input: std::option::Option<crate::model::ControlOperationStatus>) -> Self {
            self.status = input; self
        }
        /// <p>If the operation result is <code>FAILED</code>, this string contains a message explaining why the operation failed.</p>
        pub fn status_message(mut self, input: impl Into<std::string::String>) -> Self {
            self.status_message = Some(input.into());
            self
        }
        /// <p>If the operation result is <code>FAILED</code>, this string contains a message explaining why the operation failed.</p>
        pub fn set_status_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.status_message = input; self
        }
        /// Consumes the builder and constructs a [`ControlOperation`](crate::model::ControlOperation).
        pub fn build(self) -> crate::model::ControlOperation {
            crate::model::ControlOperation {
                operation_type: self.operation_type
                ,
                start_time: self.start_time
                ,
                end_time: self.end_time
                ,
                status: self.status
                ,
                status_message: self.status_message
                ,
            }
        }
    }
    
    
}
impl ControlOperation {
    /// Creates a new builder-style object to manufacture [`ControlOperation`](crate::model::ControlOperation).
    pub fn builder() -> crate::model::control_operation::Builder {
        crate::model::control_operation::Builder::default()
    }
}

/// When writing a match expression against `ControlOperationStatus`, it is important to ensure
/// your code is forward-compatible. That is, if a match arm handles a case for a
/// feature that is supported by the service but has not been represented as an enum
/// variant in a current version of SDK, your code should continue to work when you
/// upgrade SDK to a future version in which the enum does include a variant for that
/// feature.
/// 
/// Here is an example of how you can make a match expression forward-compatible:
/// 
/// ```text
/// # let controloperationstatus = unimplemented!();
/// match controloperationstatus {
///     ControlOperationStatus::Failed => { /* ... */ },
///     ControlOperationStatus::InProgress => { /* ... */ },
///     ControlOperationStatus::Succeeded => { /* ... */ },
///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
///     _ => { /* ... */ },
/// }
/// ```
/// The above code demonstrates that when `controloperationstatus` represents
/// `NewFeature`, the execution path will lead to the second last match arm,
/// even though the enum does not contain a variant `ControlOperationStatus::NewFeature`
/// in the current version of SDK. The reason is that the variable `other`,
/// created by the `@` operator, is bound to
/// `ControlOperationStatus::Unknown(UnknownVariantValue("NewFeature".to_owned()))`
/// and calling `as_str` on it yields `"NewFeature"`.
/// This match expression is forward-compatible when executed with a newer
/// version of SDK where the variant `ControlOperationStatus::NewFeature` is defined.
/// Specifically, when `controloperationstatus` represents `NewFeature`,
/// the execution path will hit the second last match arm as before by virtue of
/// calling `as_str` on `ControlOperationStatus::NewFeature` also yielding `"NewFeature"`.
/// 
/// Explicitly matching on the `Unknown` variant should
/// be avoided for two reasons:
/// - The inner data `UnknownVariantValue` is opaque, and no further information can be extracted.
/// - It might inadvertently shadow other intended match arms.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::Eq, std::cmp::Ord, std::cmp::PartialEq, std::cmp::PartialOrd, std::fmt::Debug, std::hash::Hash)]
pub enum ControlOperationStatus {
    #[allow(missing_docs)] // documentation missing in model
    Failed,
    #[allow(missing_docs)] // documentation missing in model
    InProgress,
    #[allow(missing_docs)] // documentation missing in model
    Succeeded,
    /// `Unknown` contains new variants that have been added since this code was generated.
    Unknown(crate::types::UnknownVariantValue)
}
impl std::convert::From<&str> for ControlOperationStatus {
                fn from(s: &str) -> Self {
                    match s {
                        "FAILED" => ControlOperationStatus::Failed,
"IN_PROGRESS" => ControlOperationStatus::InProgress,
"SUCCEEDED" => ControlOperationStatus::Succeeded,
other => ControlOperationStatus::Unknown(crate::types::UnknownVariantValue(other.to_owned()))
                    }
                }
            }
impl std::str::FromStr for ControlOperationStatus {
                type Err = std::convert::Infallible;

                fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                    Ok(ControlOperationStatus::from(s))
                }
            }
impl ControlOperationStatus {
                /// Returns the `&str` value of the enum member.
                pub fn as_str(&self) -> &str {
                    match self {
    ControlOperationStatus::Failed => "FAILED",
    ControlOperationStatus::InProgress => "IN_PROGRESS",
    ControlOperationStatus::Succeeded => "SUCCEEDED",
    ControlOperationStatus::Unknown(value) => value.as_str()
}
                }
                /// Returns all the `&str` representations of the enum members.
                pub const fn values() -> &'static [&'static str] {
                    &["FAILED", "IN_PROGRESS", "SUCCEEDED"]
                }
            }
impl AsRef<str> for ControlOperationStatus {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }

/// When writing a match expression against `ControlOperationType`, it is important to ensure
/// your code is forward-compatible. That is, if a match arm handles a case for a
/// feature that is supported by the service but has not been represented as an enum
/// variant in a current version of SDK, your code should continue to work when you
/// upgrade SDK to a future version in which the enum does include a variant for that
/// feature.
/// 
/// Here is an example of how you can make a match expression forward-compatible:
/// 
/// ```text
/// # let controloperationtype = unimplemented!();
/// match controloperationtype {
///     ControlOperationType::DisableControl => { /* ... */ },
///     ControlOperationType::EnableControl => { /* ... */ },
///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
///     _ => { /* ... */ },
/// }
/// ```
/// The above code demonstrates that when `controloperationtype` represents
/// `NewFeature`, the execution path will lead to the second last match arm,
/// even though the enum does not contain a variant `ControlOperationType::NewFeature`
/// in the current version of SDK. The reason is that the variable `other`,
/// created by the `@` operator, is bound to
/// `ControlOperationType::Unknown(UnknownVariantValue("NewFeature".to_owned()))`
/// and calling `as_str` on it yields `"NewFeature"`.
/// This match expression is forward-compatible when executed with a newer
/// version of SDK where the variant `ControlOperationType::NewFeature` is defined.
/// Specifically, when `controloperationtype` represents `NewFeature`,
/// the execution path will hit the second last match arm as before by virtue of
/// calling `as_str` on `ControlOperationType::NewFeature` also yielding `"NewFeature"`.
/// 
/// Explicitly matching on the `Unknown` variant should
/// be avoided for two reasons:
/// - The inner data `UnknownVariantValue` is opaque, and no further information can be extracted.
/// - It might inadvertently shadow other intended match arms.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::Eq, std::cmp::Ord, std::cmp::PartialEq, std::cmp::PartialOrd, std::fmt::Debug, std::hash::Hash)]
pub enum ControlOperationType {
    #[allow(missing_docs)] // documentation missing in model
    DisableControl,
    #[allow(missing_docs)] // documentation missing in model
    EnableControl,
    /// `Unknown` contains new variants that have been added since this code was generated.
    Unknown(crate::types::UnknownVariantValue)
}
impl std::convert::From<&str> for ControlOperationType {
                fn from(s: &str) -> Self {
                    match s {
                        "DISABLE_CONTROL" => ControlOperationType::DisableControl,
"ENABLE_CONTROL" => ControlOperationType::EnableControl,
other => ControlOperationType::Unknown(crate::types::UnknownVariantValue(other.to_owned()))
                    }
                }
            }
impl std::str::FromStr for ControlOperationType {
                type Err = std::convert::Infallible;

                fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                    Ok(ControlOperationType::from(s))
                }
            }
impl ControlOperationType {
                /// Returns the `&str` value of the enum member.
                pub fn as_str(&self) -> &str {
                    match self {
    ControlOperationType::DisableControl => "DISABLE_CONTROL",
    ControlOperationType::EnableControl => "ENABLE_CONTROL",
    ControlOperationType::Unknown(value) => value.as_str()
}
                }
                /// Returns all the `&str` representations of the enum members.
                pub const fn values() -> &'static [&'static str] {
                    &["DISABLE_CONTROL", "ENABLE_CONTROL"]
                }
            }
impl AsRef<str> for ControlOperationType {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }


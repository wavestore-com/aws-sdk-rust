// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// When writing a match expression against `PayloadFormatIndicator`, it is important to ensure
/// your code is forward-compatible. That is, if a match arm handles a case for a
/// feature that is supported by the service but has not been represented as an enum
/// variant in a current version of SDK, your code should continue to work when you
/// upgrade SDK to a future version in which the enum does include a variant for that
/// feature.
/// 
/// Here is an example of how you can make a match expression forward-compatible:
/// 
/// ```text
/// # let payloadformatindicator = unimplemented!();
/// match payloadformatindicator {
///     PayloadFormatIndicator::UnspecifiedBytes => { /* ... */ },
///     PayloadFormatIndicator::Utf8Data => { /* ... */ },
///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
///     _ => { /* ... */ },
/// }
/// ```
/// The above code demonstrates that when `payloadformatindicator` represents
/// `NewFeature`, the execution path will lead to the second last match arm,
/// even though the enum does not contain a variant `PayloadFormatIndicator::NewFeature`
/// in the current version of SDK. The reason is that the variable `other`,
/// created by the `@` operator, is bound to
/// `PayloadFormatIndicator::Unknown(UnknownVariantValue("NewFeature".to_owned()))`
/// and calling `as_str` on it yields `"NewFeature"`.
/// This match expression is forward-compatible when executed with a newer
/// version of SDK where the variant `PayloadFormatIndicator::NewFeature` is defined.
/// Specifically, when `payloadformatindicator` represents `NewFeature`,
/// the execution path will hit the second last match arm as before by virtue of
/// calling `as_str` on `PayloadFormatIndicator::NewFeature` also yielding `"NewFeature"`.
/// 
/// Explicitly matching on the `Unknown` variant should
/// be avoided for two reasons:
/// - The inner data `UnknownVariantValue` is opaque, and no further information can be extracted.
/// - It might inadvertently shadow other intended match arms.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::Eq, std::cmp::Ord, std::cmp::PartialEq, std::cmp::PartialOrd, std::fmt::Debug, std::hash::Hash)]
pub enum PayloadFormatIndicator {
    #[allow(missing_docs)] // documentation missing in model
    UnspecifiedBytes,
    #[allow(missing_docs)] // documentation missing in model
    Utf8Data,
    /// `Unknown` contains new variants that have been added since this code was generated.
    Unknown(crate::types::UnknownVariantValue)
}
impl std::convert::From<&str> for PayloadFormatIndicator {
                fn from(s: &str) -> Self {
                    match s {
                        "UNSPECIFIED_BYTES" => PayloadFormatIndicator::UnspecifiedBytes,
"UTF8_DATA" => PayloadFormatIndicator::Utf8Data,
other => PayloadFormatIndicator::Unknown(crate::types::UnknownVariantValue(other.to_owned()))
                    }
                }
            }
impl std::str::FromStr for PayloadFormatIndicator {
                type Err = std::convert::Infallible;

                fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                    Ok(PayloadFormatIndicator::from(s))
                }
            }
impl PayloadFormatIndicator {
                /// Returns the `&str` value of the enum member.
                pub fn as_str(&self) -> &str {
                    match self {
    PayloadFormatIndicator::UnspecifiedBytes => "UNSPECIFIED_BYTES",
    PayloadFormatIndicator::Utf8Data => "UTF8_DATA",
    PayloadFormatIndicator::Unknown(value) => value.as_str()
}
                }
                /// Returns all the `&str` representations of the enum members.
                pub const fn values() -> &'static [&'static str] {
                    &["UNSPECIFIED_BYTES", "UTF8_DATA"]
                }
            }
impl AsRef<str> for PayloadFormatIndicator {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }

/// <p>Information about a single retained message.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct RetainedMessageSummary  {
    /// <p>The topic name to which the retained message was published.</p>
    #[doc(hidden)]
    pub topic: std::option::Option<std::string::String>,
    /// <p>The size of the retained message's payload in bytes.</p>
    #[doc(hidden)]
    pub payload_size: i64,
    /// <p>The quality of service (QoS) level used to publish the retained message.</p>
    #[doc(hidden)]
    pub qos: i32,
    /// <p>The Epoch date and time, in milliseconds, when the retained message was stored by IoT.</p>
    #[doc(hidden)]
    pub last_modified_time: i64,
}
impl RetainedMessageSummary {
    /// <p>The topic name to which the retained message was published.</p>
    pub fn topic(&self) -> std::option::Option<& str> {
        self.topic.as_deref()
    }
    /// <p>The size of the retained message's payload in bytes.</p>
    pub fn payload_size(&self) -> i64 {
        self.payload_size
    }
    /// <p>The quality of service (QoS) level used to publish the retained message.</p>
    pub fn qos(&self) -> i32 {
        self.qos
    }
    /// <p>The Epoch date and time, in milliseconds, when the retained message was stored by IoT.</p>
    pub fn last_modified_time(&self) -> i64 {
        self.last_modified_time
    }
}
/// See [`RetainedMessageSummary`](crate::model::RetainedMessageSummary).
pub mod retained_message_summary {
    
    /// A builder for [`RetainedMessageSummary`](crate::model::RetainedMessageSummary).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) topic: std::option::Option<std::string::String>,
        pub(crate) payload_size: std::option::Option<i64>,
        pub(crate) qos: std::option::Option<i32>,
        pub(crate) last_modified_time: std::option::Option<i64>,
    }
    impl Builder {
        /// <p>The topic name to which the retained message was published.</p>
        pub fn topic(mut self, input: impl Into<std::string::String>) -> Self {
            self.topic = Some(input.into());
            self
        }
        /// <p>The topic name to which the retained message was published.</p>
        pub fn set_topic(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.topic = input; self
        }
        /// <p>The size of the retained message's payload in bytes.</p>
        pub fn payload_size(mut self, input: i64) -> Self {
            self.payload_size = Some(input);
            self
        }
        /// <p>The size of the retained message's payload in bytes.</p>
        pub fn set_payload_size(mut self, input: std::option::Option<i64>) -> Self {
            self.payload_size = input; self
        }
        /// <p>The quality of service (QoS) level used to publish the retained message.</p>
        pub fn qos(mut self, input: i32) -> Self {
            self.qos = Some(input);
            self
        }
        /// <p>The quality of service (QoS) level used to publish the retained message.</p>
        pub fn set_qos(mut self, input: std::option::Option<i32>) -> Self {
            self.qos = input; self
        }
        /// <p>The Epoch date and time, in milliseconds, when the retained message was stored by IoT.</p>
        pub fn last_modified_time(mut self, input: i64) -> Self {
            self.last_modified_time = Some(input);
            self
        }
        /// <p>The Epoch date and time, in milliseconds, when the retained message was stored by IoT.</p>
        pub fn set_last_modified_time(mut self, input: std::option::Option<i64>) -> Self {
            self.last_modified_time = input; self
        }
        /// Consumes the builder and constructs a [`RetainedMessageSummary`](crate::model::RetainedMessageSummary).
        pub fn build(self) -> crate::model::RetainedMessageSummary {
            crate::model::RetainedMessageSummary {
                topic: self.topic
                ,
                payload_size: self.payload_size
                    .unwrap_or_default()
                ,
                qos: self.qos
                    .unwrap_or_default()
                ,
                last_modified_time: self.last_modified_time
                    .unwrap_or_default()
                ,
            }
        }
    }
    
    
}
impl RetainedMessageSummary {
    /// Creates a new builder-style object to manufacture [`RetainedMessageSummary`](crate::model::RetainedMessageSummary).
    pub fn builder() -> crate::model::retained_message_summary::Builder {
        crate::model::retained_message_summary::Builder::default()
    }
}


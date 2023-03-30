// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>An error that occured when putting the metric data.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct BatchPutMetricsError  {
    /// <p>The error code of an error that occured when attempting to put metrics.</p> 
    /// <ul> 
    /// <li> <p> <code>METRIC_LIMIT_EXCEEDED</code>: The maximum amount of metrics per resource is exceeded.</p> </li> 
    /// <li> <p> <code>INTERNAL_ERROR</code>: An internal error occured.</p> </li> 
    /// <li> <p> <code>VALIDATION_ERROR</code>: The metric data failed validation.</p> </li> 
    /// <li> <p> <code>CONFLICT_ERROR</code>: Multiple requests attempted to modify the same data simultaneously.</p> </li> 
    /// </ul>
    #[doc(hidden)]
    pub code: std::option::Option<crate::model::PutMetricsErrorCode>,
    /// <p>An index that corresponds to the metric in the request.</p>
    #[doc(hidden)]
    pub metric_index: i32,
}
impl BatchPutMetricsError {
    /// <p>The error code of an error that occured when attempting to put metrics.</p> 
    /// <ul> 
    /// <li> <p> <code>METRIC_LIMIT_EXCEEDED</code>: The maximum amount of metrics per resource is exceeded.</p> </li> 
    /// <li> <p> <code>INTERNAL_ERROR</code>: An internal error occured.</p> </li> 
    /// <li> <p> <code>VALIDATION_ERROR</code>: The metric data failed validation.</p> </li> 
    /// <li> <p> <code>CONFLICT_ERROR</code>: Multiple requests attempted to modify the same data simultaneously.</p> </li> 
    /// </ul>
    pub fn code(&self) -> std::option::Option<& crate::model::PutMetricsErrorCode> {
        self.code.as_ref()
    }
    /// <p>An index that corresponds to the metric in the request.</p>
    pub fn metric_index(&self) -> i32 {
        self.metric_index
    }
}
/// See [`BatchPutMetricsError`](crate::model::BatchPutMetricsError).
pub mod batch_put_metrics_error {
    
    /// A builder for [`BatchPutMetricsError`](crate::model::BatchPutMetricsError).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) code: std::option::Option<crate::model::PutMetricsErrorCode>,
        pub(crate) metric_index: std::option::Option<i32>,
    }
    impl Builder {
        /// <p>The error code of an error that occured when attempting to put metrics.</p> 
        /// <ul> 
        /// <li> <p> <code>METRIC_LIMIT_EXCEEDED</code>: The maximum amount of metrics per resource is exceeded.</p> </li> 
        /// <li> <p> <code>INTERNAL_ERROR</code>: An internal error occured.</p> </li> 
        /// <li> <p> <code>VALIDATION_ERROR</code>: The metric data failed validation.</p> </li> 
        /// <li> <p> <code>CONFLICT_ERROR</code>: Multiple requests attempted to modify the same data simultaneously.</p> </li> 
        /// </ul>
        pub fn code(mut self, input: crate::model::PutMetricsErrorCode) -> Self {
            self.code = Some(input);
            self
        }
        /// <p>The error code of an error that occured when attempting to put metrics.</p> 
        /// <ul> 
        /// <li> <p> <code>METRIC_LIMIT_EXCEEDED</code>: The maximum amount of metrics per resource is exceeded.</p> </li> 
        /// <li> <p> <code>INTERNAL_ERROR</code>: An internal error occured.</p> </li> 
        /// <li> <p> <code>VALIDATION_ERROR</code>: The metric data failed validation.</p> </li> 
        /// <li> <p> <code>CONFLICT_ERROR</code>: Multiple requests attempted to modify the same data simultaneously.</p> </li> 
        /// </ul>
        pub fn set_code(mut self, input: std::option::Option<crate::model::PutMetricsErrorCode>) -> Self {
            self.code = input; self
        }
        /// <p>An index that corresponds to the metric in the request.</p>
        pub fn metric_index(mut self, input: i32) -> Self {
            self.metric_index = Some(input);
            self
        }
        /// <p>An index that corresponds to the metric in the request.</p>
        pub fn set_metric_index(mut self, input: std::option::Option<i32>) -> Self {
            self.metric_index = input; self
        }
        /// Consumes the builder and constructs a [`BatchPutMetricsError`](crate::model::BatchPutMetricsError).
        pub fn build(self) -> crate::model::BatchPutMetricsError {
            crate::model::BatchPutMetricsError {
                code: self.code
                ,
                metric_index: self.metric_index
                    .unwrap_or_default()
                ,
            }
        }
    }
    
    
}
impl BatchPutMetricsError {
    /// Creates a new builder-style object to manufacture [`BatchPutMetricsError`](crate::model::BatchPutMetricsError).
    pub fn builder() -> crate::model::batch_put_metrics_error::Builder {
        crate::model::batch_put_metrics_error::Builder::default()
    }
}

/// When writing a match expression against `PutMetricsErrorCode`, it is important to ensure
/// your code is forward-compatible. That is, if a match arm handles a case for a
/// feature that is supported by the service but has not been represented as an enum
/// variant in a current version of SDK, your code should continue to work when you
/// upgrade SDK to a future version in which the enum does include a variant for that
/// feature.
/// 
/// Here is an example of how you can make a match expression forward-compatible:
/// 
/// ```text
/// # let putmetricserrorcode = unimplemented!();
/// match putmetricserrorcode {
///     PutMetricsErrorCode::ConflictError => { /* ... */ },
///     PutMetricsErrorCode::InternalError => { /* ... */ },
///     PutMetricsErrorCode::MetricLimitExceeded => { /* ... */ },
///     PutMetricsErrorCode::ValidationError => { /* ... */ },
///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
///     _ => { /* ... */ },
/// }
/// ```
/// The above code demonstrates that when `putmetricserrorcode` represents
/// `NewFeature`, the execution path will lead to the second last match arm,
/// even though the enum does not contain a variant `PutMetricsErrorCode::NewFeature`
/// in the current version of SDK. The reason is that the variable `other`,
/// created by the `@` operator, is bound to
/// `PutMetricsErrorCode::Unknown(UnknownVariantValue("NewFeature".to_owned()))`
/// and calling `as_str` on it yields `"NewFeature"`.
/// This match expression is forward-compatible when executed with a newer
/// version of SDK where the variant `PutMetricsErrorCode::NewFeature` is defined.
/// Specifically, when `putmetricserrorcode` represents `NewFeature`,
/// the execution path will hit the second last match arm as before by virtue of
/// calling `as_str` on `PutMetricsErrorCode::NewFeature` also yielding `"NewFeature"`.
/// 
/// Explicitly matching on the `Unknown` variant should
/// be avoided for two reasons:
/// - The inner data `UnknownVariantValue` is opaque, and no further information can be extracted.
/// - It might inadvertently shadow other intended match arms.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::Eq, std::cmp::Ord, std::cmp::PartialEq, std::cmp::PartialOrd, std::fmt::Debug, std::hash::Hash)]
pub enum PutMetricsErrorCode {
    #[allow(missing_docs)] // documentation missing in model
    ConflictError,
    #[allow(missing_docs)] // documentation missing in model
    InternalError,
    #[allow(missing_docs)] // documentation missing in model
    MetricLimitExceeded,
    #[allow(missing_docs)] // documentation missing in model
    ValidationError,
    /// `Unknown` contains new variants that have been added since this code was generated.
    Unknown(crate::types::UnknownVariantValue)
}
impl std::convert::From<&str> for PutMetricsErrorCode {
                fn from(s: &str) -> Self {
                    match s {
                        "CONFLICT_ERROR" => PutMetricsErrorCode::ConflictError,
"INTERNAL_ERROR" => PutMetricsErrorCode::InternalError,
"METRIC_LIMIT_EXCEEDED" => PutMetricsErrorCode::MetricLimitExceeded,
"VALIDATION_ERROR" => PutMetricsErrorCode::ValidationError,
other => PutMetricsErrorCode::Unknown(crate::types::UnknownVariantValue(other.to_owned()))
                    }
                }
            }
impl std::str::FromStr for PutMetricsErrorCode {
                type Err = std::convert::Infallible;

                fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                    Ok(PutMetricsErrorCode::from(s))
                }
            }
impl PutMetricsErrorCode {
                /// Returns the `&str` value of the enum member.
                pub fn as_str(&self) -> &str {
                    match self {
    PutMetricsErrorCode::ConflictError => "CONFLICT_ERROR",
    PutMetricsErrorCode::InternalError => "INTERNAL_ERROR",
    PutMetricsErrorCode::MetricLimitExceeded => "METRIC_LIMIT_EXCEEDED",
    PutMetricsErrorCode::ValidationError => "VALIDATION_ERROR",
    PutMetricsErrorCode::Unknown(value) => value.as_str()
}
                }
                /// Returns all the `&str` representations of the enum members.
                pub const fn values() -> &'static [&'static str] {
                    &["CONFLICT_ERROR", "INTERNAL_ERROR", "METRIC_LIMIT_EXCEEDED", "VALIDATION_ERROR"]
                }
            }
impl AsRef<str> for PutMetricsErrorCode {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }

/// <p>The raw metric data to associate with the resource.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct RawMetricData  {
    /// <p>The name of the metric.</p>
    #[doc(hidden)]
    pub metric_name: std::option::Option<std::string::String>,
    /// <p>The time that the metric was recorded.</p>
    #[doc(hidden)]
    pub timestamp: std::option::Option<aws_smithy_types::DateTime>,
    /// <p>The metric step (epoch). </p>
    #[doc(hidden)]
    pub step: std::option::Option<i32>,
    /// <p>The metric value.</p>
    #[doc(hidden)]
    pub value: f64,
}
impl RawMetricData {
    /// <p>The name of the metric.</p>
    pub fn metric_name(&self) -> std::option::Option<& str> {
        self.metric_name.as_deref()
    }
    /// <p>The time that the metric was recorded.</p>
    pub fn timestamp(&self) -> std::option::Option<& aws_smithy_types::DateTime> {
        self.timestamp.as_ref()
    }
    /// <p>The metric step (epoch). </p>
    pub fn step(&self) -> std::option::Option<i32> {
        self.step
    }
    /// <p>The metric value.</p>
    pub fn value(&self) -> f64 {
        self.value
    }
}
/// See [`RawMetricData`](crate::model::RawMetricData).
pub mod raw_metric_data {
    
    /// A builder for [`RawMetricData`](crate::model::RawMetricData).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) metric_name: std::option::Option<std::string::String>,
        pub(crate) timestamp: std::option::Option<aws_smithy_types::DateTime>,
        pub(crate) step: std::option::Option<i32>,
        pub(crate) value: std::option::Option<f64>,
    }
    impl Builder {
        /// <p>The name of the metric.</p>
        pub fn metric_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.metric_name = Some(input.into());
            self
        }
        /// <p>The name of the metric.</p>
        pub fn set_metric_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.metric_name = input; self
        }
        /// <p>The time that the metric was recorded.</p>
        pub fn timestamp(mut self, input: aws_smithy_types::DateTime) -> Self {
            self.timestamp = Some(input);
            self
        }
        /// <p>The time that the metric was recorded.</p>
        pub fn set_timestamp(mut self, input: std::option::Option<aws_smithy_types::DateTime>) -> Self {
            self.timestamp = input; self
        }
        /// <p>The metric step (epoch). </p>
        pub fn step(mut self, input: i32) -> Self {
            self.step = Some(input);
            self
        }
        /// <p>The metric step (epoch). </p>
        pub fn set_step(mut self, input: std::option::Option<i32>) -> Self {
            self.step = input; self
        }
        /// <p>The metric value.</p>
        pub fn value(mut self, input: f64) -> Self {
            self.value = Some(input);
            self
        }
        /// <p>The metric value.</p>
        pub fn set_value(mut self, input: std::option::Option<f64>) -> Self {
            self.value = input; self
        }
        /// Consumes the builder and constructs a [`RawMetricData`](crate::model::RawMetricData).
        pub fn build(self) -> crate::model::RawMetricData {
            crate::model::RawMetricData {
                metric_name: self.metric_name
                ,
                timestamp: self.timestamp
                ,
                step: self.step
                ,
                value: self.value
                    .unwrap_or_default()
                ,
            }
        }
    }
    
    
}
impl RawMetricData {
    /// Creates a new builder-style object to manufacture [`RawMetricData`](crate::model::RawMetricData).
    pub fn builder() -> crate::model::raw_metric_data::Builder {
        crate::model::raw_metric_data::Builder::default()
    }
}


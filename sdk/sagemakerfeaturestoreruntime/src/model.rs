// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// When writing a match expression against `TargetStore`, it is important to ensure
/// your code is forward-compatible. That is, if a match arm handles a case for a
/// feature that is supported by the service but has not been represented as an enum
/// variant in a current version of SDK, your code should continue to work when you
/// upgrade SDK to a future version in which the enum does include a variant for that
/// feature.
/// 
/// Here is an example of how you can make a match expression forward-compatible:
/// 
/// ```text
/// # let targetstore = unimplemented!();
/// match targetstore {
///     TargetStore::OfflineStore => { /* ... */ },
///     TargetStore::OnlineStore => { /* ... */ },
///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
///     _ => { /* ... */ },
/// }
/// ```
/// The above code demonstrates that when `targetstore` represents
/// `NewFeature`, the execution path will lead to the second last match arm,
/// even though the enum does not contain a variant `TargetStore::NewFeature`
/// in the current version of SDK. The reason is that the variable `other`,
/// created by the `@` operator, is bound to
/// `TargetStore::Unknown(UnknownVariantValue("NewFeature".to_owned()))`
/// and calling `as_str` on it yields `"NewFeature"`.
/// This match expression is forward-compatible when executed with a newer
/// version of SDK where the variant `TargetStore::NewFeature` is defined.
/// Specifically, when `targetstore` represents `NewFeature`,
/// the execution path will hit the second last match arm as before by virtue of
/// calling `as_str` on `TargetStore::NewFeature` also yielding `"NewFeature"`.
/// 
/// Explicitly matching on the `Unknown` variant should
/// be avoided for two reasons:
/// - The inner data `UnknownVariantValue` is opaque, and no further information can be extracted.
/// - It might inadvertently shadow other intended match arms.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::Eq, std::cmp::Ord, std::cmp::PartialEq, std::cmp::PartialOrd, std::fmt::Debug, std::hash::Hash)]
pub enum TargetStore {
    #[allow(missing_docs)] // documentation missing in model
    OfflineStore,
    #[allow(missing_docs)] // documentation missing in model
    OnlineStore,
    /// `Unknown` contains new variants that have been added since this code was generated.
    Unknown(crate::types::UnknownVariantValue)
}
impl std::convert::From<&str> for TargetStore {
                fn from(s: &str) -> Self {
                    match s {
                        "OfflineStore" => TargetStore::OfflineStore,
"OnlineStore" => TargetStore::OnlineStore,
other => TargetStore::Unknown(crate::types::UnknownVariantValue(other.to_owned()))
                    }
                }
            }
impl std::str::FromStr for TargetStore {
                type Err = std::convert::Infallible;

                fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                    Ok(TargetStore::from(s))
                }
            }
impl TargetStore {
                /// Returns the `&str` value of the enum member.
                pub fn as_str(&self) -> &str {
                    match self {
    TargetStore::OfflineStore => "OfflineStore",
    TargetStore::OnlineStore => "OnlineStore",
    TargetStore::Unknown(value) => value.as_str()
}
                }
                /// Returns all the `&str` representations of the enum members.
                pub const fn values() -> &'static [&'static str] {
                    &["OfflineStore", "OnlineStore"]
                }
            }
impl AsRef<str> for TargetStore {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }

/// <p>The value associated with a feature.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct FeatureValue  {
    /// <p>The name of a feature that a feature value corresponds to.</p>
    #[doc(hidden)]
    pub feature_name: std::option::Option<std::string::String>,
    /// <p>The value associated with a feature, in string format. Note that features types can be String, Integral, or Fractional. This value represents all three types as a string.</p>
    #[doc(hidden)]
    pub value_as_string: std::option::Option<std::string::String>,
}
impl FeatureValue {
    /// <p>The name of a feature that a feature value corresponds to.</p>
    pub fn feature_name(&self) -> std::option::Option<& str> {
        self.feature_name.as_deref()
    }
    /// <p>The value associated with a feature, in string format. Note that features types can be String, Integral, or Fractional. This value represents all three types as a string.</p>
    pub fn value_as_string(&self) -> std::option::Option<& str> {
        self.value_as_string.as_deref()
    }
}
/// See [`FeatureValue`](crate::model::FeatureValue).
pub mod feature_value {
    
    /// A builder for [`FeatureValue`](crate::model::FeatureValue).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) feature_name: std::option::Option<std::string::String>,
        pub(crate) value_as_string: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The name of a feature that a feature value corresponds to.</p>
        pub fn feature_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.feature_name = Some(input.into());
            self
        }
        /// <p>The name of a feature that a feature value corresponds to.</p>
        pub fn set_feature_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.feature_name = input; self
        }
        /// <p>The value associated with a feature, in string format. Note that features types can be String, Integral, or Fractional. This value represents all three types as a string.</p>
        pub fn value_as_string(mut self, input: impl Into<std::string::String>) -> Self {
            self.value_as_string = Some(input.into());
            self
        }
        /// <p>The value associated with a feature, in string format. Note that features types can be String, Integral, or Fractional. This value represents all three types as a string.</p>
        pub fn set_value_as_string(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.value_as_string = input; self
        }
        /// Consumes the builder and constructs a [`FeatureValue`](crate::model::FeatureValue).
        pub fn build(self) -> crate::model::FeatureValue {
            crate::model::FeatureValue {
                feature_name: self.feature_name
                ,
                value_as_string: self.value_as_string
                ,
            }
        }
    }
    
    
}
impl FeatureValue {
    /// Creates a new builder-style object to manufacture [`FeatureValue`](crate::model::FeatureValue).
    pub fn builder() -> crate::model::feature_value::Builder {
        crate::model::feature_value::Builder::default()
    }
}

/// <p>The identifier that identifies the batch of Records you are retrieving in a batch.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct BatchGetRecordIdentifier  {
    /// <p>A <code>FeatureGroupName</code> containing Records you are retrieving in a batch.</p>
    #[doc(hidden)]
    pub feature_group_name: std::option::Option<std::string::String>,
    /// <p>The value for a list of record identifiers in string format.</p>
    #[doc(hidden)]
    pub record_identifiers_value_as_string: std::option::Option<std::vec::Vec<std::string::String>>,
    /// <p>List of names of Features to be retrieved. If not specified, the latest value for all the Features are returned.</p>
    #[doc(hidden)]
    pub feature_names: std::option::Option<std::vec::Vec<std::string::String>>,
}
impl BatchGetRecordIdentifier {
    /// <p>A <code>FeatureGroupName</code> containing Records you are retrieving in a batch.</p>
    pub fn feature_group_name(&self) -> std::option::Option<& str> {
        self.feature_group_name.as_deref()
    }
    /// <p>The value for a list of record identifiers in string format.</p>
    pub fn record_identifiers_value_as_string(&self) -> std::option::Option<& [std::string::String]> {
        self.record_identifiers_value_as_string.as_deref()
    }
    /// <p>List of names of Features to be retrieved. If not specified, the latest value for all the Features are returned.</p>
    pub fn feature_names(&self) -> std::option::Option<& [std::string::String]> {
        self.feature_names.as_deref()
    }
}
/// See [`BatchGetRecordIdentifier`](crate::model::BatchGetRecordIdentifier).
pub mod batch_get_record_identifier {
    
    /// A builder for [`BatchGetRecordIdentifier`](crate::model::BatchGetRecordIdentifier).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) feature_group_name: std::option::Option<std::string::String>,
        pub(crate) record_identifiers_value_as_string: std::option::Option<std::vec::Vec<std::string::String>>,
        pub(crate) feature_names: std::option::Option<std::vec::Vec<std::string::String>>,
    }
    impl Builder {
        /// <p>A <code>FeatureGroupName</code> containing Records you are retrieving in a batch.</p>
        pub fn feature_group_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.feature_group_name = Some(input.into());
            self
        }
        /// <p>A <code>FeatureGroupName</code> containing Records you are retrieving in a batch.</p>
        pub fn set_feature_group_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.feature_group_name = input; self
        }
        /// Appends an item to `record_identifiers_value_as_string`.
        ///
        /// To override the contents of this collection use [`set_record_identifiers_value_as_string`](Self::set_record_identifiers_value_as_string).
        ///
        /// <p>The value for a list of record identifiers in string format.</p>
        pub fn record_identifiers_value_as_string(mut self, input: impl Into<std::string::String>) -> Self {
            let mut v = self.record_identifiers_value_as_string.unwrap_or_default();
                            v.push(input.into());
                            self.record_identifiers_value_as_string = Some(v);
                            self
        }
        /// <p>The value for a list of record identifiers in string format.</p>
        pub fn set_record_identifiers_value_as_string(mut self, input: std::option::Option<std::vec::Vec<std::string::String>>) -> Self {
            self.record_identifiers_value_as_string = input; self
        }
        /// Appends an item to `feature_names`.
        ///
        /// To override the contents of this collection use [`set_feature_names`](Self::set_feature_names).
        ///
        /// <p>List of names of Features to be retrieved. If not specified, the latest value for all the Features are returned.</p>
        pub fn feature_names(mut self, input: impl Into<std::string::String>) -> Self {
            let mut v = self.feature_names.unwrap_or_default();
                            v.push(input.into());
                            self.feature_names = Some(v);
                            self
        }
        /// <p>List of names of Features to be retrieved. If not specified, the latest value for all the Features are returned.</p>
        pub fn set_feature_names(mut self, input: std::option::Option<std::vec::Vec<std::string::String>>) -> Self {
            self.feature_names = input; self
        }
        /// Consumes the builder and constructs a [`BatchGetRecordIdentifier`](crate::model::BatchGetRecordIdentifier).
        pub fn build(self) -> crate::model::BatchGetRecordIdentifier {
            crate::model::BatchGetRecordIdentifier {
                feature_group_name: self.feature_group_name
                ,
                record_identifiers_value_as_string: self.record_identifiers_value_as_string
                ,
                feature_names: self.feature_names
                ,
            }
        }
    }
    
    
}
impl BatchGetRecordIdentifier {
    /// Creates a new builder-style object to manufacture [`BatchGetRecordIdentifier`](crate::model::BatchGetRecordIdentifier).
    pub fn builder() -> crate::model::batch_get_record_identifier::Builder {
        crate::model::batch_get_record_identifier::Builder::default()
    }
}

/// <p>The error that has occurred when attempting to retrieve a batch of Records.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct BatchGetRecordError  {
    /// <p>The name of the feature group that the record belongs to.</p>
    #[doc(hidden)]
    pub feature_group_name: std::option::Option<std::string::String>,
    /// <p>The value for the <code>RecordIdentifier</code> in string format of a Record from a <code>FeatureGroup</code> that is causing an error when attempting to be retrieved.</p>
    #[doc(hidden)]
    pub record_identifier_value_as_string: std::option::Option<std::string::String>,
    /// <p>The error code of an error that has occured when attempting to retrieve a batch of Records. For more information on errors, see <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_feature_store_GetRecord.html#API_feature_store_GetRecord_Errors">Errors</a>.</p>
    #[doc(hidden)]
    pub error_code: std::option::Option<std::string::String>,
    /// <p>The error message of an error that has occured when attempting to retrieve a record in the batch.</p>
    #[doc(hidden)]
    pub error_message: std::option::Option<std::string::String>,
}
impl BatchGetRecordError {
    /// <p>The name of the feature group that the record belongs to.</p>
    pub fn feature_group_name(&self) -> std::option::Option<& str> {
        self.feature_group_name.as_deref()
    }
    /// <p>The value for the <code>RecordIdentifier</code> in string format of a Record from a <code>FeatureGroup</code> that is causing an error when attempting to be retrieved.</p>
    pub fn record_identifier_value_as_string(&self) -> std::option::Option<& str> {
        self.record_identifier_value_as_string.as_deref()
    }
    /// <p>The error code of an error that has occured when attempting to retrieve a batch of Records. For more information on errors, see <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_feature_store_GetRecord.html#API_feature_store_GetRecord_Errors">Errors</a>.</p>
    pub fn error_code(&self) -> std::option::Option<& str> {
        self.error_code.as_deref()
    }
    /// <p>The error message of an error that has occured when attempting to retrieve a record in the batch.</p>
    pub fn error_message(&self) -> std::option::Option<& str> {
        self.error_message.as_deref()
    }
}
/// See [`BatchGetRecordError`](crate::model::BatchGetRecordError).
pub mod batch_get_record_error {
    
    /// A builder for [`BatchGetRecordError`](crate::model::BatchGetRecordError).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) feature_group_name: std::option::Option<std::string::String>,
        pub(crate) record_identifier_value_as_string: std::option::Option<std::string::String>,
        pub(crate) error_code: std::option::Option<std::string::String>,
        pub(crate) error_message: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The name of the feature group that the record belongs to.</p>
        pub fn feature_group_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.feature_group_name = Some(input.into());
            self
        }
        /// <p>The name of the feature group that the record belongs to.</p>
        pub fn set_feature_group_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.feature_group_name = input; self
        }
        /// <p>The value for the <code>RecordIdentifier</code> in string format of a Record from a <code>FeatureGroup</code> that is causing an error when attempting to be retrieved.</p>
        pub fn record_identifier_value_as_string(mut self, input: impl Into<std::string::String>) -> Self {
            self.record_identifier_value_as_string = Some(input.into());
            self
        }
        /// <p>The value for the <code>RecordIdentifier</code> in string format of a Record from a <code>FeatureGroup</code> that is causing an error when attempting to be retrieved.</p>
        pub fn set_record_identifier_value_as_string(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.record_identifier_value_as_string = input; self
        }
        /// <p>The error code of an error that has occured when attempting to retrieve a batch of Records. For more information on errors, see <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_feature_store_GetRecord.html#API_feature_store_GetRecord_Errors">Errors</a>.</p>
        pub fn error_code(mut self, input: impl Into<std::string::String>) -> Self {
            self.error_code = Some(input.into());
            self
        }
        /// <p>The error code of an error that has occured when attempting to retrieve a batch of Records. For more information on errors, see <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_feature_store_GetRecord.html#API_feature_store_GetRecord_Errors">Errors</a>.</p>
        pub fn set_error_code(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.error_code = input; self
        }
        /// <p>The error message of an error that has occured when attempting to retrieve a record in the batch.</p>
        pub fn error_message(mut self, input: impl Into<std::string::String>) -> Self {
            self.error_message = Some(input.into());
            self
        }
        /// <p>The error message of an error that has occured when attempting to retrieve a record in the batch.</p>
        pub fn set_error_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.error_message = input; self
        }
        /// Consumes the builder and constructs a [`BatchGetRecordError`](crate::model::BatchGetRecordError).
        pub fn build(self) -> crate::model::BatchGetRecordError {
            crate::model::BatchGetRecordError {
                feature_group_name: self.feature_group_name
                ,
                record_identifier_value_as_string: self.record_identifier_value_as_string
                ,
                error_code: self.error_code
                ,
                error_message: self.error_message
                ,
            }
        }
    }
    
    
}
impl BatchGetRecordError {
    /// Creates a new builder-style object to manufacture [`BatchGetRecordError`](crate::model::BatchGetRecordError).
    pub fn builder() -> crate::model::batch_get_record_error::Builder {
        crate::model::batch_get_record_error::Builder::default()
    }
}

/// <p>The output of Records that have been retrieved in a batch.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct BatchGetRecordResultDetail  {
    /// <p>The <code>FeatureGroupName</code> containing Records you retrieved in a batch.</p>
    #[doc(hidden)]
    pub feature_group_name: std::option::Option<std::string::String>,
    /// <p>The value of the record identifer in string format.</p>
    #[doc(hidden)]
    pub record_identifier_value_as_string: std::option::Option<std::string::String>,
    /// <p>The <code>Record</code> retrieved.</p>
    #[doc(hidden)]
    pub record: std::option::Option<std::vec::Vec<crate::model::FeatureValue>>,
}
impl BatchGetRecordResultDetail {
    /// <p>The <code>FeatureGroupName</code> containing Records you retrieved in a batch.</p>
    pub fn feature_group_name(&self) -> std::option::Option<& str> {
        self.feature_group_name.as_deref()
    }
    /// <p>The value of the record identifer in string format.</p>
    pub fn record_identifier_value_as_string(&self) -> std::option::Option<& str> {
        self.record_identifier_value_as_string.as_deref()
    }
    /// <p>The <code>Record</code> retrieved.</p>
    pub fn record(&self) -> std::option::Option<& [crate::model::FeatureValue]> {
        self.record.as_deref()
    }
}
/// See [`BatchGetRecordResultDetail`](crate::model::BatchGetRecordResultDetail).
pub mod batch_get_record_result_detail {
    
    /// A builder for [`BatchGetRecordResultDetail`](crate::model::BatchGetRecordResultDetail).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) feature_group_name: std::option::Option<std::string::String>,
        pub(crate) record_identifier_value_as_string: std::option::Option<std::string::String>,
        pub(crate) record: std::option::Option<std::vec::Vec<crate::model::FeatureValue>>,
    }
    impl Builder {
        /// <p>The <code>FeatureGroupName</code> containing Records you retrieved in a batch.</p>
        pub fn feature_group_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.feature_group_name = Some(input.into());
            self
        }
        /// <p>The <code>FeatureGroupName</code> containing Records you retrieved in a batch.</p>
        pub fn set_feature_group_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.feature_group_name = input; self
        }
        /// <p>The value of the record identifer in string format.</p>
        pub fn record_identifier_value_as_string(mut self, input: impl Into<std::string::String>) -> Self {
            self.record_identifier_value_as_string = Some(input.into());
            self
        }
        /// <p>The value of the record identifer in string format.</p>
        pub fn set_record_identifier_value_as_string(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.record_identifier_value_as_string = input; self
        }
        /// Appends an item to `record`.
        ///
        /// To override the contents of this collection use [`set_record`](Self::set_record).
        ///
        /// <p>The <code>Record</code> retrieved.</p>
        pub fn record(mut self, input: crate::model::FeatureValue) -> Self {
            let mut v = self.record.unwrap_or_default();
                            v.push(input);
                            self.record = Some(v);
                            self
        }
        /// <p>The <code>Record</code> retrieved.</p>
        pub fn set_record(mut self, input: std::option::Option<std::vec::Vec<crate::model::FeatureValue>>) -> Self {
            self.record = input; self
        }
        /// Consumes the builder and constructs a [`BatchGetRecordResultDetail`](crate::model::BatchGetRecordResultDetail).
        pub fn build(self) -> crate::model::BatchGetRecordResultDetail {
            crate::model::BatchGetRecordResultDetail {
                feature_group_name: self.feature_group_name
                ,
                record_identifier_value_as_string: self.record_identifier_value_as_string
                ,
                record: self.record
                ,
            }
        }
    }
    
    
}
impl BatchGetRecordResultDetail {
    /// Creates a new builder-style object to manufacture [`BatchGetRecordResultDetail`](crate::model::BatchGetRecordResultDetail).
    pub fn builder() -> crate::model::batch_get_record_result_detail::Builder {
        crate::model::batch_get_record_result_detail::Builder::default()
    }
}


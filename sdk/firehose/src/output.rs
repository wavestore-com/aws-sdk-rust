// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct UpdateDestinationOutput  {
}
/// See [`UpdateDestinationOutput`](crate::output::UpdateDestinationOutput).
pub mod update_destination_output {
    
    /// A builder for [`UpdateDestinationOutput`](crate::output::UpdateDestinationOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
    }
    impl Builder {
        /// Consumes the builder and constructs a [`UpdateDestinationOutput`](crate::output::UpdateDestinationOutput).
        pub fn build(self) -> crate::output::UpdateDestinationOutput {
            crate::output::UpdateDestinationOutput {
            }
        }
    }
    
    
}
impl UpdateDestinationOutput {
    /// Creates a new builder-style object to manufacture [`UpdateDestinationOutput`](crate::output::UpdateDestinationOutput).
    pub fn builder() -> crate::output::update_destination_output::Builder {
        crate::output::update_destination_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct UntagDeliveryStreamOutput  {
}
/// See [`UntagDeliveryStreamOutput`](crate::output::UntagDeliveryStreamOutput).
pub mod untag_delivery_stream_output {
    
    /// A builder for [`UntagDeliveryStreamOutput`](crate::output::UntagDeliveryStreamOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
    }
    impl Builder {
        /// Consumes the builder and constructs a [`UntagDeliveryStreamOutput`](crate::output::UntagDeliveryStreamOutput).
        pub fn build(self) -> crate::output::UntagDeliveryStreamOutput {
            crate::output::UntagDeliveryStreamOutput {
            }
        }
    }
    
    
}
impl UntagDeliveryStreamOutput {
    /// Creates a new builder-style object to manufacture [`UntagDeliveryStreamOutput`](crate::output::UntagDeliveryStreamOutput).
    pub fn builder() -> crate::output::untag_delivery_stream_output::Builder {
        crate::output::untag_delivery_stream_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct TagDeliveryStreamOutput  {
}
/// See [`TagDeliveryStreamOutput`](crate::output::TagDeliveryStreamOutput).
pub mod tag_delivery_stream_output {
    
    /// A builder for [`TagDeliveryStreamOutput`](crate::output::TagDeliveryStreamOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
    }
    impl Builder {
        /// Consumes the builder and constructs a [`TagDeliveryStreamOutput`](crate::output::TagDeliveryStreamOutput).
        pub fn build(self) -> crate::output::TagDeliveryStreamOutput {
            crate::output::TagDeliveryStreamOutput {
            }
        }
    }
    
    
}
impl TagDeliveryStreamOutput {
    /// Creates a new builder-style object to manufacture [`TagDeliveryStreamOutput`](crate::output::TagDeliveryStreamOutput).
    pub fn builder() -> crate::output::tag_delivery_stream_output::Builder {
        crate::output::tag_delivery_stream_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct StopDeliveryStreamEncryptionOutput  {
}
/// See [`StopDeliveryStreamEncryptionOutput`](crate::output::StopDeliveryStreamEncryptionOutput).
pub mod stop_delivery_stream_encryption_output {
    
    /// A builder for [`StopDeliveryStreamEncryptionOutput`](crate::output::StopDeliveryStreamEncryptionOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
    }
    impl Builder {
        /// Consumes the builder and constructs a [`StopDeliveryStreamEncryptionOutput`](crate::output::StopDeliveryStreamEncryptionOutput).
        pub fn build(self) -> crate::output::StopDeliveryStreamEncryptionOutput {
            crate::output::StopDeliveryStreamEncryptionOutput {
            }
        }
    }
    
    
}
impl StopDeliveryStreamEncryptionOutput {
    /// Creates a new builder-style object to manufacture [`StopDeliveryStreamEncryptionOutput`](crate::output::StopDeliveryStreamEncryptionOutput).
    pub fn builder() -> crate::output::stop_delivery_stream_encryption_output::Builder {
        crate::output::stop_delivery_stream_encryption_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct StartDeliveryStreamEncryptionOutput  {
}
/// See [`StartDeliveryStreamEncryptionOutput`](crate::output::StartDeliveryStreamEncryptionOutput).
pub mod start_delivery_stream_encryption_output {
    
    /// A builder for [`StartDeliveryStreamEncryptionOutput`](crate::output::StartDeliveryStreamEncryptionOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
    }
    impl Builder {
        /// Consumes the builder and constructs a [`StartDeliveryStreamEncryptionOutput`](crate::output::StartDeliveryStreamEncryptionOutput).
        pub fn build(self) -> crate::output::StartDeliveryStreamEncryptionOutput {
            crate::output::StartDeliveryStreamEncryptionOutput {
            }
        }
    }
    
    
}
impl StartDeliveryStreamEncryptionOutput {
    /// Creates a new builder-style object to manufacture [`StartDeliveryStreamEncryptionOutput`](crate::output::StartDeliveryStreamEncryptionOutput).
    pub fn builder() -> crate::output::start_delivery_stream_encryption_output::Builder {
        crate::output::start_delivery_stream_encryption_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct PutRecordBatchOutput  {
    /// <p>The number of records that might have failed processing. This number might be greater than 0 even if the <code>PutRecordBatch</code> call succeeds. Check <code>FailedPutCount</code> to determine whether there are records that you need to resend.</p>
    #[doc(hidden)]
    pub failed_put_count: std::option::Option<i32>,
    /// <p>Indicates whether server-side encryption (SSE) was enabled during this operation.</p>
    #[doc(hidden)]
    pub encrypted: std::option::Option<bool>,
    /// <p>The results array. For each record, the index of the response element is the same as the index used in the request array.</p>
    #[doc(hidden)]
    pub request_responses: std::option::Option<std::vec::Vec<crate::model::PutRecordBatchResponseEntry>>,
}
impl PutRecordBatchOutput {
    /// <p>The number of records that might have failed processing. This number might be greater than 0 even if the <code>PutRecordBatch</code> call succeeds. Check <code>FailedPutCount</code> to determine whether there are records that you need to resend.</p>
    pub fn failed_put_count(&self) -> std::option::Option<i32> {
        self.failed_put_count
    }
    /// <p>Indicates whether server-side encryption (SSE) was enabled during this operation.</p>
    pub fn encrypted(&self) -> std::option::Option<bool> {
        self.encrypted
    }
    /// <p>The results array. For each record, the index of the response element is the same as the index used in the request array.</p>
    pub fn request_responses(&self) -> std::option::Option<& [crate::model::PutRecordBatchResponseEntry]> {
        self.request_responses.as_deref()
    }
}
/// See [`PutRecordBatchOutput`](crate::output::PutRecordBatchOutput).
pub mod put_record_batch_output {
    
    /// A builder for [`PutRecordBatchOutput`](crate::output::PutRecordBatchOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) failed_put_count: std::option::Option<i32>,
        pub(crate) encrypted: std::option::Option<bool>,
        pub(crate) request_responses: std::option::Option<std::vec::Vec<crate::model::PutRecordBatchResponseEntry>>,
    }
    impl Builder {
        /// <p>The number of records that might have failed processing. This number might be greater than 0 even if the <code>PutRecordBatch</code> call succeeds. Check <code>FailedPutCount</code> to determine whether there are records that you need to resend.</p>
        pub fn failed_put_count(mut self, input: i32) -> Self {
            self.failed_put_count = Some(input);
            self
        }
        /// <p>The number of records that might have failed processing. This number might be greater than 0 even if the <code>PutRecordBatch</code> call succeeds. Check <code>FailedPutCount</code> to determine whether there are records that you need to resend.</p>
        pub fn set_failed_put_count(mut self, input: std::option::Option<i32>) -> Self {
            self.failed_put_count = input; self
        }
        /// <p>Indicates whether server-side encryption (SSE) was enabled during this operation.</p>
        pub fn encrypted(mut self, input: bool) -> Self {
            self.encrypted = Some(input);
            self
        }
        /// <p>Indicates whether server-side encryption (SSE) was enabled during this operation.</p>
        pub fn set_encrypted(mut self, input: std::option::Option<bool>) -> Self {
            self.encrypted = input; self
        }
        /// Appends an item to `request_responses`.
        ///
        /// To override the contents of this collection use [`set_request_responses`](Self::set_request_responses).
        ///
        /// <p>The results array. For each record, the index of the response element is the same as the index used in the request array.</p>
        pub fn request_responses(mut self, input: crate::model::PutRecordBatchResponseEntry) -> Self {
            let mut v = self.request_responses.unwrap_or_default();
                            v.push(input);
                            self.request_responses = Some(v);
                            self
        }
        /// <p>The results array. For each record, the index of the response element is the same as the index used in the request array.</p>
        pub fn set_request_responses(mut self, input: std::option::Option<std::vec::Vec<crate::model::PutRecordBatchResponseEntry>>) -> Self {
            self.request_responses = input; self
        }
        /// Consumes the builder and constructs a [`PutRecordBatchOutput`](crate::output::PutRecordBatchOutput).
        pub fn build(self) -> crate::output::PutRecordBatchOutput {
            crate::output::PutRecordBatchOutput {
                failed_put_count: self.failed_put_count
                ,
                encrypted: self.encrypted
                ,
                request_responses: self.request_responses
                ,
            }
        }
    }
    
    
}
impl PutRecordBatchOutput {
    /// Creates a new builder-style object to manufacture [`PutRecordBatchOutput`](crate::output::PutRecordBatchOutput).
    pub fn builder() -> crate::output::put_record_batch_output::Builder {
        crate::output::put_record_batch_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct PutRecordOutput  {
    /// <p>The ID of the record.</p>
    #[doc(hidden)]
    pub record_id: std::option::Option<std::string::String>,
    /// <p>Indicates whether server-side encryption (SSE) was enabled during this operation.</p>
    #[doc(hidden)]
    pub encrypted: std::option::Option<bool>,
}
impl PutRecordOutput {
    /// <p>The ID of the record.</p>
    pub fn record_id(&self) -> std::option::Option<& str> {
        self.record_id.as_deref()
    }
    /// <p>Indicates whether server-side encryption (SSE) was enabled during this operation.</p>
    pub fn encrypted(&self) -> std::option::Option<bool> {
        self.encrypted
    }
}
/// See [`PutRecordOutput`](crate::output::PutRecordOutput).
pub mod put_record_output {
    
    /// A builder for [`PutRecordOutput`](crate::output::PutRecordOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) record_id: std::option::Option<std::string::String>,
        pub(crate) encrypted: std::option::Option<bool>,
    }
    impl Builder {
        /// <p>The ID of the record.</p>
        pub fn record_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.record_id = Some(input.into());
            self
        }
        /// <p>The ID of the record.</p>
        pub fn set_record_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.record_id = input; self
        }
        /// <p>Indicates whether server-side encryption (SSE) was enabled during this operation.</p>
        pub fn encrypted(mut self, input: bool) -> Self {
            self.encrypted = Some(input);
            self
        }
        /// <p>Indicates whether server-side encryption (SSE) was enabled during this operation.</p>
        pub fn set_encrypted(mut self, input: std::option::Option<bool>) -> Self {
            self.encrypted = input; self
        }
        /// Consumes the builder and constructs a [`PutRecordOutput`](crate::output::PutRecordOutput).
        pub fn build(self) -> crate::output::PutRecordOutput {
            crate::output::PutRecordOutput {
                record_id: self.record_id
                ,
                encrypted: self.encrypted
                ,
            }
        }
    }
    
    
}
impl PutRecordOutput {
    /// Creates a new builder-style object to manufacture [`PutRecordOutput`](crate::output::PutRecordOutput).
    pub fn builder() -> crate::output::put_record_output::Builder {
        crate::output::put_record_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ListTagsForDeliveryStreamOutput  {
    /// <p>A list of tags associated with <code>DeliveryStreamName</code>, starting with the first tag after <code>ExclusiveStartTagKey</code> and up to the specified <code>Limit</code>.</p>
    #[doc(hidden)]
    pub tags: std::option::Option<std::vec::Vec<crate::model::Tag>>,
    /// <p>If this is <code>true</code> in the response, more tags are available. To list the remaining tags, set <code>ExclusiveStartTagKey</code> to the key of the last tag returned and call <code>ListTagsForDeliveryStream</code> again.</p>
    #[doc(hidden)]
    pub has_more_tags: std::option::Option<bool>,
}
impl ListTagsForDeliveryStreamOutput {
    /// <p>A list of tags associated with <code>DeliveryStreamName</code>, starting with the first tag after <code>ExclusiveStartTagKey</code> and up to the specified <code>Limit</code>.</p>
    pub fn tags(&self) -> std::option::Option<& [crate::model::Tag]> {
        self.tags.as_deref()
    }
    /// <p>If this is <code>true</code> in the response, more tags are available. To list the remaining tags, set <code>ExclusiveStartTagKey</code> to the key of the last tag returned and call <code>ListTagsForDeliveryStream</code> again.</p>
    pub fn has_more_tags(&self) -> std::option::Option<bool> {
        self.has_more_tags
    }
}
/// See [`ListTagsForDeliveryStreamOutput`](crate::output::ListTagsForDeliveryStreamOutput).
pub mod list_tags_for_delivery_stream_output {
    
    /// A builder for [`ListTagsForDeliveryStreamOutput`](crate::output::ListTagsForDeliveryStreamOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) tags: std::option::Option<std::vec::Vec<crate::model::Tag>>,
        pub(crate) has_more_tags: std::option::Option<bool>,
    }
    impl Builder {
        /// Appends an item to `tags`.
        ///
        /// To override the contents of this collection use [`set_tags`](Self::set_tags).
        ///
        /// <p>A list of tags associated with <code>DeliveryStreamName</code>, starting with the first tag after <code>ExclusiveStartTagKey</code> and up to the specified <code>Limit</code>.</p>
        pub fn tags(mut self, input: crate::model::Tag) -> Self {
            let mut v = self.tags.unwrap_or_default();
                            v.push(input);
                            self.tags = Some(v);
                            self
        }
        /// <p>A list of tags associated with <code>DeliveryStreamName</code>, starting with the first tag after <code>ExclusiveStartTagKey</code> and up to the specified <code>Limit</code>.</p>
        pub fn set_tags(mut self, input: std::option::Option<std::vec::Vec<crate::model::Tag>>) -> Self {
            self.tags = input; self
        }
        /// <p>If this is <code>true</code> in the response, more tags are available. To list the remaining tags, set <code>ExclusiveStartTagKey</code> to the key of the last tag returned and call <code>ListTagsForDeliveryStream</code> again.</p>
        pub fn has_more_tags(mut self, input: bool) -> Self {
            self.has_more_tags = Some(input);
            self
        }
        /// <p>If this is <code>true</code> in the response, more tags are available. To list the remaining tags, set <code>ExclusiveStartTagKey</code> to the key of the last tag returned and call <code>ListTagsForDeliveryStream</code> again.</p>
        pub fn set_has_more_tags(mut self, input: std::option::Option<bool>) -> Self {
            self.has_more_tags = input; self
        }
        /// Consumes the builder and constructs a [`ListTagsForDeliveryStreamOutput`](crate::output::ListTagsForDeliveryStreamOutput).
        pub fn build(self) -> crate::output::ListTagsForDeliveryStreamOutput {
            crate::output::ListTagsForDeliveryStreamOutput {
                tags: self.tags
                ,
                has_more_tags: self.has_more_tags
                ,
            }
        }
    }
    
    
}
impl ListTagsForDeliveryStreamOutput {
    /// Creates a new builder-style object to manufacture [`ListTagsForDeliveryStreamOutput`](crate::output::ListTagsForDeliveryStreamOutput).
    pub fn builder() -> crate::output::list_tags_for_delivery_stream_output::Builder {
        crate::output::list_tags_for_delivery_stream_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ListDeliveryStreamsOutput  {
    /// <p>The names of the delivery streams.</p>
    #[doc(hidden)]
    pub delivery_stream_names: std::option::Option<std::vec::Vec<std::string::String>>,
    /// <p>Indicates whether there are more delivery streams available to list.</p>
    #[doc(hidden)]
    pub has_more_delivery_streams: std::option::Option<bool>,
}
impl ListDeliveryStreamsOutput {
    /// <p>The names of the delivery streams.</p>
    pub fn delivery_stream_names(&self) -> std::option::Option<& [std::string::String]> {
        self.delivery_stream_names.as_deref()
    }
    /// <p>Indicates whether there are more delivery streams available to list.</p>
    pub fn has_more_delivery_streams(&self) -> std::option::Option<bool> {
        self.has_more_delivery_streams
    }
}
/// See [`ListDeliveryStreamsOutput`](crate::output::ListDeliveryStreamsOutput).
pub mod list_delivery_streams_output {
    
    /// A builder for [`ListDeliveryStreamsOutput`](crate::output::ListDeliveryStreamsOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) delivery_stream_names: std::option::Option<std::vec::Vec<std::string::String>>,
        pub(crate) has_more_delivery_streams: std::option::Option<bool>,
    }
    impl Builder {
        /// Appends an item to `delivery_stream_names`.
        ///
        /// To override the contents of this collection use [`set_delivery_stream_names`](Self::set_delivery_stream_names).
        ///
        /// <p>The names of the delivery streams.</p>
        pub fn delivery_stream_names(mut self, input: impl Into<std::string::String>) -> Self {
            let mut v = self.delivery_stream_names.unwrap_or_default();
                            v.push(input.into());
                            self.delivery_stream_names = Some(v);
                            self
        }
        /// <p>The names of the delivery streams.</p>
        pub fn set_delivery_stream_names(mut self, input: std::option::Option<std::vec::Vec<std::string::String>>) -> Self {
            self.delivery_stream_names = input; self
        }
        /// <p>Indicates whether there are more delivery streams available to list.</p>
        pub fn has_more_delivery_streams(mut self, input: bool) -> Self {
            self.has_more_delivery_streams = Some(input);
            self
        }
        /// <p>Indicates whether there are more delivery streams available to list.</p>
        pub fn set_has_more_delivery_streams(mut self, input: std::option::Option<bool>) -> Self {
            self.has_more_delivery_streams = input; self
        }
        /// Consumes the builder and constructs a [`ListDeliveryStreamsOutput`](crate::output::ListDeliveryStreamsOutput).
        pub fn build(self) -> crate::output::ListDeliveryStreamsOutput {
            crate::output::ListDeliveryStreamsOutput {
                delivery_stream_names: self.delivery_stream_names
                ,
                has_more_delivery_streams: self.has_more_delivery_streams
                ,
            }
        }
    }
    
    
}
impl ListDeliveryStreamsOutput {
    /// Creates a new builder-style object to manufacture [`ListDeliveryStreamsOutput`](crate::output::ListDeliveryStreamsOutput).
    pub fn builder() -> crate::output::list_delivery_streams_output::Builder {
        crate::output::list_delivery_streams_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct DescribeDeliveryStreamOutput  {
    /// <p>Information about the delivery stream.</p>
    #[doc(hidden)]
    pub delivery_stream_description: std::option::Option<crate::model::DeliveryStreamDescription>,
}
impl DescribeDeliveryStreamOutput {
    /// <p>Information about the delivery stream.</p>
    pub fn delivery_stream_description(&self) -> std::option::Option<& crate::model::DeliveryStreamDescription> {
        self.delivery_stream_description.as_ref()
    }
}
/// See [`DescribeDeliveryStreamOutput`](crate::output::DescribeDeliveryStreamOutput).
pub mod describe_delivery_stream_output {
    
    /// A builder for [`DescribeDeliveryStreamOutput`](crate::output::DescribeDeliveryStreamOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) delivery_stream_description: std::option::Option<crate::model::DeliveryStreamDescription>,
    }
    impl Builder {
        /// <p>Information about the delivery stream.</p>
        pub fn delivery_stream_description(mut self, input: crate::model::DeliveryStreamDescription) -> Self {
            self.delivery_stream_description = Some(input);
            self
        }
        /// <p>Information about the delivery stream.</p>
        pub fn set_delivery_stream_description(mut self, input: std::option::Option<crate::model::DeliveryStreamDescription>) -> Self {
            self.delivery_stream_description = input; self
        }
        /// Consumes the builder and constructs a [`DescribeDeliveryStreamOutput`](crate::output::DescribeDeliveryStreamOutput).
        pub fn build(self) -> crate::output::DescribeDeliveryStreamOutput {
            crate::output::DescribeDeliveryStreamOutput {
                delivery_stream_description: self.delivery_stream_description
                ,
            }
        }
    }
    
    
}
impl DescribeDeliveryStreamOutput {
    /// Creates a new builder-style object to manufacture [`DescribeDeliveryStreamOutput`](crate::output::DescribeDeliveryStreamOutput).
    pub fn builder() -> crate::output::describe_delivery_stream_output::Builder {
        crate::output::describe_delivery_stream_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct DeleteDeliveryStreamOutput  {
}
/// See [`DeleteDeliveryStreamOutput`](crate::output::DeleteDeliveryStreamOutput).
pub mod delete_delivery_stream_output {
    
    /// A builder for [`DeleteDeliveryStreamOutput`](crate::output::DeleteDeliveryStreamOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
    }
    impl Builder {
        /// Consumes the builder and constructs a [`DeleteDeliveryStreamOutput`](crate::output::DeleteDeliveryStreamOutput).
        pub fn build(self) -> crate::output::DeleteDeliveryStreamOutput {
            crate::output::DeleteDeliveryStreamOutput {
            }
        }
    }
    
    
}
impl DeleteDeliveryStreamOutput {
    /// Creates a new builder-style object to manufacture [`DeleteDeliveryStreamOutput`](crate::output::DeleteDeliveryStreamOutput).
    pub fn builder() -> crate::output::delete_delivery_stream_output::Builder {
        crate::output::delete_delivery_stream_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct CreateDeliveryStreamOutput  {
    /// <p>The ARN of the delivery stream.</p>
    #[doc(hidden)]
    pub delivery_stream_arn: std::option::Option<std::string::String>,
}
impl CreateDeliveryStreamOutput {
    /// <p>The ARN of the delivery stream.</p>
    pub fn delivery_stream_arn(&self) -> std::option::Option<& str> {
        self.delivery_stream_arn.as_deref()
    }
}
/// See [`CreateDeliveryStreamOutput`](crate::output::CreateDeliveryStreamOutput).
pub mod create_delivery_stream_output {
    
    /// A builder for [`CreateDeliveryStreamOutput`](crate::output::CreateDeliveryStreamOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) delivery_stream_arn: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The ARN of the delivery stream.</p>
        pub fn delivery_stream_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.delivery_stream_arn = Some(input.into());
            self
        }
        /// <p>The ARN of the delivery stream.</p>
        pub fn set_delivery_stream_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.delivery_stream_arn = input; self
        }
        /// Consumes the builder and constructs a [`CreateDeliveryStreamOutput`](crate::output::CreateDeliveryStreamOutput).
        pub fn build(self) -> crate::output::CreateDeliveryStreamOutput {
            crate::output::CreateDeliveryStreamOutput {
                delivery_stream_arn: self.delivery_stream_arn
                ,
            }
        }
    }
    
    
}
impl CreateDeliveryStreamOutput {
    /// Creates a new builder-style object to manufacture [`CreateDeliveryStreamOutput`](crate::output::CreateDeliveryStreamOutput).
    pub fn builder() -> crate::output::create_delivery_stream_output::Builder {
        crate::output::create_delivery_stream_output::Builder::default()
    }
}


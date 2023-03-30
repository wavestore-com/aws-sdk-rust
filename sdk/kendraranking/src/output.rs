// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct UpdateRescoreExecutionPlanOutput  {
}
/// See [`UpdateRescoreExecutionPlanOutput`](crate::output::UpdateRescoreExecutionPlanOutput).
pub mod update_rescore_execution_plan_output {
    
    /// A builder for [`UpdateRescoreExecutionPlanOutput`](crate::output::UpdateRescoreExecutionPlanOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
    }
    impl Builder {
        /// Consumes the builder and constructs a [`UpdateRescoreExecutionPlanOutput`](crate::output::UpdateRescoreExecutionPlanOutput).
        pub fn build(self) -> crate::output::UpdateRescoreExecutionPlanOutput {
            crate::output::UpdateRescoreExecutionPlanOutput {
            }
        }
    }
    
    
}
impl UpdateRescoreExecutionPlanOutput {
    /// Creates a new builder-style object to manufacture [`UpdateRescoreExecutionPlanOutput`](crate::output::UpdateRescoreExecutionPlanOutput).
    pub fn builder() -> crate::output::update_rescore_execution_plan_output::Builder {
        crate::output::update_rescore_execution_plan_output::Builder::default()
    }
}

/// <p>If the action is successful, the service sends back an HTTP 200 response with an empty HTTP body.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct UntagResourceOutput  {
}
/// See [`UntagResourceOutput`](crate::output::UntagResourceOutput).
pub mod untag_resource_output {
    
    /// A builder for [`UntagResourceOutput`](crate::output::UntagResourceOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
    }
    impl Builder {
        /// Consumes the builder and constructs a [`UntagResourceOutput`](crate::output::UntagResourceOutput).
        pub fn build(self) -> crate::output::UntagResourceOutput {
            crate::output::UntagResourceOutput {
            }
        }
    }
    
    
}
impl UntagResourceOutput {
    /// Creates a new builder-style object to manufacture [`UntagResourceOutput`](crate::output::UntagResourceOutput).
    pub fn builder() -> crate::output::untag_resource_output::Builder {
        crate::output::untag_resource_output::Builder::default()
    }
}

/// <p>If the action is successful, the service sends back an HTTP 200 response with an empty HTTP body.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct TagResourceOutput  {
}
/// See [`TagResourceOutput`](crate::output::TagResourceOutput).
pub mod tag_resource_output {
    
    /// A builder for [`TagResourceOutput`](crate::output::TagResourceOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
    }
    impl Builder {
        /// Consumes the builder and constructs a [`TagResourceOutput`](crate::output::TagResourceOutput).
        pub fn build(self) -> crate::output::TagResourceOutput {
            crate::output::TagResourceOutput {
            }
        }
    }
    
    
}
impl TagResourceOutput {
    /// Creates a new builder-style object to manufacture [`TagResourceOutput`](crate::output::TagResourceOutput).
    pub fn builder() -> crate::output::tag_resource_output::Builder {
        crate::output::tag_resource_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct RescoreOutput  {
    /// <p>The identifier associated with the scores that Amazon Kendra Intelligent Ranking gives to the results. Amazon Kendra Intelligent Ranking rescores or re-ranks the results for the search service.</p>
    #[doc(hidden)]
    pub rescore_id: std::option::Option<std::string::String>,
    /// <p>A list of result items for documents with new relevancy scores. The results are in descending order.</p>
    #[doc(hidden)]
    pub result_items: std::option::Option<std::vec::Vec<crate::model::RescoreResultItem>>,
}
impl RescoreOutput {
    /// <p>The identifier associated with the scores that Amazon Kendra Intelligent Ranking gives to the results. Amazon Kendra Intelligent Ranking rescores or re-ranks the results for the search service.</p>
    pub fn rescore_id(&self) -> std::option::Option<& str> {
        self.rescore_id.as_deref()
    }
    /// <p>A list of result items for documents with new relevancy scores. The results are in descending order.</p>
    pub fn result_items(&self) -> std::option::Option<& [crate::model::RescoreResultItem]> {
        self.result_items.as_deref()
    }
}
/// See [`RescoreOutput`](crate::output::RescoreOutput).
pub mod rescore_output {
    
    /// A builder for [`RescoreOutput`](crate::output::RescoreOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) rescore_id: std::option::Option<std::string::String>,
        pub(crate) result_items: std::option::Option<std::vec::Vec<crate::model::RescoreResultItem>>,
    }
    impl Builder {
        /// <p>The identifier associated with the scores that Amazon Kendra Intelligent Ranking gives to the results. Amazon Kendra Intelligent Ranking rescores or re-ranks the results for the search service.</p>
        pub fn rescore_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.rescore_id = Some(input.into());
            self
        }
        /// <p>The identifier associated with the scores that Amazon Kendra Intelligent Ranking gives to the results. Amazon Kendra Intelligent Ranking rescores or re-ranks the results for the search service.</p>
        pub fn set_rescore_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.rescore_id = input; self
        }
        /// Appends an item to `result_items`.
        ///
        /// To override the contents of this collection use [`set_result_items`](Self::set_result_items).
        ///
        /// <p>A list of result items for documents with new relevancy scores. The results are in descending order.</p>
        pub fn result_items(mut self, input: crate::model::RescoreResultItem) -> Self {
            let mut v = self.result_items.unwrap_or_default();
                            v.push(input);
                            self.result_items = Some(v);
                            self
        }
        /// <p>A list of result items for documents with new relevancy scores. The results are in descending order.</p>
        pub fn set_result_items(mut self, input: std::option::Option<std::vec::Vec<crate::model::RescoreResultItem>>) -> Self {
            self.result_items = input; self
        }
        /// Consumes the builder and constructs a [`RescoreOutput`](crate::output::RescoreOutput).
        pub fn build(self) -> crate::output::RescoreOutput {
            crate::output::RescoreOutput {
                rescore_id: self.rescore_id
                ,
                result_items: self.result_items
                ,
            }
        }
    }
    
    
}
impl RescoreOutput {
    /// Creates a new builder-style object to manufacture [`RescoreOutput`](crate::output::RescoreOutput).
    pub fn builder() -> crate::output::rescore_output::Builder {
        crate::output::rescore_output::Builder::default()
    }
}

/// <p>If the action is successful, the service sends back an HTTP 200 response.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ListTagsForResourceOutput  {
    /// <p>A list of tags associated with the rescore execution plan.</p>
    #[doc(hidden)]
    pub tags: std::option::Option<std::vec::Vec<crate::model::Tag>>,
}
impl ListTagsForResourceOutput {
    /// <p>A list of tags associated with the rescore execution plan.</p>
    pub fn tags(&self) -> std::option::Option<& [crate::model::Tag]> {
        self.tags.as_deref()
    }
}
/// See [`ListTagsForResourceOutput`](crate::output::ListTagsForResourceOutput).
pub mod list_tags_for_resource_output {
    
    /// A builder for [`ListTagsForResourceOutput`](crate::output::ListTagsForResourceOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) tags: std::option::Option<std::vec::Vec<crate::model::Tag>>,
    }
    impl Builder {
        /// Appends an item to `tags`.
        ///
        /// To override the contents of this collection use [`set_tags`](Self::set_tags).
        ///
        /// <p>A list of tags associated with the rescore execution plan.</p>
        pub fn tags(mut self, input: crate::model::Tag) -> Self {
            let mut v = self.tags.unwrap_or_default();
                            v.push(input);
                            self.tags = Some(v);
                            self
        }
        /// <p>A list of tags associated with the rescore execution plan.</p>
        pub fn set_tags(mut self, input: std::option::Option<std::vec::Vec<crate::model::Tag>>) -> Self {
            self.tags = input; self
        }
        /// Consumes the builder and constructs a [`ListTagsForResourceOutput`](crate::output::ListTagsForResourceOutput).
        pub fn build(self) -> crate::output::ListTagsForResourceOutput {
            crate::output::ListTagsForResourceOutput {
                tags: self.tags
                ,
            }
        }
    }
    
    
}
impl ListTagsForResourceOutput {
    /// Creates a new builder-style object to manufacture [`ListTagsForResourceOutput`](crate::output::ListTagsForResourceOutput).
    pub fn builder() -> crate::output::list_tags_for_resource_output::Builder {
        crate::output::list_tags_for_resource_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ListRescoreExecutionPlansOutput  {
    /// <p>An array of summary information for one or more rescore execution plans.</p>
    #[doc(hidden)]
    pub summary_items: std::option::Option<std::vec::Vec<crate::model::RescoreExecutionPlanSummary>>,
    /// <p>If the response is truncated, Amazon Kendra Intelligent Ranking returns a pagination token in the response.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
}
impl ListRescoreExecutionPlansOutput {
    /// <p>An array of summary information for one or more rescore execution plans.</p>
    pub fn summary_items(&self) -> std::option::Option<& [crate::model::RescoreExecutionPlanSummary]> {
        self.summary_items.as_deref()
    }
    /// <p>If the response is truncated, Amazon Kendra Intelligent Ranking returns a pagination token in the response.</p>
    pub fn next_token(&self) -> std::option::Option<& str> {
        self.next_token.as_deref()
    }
}
/// See [`ListRescoreExecutionPlansOutput`](crate::output::ListRescoreExecutionPlansOutput).
pub mod list_rescore_execution_plans_output {
    
    /// A builder for [`ListRescoreExecutionPlansOutput`](crate::output::ListRescoreExecutionPlansOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) summary_items: std::option::Option<std::vec::Vec<crate::model::RescoreExecutionPlanSummary>>,
        pub(crate) next_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// Appends an item to `summary_items`.
        ///
        /// To override the contents of this collection use [`set_summary_items`](Self::set_summary_items).
        ///
        /// <p>An array of summary information for one or more rescore execution plans.</p>
        pub fn summary_items(mut self, input: crate::model::RescoreExecutionPlanSummary) -> Self {
            let mut v = self.summary_items.unwrap_or_default();
                            v.push(input);
                            self.summary_items = Some(v);
                            self
        }
        /// <p>An array of summary information for one or more rescore execution plans.</p>
        pub fn set_summary_items(mut self, input: std::option::Option<std::vec::Vec<crate::model::RescoreExecutionPlanSummary>>) -> Self {
            self.summary_items = input; self
        }
        /// <p>If the response is truncated, Amazon Kendra Intelligent Ranking returns a pagination token in the response.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>If the response is truncated, Amazon Kendra Intelligent Ranking returns a pagination token in the response.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input; self
        }
        /// Consumes the builder and constructs a [`ListRescoreExecutionPlansOutput`](crate::output::ListRescoreExecutionPlansOutput).
        pub fn build(self) -> crate::output::ListRescoreExecutionPlansOutput {
            crate::output::ListRescoreExecutionPlansOutput {
                summary_items: self.summary_items
                ,
                next_token: self.next_token
                ,
            }
        }
    }
    
    
}
impl ListRescoreExecutionPlansOutput {
    /// Creates a new builder-style object to manufacture [`ListRescoreExecutionPlansOutput`](crate::output::ListRescoreExecutionPlansOutput).
    pub fn builder() -> crate::output::list_rescore_execution_plans_output::Builder {
        crate::output::list_rescore_execution_plans_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct DescribeRescoreExecutionPlanOutput  {
    /// <p>The identifier of the rescore execution plan.</p>
    #[doc(hidden)]
    pub id: std::option::Option<std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of the rescore execution plan.</p>
    #[doc(hidden)]
    pub arn: std::option::Option<std::string::String>,
    /// <p>The name for the rescore execution plan.</p>
    #[doc(hidden)]
    pub name: std::option::Option<std::string::String>,
    /// <p>The description for the rescore execution plan.</p>
    #[doc(hidden)]
    pub description: std::option::Option<std::string::String>,
    /// <p>The capacity units set for the rescore execution plan. A capacity of zero indicates that the rescore execution plan is using the default capacity. For more information on the default capacity and additional capacity units, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/adjusting-capacity.html">Adjusting capacity</a>.</p>
    #[doc(hidden)]
    pub capacity_units: std::option::Option<crate::model::CapacityUnitsConfiguration>,
    /// <p>The Unix timestamp of when the rescore execution plan was created.</p>
    #[doc(hidden)]
    pub created_at: std::option::Option<aws_smithy_types::DateTime>,
    /// <p>The Unix timestamp of when the rescore execution plan was last updated.</p>
    #[doc(hidden)]
    pub updated_at: std::option::Option<aws_smithy_types::DateTime>,
    /// <p>The current status of the rescore execution plan. When the value is <code>ACTIVE</code>, the rescore execution plan is ready for use. If the <code>Status</code> field value is <code>FAILED</code>, the <code>ErrorMessage</code> field contains a message that explains why.</p>
    #[doc(hidden)]
    pub status: std::option::Option<crate::model::RescoreExecutionPlanStatus>,
    /// <p>When the <code>Status</code> field value is <code>FAILED</code>, the <code>ErrorMessage</code> field contains a message that explains why.</p>
    #[doc(hidden)]
    pub error_message: std::option::Option<std::string::String>,
}
impl DescribeRescoreExecutionPlanOutput {
    /// <p>The identifier of the rescore execution plan.</p>
    pub fn id(&self) -> std::option::Option<& str> {
        self.id.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of the rescore execution plan.</p>
    pub fn arn(&self) -> std::option::Option<& str> {
        self.arn.as_deref()
    }
    /// <p>The name for the rescore execution plan.</p>
    pub fn name(&self) -> std::option::Option<& str> {
        self.name.as_deref()
    }
    /// <p>The description for the rescore execution plan.</p>
    pub fn description(&self) -> std::option::Option<& str> {
        self.description.as_deref()
    }
    /// <p>The capacity units set for the rescore execution plan. A capacity of zero indicates that the rescore execution plan is using the default capacity. For more information on the default capacity and additional capacity units, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/adjusting-capacity.html">Adjusting capacity</a>.</p>
    pub fn capacity_units(&self) -> std::option::Option<& crate::model::CapacityUnitsConfiguration> {
        self.capacity_units.as_ref()
    }
    /// <p>The Unix timestamp of when the rescore execution plan was created.</p>
    pub fn created_at(&self) -> std::option::Option<& aws_smithy_types::DateTime> {
        self.created_at.as_ref()
    }
    /// <p>The Unix timestamp of when the rescore execution plan was last updated.</p>
    pub fn updated_at(&self) -> std::option::Option<& aws_smithy_types::DateTime> {
        self.updated_at.as_ref()
    }
    /// <p>The current status of the rescore execution plan. When the value is <code>ACTIVE</code>, the rescore execution plan is ready for use. If the <code>Status</code> field value is <code>FAILED</code>, the <code>ErrorMessage</code> field contains a message that explains why.</p>
    pub fn status(&self) -> std::option::Option<& crate::model::RescoreExecutionPlanStatus> {
        self.status.as_ref()
    }
    /// <p>When the <code>Status</code> field value is <code>FAILED</code>, the <code>ErrorMessage</code> field contains a message that explains why.</p>
    pub fn error_message(&self) -> std::option::Option<& str> {
        self.error_message.as_deref()
    }
}
/// See [`DescribeRescoreExecutionPlanOutput`](crate::output::DescribeRescoreExecutionPlanOutput).
pub mod describe_rescore_execution_plan_output {
    
    /// A builder for [`DescribeRescoreExecutionPlanOutput`](crate::output::DescribeRescoreExecutionPlanOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) id: std::option::Option<std::string::String>,
        pub(crate) arn: std::option::Option<std::string::String>,
        pub(crate) name: std::option::Option<std::string::String>,
        pub(crate) description: std::option::Option<std::string::String>,
        pub(crate) capacity_units: std::option::Option<crate::model::CapacityUnitsConfiguration>,
        pub(crate) created_at: std::option::Option<aws_smithy_types::DateTime>,
        pub(crate) updated_at: std::option::Option<aws_smithy_types::DateTime>,
        pub(crate) status: std::option::Option<crate::model::RescoreExecutionPlanStatus>,
        pub(crate) error_message: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The identifier of the rescore execution plan.</p>
        pub fn id(mut self, input: impl Into<std::string::String>) -> Self {
            self.id = Some(input.into());
            self
        }
        /// <p>The identifier of the rescore execution plan.</p>
        pub fn set_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.id = input; self
        }
        /// <p>The Amazon Resource Name (ARN) of the rescore execution plan.</p>
        pub fn arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.arn = Some(input.into());
            self
        }
        /// <p>The Amazon Resource Name (ARN) of the rescore execution plan.</p>
        pub fn set_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.arn = input; self
        }
        /// <p>The name for the rescore execution plan.</p>
        pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
            self.name = Some(input.into());
            self
        }
        /// <p>The name for the rescore execution plan.</p>
        pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.name = input; self
        }
        /// <p>The description for the rescore execution plan.</p>
        pub fn description(mut self, input: impl Into<std::string::String>) -> Self {
            self.description = Some(input.into());
            self
        }
        /// <p>The description for the rescore execution plan.</p>
        pub fn set_description(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.description = input; self
        }
        /// <p>The capacity units set for the rescore execution plan. A capacity of zero indicates that the rescore execution plan is using the default capacity. For more information on the default capacity and additional capacity units, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/adjusting-capacity.html">Adjusting capacity</a>.</p>
        pub fn capacity_units(mut self, input: crate::model::CapacityUnitsConfiguration) -> Self {
            self.capacity_units = Some(input);
            self
        }
        /// <p>The capacity units set for the rescore execution plan. A capacity of zero indicates that the rescore execution plan is using the default capacity. For more information on the default capacity and additional capacity units, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/adjusting-capacity.html">Adjusting capacity</a>.</p>
        pub fn set_capacity_units(mut self, input: std::option::Option<crate::model::CapacityUnitsConfiguration>) -> Self {
            self.capacity_units = input; self
        }
        /// <p>The Unix timestamp of when the rescore execution plan was created.</p>
        pub fn created_at(mut self, input: aws_smithy_types::DateTime) -> Self {
            self.created_at = Some(input);
            self
        }
        /// <p>The Unix timestamp of when the rescore execution plan was created.</p>
        pub fn set_created_at(mut self, input: std::option::Option<aws_smithy_types::DateTime>) -> Self {
            self.created_at = input; self
        }
        /// <p>The Unix timestamp of when the rescore execution plan was last updated.</p>
        pub fn updated_at(mut self, input: aws_smithy_types::DateTime) -> Self {
            self.updated_at = Some(input);
            self
        }
        /// <p>The Unix timestamp of when the rescore execution plan was last updated.</p>
        pub fn set_updated_at(mut self, input: std::option::Option<aws_smithy_types::DateTime>) -> Self {
            self.updated_at = input; self
        }
        /// <p>The current status of the rescore execution plan. When the value is <code>ACTIVE</code>, the rescore execution plan is ready for use. If the <code>Status</code> field value is <code>FAILED</code>, the <code>ErrorMessage</code> field contains a message that explains why.</p>
        pub fn status(mut self, input: crate::model::RescoreExecutionPlanStatus) -> Self {
            self.status = Some(input);
            self
        }
        /// <p>The current status of the rescore execution plan. When the value is <code>ACTIVE</code>, the rescore execution plan is ready for use. If the <code>Status</code> field value is <code>FAILED</code>, the <code>ErrorMessage</code> field contains a message that explains why.</p>
        pub fn set_status(mut self, input: std::option::Option<crate::model::RescoreExecutionPlanStatus>) -> Self {
            self.status = input; self
        }
        /// <p>When the <code>Status</code> field value is <code>FAILED</code>, the <code>ErrorMessage</code> field contains a message that explains why.</p>
        pub fn error_message(mut self, input: impl Into<std::string::String>) -> Self {
            self.error_message = Some(input.into());
            self
        }
        /// <p>When the <code>Status</code> field value is <code>FAILED</code>, the <code>ErrorMessage</code> field contains a message that explains why.</p>
        pub fn set_error_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.error_message = input; self
        }
        /// Consumes the builder and constructs a [`DescribeRescoreExecutionPlanOutput`](crate::output::DescribeRescoreExecutionPlanOutput).
        pub fn build(self) -> crate::output::DescribeRescoreExecutionPlanOutput {
            crate::output::DescribeRescoreExecutionPlanOutput {
                id: self.id
                ,
                arn: self.arn
                ,
                name: self.name
                ,
                description: self.description
                ,
                capacity_units: self.capacity_units
                ,
                created_at: self.created_at
                ,
                updated_at: self.updated_at
                ,
                status: self.status
                ,
                error_message: self.error_message
                ,
            }
        }
    }
    
    
}
impl DescribeRescoreExecutionPlanOutput {
    /// Creates a new builder-style object to manufacture [`DescribeRescoreExecutionPlanOutput`](crate::output::DescribeRescoreExecutionPlanOutput).
    pub fn builder() -> crate::output::describe_rescore_execution_plan_output::Builder {
        crate::output::describe_rescore_execution_plan_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct DeleteRescoreExecutionPlanOutput  {
}
/// See [`DeleteRescoreExecutionPlanOutput`](crate::output::DeleteRescoreExecutionPlanOutput).
pub mod delete_rescore_execution_plan_output {
    
    /// A builder for [`DeleteRescoreExecutionPlanOutput`](crate::output::DeleteRescoreExecutionPlanOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
    }
    impl Builder {
        /// Consumes the builder and constructs a [`DeleteRescoreExecutionPlanOutput`](crate::output::DeleteRescoreExecutionPlanOutput).
        pub fn build(self) -> crate::output::DeleteRescoreExecutionPlanOutput {
            crate::output::DeleteRescoreExecutionPlanOutput {
            }
        }
    }
    
    
}
impl DeleteRescoreExecutionPlanOutput {
    /// Creates a new builder-style object to manufacture [`DeleteRescoreExecutionPlanOutput`](crate::output::DeleteRescoreExecutionPlanOutput).
    pub fn builder() -> crate::output::delete_rescore_execution_plan_output::Builder {
        crate::output::delete_rescore_execution_plan_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct CreateRescoreExecutionPlanOutput  {
    /// <p>The identifier of the rescore execution plan.</p>
    #[doc(hidden)]
    pub id: std::option::Option<std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of the rescore execution plan.</p>
    #[doc(hidden)]
    pub arn: std::option::Option<std::string::String>,
}
impl CreateRescoreExecutionPlanOutput {
    /// <p>The identifier of the rescore execution plan.</p>
    pub fn id(&self) -> std::option::Option<& str> {
        self.id.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of the rescore execution plan.</p>
    pub fn arn(&self) -> std::option::Option<& str> {
        self.arn.as_deref()
    }
}
/// See [`CreateRescoreExecutionPlanOutput`](crate::output::CreateRescoreExecutionPlanOutput).
pub mod create_rescore_execution_plan_output {
    
    /// A builder for [`CreateRescoreExecutionPlanOutput`](crate::output::CreateRescoreExecutionPlanOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) id: std::option::Option<std::string::String>,
        pub(crate) arn: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The identifier of the rescore execution plan.</p>
        pub fn id(mut self, input: impl Into<std::string::String>) -> Self {
            self.id = Some(input.into());
            self
        }
        /// <p>The identifier of the rescore execution plan.</p>
        pub fn set_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.id = input; self
        }
        /// <p>The Amazon Resource Name (ARN) of the rescore execution plan.</p>
        pub fn arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.arn = Some(input.into());
            self
        }
        /// <p>The Amazon Resource Name (ARN) of the rescore execution plan.</p>
        pub fn set_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.arn = input; self
        }
        /// Consumes the builder and constructs a [`CreateRescoreExecutionPlanOutput`](crate::output::CreateRescoreExecutionPlanOutput).
        pub fn build(self) -> crate::output::CreateRescoreExecutionPlanOutput {
            crate::output::CreateRescoreExecutionPlanOutput {
                id: self.id
                ,
                arn: self.arn
                ,
            }
        }
    }
    
    
}
impl CreateRescoreExecutionPlanOutput {
    /// Creates a new builder-style object to manufacture [`CreateRescoreExecutionPlanOutput`](crate::output::CreateRescoreExecutionPlanOutput).
    pub fn builder() -> crate::output::create_rescore_execution_plan_output::Builder {
        crate::output::create_rescore_execution_plan_output::Builder::default()
    }
}


// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct UpdateEnvironmentMembershipOutput  {
    /// <p>Information about the environment member whose settings were changed.</p>
    #[doc(hidden)]
    pub membership: std::option::Option<crate::model::EnvironmentMember>,
}
impl UpdateEnvironmentMembershipOutput {
    /// <p>Information about the environment member whose settings were changed.</p>
    pub fn membership(&self) -> std::option::Option<& crate::model::EnvironmentMember> {
        self.membership.as_ref()
    }
}
/// See [`UpdateEnvironmentMembershipOutput`](crate::output::UpdateEnvironmentMembershipOutput).
pub mod update_environment_membership_output {
    
    /// A builder for [`UpdateEnvironmentMembershipOutput`](crate::output::UpdateEnvironmentMembershipOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) membership: std::option::Option<crate::model::EnvironmentMember>,
    }
    impl Builder {
        /// <p>Information about the environment member whose settings were changed.</p>
        pub fn membership(mut self, input: crate::model::EnvironmentMember) -> Self {
            self.membership = Some(input);
            self
        }
        /// <p>Information about the environment member whose settings were changed.</p>
        pub fn set_membership(mut self, input: std::option::Option<crate::model::EnvironmentMember>) -> Self {
            self.membership = input; self
        }
        /// Consumes the builder and constructs a [`UpdateEnvironmentMembershipOutput`](crate::output::UpdateEnvironmentMembershipOutput).
        pub fn build(self) -> crate::output::UpdateEnvironmentMembershipOutput {
            crate::output::UpdateEnvironmentMembershipOutput {
                membership: self.membership
                ,
            }
        }
    }
    
    
}
impl UpdateEnvironmentMembershipOutput {
    /// Creates a new builder-style object to manufacture [`UpdateEnvironmentMembershipOutput`](crate::output::UpdateEnvironmentMembershipOutput).
    pub fn builder() -> crate::output::update_environment_membership_output::Builder {
        crate::output::update_environment_membership_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct UpdateEnvironmentOutput  {
}
/// See [`UpdateEnvironmentOutput`](crate::output::UpdateEnvironmentOutput).
pub mod update_environment_output {
    
    /// A builder for [`UpdateEnvironmentOutput`](crate::output::UpdateEnvironmentOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
    }
    impl Builder {
        /// Consumes the builder and constructs a [`UpdateEnvironmentOutput`](crate::output::UpdateEnvironmentOutput).
        pub fn build(self) -> crate::output::UpdateEnvironmentOutput {
            crate::output::UpdateEnvironmentOutput {
            }
        }
    }
    
    
}
impl UpdateEnvironmentOutput {
    /// Creates a new builder-style object to manufacture [`UpdateEnvironmentOutput`](crate::output::UpdateEnvironmentOutput).
    pub fn builder() -> crate::output::update_environment_output::Builder {
        crate::output::update_environment_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
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

#[allow(missing_docs)] // documentation missing in model
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
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ListTagsForResourceOutput  {
    /// <p>The list of tags associated with the Cloud9 development environment.</p>
    #[doc(hidden)]
    pub tags: std::option::Option<std::vec::Vec<crate::model::Tag>>,
}
impl ListTagsForResourceOutput {
    /// <p>The list of tags associated with the Cloud9 development environment.</p>
    pub fn tags(&self) -> std::option::Option<& [crate::model::Tag]> {
        self.tags.as_deref()
    }
}
impl  std::fmt::Debug for ListTagsForResourceOutput  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ListTagsForResourceOutput");
        formatter.field("tags", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
/// See [`ListTagsForResourceOutput`](crate::output::ListTagsForResourceOutput).
pub mod list_tags_for_resource_output {
    
    /// A builder for [`ListTagsForResourceOutput`](crate::output::ListTagsForResourceOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default)]
    pub struct Builder {
        pub(crate) tags: std::option::Option<std::vec::Vec<crate::model::Tag>>,
    }
    impl Builder {
        /// Appends an item to `tags`.
        ///
        /// To override the contents of this collection use [`set_tags`](Self::set_tags).
        ///
        /// <p>The list of tags associated with the Cloud9 development environment.</p>
        pub fn tags(mut self, input: crate::model::Tag) -> Self {
            let mut v = self.tags.unwrap_or_default();
                            v.push(input);
                            self.tags = Some(v);
                            self
        }
        /// <p>The list of tags associated with the Cloud9 development environment.</p>
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
    impl std::fmt::Debug for Builder {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut formatter = f.debug_struct("Builder");
            formatter.field("tags", &"*** Sensitive Data Redacted ***");
            formatter.finish()
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
pub struct ListEnvironmentsOutput  {
    /// <p>If there are more than 25 items in the list, only the first 25 items are returned, along with a unique string called a <i>next token</i>. To get the next batch of items in the list, call this operation again, adding the next token to the call.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
    /// <p>The list of environment identifiers.</p>
    #[doc(hidden)]
    pub environment_ids: std::option::Option<std::vec::Vec<std::string::String>>,
}
impl ListEnvironmentsOutput {
    /// <p>If there are more than 25 items in the list, only the first 25 items are returned, along with a unique string called a <i>next token</i>. To get the next batch of items in the list, call this operation again, adding the next token to the call.</p>
    pub fn next_token(&self) -> std::option::Option<& str> {
        self.next_token.as_deref()
    }
    /// <p>The list of environment identifiers.</p>
    pub fn environment_ids(&self) -> std::option::Option<& [std::string::String]> {
        self.environment_ids.as_deref()
    }
}
/// See [`ListEnvironmentsOutput`](crate::output::ListEnvironmentsOutput).
pub mod list_environments_output {
    
    /// A builder for [`ListEnvironmentsOutput`](crate::output::ListEnvironmentsOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) next_token: std::option::Option<std::string::String>,
        pub(crate) environment_ids: std::option::Option<std::vec::Vec<std::string::String>>,
    }
    impl Builder {
        /// <p>If there are more than 25 items in the list, only the first 25 items are returned, along with a unique string called a <i>next token</i>. To get the next batch of items in the list, call this operation again, adding the next token to the call.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>If there are more than 25 items in the list, only the first 25 items are returned, along with a unique string called a <i>next token</i>. To get the next batch of items in the list, call this operation again, adding the next token to the call.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input; self
        }
        /// Appends an item to `environment_ids`.
        ///
        /// To override the contents of this collection use [`set_environment_ids`](Self::set_environment_ids).
        ///
        /// <p>The list of environment identifiers.</p>
        pub fn environment_ids(mut self, input: impl Into<std::string::String>) -> Self {
            let mut v = self.environment_ids.unwrap_or_default();
                            v.push(input.into());
                            self.environment_ids = Some(v);
                            self
        }
        /// <p>The list of environment identifiers.</p>
        pub fn set_environment_ids(mut self, input: std::option::Option<std::vec::Vec<std::string::String>>) -> Self {
            self.environment_ids = input; self
        }
        /// Consumes the builder and constructs a [`ListEnvironmentsOutput`](crate::output::ListEnvironmentsOutput).
        pub fn build(self) -> crate::output::ListEnvironmentsOutput {
            crate::output::ListEnvironmentsOutput {
                next_token: self.next_token
                ,
                environment_ids: self.environment_ids
                ,
            }
        }
    }
    
    
}
impl ListEnvironmentsOutput {
    /// Creates a new builder-style object to manufacture [`ListEnvironmentsOutput`](crate::output::ListEnvironmentsOutput).
    pub fn builder() -> crate::output::list_environments_output::Builder {
        crate::output::list_environments_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct DescribeEnvironmentStatusOutput  {
    /// <p>The status of the environment. Available values include:</p> 
    /// <ul> 
    /// <li> <p> <code>connecting</code>: The environment is connecting.</p> </li> 
    /// <li> <p> <code>creating</code>: The environment is being created.</p> </li> 
    /// <li> <p> <code>deleting</code>: The environment is being deleted.</p> </li> 
    /// <li> <p> <code>error</code>: The environment is in an error state.</p> </li> 
    /// <li> <p> <code>ready</code>: The environment is ready.</p> </li> 
    /// <li> <p> <code>stopped</code>: The environment is stopped.</p> </li> 
    /// <li> <p> <code>stopping</code>: The environment is stopping.</p> </li> 
    /// </ul>
    #[doc(hidden)]
    pub status: std::option::Option<crate::model::EnvironmentStatus>,
    /// <p>Any informational message about the status of the environment.</p>
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
}
impl DescribeEnvironmentStatusOutput {
    /// <p>The status of the environment. Available values include:</p> 
    /// <ul> 
    /// <li> <p> <code>connecting</code>: The environment is connecting.</p> </li> 
    /// <li> <p> <code>creating</code>: The environment is being created.</p> </li> 
    /// <li> <p> <code>deleting</code>: The environment is being deleted.</p> </li> 
    /// <li> <p> <code>error</code>: The environment is in an error state.</p> </li> 
    /// <li> <p> <code>ready</code>: The environment is ready.</p> </li> 
    /// <li> <p> <code>stopped</code>: The environment is stopped.</p> </li> 
    /// <li> <p> <code>stopping</code>: The environment is stopping.</p> </li> 
    /// </ul>
    pub fn status(&self) -> std::option::Option<& crate::model::EnvironmentStatus> {
        self.status.as_ref()
    }
    /// <p>Any informational message about the status of the environment.</p>
    pub fn message(&self) -> std::option::Option<& str> {
        self.message.as_deref()
    }
}
/// See [`DescribeEnvironmentStatusOutput`](crate::output::DescribeEnvironmentStatusOutput).
pub mod describe_environment_status_output {
    
    /// A builder for [`DescribeEnvironmentStatusOutput`](crate::output::DescribeEnvironmentStatusOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) status: std::option::Option<crate::model::EnvironmentStatus>,
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The status of the environment. Available values include:</p> 
        /// <ul> 
        /// <li> <p> <code>connecting</code>: The environment is connecting.</p> </li> 
        /// <li> <p> <code>creating</code>: The environment is being created.</p> </li> 
        /// <li> <p> <code>deleting</code>: The environment is being deleted.</p> </li> 
        /// <li> <p> <code>error</code>: The environment is in an error state.</p> </li> 
        /// <li> <p> <code>ready</code>: The environment is ready.</p> </li> 
        /// <li> <p> <code>stopped</code>: The environment is stopped.</p> </li> 
        /// <li> <p> <code>stopping</code>: The environment is stopping.</p> </li> 
        /// </ul>
        pub fn status(mut self, input: crate::model::EnvironmentStatus) -> Self {
            self.status = Some(input);
            self
        }
        /// <p>The status of the environment. Available values include:</p> 
        /// <ul> 
        /// <li> <p> <code>connecting</code>: The environment is connecting.</p> </li> 
        /// <li> <p> <code>creating</code>: The environment is being created.</p> </li> 
        /// <li> <p> <code>deleting</code>: The environment is being deleted.</p> </li> 
        /// <li> <p> <code>error</code>: The environment is in an error state.</p> </li> 
        /// <li> <p> <code>ready</code>: The environment is ready.</p> </li> 
        /// <li> <p> <code>stopped</code>: The environment is stopped.</p> </li> 
        /// <li> <p> <code>stopping</code>: The environment is stopping.</p> </li> 
        /// </ul>
        pub fn set_status(mut self, input: std::option::Option<crate::model::EnvironmentStatus>) -> Self {
            self.status = input; self
        }
        /// <p>Any informational message about the status of the environment.</p>
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        /// <p>Any informational message about the status of the environment.</p>
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input; self
        }
        /// Consumes the builder and constructs a [`DescribeEnvironmentStatusOutput`](crate::output::DescribeEnvironmentStatusOutput).
        pub fn build(self) -> crate::output::DescribeEnvironmentStatusOutput {
            crate::output::DescribeEnvironmentStatusOutput {
                status: self.status
                ,
                message: self.message
                ,
            }
        }
    }
    
    
}
impl DescribeEnvironmentStatusOutput {
    /// Creates a new builder-style object to manufacture [`DescribeEnvironmentStatusOutput`](crate::output::DescribeEnvironmentStatusOutput).
    pub fn builder() -> crate::output::describe_environment_status_output::Builder {
        crate::output::describe_environment_status_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct DescribeEnvironmentsOutput  {
    /// <p>Information about the environments that are returned.</p>
    #[doc(hidden)]
    pub environments: std::option::Option<std::vec::Vec<crate::model::Environment>>,
}
impl DescribeEnvironmentsOutput {
    /// <p>Information about the environments that are returned.</p>
    pub fn environments(&self) -> std::option::Option<& [crate::model::Environment]> {
        self.environments.as_deref()
    }
}
/// See [`DescribeEnvironmentsOutput`](crate::output::DescribeEnvironmentsOutput).
pub mod describe_environments_output {
    
    /// A builder for [`DescribeEnvironmentsOutput`](crate::output::DescribeEnvironmentsOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) environments: std::option::Option<std::vec::Vec<crate::model::Environment>>,
    }
    impl Builder {
        /// Appends an item to `environments`.
        ///
        /// To override the contents of this collection use [`set_environments`](Self::set_environments).
        ///
        /// <p>Information about the environments that are returned.</p>
        pub fn environments(mut self, input: crate::model::Environment) -> Self {
            let mut v = self.environments.unwrap_or_default();
                            v.push(input);
                            self.environments = Some(v);
                            self
        }
        /// <p>Information about the environments that are returned.</p>
        pub fn set_environments(mut self, input: std::option::Option<std::vec::Vec<crate::model::Environment>>) -> Self {
            self.environments = input; self
        }
        /// Consumes the builder and constructs a [`DescribeEnvironmentsOutput`](crate::output::DescribeEnvironmentsOutput).
        pub fn build(self) -> crate::output::DescribeEnvironmentsOutput {
            crate::output::DescribeEnvironmentsOutput {
                environments: self.environments
                ,
            }
        }
    }
    
    
}
impl DescribeEnvironmentsOutput {
    /// Creates a new builder-style object to manufacture [`DescribeEnvironmentsOutput`](crate::output::DescribeEnvironmentsOutput).
    pub fn builder() -> crate::output::describe_environments_output::Builder {
        crate::output::describe_environments_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct DescribeEnvironmentMembershipsOutput  {
    /// <p>Information about the environment members for the environment.</p>
    #[doc(hidden)]
    pub memberships: std::option::Option<std::vec::Vec<crate::model::EnvironmentMember>>,
    /// <p>If there are more than 25 items in the list, only the first 25 items are returned, along with a unique string called a <i>next token</i>. To get the next batch of items in the list, call this operation again, adding the next token to the call.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
}
impl DescribeEnvironmentMembershipsOutput {
    /// <p>Information about the environment members for the environment.</p>
    pub fn memberships(&self) -> std::option::Option<& [crate::model::EnvironmentMember]> {
        self.memberships.as_deref()
    }
    /// <p>If there are more than 25 items in the list, only the first 25 items are returned, along with a unique string called a <i>next token</i>. To get the next batch of items in the list, call this operation again, adding the next token to the call.</p>
    pub fn next_token(&self) -> std::option::Option<& str> {
        self.next_token.as_deref()
    }
}
/// See [`DescribeEnvironmentMembershipsOutput`](crate::output::DescribeEnvironmentMembershipsOutput).
pub mod describe_environment_memberships_output {
    
    /// A builder for [`DescribeEnvironmentMembershipsOutput`](crate::output::DescribeEnvironmentMembershipsOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) memberships: std::option::Option<std::vec::Vec<crate::model::EnvironmentMember>>,
        pub(crate) next_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// Appends an item to `memberships`.
        ///
        /// To override the contents of this collection use [`set_memberships`](Self::set_memberships).
        ///
        /// <p>Information about the environment members for the environment.</p>
        pub fn memberships(mut self, input: crate::model::EnvironmentMember) -> Self {
            let mut v = self.memberships.unwrap_or_default();
                            v.push(input);
                            self.memberships = Some(v);
                            self
        }
        /// <p>Information about the environment members for the environment.</p>
        pub fn set_memberships(mut self, input: std::option::Option<std::vec::Vec<crate::model::EnvironmentMember>>) -> Self {
            self.memberships = input; self
        }
        /// <p>If there are more than 25 items in the list, only the first 25 items are returned, along with a unique string called a <i>next token</i>. To get the next batch of items in the list, call this operation again, adding the next token to the call.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>If there are more than 25 items in the list, only the first 25 items are returned, along with a unique string called a <i>next token</i>. To get the next batch of items in the list, call this operation again, adding the next token to the call.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input; self
        }
        /// Consumes the builder and constructs a [`DescribeEnvironmentMembershipsOutput`](crate::output::DescribeEnvironmentMembershipsOutput).
        pub fn build(self) -> crate::output::DescribeEnvironmentMembershipsOutput {
            crate::output::DescribeEnvironmentMembershipsOutput {
                memberships: self.memberships
                ,
                next_token: self.next_token
                ,
            }
        }
    }
    
    
}
impl DescribeEnvironmentMembershipsOutput {
    /// Creates a new builder-style object to manufacture [`DescribeEnvironmentMembershipsOutput`](crate::output::DescribeEnvironmentMembershipsOutput).
    pub fn builder() -> crate::output::describe_environment_memberships_output::Builder {
        crate::output::describe_environment_memberships_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct DeleteEnvironmentMembershipOutput  {
}
/// See [`DeleteEnvironmentMembershipOutput`](crate::output::DeleteEnvironmentMembershipOutput).
pub mod delete_environment_membership_output {
    
    /// A builder for [`DeleteEnvironmentMembershipOutput`](crate::output::DeleteEnvironmentMembershipOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
    }
    impl Builder {
        /// Consumes the builder and constructs a [`DeleteEnvironmentMembershipOutput`](crate::output::DeleteEnvironmentMembershipOutput).
        pub fn build(self) -> crate::output::DeleteEnvironmentMembershipOutput {
            crate::output::DeleteEnvironmentMembershipOutput {
            }
        }
    }
    
    
}
impl DeleteEnvironmentMembershipOutput {
    /// Creates a new builder-style object to manufacture [`DeleteEnvironmentMembershipOutput`](crate::output::DeleteEnvironmentMembershipOutput).
    pub fn builder() -> crate::output::delete_environment_membership_output::Builder {
        crate::output::delete_environment_membership_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct DeleteEnvironmentOutput  {
}
/// See [`DeleteEnvironmentOutput`](crate::output::DeleteEnvironmentOutput).
pub mod delete_environment_output {
    
    /// A builder for [`DeleteEnvironmentOutput`](crate::output::DeleteEnvironmentOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
    }
    impl Builder {
        /// Consumes the builder and constructs a [`DeleteEnvironmentOutput`](crate::output::DeleteEnvironmentOutput).
        pub fn build(self) -> crate::output::DeleteEnvironmentOutput {
            crate::output::DeleteEnvironmentOutput {
            }
        }
    }
    
    
}
impl DeleteEnvironmentOutput {
    /// Creates a new builder-style object to manufacture [`DeleteEnvironmentOutput`](crate::output::DeleteEnvironmentOutput).
    pub fn builder() -> crate::output::delete_environment_output::Builder {
        crate::output::delete_environment_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct CreateEnvironmentMembershipOutput  {
    /// <p>Information about the environment member that was added.</p>
    #[doc(hidden)]
    pub membership: std::option::Option<crate::model::EnvironmentMember>,
}
impl CreateEnvironmentMembershipOutput {
    /// <p>Information about the environment member that was added.</p>
    pub fn membership(&self) -> std::option::Option<& crate::model::EnvironmentMember> {
        self.membership.as_ref()
    }
}
/// See [`CreateEnvironmentMembershipOutput`](crate::output::CreateEnvironmentMembershipOutput).
pub mod create_environment_membership_output {
    
    /// A builder for [`CreateEnvironmentMembershipOutput`](crate::output::CreateEnvironmentMembershipOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) membership: std::option::Option<crate::model::EnvironmentMember>,
    }
    impl Builder {
        /// <p>Information about the environment member that was added.</p>
        pub fn membership(mut self, input: crate::model::EnvironmentMember) -> Self {
            self.membership = Some(input);
            self
        }
        /// <p>Information about the environment member that was added.</p>
        pub fn set_membership(mut self, input: std::option::Option<crate::model::EnvironmentMember>) -> Self {
            self.membership = input; self
        }
        /// Consumes the builder and constructs a [`CreateEnvironmentMembershipOutput`](crate::output::CreateEnvironmentMembershipOutput).
        pub fn build(self) -> crate::output::CreateEnvironmentMembershipOutput {
            crate::output::CreateEnvironmentMembershipOutput {
                membership: self.membership
                ,
            }
        }
    }
    
    
}
impl CreateEnvironmentMembershipOutput {
    /// Creates a new builder-style object to manufacture [`CreateEnvironmentMembershipOutput`](crate::output::CreateEnvironmentMembershipOutput).
    pub fn builder() -> crate::output::create_environment_membership_output::Builder {
        crate::output::create_environment_membership_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct CreateEnvironmentEc2Output  {
    /// <p>The ID of the environment that was created.</p>
    #[doc(hidden)]
    pub environment_id: std::option::Option<std::string::String>,
}
impl CreateEnvironmentEc2Output {
    /// <p>The ID of the environment that was created.</p>
    pub fn environment_id(&self) -> std::option::Option<& str> {
        self.environment_id.as_deref()
    }
}
/// See [`CreateEnvironmentEc2Output`](crate::output::CreateEnvironmentEc2Output).
pub mod create_environment_ec2_output {
    
    /// A builder for [`CreateEnvironmentEc2Output`](crate::output::CreateEnvironmentEc2Output).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) environment_id: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The ID of the environment that was created.</p>
        pub fn environment_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.environment_id = Some(input.into());
            self
        }
        /// <p>The ID of the environment that was created.</p>
        pub fn set_environment_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.environment_id = input; self
        }
        /// Consumes the builder and constructs a [`CreateEnvironmentEc2Output`](crate::output::CreateEnvironmentEc2Output).
        pub fn build(self) -> crate::output::CreateEnvironmentEc2Output {
            crate::output::CreateEnvironmentEc2Output {
                environment_id: self.environment_id
                ,
            }
        }
    }
    
    
}
impl CreateEnvironmentEc2Output {
    /// Creates a new builder-style object to manufacture [`CreateEnvironmentEc2Output`](crate::output::CreateEnvironmentEc2Output).
    pub fn builder() -> crate::output::create_environment_ec2_output::Builder {
        crate::output::create_environment_ec2_output::Builder::default()
    }
}


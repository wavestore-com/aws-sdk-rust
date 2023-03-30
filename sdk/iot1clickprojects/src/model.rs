// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>An object defining the template for a placement.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct PlacementTemplate  {
    /// <p>The default attributes (key/value pairs) to be applied to all placements using this template.</p>
    #[doc(hidden)]
    pub default_attributes: std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
    /// <p>An object specifying the <code>DeviceTemplate</code> for all placements using this (<code>PlacementTemplate</code>) template.</p>
    #[doc(hidden)]
    pub device_templates: std::option::Option<std::collections::HashMap<std::string::String, crate::model::DeviceTemplate>>,
}
impl PlacementTemplate {
    /// <p>The default attributes (key/value pairs) to be applied to all placements using this template.</p>
    pub fn default_attributes(&self) -> std::option::Option<& std::collections::HashMap<std::string::String, std::string::String>> {
        self.default_attributes.as_ref()
    }
    /// <p>An object specifying the <code>DeviceTemplate</code> for all placements using this (<code>PlacementTemplate</code>) template.</p>
    pub fn device_templates(&self) -> std::option::Option<& std::collections::HashMap<std::string::String, crate::model::DeviceTemplate>> {
        self.device_templates.as_ref()
    }
}
/// See [`PlacementTemplate`](crate::model::PlacementTemplate).
pub mod placement_template {
    
    /// A builder for [`PlacementTemplate`](crate::model::PlacementTemplate).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) default_attributes: std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
        pub(crate) device_templates: std::option::Option<std::collections::HashMap<std::string::String, crate::model::DeviceTemplate>>,
    }
    impl Builder {
        /// Adds a key-value pair to `default_attributes`.
        ///
        /// To override the contents of this collection use [`set_default_attributes`](Self::set_default_attributes).
        ///
        /// <p>The default attributes (key/value pairs) to be applied to all placements using this template.</p>
        pub fn default_attributes(mut self, k: impl Into<std::string::String>, v: impl Into<std::string::String>) -> Self {
            let mut hash_map = self.default_attributes.unwrap_or_default();
                            hash_map.insert(k.into(), v.into());
                            self.default_attributes = Some(hash_map);
                            self
        }
        /// <p>The default attributes (key/value pairs) to be applied to all placements using this template.</p>
        pub fn set_default_attributes(mut self, input: std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>) -> Self {
            self.default_attributes = input; self
        }
        /// Adds a key-value pair to `device_templates`.
        ///
        /// To override the contents of this collection use [`set_device_templates`](Self::set_device_templates).
        ///
        /// <p>An object specifying the <code>DeviceTemplate</code> for all placements using this (<code>PlacementTemplate</code>) template.</p>
        pub fn device_templates(mut self, k: impl Into<std::string::String>, v: crate::model::DeviceTemplate) -> Self {
            let mut hash_map = self.device_templates.unwrap_or_default();
                            hash_map.insert(k.into(), v);
                            self.device_templates = Some(hash_map);
                            self
        }
        /// <p>An object specifying the <code>DeviceTemplate</code> for all placements using this (<code>PlacementTemplate</code>) template.</p>
        pub fn set_device_templates(mut self, input: std::option::Option<std::collections::HashMap<std::string::String, crate::model::DeviceTemplate>>) -> Self {
            self.device_templates = input; self
        }
        /// Consumes the builder and constructs a [`PlacementTemplate`](crate::model::PlacementTemplate).
        pub fn build(self) -> crate::model::PlacementTemplate {
            crate::model::PlacementTemplate {
                default_attributes: self.default_attributes
                ,
                device_templates: self.device_templates
                ,
            }
        }
    }
    
    
}
impl PlacementTemplate {
    /// Creates a new builder-style object to manufacture [`PlacementTemplate`](crate::model::PlacementTemplate).
    pub fn builder() -> crate::model::placement_template::Builder {
        crate::model::placement_template::Builder::default()
    }
}

/// <p>An object representing a device for a placement template (see <code>PlacementTemplate</code>).</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct DeviceTemplate  {
    /// <p>The device type, which currently must be <code>"button"</code>.</p>
    #[doc(hidden)]
    pub device_type: std::option::Option<std::string::String>,
    /// <p>An optional Lambda function to invoke instead of the default Lambda function provided by the placement template.</p>
    #[doc(hidden)]
    pub callback_overrides: std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
}
impl DeviceTemplate {
    /// <p>The device type, which currently must be <code>"button"</code>.</p>
    pub fn device_type(&self) -> std::option::Option<& str> {
        self.device_type.as_deref()
    }
    /// <p>An optional Lambda function to invoke instead of the default Lambda function provided by the placement template.</p>
    pub fn callback_overrides(&self) -> std::option::Option<& std::collections::HashMap<std::string::String, std::string::String>> {
        self.callback_overrides.as_ref()
    }
}
/// See [`DeviceTemplate`](crate::model::DeviceTemplate).
pub mod device_template {
    
    /// A builder for [`DeviceTemplate`](crate::model::DeviceTemplate).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) device_type: std::option::Option<std::string::String>,
        pub(crate) callback_overrides: std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
    }
    impl Builder {
        /// <p>The device type, which currently must be <code>"button"</code>.</p>
        pub fn device_type(mut self, input: impl Into<std::string::String>) -> Self {
            self.device_type = Some(input.into());
            self
        }
        /// <p>The device type, which currently must be <code>"button"</code>.</p>
        pub fn set_device_type(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.device_type = input; self
        }
        /// Adds a key-value pair to `callback_overrides`.
        ///
        /// To override the contents of this collection use [`set_callback_overrides`](Self::set_callback_overrides).
        ///
        /// <p>An optional Lambda function to invoke instead of the default Lambda function provided by the placement template.</p>
        pub fn callback_overrides(mut self, k: impl Into<std::string::String>, v: impl Into<std::string::String>) -> Self {
            let mut hash_map = self.callback_overrides.unwrap_or_default();
                            hash_map.insert(k.into(), v.into());
                            self.callback_overrides = Some(hash_map);
                            self
        }
        /// <p>An optional Lambda function to invoke instead of the default Lambda function provided by the placement template.</p>
        pub fn set_callback_overrides(mut self, input: std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>) -> Self {
            self.callback_overrides = input; self
        }
        /// Consumes the builder and constructs a [`DeviceTemplate`](crate::model::DeviceTemplate).
        pub fn build(self) -> crate::model::DeviceTemplate {
            crate::model::DeviceTemplate {
                device_type: self.device_type
                ,
                callback_overrides: self.callback_overrides
                ,
            }
        }
    }
    
    
}
impl DeviceTemplate {
    /// Creates a new builder-style object to manufacture [`DeviceTemplate`](crate::model::DeviceTemplate).
    pub fn builder() -> crate::model::device_template::Builder {
        crate::model::device_template::Builder::default()
    }
}

/// <p>An object providing summary information for a particular project for an associated AWS account and region.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ProjectSummary  {
    /// <p>The ARN of the project.</p>
    #[doc(hidden)]
    pub arn: std::option::Option<std::string::String>,
    /// <p>The name of the project being summarized.</p>
    #[doc(hidden)]
    pub project_name: std::option::Option<std::string::String>,
    /// <p>The date when the project was originally created, in UNIX epoch time format.</p>
    #[doc(hidden)]
    pub created_date: std::option::Option<aws_smithy_types::DateTime>,
    /// <p>The date when the project was last updated, in UNIX epoch time format. If the project was not updated, then <code>createdDate</code> and <code>updatedDate</code> are the same.</p>
    #[doc(hidden)]
    pub updated_date: std::option::Option<aws_smithy_types::DateTime>,
    /// <p>The tags (metadata key/value pairs) associated with the project.</p>
    #[doc(hidden)]
    pub tags: std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
}
impl ProjectSummary {
    /// <p>The ARN of the project.</p>
    pub fn arn(&self) -> std::option::Option<& str> {
        self.arn.as_deref()
    }
    /// <p>The name of the project being summarized.</p>
    pub fn project_name(&self) -> std::option::Option<& str> {
        self.project_name.as_deref()
    }
    /// <p>The date when the project was originally created, in UNIX epoch time format.</p>
    pub fn created_date(&self) -> std::option::Option<& aws_smithy_types::DateTime> {
        self.created_date.as_ref()
    }
    /// <p>The date when the project was last updated, in UNIX epoch time format. If the project was not updated, then <code>createdDate</code> and <code>updatedDate</code> are the same.</p>
    pub fn updated_date(&self) -> std::option::Option<& aws_smithy_types::DateTime> {
        self.updated_date.as_ref()
    }
    /// <p>The tags (metadata key/value pairs) associated with the project.</p>
    pub fn tags(&self) -> std::option::Option<& std::collections::HashMap<std::string::String, std::string::String>> {
        self.tags.as_ref()
    }
}
/// See [`ProjectSummary`](crate::model::ProjectSummary).
pub mod project_summary {
    
    /// A builder for [`ProjectSummary`](crate::model::ProjectSummary).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) arn: std::option::Option<std::string::String>,
        pub(crate) project_name: std::option::Option<std::string::String>,
        pub(crate) created_date: std::option::Option<aws_smithy_types::DateTime>,
        pub(crate) updated_date: std::option::Option<aws_smithy_types::DateTime>,
        pub(crate) tags: std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
    }
    impl Builder {
        /// <p>The ARN of the project.</p>
        pub fn arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.arn = Some(input.into());
            self
        }
        /// <p>The ARN of the project.</p>
        pub fn set_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.arn = input; self
        }
        /// <p>The name of the project being summarized.</p>
        pub fn project_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.project_name = Some(input.into());
            self
        }
        /// <p>The name of the project being summarized.</p>
        pub fn set_project_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.project_name = input; self
        }
        /// <p>The date when the project was originally created, in UNIX epoch time format.</p>
        pub fn created_date(mut self, input: aws_smithy_types::DateTime) -> Self {
            self.created_date = Some(input);
            self
        }
        /// <p>The date when the project was originally created, in UNIX epoch time format.</p>
        pub fn set_created_date(mut self, input: std::option::Option<aws_smithy_types::DateTime>) -> Self {
            self.created_date = input; self
        }
        /// <p>The date when the project was last updated, in UNIX epoch time format. If the project was not updated, then <code>createdDate</code> and <code>updatedDate</code> are the same.</p>
        pub fn updated_date(mut self, input: aws_smithy_types::DateTime) -> Self {
            self.updated_date = Some(input);
            self
        }
        /// <p>The date when the project was last updated, in UNIX epoch time format. If the project was not updated, then <code>createdDate</code> and <code>updatedDate</code> are the same.</p>
        pub fn set_updated_date(mut self, input: std::option::Option<aws_smithy_types::DateTime>) -> Self {
            self.updated_date = input; self
        }
        /// Adds a key-value pair to `tags`.
        ///
        /// To override the contents of this collection use [`set_tags`](Self::set_tags).
        ///
        /// <p>The tags (metadata key/value pairs) associated with the project.</p>
        pub fn tags(mut self, k: impl Into<std::string::String>, v: impl Into<std::string::String>) -> Self {
            let mut hash_map = self.tags.unwrap_or_default();
                            hash_map.insert(k.into(), v.into());
                            self.tags = Some(hash_map);
                            self
        }
        /// <p>The tags (metadata key/value pairs) associated with the project.</p>
        pub fn set_tags(mut self, input: std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>) -> Self {
            self.tags = input; self
        }
        /// Consumes the builder and constructs a [`ProjectSummary`](crate::model::ProjectSummary).
        pub fn build(self) -> crate::model::ProjectSummary {
            crate::model::ProjectSummary {
                arn: self.arn
                ,
                project_name: self.project_name
                ,
                created_date: self.created_date
                ,
                updated_date: self.updated_date
                ,
                tags: self.tags
                ,
            }
        }
    }
    
    
}
impl ProjectSummary {
    /// Creates a new builder-style object to manufacture [`ProjectSummary`](crate::model::ProjectSummary).
    pub fn builder() -> crate::model::project_summary::Builder {
        crate::model::project_summary::Builder::default()
    }
}

/// <p>An object providing summary information for a particular placement.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct PlacementSummary  {
    /// <p>The name of the project containing the placement.</p>
    #[doc(hidden)]
    pub project_name: std::option::Option<std::string::String>,
    /// <p>The name of the placement being summarized.</p>
    #[doc(hidden)]
    pub placement_name: std::option::Option<std::string::String>,
    /// <p>The date when the placement was originally created, in UNIX epoch time format.</p>
    #[doc(hidden)]
    pub created_date: std::option::Option<aws_smithy_types::DateTime>,
    /// <p>The date when the placement was last updated, in UNIX epoch time format. If the placement was not updated, then <code>createdDate</code> and <code>updatedDate</code> are the same.</p>
    #[doc(hidden)]
    pub updated_date: std::option::Option<aws_smithy_types::DateTime>,
}
impl PlacementSummary {
    /// <p>The name of the project containing the placement.</p>
    pub fn project_name(&self) -> std::option::Option<& str> {
        self.project_name.as_deref()
    }
    /// <p>The name of the placement being summarized.</p>
    pub fn placement_name(&self) -> std::option::Option<& str> {
        self.placement_name.as_deref()
    }
    /// <p>The date when the placement was originally created, in UNIX epoch time format.</p>
    pub fn created_date(&self) -> std::option::Option<& aws_smithy_types::DateTime> {
        self.created_date.as_ref()
    }
    /// <p>The date when the placement was last updated, in UNIX epoch time format. If the placement was not updated, then <code>createdDate</code> and <code>updatedDate</code> are the same.</p>
    pub fn updated_date(&self) -> std::option::Option<& aws_smithy_types::DateTime> {
        self.updated_date.as_ref()
    }
}
/// See [`PlacementSummary`](crate::model::PlacementSummary).
pub mod placement_summary {
    
    /// A builder for [`PlacementSummary`](crate::model::PlacementSummary).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) project_name: std::option::Option<std::string::String>,
        pub(crate) placement_name: std::option::Option<std::string::String>,
        pub(crate) created_date: std::option::Option<aws_smithy_types::DateTime>,
        pub(crate) updated_date: std::option::Option<aws_smithy_types::DateTime>,
    }
    impl Builder {
        /// <p>The name of the project containing the placement.</p>
        pub fn project_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.project_name = Some(input.into());
            self
        }
        /// <p>The name of the project containing the placement.</p>
        pub fn set_project_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.project_name = input; self
        }
        /// <p>The name of the placement being summarized.</p>
        pub fn placement_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.placement_name = Some(input.into());
            self
        }
        /// <p>The name of the placement being summarized.</p>
        pub fn set_placement_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.placement_name = input; self
        }
        /// <p>The date when the placement was originally created, in UNIX epoch time format.</p>
        pub fn created_date(mut self, input: aws_smithy_types::DateTime) -> Self {
            self.created_date = Some(input);
            self
        }
        /// <p>The date when the placement was originally created, in UNIX epoch time format.</p>
        pub fn set_created_date(mut self, input: std::option::Option<aws_smithy_types::DateTime>) -> Self {
            self.created_date = input; self
        }
        /// <p>The date when the placement was last updated, in UNIX epoch time format. If the placement was not updated, then <code>createdDate</code> and <code>updatedDate</code> are the same.</p>
        pub fn updated_date(mut self, input: aws_smithy_types::DateTime) -> Self {
            self.updated_date = Some(input);
            self
        }
        /// <p>The date when the placement was last updated, in UNIX epoch time format. If the placement was not updated, then <code>createdDate</code> and <code>updatedDate</code> are the same.</p>
        pub fn set_updated_date(mut self, input: std::option::Option<aws_smithy_types::DateTime>) -> Self {
            self.updated_date = input; self
        }
        /// Consumes the builder and constructs a [`PlacementSummary`](crate::model::PlacementSummary).
        pub fn build(self) -> crate::model::PlacementSummary {
            crate::model::PlacementSummary {
                project_name: self.project_name
                ,
                placement_name: self.placement_name
                ,
                created_date: self.created_date
                ,
                updated_date: self.updated_date
                ,
            }
        }
    }
    
    
}
impl PlacementSummary {
    /// Creates a new builder-style object to manufacture [`PlacementSummary`](crate::model::PlacementSummary).
    pub fn builder() -> crate::model::placement_summary::Builder {
        crate::model::placement_summary::Builder::default()
    }
}

/// <p>An object providing detailed information for a particular project associated with an AWS account and region.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ProjectDescription  {
    /// <p>The ARN of the project.</p>
    #[doc(hidden)]
    pub arn: std::option::Option<std::string::String>,
    /// <p>The name of the project for which to obtain information from.</p>
    #[doc(hidden)]
    pub project_name: std::option::Option<std::string::String>,
    /// <p>The description of the project.</p>
    #[doc(hidden)]
    pub description: std::option::Option<std::string::String>,
    /// <p>The date when the project was originally created, in UNIX epoch time format.</p>
    #[doc(hidden)]
    pub created_date: std::option::Option<aws_smithy_types::DateTime>,
    /// <p>The date when the project was last updated, in UNIX epoch time format. If the project was not updated, then <code>createdDate</code> and <code>updatedDate</code> are the same.</p>
    #[doc(hidden)]
    pub updated_date: std::option::Option<aws_smithy_types::DateTime>,
    /// <p>An object describing the project's placement specifications.</p>
    #[doc(hidden)]
    pub placement_template: std::option::Option<crate::model::PlacementTemplate>,
    /// <p>The tags (metadata key/value pairs) associated with the project.</p>
    #[doc(hidden)]
    pub tags: std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
}
impl ProjectDescription {
    /// <p>The ARN of the project.</p>
    pub fn arn(&self) -> std::option::Option<& str> {
        self.arn.as_deref()
    }
    /// <p>The name of the project for which to obtain information from.</p>
    pub fn project_name(&self) -> std::option::Option<& str> {
        self.project_name.as_deref()
    }
    /// <p>The description of the project.</p>
    pub fn description(&self) -> std::option::Option<& str> {
        self.description.as_deref()
    }
    /// <p>The date when the project was originally created, in UNIX epoch time format.</p>
    pub fn created_date(&self) -> std::option::Option<& aws_smithy_types::DateTime> {
        self.created_date.as_ref()
    }
    /// <p>The date when the project was last updated, in UNIX epoch time format. If the project was not updated, then <code>createdDate</code> and <code>updatedDate</code> are the same.</p>
    pub fn updated_date(&self) -> std::option::Option<& aws_smithy_types::DateTime> {
        self.updated_date.as_ref()
    }
    /// <p>An object describing the project's placement specifications.</p>
    pub fn placement_template(&self) -> std::option::Option<& crate::model::PlacementTemplate> {
        self.placement_template.as_ref()
    }
    /// <p>The tags (metadata key/value pairs) associated with the project.</p>
    pub fn tags(&self) -> std::option::Option<& std::collections::HashMap<std::string::String, std::string::String>> {
        self.tags.as_ref()
    }
}
/// See [`ProjectDescription`](crate::model::ProjectDescription).
pub mod project_description {
    
    /// A builder for [`ProjectDescription`](crate::model::ProjectDescription).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) arn: std::option::Option<std::string::String>,
        pub(crate) project_name: std::option::Option<std::string::String>,
        pub(crate) description: std::option::Option<std::string::String>,
        pub(crate) created_date: std::option::Option<aws_smithy_types::DateTime>,
        pub(crate) updated_date: std::option::Option<aws_smithy_types::DateTime>,
        pub(crate) placement_template: std::option::Option<crate::model::PlacementTemplate>,
        pub(crate) tags: std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
    }
    impl Builder {
        /// <p>The ARN of the project.</p>
        pub fn arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.arn = Some(input.into());
            self
        }
        /// <p>The ARN of the project.</p>
        pub fn set_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.arn = input; self
        }
        /// <p>The name of the project for which to obtain information from.</p>
        pub fn project_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.project_name = Some(input.into());
            self
        }
        /// <p>The name of the project for which to obtain information from.</p>
        pub fn set_project_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.project_name = input; self
        }
        /// <p>The description of the project.</p>
        pub fn description(mut self, input: impl Into<std::string::String>) -> Self {
            self.description = Some(input.into());
            self
        }
        /// <p>The description of the project.</p>
        pub fn set_description(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.description = input; self
        }
        /// <p>The date when the project was originally created, in UNIX epoch time format.</p>
        pub fn created_date(mut self, input: aws_smithy_types::DateTime) -> Self {
            self.created_date = Some(input);
            self
        }
        /// <p>The date when the project was originally created, in UNIX epoch time format.</p>
        pub fn set_created_date(mut self, input: std::option::Option<aws_smithy_types::DateTime>) -> Self {
            self.created_date = input; self
        }
        /// <p>The date when the project was last updated, in UNIX epoch time format. If the project was not updated, then <code>createdDate</code> and <code>updatedDate</code> are the same.</p>
        pub fn updated_date(mut self, input: aws_smithy_types::DateTime) -> Self {
            self.updated_date = Some(input);
            self
        }
        /// <p>The date when the project was last updated, in UNIX epoch time format. If the project was not updated, then <code>createdDate</code> and <code>updatedDate</code> are the same.</p>
        pub fn set_updated_date(mut self, input: std::option::Option<aws_smithy_types::DateTime>) -> Self {
            self.updated_date = input; self
        }
        /// <p>An object describing the project's placement specifications.</p>
        pub fn placement_template(mut self, input: crate::model::PlacementTemplate) -> Self {
            self.placement_template = Some(input);
            self
        }
        /// <p>An object describing the project's placement specifications.</p>
        pub fn set_placement_template(mut self, input: std::option::Option<crate::model::PlacementTemplate>) -> Self {
            self.placement_template = input; self
        }
        /// Adds a key-value pair to `tags`.
        ///
        /// To override the contents of this collection use [`set_tags`](Self::set_tags).
        ///
        /// <p>The tags (metadata key/value pairs) associated with the project.</p>
        pub fn tags(mut self, k: impl Into<std::string::String>, v: impl Into<std::string::String>) -> Self {
            let mut hash_map = self.tags.unwrap_or_default();
                            hash_map.insert(k.into(), v.into());
                            self.tags = Some(hash_map);
                            self
        }
        /// <p>The tags (metadata key/value pairs) associated with the project.</p>
        pub fn set_tags(mut self, input: std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>) -> Self {
            self.tags = input; self
        }
        /// Consumes the builder and constructs a [`ProjectDescription`](crate::model::ProjectDescription).
        pub fn build(self) -> crate::model::ProjectDescription {
            crate::model::ProjectDescription {
                arn: self.arn
                ,
                project_name: self.project_name
                ,
                description: self.description
                ,
                created_date: self.created_date
                ,
                updated_date: self.updated_date
                ,
                placement_template: self.placement_template
                ,
                tags: self.tags
                ,
            }
        }
    }
    
    
}
impl ProjectDescription {
    /// Creates a new builder-style object to manufacture [`ProjectDescription`](crate::model::ProjectDescription).
    pub fn builder() -> crate::model::project_description::Builder {
        crate::model::project_description::Builder::default()
    }
}

/// <p>An object describing a project's placement.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct PlacementDescription  {
    /// <p>The name of the project containing the placement.</p>
    #[doc(hidden)]
    pub project_name: std::option::Option<std::string::String>,
    /// <p>The name of the placement.</p>
    #[doc(hidden)]
    pub placement_name: std::option::Option<std::string::String>,
    /// <p>The user-defined attributes associated with the placement.</p>
    #[doc(hidden)]
    pub attributes: std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
    /// <p>The date when the placement was initially created, in UNIX epoch time format.</p>
    #[doc(hidden)]
    pub created_date: std::option::Option<aws_smithy_types::DateTime>,
    /// <p>The date when the placement was last updated, in UNIX epoch time format. If the placement was not updated, then <code>createdDate</code> and <code>updatedDate</code> are the same.</p>
    #[doc(hidden)]
    pub updated_date: std::option::Option<aws_smithy_types::DateTime>,
}
impl PlacementDescription {
    /// <p>The name of the project containing the placement.</p>
    pub fn project_name(&self) -> std::option::Option<& str> {
        self.project_name.as_deref()
    }
    /// <p>The name of the placement.</p>
    pub fn placement_name(&self) -> std::option::Option<& str> {
        self.placement_name.as_deref()
    }
    /// <p>The user-defined attributes associated with the placement.</p>
    pub fn attributes(&self) -> std::option::Option<& std::collections::HashMap<std::string::String, std::string::String>> {
        self.attributes.as_ref()
    }
    /// <p>The date when the placement was initially created, in UNIX epoch time format.</p>
    pub fn created_date(&self) -> std::option::Option<& aws_smithy_types::DateTime> {
        self.created_date.as_ref()
    }
    /// <p>The date when the placement was last updated, in UNIX epoch time format. If the placement was not updated, then <code>createdDate</code> and <code>updatedDate</code> are the same.</p>
    pub fn updated_date(&self) -> std::option::Option<& aws_smithy_types::DateTime> {
        self.updated_date.as_ref()
    }
}
/// See [`PlacementDescription`](crate::model::PlacementDescription).
pub mod placement_description {
    
    /// A builder for [`PlacementDescription`](crate::model::PlacementDescription).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) project_name: std::option::Option<std::string::String>,
        pub(crate) placement_name: std::option::Option<std::string::String>,
        pub(crate) attributes: std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
        pub(crate) created_date: std::option::Option<aws_smithy_types::DateTime>,
        pub(crate) updated_date: std::option::Option<aws_smithy_types::DateTime>,
    }
    impl Builder {
        /// <p>The name of the project containing the placement.</p>
        pub fn project_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.project_name = Some(input.into());
            self
        }
        /// <p>The name of the project containing the placement.</p>
        pub fn set_project_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.project_name = input; self
        }
        /// <p>The name of the placement.</p>
        pub fn placement_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.placement_name = Some(input.into());
            self
        }
        /// <p>The name of the placement.</p>
        pub fn set_placement_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.placement_name = input; self
        }
        /// Adds a key-value pair to `attributes`.
        ///
        /// To override the contents of this collection use [`set_attributes`](Self::set_attributes).
        ///
        /// <p>The user-defined attributes associated with the placement.</p>
        pub fn attributes(mut self, k: impl Into<std::string::String>, v: impl Into<std::string::String>) -> Self {
            let mut hash_map = self.attributes.unwrap_or_default();
                            hash_map.insert(k.into(), v.into());
                            self.attributes = Some(hash_map);
                            self
        }
        /// <p>The user-defined attributes associated with the placement.</p>
        pub fn set_attributes(mut self, input: std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>) -> Self {
            self.attributes = input; self
        }
        /// <p>The date when the placement was initially created, in UNIX epoch time format.</p>
        pub fn created_date(mut self, input: aws_smithy_types::DateTime) -> Self {
            self.created_date = Some(input);
            self
        }
        /// <p>The date when the placement was initially created, in UNIX epoch time format.</p>
        pub fn set_created_date(mut self, input: std::option::Option<aws_smithy_types::DateTime>) -> Self {
            self.created_date = input; self
        }
        /// <p>The date when the placement was last updated, in UNIX epoch time format. If the placement was not updated, then <code>createdDate</code> and <code>updatedDate</code> are the same.</p>
        pub fn updated_date(mut self, input: aws_smithy_types::DateTime) -> Self {
            self.updated_date = Some(input);
            self
        }
        /// <p>The date when the placement was last updated, in UNIX epoch time format. If the placement was not updated, then <code>createdDate</code> and <code>updatedDate</code> are the same.</p>
        pub fn set_updated_date(mut self, input: std::option::Option<aws_smithy_types::DateTime>) -> Self {
            self.updated_date = input; self
        }
        /// Consumes the builder and constructs a [`PlacementDescription`](crate::model::PlacementDescription).
        pub fn build(self) -> crate::model::PlacementDescription {
            crate::model::PlacementDescription {
                project_name: self.project_name
                ,
                placement_name: self.placement_name
                ,
                attributes: self.attributes
                ,
                created_date: self.created_date
                ,
                updated_date: self.updated_date
                ,
            }
        }
    }
    
    
}
impl PlacementDescription {
    /// Creates a new builder-style object to manufacture [`PlacementDescription`](crate::model::PlacementDescription).
    pub fn builder() -> crate::model::placement_description::Builder {
        crate::model::placement_description::Builder::default()
    }
}


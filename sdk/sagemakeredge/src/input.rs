// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
use std::fmt::Write;
/// See [`GetDeviceRegistrationInput`](crate::input::GetDeviceRegistrationInput)
pub mod get_device_registration_input {
    /// A builder for [`GetDeviceRegistrationInput`](crate::input::GetDeviceRegistrationInput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) device_name: std::option::Option<std::string::String>,
        pub(crate) device_fleet_name: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The unique name of the device you want to get the registration status from.</p>
        pub fn device_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.device_name = Some(input.into());
            self
        }
        /// <p>The unique name of the device you want to get the registration status from.</p>
        pub fn set_device_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.device_name = input;
            self
        }
        /// <p>The name of the fleet that the device belongs to.</p>
        pub fn device_fleet_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.device_fleet_name = Some(input.into());
            self
        }
        /// <p>The name of the fleet that the device belongs to.</p>
        pub fn set_device_fleet_name(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.device_fleet_name = input;
            self
        }
        /// Consumes the builder and constructs a [`GetDeviceRegistrationInput`](crate::input::GetDeviceRegistrationInput)
        pub fn build(
            self,
        ) -> std::result::Result<
            crate::input::GetDeviceRegistrationInput,
            aws_smithy_http::operation::BuildError,
        > {
            Ok(crate::input::GetDeviceRegistrationInput {
                device_name: self.device_name,
                device_fleet_name: self.device_fleet_name,
            })
        }
    }
}
#[doc(hidden)]
pub type GetDeviceRegistrationInputOperationOutputAlias = crate::operation::GetDeviceRegistration;
#[doc(hidden)]
pub type GetDeviceRegistrationInputOperationRetryAlias = aws_http::retry::AwsErrorRetryPolicy;
impl GetDeviceRegistrationInput {
    /// Consumes the builder and constructs an Operation<[`GetDeviceRegistration`](crate::operation::GetDeviceRegistration)>
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        aws_smithy_http::operation::Operation<
            crate::operation::GetDeviceRegistration,
            aws_http::retry::AwsErrorRetryPolicy,
        >,
        aws_smithy_http::operation::BuildError,
    > {
        fn uri_base(
            _input: &crate::input::GetDeviceRegistrationInput,
            output: &mut String,
        ) -> Result<(), aws_smithy_http::operation::BuildError> {
            write!(output, "/GetDeviceRegistration").expect("formatting should succeed");
            Ok(())
        }
        #[allow(clippy::unnecessary_wraps)]
        fn update_http_builder(
            input: &crate::input::GetDeviceRegistrationInput,
            builder: http::request::Builder,
        ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError>
        {
            let mut uri = String::new();
            uri_base(input, &mut uri)?;
            Ok(builder.method("POST").uri(uri))
        }
        #[allow(clippy::unnecessary_wraps)]
        fn request_builder_base(
            input: &crate::input::GetDeviceRegistrationInput,
        ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError>
        {
            #[allow(unused_mut)]
            let mut builder = update_http_builder(input, http::request::Builder::new())?;
            builder = aws_smithy_http::header::set_header_if_absent(
                builder,
                http::header::HeaderName::from_static("content-type"),
                "application/json",
            );
            Ok(builder)
        }
        let properties = aws_smithy_http::property_bag::SharedPropertyBag::new();
        let request = request_builder_base(&self)?;
        let body =
            crate::operation_ser::serialize_operation_crate_operation_get_device_registration(
                &self,
            )?;
        let request = Self::assemble(request, body);
        #[allow(unused_mut)]
        let mut request = aws_smithy_http::operation::Request::from_parts(
            request.map(aws_smithy_http::body::SdkBody::from),
            properties,
        );
        let mut user_agent = aws_http::user_agent::AwsUserAgent::new_from_environment(
            aws_types::os_shim_internal::Env::real(),
            crate::API_METADATA.clone(),
        );
        if let Some(app_name) = _config.app_name() {
            user_agent = user_agent.with_app_name(app_name.clone());
        }
        request.properties_mut().insert(user_agent);
        #[allow(unused_mut)]
        let mut signing_config = aws_sig_auth::signer::OperationSigningConfig::default_config();
        request.properties_mut().insert(signing_config);
        request
            .properties_mut()
            .insert(aws_types::SigningService::from_static(
                _config.signing_service(),
            ));
        aws_endpoint::set_endpoint_resolver(
            &mut request.properties_mut(),
            _config.endpoint_resolver.clone(),
        );
        if let Some(region) = &_config.region {
            request.properties_mut().insert(region.clone());
        }
        aws_http::auth::set_provider(
            &mut request.properties_mut(),
            _config.credentials_provider.clone(),
        );
        let op = aws_smithy_http::operation::Operation::new(
            request,
            crate::operation::GetDeviceRegistration::new(),
        )
        .with_metadata(aws_smithy_http::operation::Metadata::new(
            "GetDeviceRegistration",
            "sagemakeredge",
        ));
        let op = op.with_retry_policy(aws_http::retry::AwsErrorRetryPolicy::new());
        Ok(op)
    }
    fn assemble(
        builder: http::request::Builder,
        body: aws_smithy_http::body::SdkBody,
    ) -> http::request::Request<aws_smithy_http::body::SdkBody> {
        let mut builder = builder;
        if let Some(content_length) = body.content_length() {
            builder = aws_smithy_http::header::set_header_if_absent(
                builder,
                http::header::CONTENT_LENGTH,
                content_length,
            );
        }
        builder.body(body).expect("should be valid request")
    }
    /// Creates a new builder-style object to manufacture [`GetDeviceRegistrationInput`](crate::input::GetDeviceRegistrationInput)
    pub fn builder() -> crate::input::get_device_registration_input::Builder {
        crate::input::get_device_registration_input::Builder::default()
    }
}

/// See [`SendHeartbeatInput`](crate::input::SendHeartbeatInput)
pub mod send_heartbeat_input {
    /// A builder for [`SendHeartbeatInput`](crate::input::SendHeartbeatInput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) agent_metrics: std::option::Option<std::vec::Vec<crate::model::EdgeMetric>>,
        pub(crate) models: std::option::Option<std::vec::Vec<crate::model::Model>>,
        pub(crate) agent_version: std::option::Option<std::string::String>,
        pub(crate) device_name: std::option::Option<std::string::String>,
        pub(crate) device_fleet_name: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// Appends an item to `agent_metrics`.
        ///
        /// To override the contents of this collection use [`set_agent_metrics`](Self::set_agent_metrics).
        ///
        /// <p>For internal use. Returns a list of SageMaker Edge Manager agent operating metrics.</p>
        pub fn agent_metrics(mut self, input: crate::model::EdgeMetric) -> Self {
            let mut v = self.agent_metrics.unwrap_or_default();
            v.push(input);
            self.agent_metrics = Some(v);
            self
        }
        /// <p>For internal use. Returns a list of SageMaker Edge Manager agent operating metrics.</p>
        pub fn set_agent_metrics(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::EdgeMetric>>,
        ) -> Self {
            self.agent_metrics = input;
            self
        }
        /// Appends an item to `models`.
        ///
        /// To override the contents of this collection use [`set_models`](Self::set_models).
        ///
        /// <p>Returns a list of models deployed on the the device.</p>
        pub fn models(mut self, input: crate::model::Model) -> Self {
            let mut v = self.models.unwrap_or_default();
            v.push(input);
            self.models = Some(v);
            self
        }
        /// <p>Returns a list of models deployed on the the device.</p>
        pub fn set_models(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::Model>>,
        ) -> Self {
            self.models = input;
            self
        }
        /// <p>Returns the version of the agent.</p>
        pub fn agent_version(mut self, input: impl Into<std::string::String>) -> Self {
            self.agent_version = Some(input.into());
            self
        }
        /// <p>Returns the version of the agent.</p>
        pub fn set_agent_version(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.agent_version = input;
            self
        }
        /// <p>The unique name of the device.</p>
        pub fn device_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.device_name = Some(input.into());
            self
        }
        /// <p>The unique name of the device.</p>
        pub fn set_device_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.device_name = input;
            self
        }
        /// <p>The name of the fleet that the device belongs to.</p>
        pub fn device_fleet_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.device_fleet_name = Some(input.into());
            self
        }
        /// <p>The name of the fleet that the device belongs to.</p>
        pub fn set_device_fleet_name(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.device_fleet_name = input;
            self
        }
        /// Consumes the builder and constructs a [`SendHeartbeatInput`](crate::input::SendHeartbeatInput)
        pub fn build(
            self,
        ) -> std::result::Result<
            crate::input::SendHeartbeatInput,
            aws_smithy_http::operation::BuildError,
        > {
            Ok(crate::input::SendHeartbeatInput {
                agent_metrics: self.agent_metrics,
                models: self.models,
                agent_version: self.agent_version,
                device_name: self.device_name,
                device_fleet_name: self.device_fleet_name,
            })
        }
    }
}
#[doc(hidden)]
pub type SendHeartbeatInputOperationOutputAlias = crate::operation::SendHeartbeat;
#[doc(hidden)]
pub type SendHeartbeatInputOperationRetryAlias = aws_http::retry::AwsErrorRetryPolicy;
impl SendHeartbeatInput {
    /// Consumes the builder and constructs an Operation<[`SendHeartbeat`](crate::operation::SendHeartbeat)>
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        aws_smithy_http::operation::Operation<
            crate::operation::SendHeartbeat,
            aws_http::retry::AwsErrorRetryPolicy,
        >,
        aws_smithy_http::operation::BuildError,
    > {
        fn uri_base(
            _input: &crate::input::SendHeartbeatInput,
            output: &mut String,
        ) -> Result<(), aws_smithy_http::operation::BuildError> {
            write!(output, "/SendHeartbeat").expect("formatting should succeed");
            Ok(())
        }
        #[allow(clippy::unnecessary_wraps)]
        fn update_http_builder(
            input: &crate::input::SendHeartbeatInput,
            builder: http::request::Builder,
        ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError>
        {
            let mut uri = String::new();
            uri_base(input, &mut uri)?;
            Ok(builder.method("POST").uri(uri))
        }
        #[allow(clippy::unnecessary_wraps)]
        fn request_builder_base(
            input: &crate::input::SendHeartbeatInput,
        ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError>
        {
            #[allow(unused_mut)]
            let mut builder = update_http_builder(input, http::request::Builder::new())?;
            builder = aws_smithy_http::header::set_header_if_absent(
                builder,
                http::header::HeaderName::from_static("content-type"),
                "application/json",
            );
            Ok(builder)
        }
        let properties = aws_smithy_http::property_bag::SharedPropertyBag::new();
        let request = request_builder_base(&self)?;
        let body = crate::operation_ser::serialize_operation_crate_operation_send_heartbeat(&self)?;
        let request = Self::assemble(request, body);
        #[allow(unused_mut)]
        let mut request = aws_smithy_http::operation::Request::from_parts(
            request.map(aws_smithy_http::body::SdkBody::from),
            properties,
        );
        let mut user_agent = aws_http::user_agent::AwsUserAgent::new_from_environment(
            aws_types::os_shim_internal::Env::real(),
            crate::API_METADATA.clone(),
        );
        if let Some(app_name) = _config.app_name() {
            user_agent = user_agent.with_app_name(app_name.clone());
        }
        request.properties_mut().insert(user_agent);
        #[allow(unused_mut)]
        let mut signing_config = aws_sig_auth::signer::OperationSigningConfig::default_config();
        request.properties_mut().insert(signing_config);
        request
            .properties_mut()
            .insert(aws_types::SigningService::from_static(
                _config.signing_service(),
            ));
        aws_endpoint::set_endpoint_resolver(
            &mut request.properties_mut(),
            _config.endpoint_resolver.clone(),
        );
        if let Some(region) = &_config.region {
            request.properties_mut().insert(region.clone());
        }
        aws_http::auth::set_provider(
            &mut request.properties_mut(),
            _config.credentials_provider.clone(),
        );
        let op = aws_smithy_http::operation::Operation::new(
            request,
            crate::operation::SendHeartbeat::new(),
        )
        .with_metadata(aws_smithy_http::operation::Metadata::new(
            "SendHeartbeat",
            "sagemakeredge",
        ));
        let op = op.with_retry_policy(aws_http::retry::AwsErrorRetryPolicy::new());
        Ok(op)
    }
    fn assemble(
        builder: http::request::Builder,
        body: aws_smithy_http::body::SdkBody,
    ) -> http::request::Request<aws_smithy_http::body::SdkBody> {
        let mut builder = builder;
        if let Some(content_length) = body.content_length() {
            builder = aws_smithy_http::header::set_header_if_absent(
                builder,
                http::header::CONTENT_LENGTH,
                content_length,
            );
        }
        builder.body(body).expect("should be valid request")
    }
    /// Creates a new builder-style object to manufacture [`SendHeartbeatInput`](crate::input::SendHeartbeatInput)
    pub fn builder() -> crate::input::send_heartbeat_input::Builder {
        crate::input::send_heartbeat_input::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct SendHeartbeatInput {
    /// <p>For internal use. Returns a list of SageMaker Edge Manager agent operating metrics.</p>
    pub agent_metrics: std::option::Option<std::vec::Vec<crate::model::EdgeMetric>>,
    /// <p>Returns a list of models deployed on the the device.</p>
    pub models: std::option::Option<std::vec::Vec<crate::model::Model>>,
    /// <p>Returns the version of the agent.</p>
    pub agent_version: std::option::Option<std::string::String>,
    /// <p>The unique name of the device.</p>
    pub device_name: std::option::Option<std::string::String>,
    /// <p>The name of the fleet that the device belongs to.</p>
    pub device_fleet_name: std::option::Option<std::string::String>,
}
impl SendHeartbeatInput {
    /// <p>For internal use. Returns a list of SageMaker Edge Manager agent operating metrics.</p>
    pub fn agent_metrics(&self) -> std::option::Option<&[crate::model::EdgeMetric]> {
        self.agent_metrics.as_deref()
    }
    /// <p>Returns a list of models deployed on the the device.</p>
    pub fn models(&self) -> std::option::Option<&[crate::model::Model]> {
        self.models.as_deref()
    }
    /// <p>Returns the version of the agent.</p>
    pub fn agent_version(&self) -> std::option::Option<&str> {
        self.agent_version.as_deref()
    }
    /// <p>The unique name of the device.</p>
    pub fn device_name(&self) -> std::option::Option<&str> {
        self.device_name.as_deref()
    }
    /// <p>The name of the fleet that the device belongs to.</p>
    pub fn device_fleet_name(&self) -> std::option::Option<&str> {
        self.device_fleet_name.as_deref()
    }
}
impl std::fmt::Debug for SendHeartbeatInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("SendHeartbeatInput");
        formatter.field("agent_metrics", &self.agent_metrics);
        formatter.field("models", &self.models);
        formatter.field("agent_version", &self.agent_version);
        formatter.field("device_name", &self.device_name);
        formatter.field("device_fleet_name", &self.device_fleet_name);
        formatter.finish()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct GetDeviceRegistrationInput {
    /// <p>The unique name of the device you want to get the registration status from.</p>
    pub device_name: std::option::Option<std::string::String>,
    /// <p>The name of the fleet that the device belongs to.</p>
    pub device_fleet_name: std::option::Option<std::string::String>,
}
impl GetDeviceRegistrationInput {
    /// <p>The unique name of the device you want to get the registration status from.</p>
    pub fn device_name(&self) -> std::option::Option<&str> {
        self.device_name.as_deref()
    }
    /// <p>The name of the fleet that the device belongs to.</p>
    pub fn device_fleet_name(&self) -> std::option::Option<&str> {
        self.device_fleet_name.as_deref()
    }
}
impl std::fmt::Debug for GetDeviceRegistrationInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("GetDeviceRegistrationInput");
        formatter.field("device_name", &self.device_name);
        formatter.field("device_fleet_name", &self.device_fleet_name);
        formatter.finish()
    }
}

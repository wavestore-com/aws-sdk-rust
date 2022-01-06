// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
use std::fmt::Write;
/// See [`DeleteConnectionInput`](crate::input::DeleteConnectionInput)
pub mod delete_connection_input {
    /// A builder for [`DeleteConnectionInput`](crate::input::DeleteConnectionInput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) connection_id: std::option::Option<std::string::String>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn connection_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.connection_id = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_connection_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.connection_id = input;
            self
        }
        /// Consumes the builder and constructs a [`DeleteConnectionInput`](crate::input::DeleteConnectionInput)
        pub fn build(
            self,
        ) -> std::result::Result<
            crate::input::DeleteConnectionInput,
            aws_smithy_http::operation::BuildError,
        > {
            Ok(crate::input::DeleteConnectionInput {
                connection_id: self.connection_id,
            })
        }
    }
}
#[doc(hidden)]
pub type DeleteConnectionInputOperationOutputAlias = crate::operation::DeleteConnection;
#[doc(hidden)]
pub type DeleteConnectionInputOperationRetryAlias = aws_http::retry::AwsErrorRetryPolicy;
impl DeleteConnectionInput {
    /// Consumes the builder and constructs an Operation<[`DeleteConnection`](crate::operation::DeleteConnection)>
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        aws_smithy_http::operation::Operation<
            crate::operation::DeleteConnection,
            aws_http::retry::AwsErrorRetryPolicy,
        >,
        aws_smithy_http::operation::BuildError,
    > {
        fn uri_base(
            _input: &crate::input::DeleteConnectionInput,
            output: &mut String,
        ) -> Result<(), aws_smithy_http::operation::BuildError> {
            let input_1 = &_input.connection_id;
            let input_1 =
                input_1
                    .as_ref()
                    .ok_or(aws_smithy_http::operation::BuildError::MissingField {
                        field: "connection_id",
                        details: "cannot be empty or unset",
                    })?;
            let connection_id = aws_smithy_http::label::fmt_string(input_1, false);
            if connection_id.is_empty() {
                return Err(aws_smithy_http::operation::BuildError::MissingField {
                    field: "connection_id",
                    details: "cannot be empty or unset",
                });
            }
            write!(
                output,
                "/@connections/{ConnectionId}",
                ConnectionId = connection_id
            )
            .expect("formatting should succeed");
            Ok(())
        }
        #[allow(clippy::unnecessary_wraps)]
        fn update_http_builder(
            input: &crate::input::DeleteConnectionInput,
            builder: http::request::Builder,
        ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError>
        {
            let mut uri = String::new();
            uri_base(input, &mut uri)?;
            Ok(builder.method("DELETE").uri(uri))
        }
        #[allow(clippy::unnecessary_wraps)]
        fn request_builder_base(
            input: &crate::input::DeleteConnectionInput,
        ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError>
        {
            #[allow(unused_mut)]
            let mut builder = update_http_builder(input, http::request::Builder::new())?;
            Ok(builder)
        }
        let properties = aws_smithy_http::property_bag::SharedPropertyBag::new();
        let request = request_builder_base(&self)?;
        let body = aws_smithy_http::body::SdkBody::from("");
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
            crate::operation::DeleteConnection::new(),
        )
        .with_metadata(aws_smithy_http::operation::Metadata::new(
            "DeleteConnection",
            "apigatewaymanagementapi",
        ));
        let op = op.with_retry_policy(aws_http::retry::AwsErrorRetryPolicy::new());
        Ok(op)
    }
    fn assemble(
        builder: http::request::Builder,
        body: aws_smithy_http::body::SdkBody,
    ) -> http::request::Request<aws_smithy_http::body::SdkBody> {
        builder.body(body).expect("should be valid request")
    }
    /// Creates a new builder-style object to manufacture [`DeleteConnectionInput`](crate::input::DeleteConnectionInput)
    pub fn builder() -> crate::input::delete_connection_input::Builder {
        crate::input::delete_connection_input::Builder::default()
    }
}

/// See [`GetConnectionInput`](crate::input::GetConnectionInput)
pub mod get_connection_input {
    /// A builder for [`GetConnectionInput`](crate::input::GetConnectionInput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) connection_id: std::option::Option<std::string::String>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn connection_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.connection_id = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_connection_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.connection_id = input;
            self
        }
        /// Consumes the builder and constructs a [`GetConnectionInput`](crate::input::GetConnectionInput)
        pub fn build(
            self,
        ) -> std::result::Result<
            crate::input::GetConnectionInput,
            aws_smithy_http::operation::BuildError,
        > {
            Ok(crate::input::GetConnectionInput {
                connection_id: self.connection_id,
            })
        }
    }
}
#[doc(hidden)]
pub type GetConnectionInputOperationOutputAlias = crate::operation::GetConnection;
#[doc(hidden)]
pub type GetConnectionInputOperationRetryAlias = aws_http::retry::AwsErrorRetryPolicy;
impl GetConnectionInput {
    /// Consumes the builder and constructs an Operation<[`GetConnection`](crate::operation::GetConnection)>
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        aws_smithy_http::operation::Operation<
            crate::operation::GetConnection,
            aws_http::retry::AwsErrorRetryPolicy,
        >,
        aws_smithy_http::operation::BuildError,
    > {
        fn uri_base(
            _input: &crate::input::GetConnectionInput,
            output: &mut String,
        ) -> Result<(), aws_smithy_http::operation::BuildError> {
            let input_2 = &_input.connection_id;
            let input_2 =
                input_2
                    .as_ref()
                    .ok_or(aws_smithy_http::operation::BuildError::MissingField {
                        field: "connection_id",
                        details: "cannot be empty or unset",
                    })?;
            let connection_id = aws_smithy_http::label::fmt_string(input_2, false);
            if connection_id.is_empty() {
                return Err(aws_smithy_http::operation::BuildError::MissingField {
                    field: "connection_id",
                    details: "cannot be empty or unset",
                });
            }
            write!(
                output,
                "/@connections/{ConnectionId}",
                ConnectionId = connection_id
            )
            .expect("formatting should succeed");
            Ok(())
        }
        #[allow(clippy::unnecessary_wraps)]
        fn update_http_builder(
            input: &crate::input::GetConnectionInput,
            builder: http::request::Builder,
        ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError>
        {
            let mut uri = String::new();
            uri_base(input, &mut uri)?;
            Ok(builder.method("GET").uri(uri))
        }
        #[allow(clippy::unnecessary_wraps)]
        fn request_builder_base(
            input: &crate::input::GetConnectionInput,
        ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError>
        {
            #[allow(unused_mut)]
            let mut builder = update_http_builder(input, http::request::Builder::new())?;
            Ok(builder)
        }
        let properties = aws_smithy_http::property_bag::SharedPropertyBag::new();
        let request = request_builder_base(&self)?;
        let body = aws_smithy_http::body::SdkBody::from("");
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
            crate::operation::GetConnection::new(),
        )
        .with_metadata(aws_smithy_http::operation::Metadata::new(
            "GetConnection",
            "apigatewaymanagementapi",
        ));
        let op = op.with_retry_policy(aws_http::retry::AwsErrorRetryPolicy::new());
        Ok(op)
    }
    fn assemble(
        builder: http::request::Builder,
        body: aws_smithy_http::body::SdkBody,
    ) -> http::request::Request<aws_smithy_http::body::SdkBody> {
        builder.body(body).expect("should be valid request")
    }
    /// Creates a new builder-style object to manufacture [`GetConnectionInput`](crate::input::GetConnectionInput)
    pub fn builder() -> crate::input::get_connection_input::Builder {
        crate::input::get_connection_input::Builder::default()
    }
}

/// See [`PostToConnectionInput`](crate::input::PostToConnectionInput)
pub mod post_to_connection_input {
    /// A builder for [`PostToConnectionInput`](crate::input::PostToConnectionInput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) data: std::option::Option<aws_smithy_types::Blob>,
        pub(crate) connection_id: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The data to be sent to the client specified by its connection id.</p>
        pub fn data(mut self, input: aws_smithy_types::Blob) -> Self {
            self.data = Some(input);
            self
        }
        /// <p>The data to be sent to the client specified by its connection id.</p>
        pub fn set_data(mut self, input: std::option::Option<aws_smithy_types::Blob>) -> Self {
            self.data = input;
            self
        }
        /// <p>The identifier of the connection that a specific client is using.</p>
        pub fn connection_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.connection_id = Some(input.into());
            self
        }
        /// <p>The identifier of the connection that a specific client is using.</p>
        pub fn set_connection_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.connection_id = input;
            self
        }
        /// Consumes the builder and constructs a [`PostToConnectionInput`](crate::input::PostToConnectionInput)
        pub fn build(
            self,
        ) -> std::result::Result<
            crate::input::PostToConnectionInput,
            aws_smithy_http::operation::BuildError,
        > {
            Ok(crate::input::PostToConnectionInput {
                data: self.data,
                connection_id: self.connection_id,
            })
        }
    }
}
#[doc(hidden)]
pub type PostToConnectionInputOperationOutputAlias = crate::operation::PostToConnection;
#[doc(hidden)]
pub type PostToConnectionInputOperationRetryAlias = aws_http::retry::AwsErrorRetryPolicy;
impl PostToConnectionInput {
    /// Consumes the builder and constructs an Operation<[`PostToConnection`](crate::operation::PostToConnection)>
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        aws_smithy_http::operation::Operation<
            crate::operation::PostToConnection,
            aws_http::retry::AwsErrorRetryPolicy,
        >,
        aws_smithy_http::operation::BuildError,
    > {
        fn uri_base(
            _input: &crate::input::PostToConnectionInput,
            output: &mut String,
        ) -> Result<(), aws_smithy_http::operation::BuildError> {
            let input_3 = &_input.connection_id;
            let input_3 =
                input_3
                    .as_ref()
                    .ok_or(aws_smithy_http::operation::BuildError::MissingField {
                        field: "connection_id",
                        details: "cannot be empty or unset",
                    })?;
            let connection_id = aws_smithy_http::label::fmt_string(input_3, false);
            if connection_id.is_empty() {
                return Err(aws_smithy_http::operation::BuildError::MissingField {
                    field: "connection_id",
                    details: "cannot be empty or unset",
                });
            }
            write!(
                output,
                "/@connections/{ConnectionId}",
                ConnectionId = connection_id
            )
            .expect("formatting should succeed");
            Ok(())
        }
        #[allow(clippy::unnecessary_wraps)]
        fn update_http_builder(
            input: &crate::input::PostToConnectionInput,
            builder: http::request::Builder,
        ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError>
        {
            let mut uri = String::new();
            uri_base(input, &mut uri)?;
            Ok(builder.method("POST").uri(uri))
        }
        #[allow(clippy::unnecessary_wraps)]
        fn request_builder_base(
            input: &crate::input::PostToConnectionInput,
        ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError>
        {
            #[allow(unused_mut)]
            let mut builder = update_http_builder(input, http::request::Builder::new())?;
            builder = aws_smithy_http::header::set_header_if_absent(
                builder,
                http::header::HeaderName::from_static("content-type"),
                "application/octet-stream",
            );
            Ok(builder)
        }
        let properties = aws_smithy_http::property_bag::SharedPropertyBag::new();
        let request = request_builder_base(&self)?;
        let body = crate::operation_ser::ser_payload_post_to_connection_input(self.data)?;
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
            crate::operation::PostToConnection::new(),
        )
        .with_metadata(aws_smithy_http::operation::Metadata::new(
            "PostToConnection",
            "apigatewaymanagementapi",
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
    /// Creates a new builder-style object to manufacture [`PostToConnectionInput`](crate::input::PostToConnectionInput)
    pub fn builder() -> crate::input::post_to_connection_input::Builder {
        crate::input::post_to_connection_input::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct PostToConnectionInput {
    /// <p>The data to be sent to the client specified by its connection id.</p>
    pub data: std::option::Option<aws_smithy_types::Blob>,
    /// <p>The identifier of the connection that a specific client is using.</p>
    pub connection_id: std::option::Option<std::string::String>,
}
impl PostToConnectionInput {
    /// <p>The data to be sent to the client specified by its connection id.</p>
    pub fn data(&self) -> std::option::Option<&aws_smithy_types::Blob> {
        self.data.as_ref()
    }
    /// <p>The identifier of the connection that a specific client is using.</p>
    pub fn connection_id(&self) -> std::option::Option<&str> {
        self.connection_id.as_deref()
    }
}
impl std::fmt::Debug for PostToConnectionInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("PostToConnectionInput");
        formatter.field("data", &self.data);
        formatter.field("connection_id", &self.connection_id);
        formatter.finish()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct GetConnectionInput {
    #[allow(missing_docs)] // documentation missing in model
    pub connection_id: std::option::Option<std::string::String>,
}
impl GetConnectionInput {
    #[allow(missing_docs)] // documentation missing in model
    pub fn connection_id(&self) -> std::option::Option<&str> {
        self.connection_id.as_deref()
    }
}
impl std::fmt::Debug for GetConnectionInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("GetConnectionInput");
        formatter.field("connection_id", &self.connection_id);
        formatter.finish()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DeleteConnectionInput {
    #[allow(missing_docs)] // documentation missing in model
    pub connection_id: std::option::Option<std::string::String>,
}
impl DeleteConnectionInput {
    #[allow(missing_docs)] // documentation missing in model
    pub fn connection_id(&self) -> std::option::Option<&str> {
        self.connection_id.as_deref()
    }
}
impl std::fmt::Debug for DeleteConnectionInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DeleteConnectionInput");
        formatter.field("connection_id", &self.connection_id);
        formatter.finish()
    }
}

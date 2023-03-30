// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
use std::fmt::Write;

/// See [`GetIceServerConfigInput`](crate::input::GetIceServerConfigInput).
pub mod get_ice_server_config_input {
    
    /// A builder for [`GetIceServerConfigInput`](crate::input::GetIceServerConfigInput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) channel_arn: std::option::Option<std::string::String>,
        pub(crate) client_id: std::option::Option<std::string::String>,
        pub(crate) service: std::option::Option<crate::model::Service>,
        pub(crate) username: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The ARN of the signaling channel to be used for the peer-to-peer connection between configured peers. </p>
        pub fn channel_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.channel_arn = Some(input.into());
            self
        }
        /// <p>The ARN of the signaling channel to be used for the peer-to-peer connection between configured peers. </p>
        pub fn set_channel_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.channel_arn = input; self
        }
        /// <p>Unique identifier for the viewer. Must be unique within the signaling channel.</p>
        pub fn client_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.client_id = Some(input.into());
            self
        }
        /// <p>Unique identifier for the viewer. Must be unique within the signaling channel.</p>
        pub fn set_client_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.client_id = input; self
        }
        /// <p>Specifies the desired service. Currently, <code>TURN</code> is the only valid value.</p>
        pub fn service(mut self, input: crate::model::Service) -> Self {
            self.service = Some(input);
            self
        }
        /// <p>Specifies the desired service. Currently, <code>TURN</code> is the only valid value.</p>
        pub fn set_service(mut self, input: std::option::Option<crate::model::Service>) -> Self {
            self.service = input; self
        }
        /// <p>An optional user ID to be associated with the credentials.</p>
        pub fn username(mut self, input: impl Into<std::string::String>) -> Self {
            self.username = Some(input.into());
            self
        }
        /// <p>An optional user ID to be associated with the credentials.</p>
        pub fn set_username(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.username = input; self
        }
        /// Consumes the builder and constructs a [`GetIceServerConfigInput`](crate::input::GetIceServerConfigInput).
        pub fn build(self) -> Result<crate::input::GetIceServerConfigInput, aws_smithy_http::operation::error::BuildError> {
            Ok(
                crate::input::GetIceServerConfigInput {
                    channel_arn: self.channel_arn
                    ,
                    client_id: self.client_id
                    ,
                    service: self.service
                    ,
                    username: self.username
                    ,
                }
            )
        }
    }
    
    
}
impl GetIceServerConfigInput {
    /// Consumes the builder and constructs an Operation<[`GetIceServerConfig`](crate::operation::GetIceServerConfig)>
    #[allow(unused_mut)]
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(&self, _config: &crate::config::Config) -> std::result::Result<aws_smithy_http::operation::Operation<crate::operation::GetIceServerConfig, aws_http::retry::AwsResponseRetryClassifier>, aws_smithy_http::operation::error::BuildError> {
        let params_result = crate::endpoint::Params::builder().set_region(_config.region.as_ref().map(|r|r.as_ref().to_owned()))
        .set_use_dual_stack(_config.use_dual_stack)
        .set_use_fips(_config.use_fips)
        .set_endpoint(_config.endpoint_url
        .clone()).build()
                                    .map_err(|err|aws_smithy_http::endpoint::ResolveEndpointError::from_source("could not construct endpoint parameters", err));
                                let (endpoint_result, params) = match params_result {
                                    Ok(params) => (_config.endpoint_resolver.resolve_endpoint(&params), Some(params)),
                                    Err(e) => (Err(e), None)
                                };
        let mut request = {
            fn uri_base(_input: &crate::input::GetIceServerConfigInput, output: &mut String) -> Result<(), aws_smithy_http::operation::error::BuildError> {
                write!(output, "/v1/get-ice-server-config").expect("formatting should succeed");
                Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                            input: &crate::input::GetIceServerConfigInput,
                            builder: http::request::Builder
                        ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::error::BuildError> {
                let mut uri = String::new();
                uri_base(input, &mut uri)?;
                Ok(builder.method("POST").uri(uri))
            }
            let mut builder = update_http_builder(&self, http::request::Builder::new())?;
            builder = aws_smithy_http::header::set_request_header_if_absent(builder, http::header::CONTENT_TYPE, "application/json");
            builder
        };
        let mut properties = aws_smithy_http::property_bag::SharedPropertyBag::new();
        #[allow(clippy::useless_conversion)]
        let body = aws_smithy_http::body::SdkBody::from(
            crate::operation_ser::serialize_operation_crate_operation_get_ice_server_config(&self)?
        );
        if let Some(content_length) = body.content_length() {
                                request = aws_smithy_http::header::set_request_header_if_absent(request, http::header::CONTENT_LENGTH, content_length);
                            }
        let request = request.body(body).expect("should be valid request");
        let mut request = aws_smithy_http::operation::Request::from_parts(request, properties);
        request.properties_mut().insert(endpoint_result);
        if let Some(params) = params { request.properties_mut().insert(params); }
        request.properties_mut().insert(aws_smithy_http::http_versions::DEFAULT_HTTP_VERSION_LIST.clone());
        let mut user_agent = aws_http::user_agent::AwsUserAgent::new_from_environment(
                                aws_types::os_shim_internal::Env::real(),
                                crate::API_METADATA.clone(),
                            );
                            if let Some(app_name) = _config.app_name() {
                                user_agent = user_agent.with_app_name(app_name.clone());
                            }
                            request.properties_mut().insert(user_agent);
        let mut signing_config = aws_sig_auth::signer::OperationSigningConfig::default_config();
        request.properties_mut().insert(signing_config);
                            request.properties_mut().insert(aws_types::SigningService::from_static(_config.signing_service()));
                            if let Some(region) = &_config.region {
                                request.properties_mut().insert(aws_types::region::SigningRegion::from(region.clone()));
                            }
        if let Some(region) = &_config.region {
                                request.properties_mut().insert(region.clone());
                            }
        aws_http::auth::set_credentials_cache(&mut request.properties_mut(), _config.credentials_cache.clone());
        let op = aws_smithy_http::operation::Operation::new(request, crate::operation::GetIceServerConfig::new())
                            .with_metadata(aws_smithy_http::operation::Metadata::new("GetIceServerConfig", "kinesisvideosignaling"));
        let op = op.with_retry_classifier(aws_http::retry::AwsResponseRetryClassifier::new());
        Ok(op)
    }
    /// Creates a new builder-style object to manufacture [`GetIceServerConfigInput`](crate::input::GetIceServerConfigInput).
    pub fn builder() -> crate::input::get_ice_server_config_input::Builder {
        crate::input::get_ice_server_config_input::Builder::default()
    }
}

/// See [`SendAlexaOfferToMasterInput`](crate::input::SendAlexaOfferToMasterInput).
pub mod send_alexa_offer_to_master_input {
    
    /// A builder for [`SendAlexaOfferToMasterInput`](crate::input::SendAlexaOfferToMasterInput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) channel_arn: std::option::Option<std::string::String>,
        pub(crate) sender_client_id: std::option::Option<std::string::String>,
        pub(crate) message_payload: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The ARN of the signaling channel by which Alexa and the master peer communicate.</p>
        pub fn channel_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.channel_arn = Some(input.into());
            self
        }
        /// <p>The ARN of the signaling channel by which Alexa and the master peer communicate.</p>
        pub fn set_channel_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.channel_arn = input; self
        }
        /// <p>The unique identifier for the sender client.</p>
        pub fn sender_client_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.sender_client_id = Some(input.into());
            self
        }
        /// <p>The unique identifier for the sender client.</p>
        pub fn set_sender_client_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.sender_client_id = input; self
        }
        /// <p>The base64-encoded SDP offer content.</p>
        pub fn message_payload(mut self, input: impl Into<std::string::String>) -> Self {
            self.message_payload = Some(input.into());
            self
        }
        /// <p>The base64-encoded SDP offer content.</p>
        pub fn set_message_payload(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message_payload = input; self
        }
        /// Consumes the builder and constructs a [`SendAlexaOfferToMasterInput`](crate::input::SendAlexaOfferToMasterInput).
        pub fn build(self) -> Result<crate::input::SendAlexaOfferToMasterInput, aws_smithy_http::operation::error::BuildError> {
            Ok(
                crate::input::SendAlexaOfferToMasterInput {
                    channel_arn: self.channel_arn
                    ,
                    sender_client_id: self.sender_client_id
                    ,
                    message_payload: self.message_payload
                    ,
                }
            )
        }
    }
    
    
}
impl SendAlexaOfferToMasterInput {
    /// Consumes the builder and constructs an Operation<[`SendAlexaOfferToMaster`](crate::operation::SendAlexaOfferToMaster)>
    #[allow(unused_mut)]
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(&self, _config: &crate::config::Config) -> std::result::Result<aws_smithy_http::operation::Operation<crate::operation::SendAlexaOfferToMaster, aws_http::retry::AwsResponseRetryClassifier>, aws_smithy_http::operation::error::BuildError> {
        let params_result = crate::endpoint::Params::builder().set_region(_config.region.as_ref().map(|r|r.as_ref().to_owned()))
        .set_use_dual_stack(_config.use_dual_stack)
        .set_use_fips(_config.use_fips)
        .set_endpoint(_config.endpoint_url
        .clone()).build()
                                    .map_err(|err|aws_smithy_http::endpoint::ResolveEndpointError::from_source("could not construct endpoint parameters", err));
                                let (endpoint_result, params) = match params_result {
                                    Ok(params) => (_config.endpoint_resolver.resolve_endpoint(&params), Some(params)),
                                    Err(e) => (Err(e), None)
                                };
        let mut request = {
            fn uri_base(_input: &crate::input::SendAlexaOfferToMasterInput, output: &mut String) -> Result<(), aws_smithy_http::operation::error::BuildError> {
                write!(output, "/v1/send-alexa-offer-to-master").expect("formatting should succeed");
                Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                            input: &crate::input::SendAlexaOfferToMasterInput,
                            builder: http::request::Builder
                        ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::error::BuildError> {
                let mut uri = String::new();
                uri_base(input, &mut uri)?;
                Ok(builder.method("POST").uri(uri))
            }
            let mut builder = update_http_builder(&self, http::request::Builder::new())?;
            builder = aws_smithy_http::header::set_request_header_if_absent(builder, http::header::CONTENT_TYPE, "application/json");
            builder
        };
        let mut properties = aws_smithy_http::property_bag::SharedPropertyBag::new();
        #[allow(clippy::useless_conversion)]
        let body = aws_smithy_http::body::SdkBody::from(
            crate::operation_ser::serialize_operation_crate_operation_send_alexa_offer_to_master(&self)?
        );
        if let Some(content_length) = body.content_length() {
                                request = aws_smithy_http::header::set_request_header_if_absent(request, http::header::CONTENT_LENGTH, content_length);
                            }
        let request = request.body(body).expect("should be valid request");
        let mut request = aws_smithy_http::operation::Request::from_parts(request, properties);
        request.properties_mut().insert(endpoint_result);
        if let Some(params) = params { request.properties_mut().insert(params); }
        request.properties_mut().insert(aws_smithy_http::http_versions::DEFAULT_HTTP_VERSION_LIST.clone());
        let mut user_agent = aws_http::user_agent::AwsUserAgent::new_from_environment(
                                aws_types::os_shim_internal::Env::real(),
                                crate::API_METADATA.clone(),
                            );
                            if let Some(app_name) = _config.app_name() {
                                user_agent = user_agent.with_app_name(app_name.clone());
                            }
                            request.properties_mut().insert(user_agent);
        let mut signing_config = aws_sig_auth::signer::OperationSigningConfig::default_config();
        request.properties_mut().insert(signing_config);
                            request.properties_mut().insert(aws_types::SigningService::from_static(_config.signing_service()));
                            if let Some(region) = &_config.region {
                                request.properties_mut().insert(aws_types::region::SigningRegion::from(region.clone()));
                            }
        if let Some(region) = &_config.region {
                                request.properties_mut().insert(region.clone());
                            }
        aws_http::auth::set_credentials_cache(&mut request.properties_mut(), _config.credentials_cache.clone());
        let op = aws_smithy_http::operation::Operation::new(request, crate::operation::SendAlexaOfferToMaster::new())
                            .with_metadata(aws_smithy_http::operation::Metadata::new("SendAlexaOfferToMaster", "kinesisvideosignaling"));
        let op = op.with_retry_classifier(aws_http::retry::AwsResponseRetryClassifier::new());
        Ok(op)
    }
    /// Creates a new builder-style object to manufacture [`SendAlexaOfferToMasterInput`](crate::input::SendAlexaOfferToMasterInput).
    pub fn builder() -> crate::input::send_alexa_offer_to_master_input::Builder {
        crate::input::send_alexa_offer_to_master_input::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct SendAlexaOfferToMasterInput  {
    /// <p>The ARN of the signaling channel by which Alexa and the master peer communicate.</p>
    #[doc(hidden)]
    pub channel_arn: std::option::Option<std::string::String>,
    /// <p>The unique identifier for the sender client.</p>
    #[doc(hidden)]
    pub sender_client_id: std::option::Option<std::string::String>,
    /// <p>The base64-encoded SDP offer content.</p>
    #[doc(hidden)]
    pub message_payload: std::option::Option<std::string::String>,
}
impl SendAlexaOfferToMasterInput {
    /// <p>The ARN of the signaling channel by which Alexa and the master peer communicate.</p>
    pub fn channel_arn(&self) -> std::option::Option<& str> {
        self.channel_arn.as_deref()
    }
    /// <p>The unique identifier for the sender client.</p>
    pub fn sender_client_id(&self) -> std::option::Option<& str> {
        self.sender_client_id.as_deref()
    }
    /// <p>The base64-encoded SDP offer content.</p>
    pub fn message_payload(&self) -> std::option::Option<& str> {
        self.message_payload.as_deref()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct GetIceServerConfigInput  {
    /// <p>The ARN of the signaling channel to be used for the peer-to-peer connection between configured peers. </p>
    #[doc(hidden)]
    pub channel_arn: std::option::Option<std::string::String>,
    /// <p>Unique identifier for the viewer. Must be unique within the signaling channel.</p>
    #[doc(hidden)]
    pub client_id: std::option::Option<std::string::String>,
    /// <p>Specifies the desired service. Currently, <code>TURN</code> is the only valid value.</p>
    #[doc(hidden)]
    pub service: std::option::Option<crate::model::Service>,
    /// <p>An optional user ID to be associated with the credentials.</p>
    #[doc(hidden)]
    pub username: std::option::Option<std::string::String>,
}
impl GetIceServerConfigInput {
    /// <p>The ARN of the signaling channel to be used for the peer-to-peer connection between configured peers. </p>
    pub fn channel_arn(&self) -> std::option::Option<& str> {
        self.channel_arn.as_deref()
    }
    /// <p>Unique identifier for the viewer. Must be unique within the signaling channel.</p>
    pub fn client_id(&self) -> std::option::Option<& str> {
        self.client_id.as_deref()
    }
    /// <p>Specifies the desired service. Currently, <code>TURN</code> is the only valid value.</p>
    pub fn service(&self) -> std::option::Option<& crate::model::Service> {
        self.service.as_ref()
    }
    /// <p>An optional user ID to be associated with the credentials.</p>
    pub fn username(&self) -> std::option::Option<& str> {
        self.username.as_deref()
    }
}


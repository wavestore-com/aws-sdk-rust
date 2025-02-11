// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::modify_verified_access_endpoint::_modify_verified_access_endpoint_output::ModifyVerifiedAccessEndpointOutputBuilder;

pub use crate::operation::modify_verified_access_endpoint::_modify_verified_access_endpoint_input::ModifyVerifiedAccessEndpointInputBuilder;

impl crate::operation::modify_verified_access_endpoint::builders::ModifyVerifiedAccessEndpointInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::modify_verified_access_endpoint::ModifyVerifiedAccessEndpointOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::modify_verified_access_endpoint::ModifyVerifiedAccessEndpointError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.modify_verified_access_endpoint();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ModifyVerifiedAccessEndpoint`.
///
/// <p>Modifies the configuration of the specified Amazon Web Services Verified Access endpoint.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ModifyVerifiedAccessEndpointFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::modify_verified_access_endpoint::builders::ModifyVerifiedAccessEndpointInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::modify_verified_access_endpoint::ModifyVerifiedAccessEndpointOutput,
        crate::operation::modify_verified_access_endpoint::ModifyVerifiedAccessEndpointError,
    > for ModifyVerifiedAccessEndpointFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::modify_verified_access_endpoint::ModifyVerifiedAccessEndpointOutput,
            crate::operation::modify_verified_access_endpoint::ModifyVerifiedAccessEndpointError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ModifyVerifiedAccessEndpointFluentBuilder {
    /// Creates a new `ModifyVerifiedAccessEndpointFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ModifyVerifiedAccessEndpoint as a reference.
    pub fn as_input(&self) -> &crate::operation::modify_verified_access_endpoint::builders::ModifyVerifiedAccessEndpointInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::modify_verified_access_endpoint::ModifyVerifiedAccessEndpointOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::modify_verified_access_endpoint::ModifyVerifiedAccessEndpointError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::modify_verified_access_endpoint::ModifyVerifiedAccessEndpoint::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::modify_verified_access_endpoint::ModifyVerifiedAccessEndpoint::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::modify_verified_access_endpoint::ModifyVerifiedAccessEndpointOutput,
        crate::operation::modify_verified_access_endpoint::ModifyVerifiedAccessEndpointError,
        Self,
    > {
        crate::client::customize::CustomizableOperation::new(self)
    }
    pub(crate) fn config_override(mut self, config_override: impl ::std::convert::Into<crate::config::Builder>) -> Self {
        self.set_config_override(::std::option::Option::Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: ::std::option::Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
    /// <p>The ID of the Verified Access endpoint.</p>
    pub fn verified_access_endpoint_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.verified_access_endpoint_id(input.into());
        self
    }
    /// <p>The ID of the Verified Access endpoint.</p>
    pub fn set_verified_access_endpoint_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_verified_access_endpoint_id(input);
        self
    }
    /// <p>The ID of the Verified Access endpoint.</p>
    pub fn get_verified_access_endpoint_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_verified_access_endpoint_id()
    }
    /// <p>The ID of the Verified Access group.</p>
    pub fn verified_access_group_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.verified_access_group_id(input.into());
        self
    }
    /// <p>The ID of the Verified Access group.</p>
    pub fn set_verified_access_group_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_verified_access_group_id(input);
        self
    }
    /// <p>The ID of the Verified Access group.</p>
    pub fn get_verified_access_group_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_verified_access_group_id()
    }
    /// <p>The load balancer details if creating the Verified Access endpoint as <code>load-balancer</code>type.</p>
    pub fn load_balancer_options(mut self, input: crate::types::ModifyVerifiedAccessEndpointLoadBalancerOptions) -> Self {
        self.inner = self.inner.load_balancer_options(input);
        self
    }
    /// <p>The load balancer details if creating the Verified Access endpoint as <code>load-balancer</code>type.</p>
    pub fn set_load_balancer_options(mut self, input: ::std::option::Option<crate::types::ModifyVerifiedAccessEndpointLoadBalancerOptions>) -> Self {
        self.inner = self.inner.set_load_balancer_options(input);
        self
    }
    /// <p>The load balancer details if creating the Verified Access endpoint as <code>load-balancer</code>type.</p>
    pub fn get_load_balancer_options(&self) -> &::std::option::Option<crate::types::ModifyVerifiedAccessEndpointLoadBalancerOptions> {
        self.inner.get_load_balancer_options()
    }
    /// <p>The network interface options.</p>
    pub fn network_interface_options(mut self, input: crate::types::ModifyVerifiedAccessEndpointEniOptions) -> Self {
        self.inner = self.inner.network_interface_options(input);
        self
    }
    /// <p>The network interface options.</p>
    pub fn set_network_interface_options(mut self, input: ::std::option::Option<crate::types::ModifyVerifiedAccessEndpointEniOptions>) -> Self {
        self.inner = self.inner.set_network_interface_options(input);
        self
    }
    /// <p>The network interface options.</p>
    pub fn get_network_interface_options(&self) -> &::std::option::Option<crate::types::ModifyVerifiedAccessEndpointEniOptions> {
        self.inner.get_network_interface_options()
    }
    /// <p>A description for the Verified Access endpoint.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>A description for the Verified Access endpoint.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>A description for the Verified Access endpoint.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_description()
    }
    /// <p>A unique, case-sensitive token that you provide to ensure idempotency of your modification request. For more information, see <a href="https://docs.aws.amazon.com/ec2/latest/devguide/ec2-api-idempotency.html">Ensuring idempotency</a>.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>A unique, case-sensitive token that you provide to ensure idempotency of your modification request. For more information, see <a href="https://docs.aws.amazon.com/ec2/latest/devguide/ec2-api-idempotency.html">Ensuring idempotency</a>.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>A unique, case-sensitive token that you provide to ensure idempotency of your modification request. For more information, see <a href="https://docs.aws.amazon.com/ec2/latest/devguide/ec2-api-idempotency.html">Ensuring idempotency</a>.</p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_token()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn get_dry_run(&self) -> &::std::option::Option<bool> {
        self.inner.get_dry_run()
    }
    /// <p>The RDS options.</p>
    pub fn rds_options(mut self, input: crate::types::ModifyVerifiedAccessEndpointRdsOptions) -> Self {
        self.inner = self.inner.rds_options(input);
        self
    }
    /// <p>The RDS options.</p>
    pub fn set_rds_options(mut self, input: ::std::option::Option<crate::types::ModifyVerifiedAccessEndpointRdsOptions>) -> Self {
        self.inner = self.inner.set_rds_options(input);
        self
    }
    /// <p>The RDS options.</p>
    pub fn get_rds_options(&self) -> &::std::option::Option<crate::types::ModifyVerifiedAccessEndpointRdsOptions> {
        self.inner.get_rds_options()
    }
    /// <p>The CIDR options.</p>
    pub fn cidr_options(mut self, input: crate::types::ModifyVerifiedAccessEndpointCidrOptions) -> Self {
        self.inner = self.inner.cidr_options(input);
        self
    }
    /// <p>The CIDR options.</p>
    pub fn set_cidr_options(mut self, input: ::std::option::Option<crate::types::ModifyVerifiedAccessEndpointCidrOptions>) -> Self {
        self.inner = self.inner.set_cidr_options(input);
        self
    }
    /// <p>The CIDR options.</p>
    pub fn get_cidr_options(&self) -> &::std::option::Option<crate::types::ModifyVerifiedAccessEndpointCidrOptions> {
        self.inner.get_cidr_options()
    }
}

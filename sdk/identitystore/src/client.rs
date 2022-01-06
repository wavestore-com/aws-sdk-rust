// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[derive(Debug)]
pub(crate) struct Handle<
    C = aws_smithy_client::erase::DynConnector,
    M = crate::middleware::DefaultMiddleware,
    R = aws_smithy_client::retry::Standard,
> {
    pub(crate) client: aws_smithy_client::Client<C, M, R>,
    pub(crate) conf: crate::Config,
}

/// Client for AWS SSO Identity Store
///
/// Client for invoking operations on AWS SSO Identity Store. Each operation on AWS SSO Identity Store is a method on this
/// this struct. `.send()` MUST be invoked on the generated operations to dispatch the request to the service.
///
/// # Examples
/// **Constructing a client and invoking an operation**
/// ```rust,no_run
/// # async fn docs() {
///     // create a shared configuration. This can be used & shared between multiple service clients.
///     let shared_config = aws_config::load_from_env().await;
///     let client = aws_sdk_identitystore::Client::new(&shared_config);
///     // invoke an operation
///     /* let rsp = client
///         .<operation_name>().
///         .<param>("some value")
///         .send().await; */
/// # }
/// ```
/// **Constructing a client with custom configuration**
/// ```rust,no_run
/// use aws_config::RetryConfig;
/// # async fn docs() {
///     let shared_config = aws_config::load_from_env().await;
///     let config = aws_sdk_identitystore::config::Builder::from(&shared_config)
///         .retry_config(RetryConfig::disabled())
///         .build();
///     let client = aws_sdk_identitystore::Client::from_conf(config);
/// # }
#[derive(std::fmt::Debug)]
pub struct Client<
    C = aws_smithy_client::erase::DynConnector,
    M = crate::middleware::DefaultMiddleware,
    R = aws_smithy_client::retry::Standard,
> {
    handle: std::sync::Arc<Handle<C, M, R>>,
}

impl<C, M, R> std::clone::Clone for Client<C, M, R> {
    fn clone(&self) -> Self {
        Self {
            handle: self.handle.clone(),
        }
    }
}

#[doc(inline)]
pub use aws_smithy_client::Builder;

impl<C, M, R> From<aws_smithy_client::Client<C, M, R>> for Client<C, M, R> {
    fn from(client: aws_smithy_client::Client<C, M, R>) -> Self {
        Self::with_config(client, crate::Config::builder().build())
    }
}

impl<C, M, R> Client<C, M, R> {
    /// Creates a client with the given service configuration.
    pub fn with_config(client: aws_smithy_client::Client<C, M, R>, conf: crate::Config) -> Self {
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }

    /// Returns the client's configuration.
    pub fn conf(&self) -> &crate::Config {
        &self.handle.conf
    }
}
impl<C, M, R> Client<C, M, R>
where
    C: aws_smithy_client::bounds::SmithyConnector,
    M: aws_smithy_client::bounds::SmithyMiddleware<C>,
    R: aws_smithy_client::retry::NewRequestPolicy,
{
    /// Constructs a fluent builder for the `DescribeGroup` operation.
    ///
    /// See [`DescribeGroup`](crate::client::fluent_builders::DescribeGroup) for more information about the
    /// operation and its arguments.
    pub fn describe_group(&self) -> fluent_builders::DescribeGroup<C, M, R> {
        fluent_builders::DescribeGroup::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the `DescribeUser` operation.
    ///
    /// See [`DescribeUser`](crate::client::fluent_builders::DescribeUser) for more information about the
    /// operation and its arguments.
    pub fn describe_user(&self) -> fluent_builders::DescribeUser<C, M, R> {
        fluent_builders::DescribeUser::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the `ListGroups` operation.
    ///
    /// See [`ListGroups`](crate::client::fluent_builders::ListGroups) for more information about the
    /// operation and its arguments.
    /// This operation supports pagination. See [`into_paginator()`](crate::client::fluent_builders::ListGroups::into_paginator).
    pub fn list_groups(&self) -> fluent_builders::ListGroups<C, M, R> {
        fluent_builders::ListGroups::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the `ListUsers` operation.
    ///
    /// See [`ListUsers`](crate::client::fluent_builders::ListUsers) for more information about the
    /// operation and its arguments.
    /// This operation supports pagination. See [`into_paginator()`](crate::client::fluent_builders::ListUsers::into_paginator).
    pub fn list_users(&self) -> fluent_builders::ListUsers<C, M, R> {
        fluent_builders::ListUsers::new(self.handle.clone())
    }
}
pub mod fluent_builders {
    //!
    //! Utilities to ergonomically construct a request to the service.
    //!
    //! Fluent builders are created through the [`Client`](crate::client::Client) by calling
    //! one if its operation methods. After parameters are set using the builder methods,
    //! the `send` method can be called to initiate the request.
    //!
    /// Fluent builder constructing a request to `DescribeGroup`.
    ///
    /// <p>Retrieves the group metadata and attributes from <code>GroupId</code> in an identity store.</p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct DescribeGroup<
        C = aws_smithy_client::erase::DynConnector,
        M = crate::middleware::DefaultMiddleware,
        R = aws_smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::describe_group_input::Builder,
    }
    impl<C, M, R> DescribeGroup<C, M, R>
    where
        C: aws_smithy_client::bounds::SmithyConnector,
        M: aws_smithy_client::bounds::SmithyMiddleware<C>,
        R: aws_smithy_client::retry::NewRequestPolicy,
    {
        /// Creates a new `DescribeGroup`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
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
        ) -> std::result::Result<
            crate::output::DescribeGroupOutput,
            aws_smithy_http::result::SdkError<crate::error::DescribeGroupError>,
        >
        where
            R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
                crate::input::DescribeGroupInputOperationOutputAlias,
                crate::output::DescribeGroupOutput,
                crate::error::DescribeGroupError,
                crate::input::DescribeGroupInputOperationRetryAlias,
            >,
        {
            let op = self
                .inner
                .build()
                .map_err(|err| aws_smithy_http::result::SdkError::ConstructionFailure(err.into()))?
                .make_operation(&self.handle.conf)
                .await
                .map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                })?;
            self.handle.client.call(op).await
        }
        /// <p>The globally unique identifier for the identity store, such as <code>d-1234567890</code>. In this example, <code>d-</code> is a fixed prefix, and <code>1234567890</code> is a randomly generated string that contains number and lower case letters. This value is generated at the time that a new identity store is created.</p>
        pub fn identity_store_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.identity_store_id(input.into());
            self
        }
        /// <p>The globally unique identifier for the identity store, such as <code>d-1234567890</code>. In this example, <code>d-</code> is a fixed prefix, and <code>1234567890</code> is a randomly generated string that contains number and lower case letters. This value is generated at the time that a new identity store is created.</p>
        pub fn set_identity_store_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_identity_store_id(input);
            self
        }
        /// <p>The identifier for a group in the identity store.</p>
        pub fn group_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.group_id(input.into());
            self
        }
        /// <p>The identifier for a group in the identity store.</p>
        pub fn set_group_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_group_id(input);
            self
        }
    }
    /// Fluent builder constructing a request to `DescribeUser`.
    ///
    /// <p>Retrieves the user metadata and attributes from <code>UserId</code> in an identity store.</p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct DescribeUser<
        C = aws_smithy_client::erase::DynConnector,
        M = crate::middleware::DefaultMiddleware,
        R = aws_smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::describe_user_input::Builder,
    }
    impl<C, M, R> DescribeUser<C, M, R>
    where
        C: aws_smithy_client::bounds::SmithyConnector,
        M: aws_smithy_client::bounds::SmithyMiddleware<C>,
        R: aws_smithy_client::retry::NewRequestPolicy,
    {
        /// Creates a new `DescribeUser`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
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
        ) -> std::result::Result<
            crate::output::DescribeUserOutput,
            aws_smithy_http::result::SdkError<crate::error::DescribeUserError>,
        >
        where
            R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
                crate::input::DescribeUserInputOperationOutputAlias,
                crate::output::DescribeUserOutput,
                crate::error::DescribeUserError,
                crate::input::DescribeUserInputOperationRetryAlias,
            >,
        {
            let op = self
                .inner
                .build()
                .map_err(|err| aws_smithy_http::result::SdkError::ConstructionFailure(err.into()))?
                .make_operation(&self.handle.conf)
                .await
                .map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                })?;
            self.handle.client.call(op).await
        }
        /// <p>The globally unique identifier for the identity store, such as <code>d-1234567890</code>. In this example, <code>d-</code> is a fixed prefix, and <code>1234567890</code> is a randomly generated string that contains number and lower case letters. This value is generated at the time that a new identity store is created.</p>
        pub fn identity_store_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.identity_store_id(input.into());
            self
        }
        /// <p>The globally unique identifier for the identity store, such as <code>d-1234567890</code>. In this example, <code>d-</code> is a fixed prefix, and <code>1234567890</code> is a randomly generated string that contains number and lower case letters. This value is generated at the time that a new identity store is created.</p>
        pub fn set_identity_store_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_identity_store_id(input);
            self
        }
        /// <p>The identifier for a user in the identity store.</p>
        pub fn user_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.user_id(input.into());
            self
        }
        /// <p>The identifier for a user in the identity store.</p>
        pub fn set_user_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_user_id(input);
            self
        }
    }
    /// Fluent builder constructing a request to `ListGroups`.
    ///
    /// <p>Lists the attribute name and value of the group that you specified in the search. We only support <code>DisplayName</code> as a valid filter attribute path currently, and filter is required. This API returns minimum attributes, including <code>GroupId</code> and group <code>DisplayName</code> in the response.</p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct ListGroups<
        C = aws_smithy_client::erase::DynConnector,
        M = crate::middleware::DefaultMiddleware,
        R = aws_smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::list_groups_input::Builder,
    }
    impl<C, M, R> ListGroups<C, M, R>
    where
        C: aws_smithy_client::bounds::SmithyConnector,
        M: aws_smithy_client::bounds::SmithyMiddleware<C>,
        R: aws_smithy_client::retry::NewRequestPolicy,
    {
        /// Creates a new `ListGroups`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
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
        ) -> std::result::Result<
            crate::output::ListGroupsOutput,
            aws_smithy_http::result::SdkError<crate::error::ListGroupsError>,
        >
        where
            R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
                crate::input::ListGroupsInputOperationOutputAlias,
                crate::output::ListGroupsOutput,
                crate::error::ListGroupsError,
                crate::input::ListGroupsInputOperationRetryAlias,
            >,
        {
            let op = self
                .inner
                .build()
                .map_err(|err| aws_smithy_http::result::SdkError::ConstructionFailure(err.into()))?
                .make_operation(&self.handle.conf)
                .await
                .map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                })?;
            self.handle.client.call(op).await
        }
        /// Create a paginator for this request
        ///
        /// Paginators are used by calling [`send().await`](crate::paginator::ListGroupsPaginator::send) which returns a [`Stream`](tokio_stream::Stream).
        pub fn into_paginator(self) -> crate::paginator::ListGroupsPaginator<C, M, R> {
            crate::paginator::ListGroupsPaginator::new(self.handle, self.inner)
        }
        /// <p>The globally unique identifier for the identity store, such as <code>d-1234567890</code>. In this example, <code>d-</code> is a fixed prefix, and <code>1234567890</code> is a randomly generated string that contains number and lower case letters. This value is generated at the time that a new identity store is created.</p>
        pub fn identity_store_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.identity_store_id(input.into());
            self
        }
        /// <p>The globally unique identifier for the identity store, such as <code>d-1234567890</code>. In this example, <code>d-</code> is a fixed prefix, and <code>1234567890</code> is a randomly generated string that contains number and lower case letters. This value is generated at the time that a new identity store is created.</p>
        pub fn set_identity_store_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_identity_store_id(input);
            self
        }
        /// <p>The maximum number of results to be returned per request. This parameter is used in the <code>ListUsers</code> and <code>ListGroups</code> request to specify how many results to return in one page. The length limit is 50 characters.</p>
        pub fn max_results(mut self, input: i32) -> Self {
            self.inner = self.inner.max_results(input);
            self
        }
        /// <p>The maximum number of results to be returned per request. This parameter is used in the <code>ListUsers</code> and <code>ListGroups</code> request to specify how many results to return in one page. The length limit is 50 characters.</p>
        pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_max_results(input);
            self
        }
        /// <p>The pagination token used for the <code>ListUsers</code> and <code>ListGroups</code> API operations. This value is generated by the identity store service. It is returned in the API response if the total results are more than the size of one page. This token is also returned when it is used in the API request to search for the next page.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.next_token(input.into());
            self
        }
        /// <p>The pagination token used for the <code>ListUsers</code> and <code>ListGroups</code> API operations. This value is generated by the identity store service. It is returned in the API response if the total results are more than the size of one page. This token is also returned when it is used in the API request to search for the next page.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_next_token(input);
            self
        }
        /// Appends an item to `Filters`.
        ///
        /// To override the contents of this collection use [`set_filters`](Self::set_filters).
        ///
        /// <p>A list of <code>Filter</code> objects, which is used in the <code>ListUsers</code> and <code>ListGroups</code> request. </p>
        pub fn filters(mut self, input: crate::model::Filter) -> Self {
            self.inner = self.inner.filters(input);
            self
        }
        /// <p>A list of <code>Filter</code> objects, which is used in the <code>ListUsers</code> and <code>ListGroups</code> request. </p>
        pub fn set_filters(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::Filter>>,
        ) -> Self {
            self.inner = self.inner.set_filters(input);
            self
        }
    }
    /// Fluent builder constructing a request to `ListUsers`.
    ///
    /// <p>Lists the attribute name and value of the user that you specified in the search. We only support <code>UserName</code> as a valid filter attribute path currently, and filter is required. This API returns minimum attributes, including <code>UserId</code> and <code>UserName</code> in the response.</p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct ListUsers<
        C = aws_smithy_client::erase::DynConnector,
        M = crate::middleware::DefaultMiddleware,
        R = aws_smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::list_users_input::Builder,
    }
    impl<C, M, R> ListUsers<C, M, R>
    where
        C: aws_smithy_client::bounds::SmithyConnector,
        M: aws_smithy_client::bounds::SmithyMiddleware<C>,
        R: aws_smithy_client::retry::NewRequestPolicy,
    {
        /// Creates a new `ListUsers`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
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
        ) -> std::result::Result<
            crate::output::ListUsersOutput,
            aws_smithy_http::result::SdkError<crate::error::ListUsersError>,
        >
        where
            R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
                crate::input::ListUsersInputOperationOutputAlias,
                crate::output::ListUsersOutput,
                crate::error::ListUsersError,
                crate::input::ListUsersInputOperationRetryAlias,
            >,
        {
            let op = self
                .inner
                .build()
                .map_err(|err| aws_smithy_http::result::SdkError::ConstructionFailure(err.into()))?
                .make_operation(&self.handle.conf)
                .await
                .map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                })?;
            self.handle.client.call(op).await
        }
        /// Create a paginator for this request
        ///
        /// Paginators are used by calling [`send().await`](crate::paginator::ListUsersPaginator::send) which returns a [`Stream`](tokio_stream::Stream).
        pub fn into_paginator(self) -> crate::paginator::ListUsersPaginator<C, M, R> {
            crate::paginator::ListUsersPaginator::new(self.handle, self.inner)
        }
        /// <p>The globally unique identifier for the identity store, such as <code>d-1234567890</code>. In this example, <code>d-</code> is a fixed prefix, and <code>1234567890</code> is a randomly generated string that contains number and lower case letters. This value is generated at the time that a new identity store is created.</p>
        pub fn identity_store_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.identity_store_id(input.into());
            self
        }
        /// <p>The globally unique identifier for the identity store, such as <code>d-1234567890</code>. In this example, <code>d-</code> is a fixed prefix, and <code>1234567890</code> is a randomly generated string that contains number and lower case letters. This value is generated at the time that a new identity store is created.</p>
        pub fn set_identity_store_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_identity_store_id(input);
            self
        }
        /// <p>The maximum number of results to be returned per request. This parameter is used in the <code>ListUsers</code> and <code>ListGroups</code> request to specify how many results to return in one page. The length limit is 50 characters.</p>
        pub fn max_results(mut self, input: i32) -> Self {
            self.inner = self.inner.max_results(input);
            self
        }
        /// <p>The maximum number of results to be returned per request. This parameter is used in the <code>ListUsers</code> and <code>ListGroups</code> request to specify how many results to return in one page. The length limit is 50 characters.</p>
        pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_max_results(input);
            self
        }
        /// <p>The pagination token used for the <code>ListUsers</code> and <code>ListGroups</code> API operations. This value is generated by the identity store service. It is returned in the API response if the total results are more than the size of one page. This token is also returned when it is used in the API request to search for the next page.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.next_token(input.into());
            self
        }
        /// <p>The pagination token used for the <code>ListUsers</code> and <code>ListGroups</code> API operations. This value is generated by the identity store service. It is returned in the API response if the total results are more than the size of one page. This token is also returned when it is used in the API request to search for the next page.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_next_token(input);
            self
        }
        /// Appends an item to `Filters`.
        ///
        /// To override the contents of this collection use [`set_filters`](Self::set_filters).
        ///
        /// <p>A list of <code>Filter</code> objects, which is used in the <code>ListUsers</code> and <code>ListGroups</code> request. </p>
        pub fn filters(mut self, input: crate::model::Filter) -> Self {
            self.inner = self.inner.filters(input);
            self
        }
        /// <p>A list of <code>Filter</code> objects, which is used in the <code>ListUsers</code> and <code>ListGroups</code> request. </p>
        pub fn set_filters(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::Filter>>,
        ) -> Self {
            self.inner = self.inner.set_filters(input);
            self
        }
    }
}

impl<C> Client<C, crate::middleware::DefaultMiddleware, aws_smithy_client::retry::Standard> {
    /// Creates a client with the given service config and connector override.
    pub fn from_conf_conn(conf: crate::Config, conn: C) -> Self {
        let retry_config = conf.retry_config.as_ref().cloned().unwrap_or_default();
        let timeout_config = conf.timeout_config.as_ref().cloned().unwrap_or_default();
        let sleep_impl = conf.sleep_impl.clone();
        let mut builder = aws_smithy_client::Builder::new()
            .connector(conn)
            .middleware(crate::middleware::DefaultMiddleware::new());
        builder.set_retry_config(retry_config.into());
        builder.set_timeout_config(timeout_config);
        if let Some(sleep_impl) = sleep_impl {
            builder.set_sleep_impl(Some(sleep_impl));
        }
        let client = builder.build();
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }
}
impl
    Client<
        aws_smithy_client::erase::DynConnector,
        crate::middleware::DefaultMiddleware,
        aws_smithy_client::retry::Standard,
    >
{
    /// Creates a new client from a shared config.
    #[cfg(any(feature = "rustls", feature = "native-tls"))]
    pub fn new(config: &aws_types::config::Config) -> Self {
        Self::from_conf(config.into())
    }

    /// Creates a new client from the service [`Config`](crate::Config).
    #[cfg(any(feature = "rustls", feature = "native-tls"))]
    pub fn from_conf(conf: crate::Config) -> Self {
        let retry_config = conf.retry_config.as_ref().cloned().unwrap_or_default();
        let timeout_config = conf.timeout_config.as_ref().cloned().unwrap_or_default();
        let sleep_impl = conf.sleep_impl.clone();
        let mut builder = aws_smithy_client::Builder::dyn_https()
            .middleware(crate::middleware::DefaultMiddleware::new());
        builder.set_retry_config(retry_config.into());
        builder.set_timeout_config(timeout_config);
        // the builder maintains a try-state. To avoid suppressing the warning when sleep is unset,
        // only set it if we actually have a sleep impl.
        if let Some(sleep_impl) = sleep_impl {
            builder.set_sleep_impl(Some(sleep_impl));
        }
        let client = builder.build();

        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }
}

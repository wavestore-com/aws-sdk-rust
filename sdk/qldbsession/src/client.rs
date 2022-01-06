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

/// Client for Amazon QLDB Session
///
/// Client for invoking operations on Amazon QLDB Session. Each operation on Amazon QLDB Session is a method on this
/// this struct. `.send()` MUST be invoked on the generated operations to dispatch the request to the service.
///
/// # Examples
/// **Constructing a client and invoking an operation**
/// ```rust,no_run
/// # async fn docs() {
///     // create a shared configuration. This can be used & shared between multiple service clients.
///     let shared_config = aws_config::load_from_env().await;
///     let client = aws_sdk_qldbsession::Client::new(&shared_config);
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
///     let config = aws_sdk_qldbsession::config::Builder::from(&shared_config)
///         .retry_config(RetryConfig::disabled())
///         .build();
///     let client = aws_sdk_qldbsession::Client::from_conf(config);
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
    /// Constructs a fluent builder for the `SendCommand` operation.
    ///
    /// See [`SendCommand`](crate::client::fluent_builders::SendCommand) for more information about the
    /// operation and its arguments.
    pub fn send_command(&self) -> fluent_builders::SendCommand<C, M, R> {
        fluent_builders::SendCommand::new(self.handle.clone())
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
    /// Fluent builder constructing a request to `SendCommand`.
    ///
    /// <p>Sends a command to an Amazon QLDB ledger.</p> <note>
    /// <p>Instead of interacting directly with this API, we recommend using the QLDB driver or the QLDB shell to execute data transactions on a ledger.</p>
    /// <ul>
    /// <li> <p>If you are working with an AWS SDK, use the QLDB driver. The driver provides a high-level abstraction layer above this <i>QLDB Session</i> data plane and manages <code>SendCommand</code> API calls for you. For information and a list of supported programming languages, see <a href="https://docs.aws.amazon.com/qldb/latest/developerguide/getting-started-driver.html">Getting started with the driver</a> in the <i>Amazon QLDB Developer Guide</i>.</p> </li>
    /// <li> <p>If you are working with the AWS Command Line Interface (AWS CLI), use the QLDB shell. The shell is a command line interface that uses the QLDB driver to interact with a ledger. For information, see <a href="https://docs.aws.amazon.com/qldb/latest/developerguide/data-shell.html">Accessing Amazon QLDB using the QLDB shell</a>.</p> </li>
    /// </ul>
    /// </note>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct SendCommand<
        C = aws_smithy_client::erase::DynConnector,
        M = crate::middleware::DefaultMiddleware,
        R = aws_smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::send_command_input::Builder,
    }
    impl<C, M, R> SendCommand<C, M, R>
    where
        C: aws_smithy_client::bounds::SmithyConnector,
        M: aws_smithy_client::bounds::SmithyMiddleware<C>,
        R: aws_smithy_client::retry::NewRequestPolicy,
    {
        /// Creates a new `SendCommand`.
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
            crate::output::SendCommandOutput,
            aws_smithy_http::result::SdkError<crate::error::SendCommandError>,
        >
        where
            R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
                crate::input::SendCommandInputOperationOutputAlias,
                crate::output::SendCommandOutput,
                crate::error::SendCommandError,
                crate::input::SendCommandInputOperationRetryAlias,
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
        /// <p>Specifies the session token for the current command. A session token is constant throughout the life of the session.</p>
        /// <p>To obtain a session token, run the <code>StartSession</code> command. This <code>SessionToken</code> is required for every subsequent command that is issued during the current session.</p>
        pub fn session_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.session_token(input.into());
            self
        }
        /// <p>Specifies the session token for the current command. A session token is constant throughout the life of the session.</p>
        /// <p>To obtain a session token, run the <code>StartSession</code> command. This <code>SessionToken</code> is required for every subsequent command that is issued during the current session.</p>
        pub fn set_session_token(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_session_token(input);
            self
        }
        /// <p>Command to start a new session. A session token is obtained as part of the response.</p>
        pub fn start_session(mut self, input: crate::model::StartSessionRequest) -> Self {
            self.inner = self.inner.start_session(input);
            self
        }
        /// <p>Command to start a new session. A session token is obtained as part of the response.</p>
        pub fn set_start_session(
            mut self,
            input: std::option::Option<crate::model::StartSessionRequest>,
        ) -> Self {
            self.inner = self.inner.set_start_session(input);
            self
        }
        /// <p>Command to start a new transaction.</p>
        pub fn start_transaction(mut self, input: crate::model::StartTransactionRequest) -> Self {
            self.inner = self.inner.start_transaction(input);
            self
        }
        /// <p>Command to start a new transaction.</p>
        pub fn set_start_transaction(
            mut self,
            input: std::option::Option<crate::model::StartTransactionRequest>,
        ) -> Self {
            self.inner = self.inner.set_start_transaction(input);
            self
        }
        /// <p>Command to end the current session.</p>
        pub fn end_session(mut self, input: crate::model::EndSessionRequest) -> Self {
            self.inner = self.inner.end_session(input);
            self
        }
        /// <p>Command to end the current session.</p>
        pub fn set_end_session(
            mut self,
            input: std::option::Option<crate::model::EndSessionRequest>,
        ) -> Self {
            self.inner = self.inner.set_end_session(input);
            self
        }
        /// <p>Command to commit the specified transaction.</p>
        pub fn commit_transaction(mut self, input: crate::model::CommitTransactionRequest) -> Self {
            self.inner = self.inner.commit_transaction(input);
            self
        }
        /// <p>Command to commit the specified transaction.</p>
        pub fn set_commit_transaction(
            mut self,
            input: std::option::Option<crate::model::CommitTransactionRequest>,
        ) -> Self {
            self.inner = self.inner.set_commit_transaction(input);
            self
        }
        /// <p>Command to abort the current transaction.</p>
        pub fn abort_transaction(mut self, input: crate::model::AbortTransactionRequest) -> Self {
            self.inner = self.inner.abort_transaction(input);
            self
        }
        /// <p>Command to abort the current transaction.</p>
        pub fn set_abort_transaction(
            mut self,
            input: std::option::Option<crate::model::AbortTransactionRequest>,
        ) -> Self {
            self.inner = self.inner.set_abort_transaction(input);
            self
        }
        /// <p>Command to execute a statement in the specified transaction.</p>
        pub fn execute_statement(mut self, input: crate::model::ExecuteStatementRequest) -> Self {
            self.inner = self.inner.execute_statement(input);
            self
        }
        /// <p>Command to execute a statement in the specified transaction.</p>
        pub fn set_execute_statement(
            mut self,
            input: std::option::Option<crate::model::ExecuteStatementRequest>,
        ) -> Self {
            self.inner = self.inner.set_execute_statement(input);
            self
        }
        /// <p>Command to fetch a page.</p>
        pub fn fetch_page(mut self, input: crate::model::FetchPageRequest) -> Self {
            self.inner = self.inner.fetch_page(input);
            self
        }
        /// <p>Command to fetch a page.</p>
        pub fn set_fetch_page(
            mut self,
            input: std::option::Option<crate::model::FetchPageRequest>,
        ) -> Self {
            self.inner = self.inner.set_fetch_page(input);
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

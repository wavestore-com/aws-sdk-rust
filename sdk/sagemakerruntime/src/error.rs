// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Error type for the `InvokeEndpointAsync` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct InvokeEndpointAsyncError {
    /// Kind of error that occurred.
                    pub kind: InvokeEndpointAsyncErrorKind,
                    /// Additional metadata about the error, including error code, message, and request ID.
                    pub (crate) meta: aws_smithy_types::Error
}
impl aws_smithy_http::result::CreateUnhandledError for InvokeEndpointAsyncError {
    fn create_unhandled_error(source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        Self {
            kind: InvokeEndpointAsyncErrorKind::Unhandled(crate::error::Unhandled::new(source)),
            meta: Default::default()
        }
    }
}
/// Types of errors that can occur for the `InvokeEndpointAsync` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum InvokeEndpointAsyncErrorKind {
    /// <p> An internal failure occurred. </p>
    InternalFailure(crate::error::InternalFailure),
    /// <p> The service is unavailable. Try your call again. </p>
    ServiceUnavailable(crate::error::ServiceUnavailable),
    /// <p> Inspect your request and try again. </p>
    ValidationError(crate::error::ValidationError),
    /// 
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    /// 
    /// When logging an error from the SDK, it is recommended that you either wrap the error in
    /// [`DisplayErrorContext`](crate::types::DisplayErrorContext), use another
    /// error reporter library that visits the error's cause/source chain, or call
    /// [`Error::source`](std::error::Error::source) for more details about the underlying cause.
    /// 
    Unhandled(crate::error::Unhandled),
}
impl std::fmt::Display for InvokeEndpointAsyncError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            InvokeEndpointAsyncErrorKind::InternalFailure(_inner) =>
            _inner.fmt(f)
            ,
            InvokeEndpointAsyncErrorKind::ServiceUnavailable(_inner) =>
            _inner.fmt(f)
            ,
            InvokeEndpointAsyncErrorKind::ValidationError(_inner) =>
            _inner.fmt(f)
            ,
            InvokeEndpointAsyncErrorKind::Unhandled(_inner) => {
                _inner.fmt(f)
            }
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for InvokeEndpointAsyncError {
    fn code(&self) -> Option<&str> {
        InvokeEndpointAsyncError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl InvokeEndpointAsyncError {
    /// Creates a new `InvokeEndpointAsyncError`.
                    pub fn new(kind: InvokeEndpointAsyncErrorKind, meta: aws_smithy_types::Error) -> Self {
                        Self { kind, meta }
                    }
    
                    /// Creates the `InvokeEndpointAsyncError::Unhandled` variant from any error type.
                    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
                        Self {
                            kind: InvokeEndpointAsyncErrorKind::Unhandled(crate::error::Unhandled::new(err.into())),
                            meta: Default::default()
                        }
                    }
    
                    /// Creates the `InvokeEndpointAsyncError::Unhandled` variant from a `aws_smithy_types::Error`.
                    pub fn generic(err: aws_smithy_types::Error) -> Self {
                        Self {
                            meta: err.clone(),
                            kind: InvokeEndpointAsyncErrorKind::Unhandled(crate::error::Unhandled::new(err.into())),
                        }
                    }
    
                    /// Returns the error message if one is available.
                    pub fn message(&self) -> Option<&str> {
                        self.meta.message()
                    }
    
                    /// Returns error metadata, which includes the error code, message,
                    /// request ID, and potentially additional information.
                    pub fn meta(&self) -> &aws_smithy_types::Error {
                        &self.meta
                    }
    
                    /// Returns the request ID if it's available.
                    pub fn request_id(&self) -> Option<&str> {
                        self.meta.request_id()
                    }
    
                    /// Returns the error code if it's available.
                    pub fn code(&self) -> Option<&str> {
                        self.meta.code()
                    }
    /// Returns `true` if the error kind is `InvokeEndpointAsyncErrorKind::InternalFailure`.
    pub fn is_internal_failure(&self) -> bool {
        matches!(&self.kind, InvokeEndpointAsyncErrorKind::InternalFailure(_))
    }
    /// Returns `true` if the error kind is `InvokeEndpointAsyncErrorKind::ServiceUnavailable`.
    pub fn is_service_unavailable(&self) -> bool {
        matches!(&self.kind, InvokeEndpointAsyncErrorKind::ServiceUnavailable(_))
    }
    /// Returns `true` if the error kind is `InvokeEndpointAsyncErrorKind::ValidationError`.
    pub fn is_validation_error(&self) -> bool {
        matches!(&self.kind, InvokeEndpointAsyncErrorKind::ValidationError(_))
    }
}
impl std::error::Error for InvokeEndpointAsyncError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            InvokeEndpointAsyncErrorKind::InternalFailure(_inner) =>
            Some(_inner)
            ,
            InvokeEndpointAsyncErrorKind::ServiceUnavailable(_inner) =>
            Some(_inner)
            ,
            InvokeEndpointAsyncErrorKind::ValidationError(_inner) =>
            Some(_inner)
            ,
            InvokeEndpointAsyncErrorKind::Unhandled(_inner) => {
                Some(_inner)
            }
        }
    }
}

/// <p> Inspect your request and try again. </p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ValidationError  {
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
}
impl ValidationError {
    /// Returns the error message.
                        pub fn message(&self) -> std::option::Option<& str> { self.message.as_deref() }
}
impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ValidationError")?;
        if let Some(inner_1) = &self.message {
             {
                write!(f, ": {}", inner_1)?;
            }
        }
        Ok(())
    }
}
impl std::error::Error for ValidationError {}
/// See [`ValidationError`](crate::error::ValidationError).
pub mod validation_error {
    
    /// A builder for [`ValidationError`](crate::error::ValidationError).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input; self
        }
        /// Consumes the builder and constructs a [`ValidationError`](crate::error::ValidationError).
        pub fn build(self) -> crate::error::ValidationError {
            crate::error::ValidationError {
                message: self.message
                ,
            }
        }
    }
    
    
}
impl ValidationError {
    /// Creates a new builder-style object to manufacture [`ValidationError`](crate::error::ValidationError).
    pub fn builder() -> crate::error::validation_error::Builder {
        crate::error::validation_error::Builder::default()
    }
}

/// <p> The service is unavailable. Try your call again. </p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ServiceUnavailable  {
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
}
impl ServiceUnavailable {
    /// Returns the error message.
                        pub fn message(&self) -> std::option::Option<& str> { self.message.as_deref() }
}
impl std::fmt::Display for ServiceUnavailable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ServiceUnavailable")?;
        if let Some(inner_2) = &self.message {
             {
                write!(f, ": {}", inner_2)?;
            }
        }
        Ok(())
    }
}
impl std::error::Error for ServiceUnavailable {}
/// See [`ServiceUnavailable`](crate::error::ServiceUnavailable).
pub mod service_unavailable {
    
    /// A builder for [`ServiceUnavailable`](crate::error::ServiceUnavailable).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input; self
        }
        /// Consumes the builder and constructs a [`ServiceUnavailable`](crate::error::ServiceUnavailable).
        pub fn build(self) -> crate::error::ServiceUnavailable {
            crate::error::ServiceUnavailable {
                message: self.message
                ,
            }
        }
    }
    
    
}
impl ServiceUnavailable {
    /// Creates a new builder-style object to manufacture [`ServiceUnavailable`](crate::error::ServiceUnavailable).
    pub fn builder() -> crate::error::service_unavailable::Builder {
        crate::error::service_unavailable::Builder::default()
    }
}

/// <p> An internal failure occurred. </p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct InternalFailure  {
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
}
impl InternalFailure {
    /// Returns the error message.
                        pub fn message(&self) -> std::option::Option<& str> { self.message.as_deref() }
}
impl std::fmt::Display for InternalFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InternalFailure")?;
        if let Some(inner_3) = &self.message {
             {
                write!(f, ": {}", inner_3)?;
            }
        }
        Ok(())
    }
}
impl std::error::Error for InternalFailure {}
/// See [`InternalFailure`](crate::error::InternalFailure).
pub mod internal_failure {
    
    /// A builder for [`InternalFailure`](crate::error::InternalFailure).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input; self
        }
        /// Consumes the builder and constructs a [`InternalFailure`](crate::error::InternalFailure).
        pub fn build(self) -> crate::error::InternalFailure {
            crate::error::InternalFailure {
                message: self.message
                ,
            }
        }
    }
    
    
}
impl InternalFailure {
    /// Creates a new builder-style object to manufacture [`InternalFailure`](crate::error::InternalFailure).
    pub fn builder() -> crate::error::internal_failure::Builder {
        crate::error::internal_failure::Builder::default()
    }
}

/// Error type for the `InvokeEndpoint` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct InvokeEndpointError {
    /// Kind of error that occurred.
                    pub kind: InvokeEndpointErrorKind,
                    /// Additional metadata about the error, including error code, message, and request ID.
                    pub (crate) meta: aws_smithy_types::Error
}
impl aws_smithy_http::result::CreateUnhandledError for InvokeEndpointError {
    fn create_unhandled_error(source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        Self {
            kind: InvokeEndpointErrorKind::Unhandled(crate::error::Unhandled::new(source)),
            meta: Default::default()
        }
    }
}
/// Types of errors that can occur for the `InvokeEndpoint` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum InvokeEndpointErrorKind {
    /// <p>Your request caused an exception with an internal dependency. Contact customer support. </p>
    InternalDependencyException(crate::error::InternalDependencyException),
    /// <p> An internal failure occurred. </p>
    InternalFailure(crate::error::InternalFailure),
    /// <p> Model (owned by the customer in the container) returned 4xx or 5xx error code. </p>
    ModelError(crate::error::ModelError),
    /// <p>Either a serverless endpoint variant's resources are still being provisioned, or a multi-model endpoint is still downloading or loading the target model. Wait and try your request again.</p>
    ModelNotReadyException(crate::error::ModelNotReadyException),
    /// <p> The service is unavailable. Try your call again. </p>
    ServiceUnavailable(crate::error::ServiceUnavailable),
    /// <p> Inspect your request and try again. </p>
    ValidationError(crate::error::ValidationError),
    /// 
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    /// 
    /// When logging an error from the SDK, it is recommended that you either wrap the error in
    /// [`DisplayErrorContext`](crate::types::DisplayErrorContext), use another
    /// error reporter library that visits the error's cause/source chain, or call
    /// [`Error::source`](std::error::Error::source) for more details about the underlying cause.
    /// 
    Unhandled(crate::error::Unhandled),
}
impl std::fmt::Display for InvokeEndpointError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            InvokeEndpointErrorKind::InternalDependencyException(_inner) =>
            _inner.fmt(f)
            ,
            InvokeEndpointErrorKind::InternalFailure(_inner) =>
            _inner.fmt(f)
            ,
            InvokeEndpointErrorKind::ModelError(_inner) =>
            _inner.fmt(f)
            ,
            InvokeEndpointErrorKind::ModelNotReadyException(_inner) =>
            _inner.fmt(f)
            ,
            InvokeEndpointErrorKind::ServiceUnavailable(_inner) =>
            _inner.fmt(f)
            ,
            InvokeEndpointErrorKind::ValidationError(_inner) =>
            _inner.fmt(f)
            ,
            InvokeEndpointErrorKind::Unhandled(_inner) => {
                _inner.fmt(f)
            }
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for InvokeEndpointError {
    fn code(&self) -> Option<&str> {
        InvokeEndpointError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl InvokeEndpointError {
    /// Creates a new `InvokeEndpointError`.
                    pub fn new(kind: InvokeEndpointErrorKind, meta: aws_smithy_types::Error) -> Self {
                        Self { kind, meta }
                    }
    
                    /// Creates the `InvokeEndpointError::Unhandled` variant from any error type.
                    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
                        Self {
                            kind: InvokeEndpointErrorKind::Unhandled(crate::error::Unhandled::new(err.into())),
                            meta: Default::default()
                        }
                    }
    
                    /// Creates the `InvokeEndpointError::Unhandled` variant from a `aws_smithy_types::Error`.
                    pub fn generic(err: aws_smithy_types::Error) -> Self {
                        Self {
                            meta: err.clone(),
                            kind: InvokeEndpointErrorKind::Unhandled(crate::error::Unhandled::new(err.into())),
                        }
                    }
    
                    /// Returns the error message if one is available.
                    pub fn message(&self) -> Option<&str> {
                        self.meta.message()
                    }
    
                    /// Returns error metadata, which includes the error code, message,
                    /// request ID, and potentially additional information.
                    pub fn meta(&self) -> &aws_smithy_types::Error {
                        &self.meta
                    }
    
                    /// Returns the request ID if it's available.
                    pub fn request_id(&self) -> Option<&str> {
                        self.meta.request_id()
                    }
    
                    /// Returns the error code if it's available.
                    pub fn code(&self) -> Option<&str> {
                        self.meta.code()
                    }
    /// Returns `true` if the error kind is `InvokeEndpointErrorKind::InternalDependencyException`.
    pub fn is_internal_dependency_exception(&self) -> bool {
        matches!(&self.kind, InvokeEndpointErrorKind::InternalDependencyException(_))
    }
    /// Returns `true` if the error kind is `InvokeEndpointErrorKind::InternalFailure`.
    pub fn is_internal_failure(&self) -> bool {
        matches!(&self.kind, InvokeEndpointErrorKind::InternalFailure(_))
    }
    /// Returns `true` if the error kind is `InvokeEndpointErrorKind::ModelError`.
    pub fn is_model_error(&self) -> bool {
        matches!(&self.kind, InvokeEndpointErrorKind::ModelError(_))
    }
    /// Returns `true` if the error kind is `InvokeEndpointErrorKind::ModelNotReadyException`.
    pub fn is_model_not_ready_exception(&self) -> bool {
        matches!(&self.kind, InvokeEndpointErrorKind::ModelNotReadyException(_))
    }
    /// Returns `true` if the error kind is `InvokeEndpointErrorKind::ServiceUnavailable`.
    pub fn is_service_unavailable(&self) -> bool {
        matches!(&self.kind, InvokeEndpointErrorKind::ServiceUnavailable(_))
    }
    /// Returns `true` if the error kind is `InvokeEndpointErrorKind::ValidationError`.
    pub fn is_validation_error(&self) -> bool {
        matches!(&self.kind, InvokeEndpointErrorKind::ValidationError(_))
    }
}
impl std::error::Error for InvokeEndpointError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            InvokeEndpointErrorKind::InternalDependencyException(_inner) =>
            Some(_inner)
            ,
            InvokeEndpointErrorKind::InternalFailure(_inner) =>
            Some(_inner)
            ,
            InvokeEndpointErrorKind::ModelError(_inner) =>
            Some(_inner)
            ,
            InvokeEndpointErrorKind::ModelNotReadyException(_inner) =>
            Some(_inner)
            ,
            InvokeEndpointErrorKind::ServiceUnavailable(_inner) =>
            Some(_inner)
            ,
            InvokeEndpointErrorKind::ValidationError(_inner) =>
            Some(_inner)
            ,
            InvokeEndpointErrorKind::Unhandled(_inner) => {
                Some(_inner)
            }
        }
    }
}

/// <p>Either a serverless endpoint variant's resources are still being provisioned, or a multi-model endpoint is still downloading or loading the target model. Wait and try your request again.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ModelNotReadyException  {
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
}
impl ModelNotReadyException {
    /// Returns the error message.
                        pub fn message(&self) -> std::option::Option<& str> { self.message.as_deref() }
}
impl std::fmt::Display for ModelNotReadyException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ModelNotReadyException")?;
        if let Some(inner_4) = &self.message {
             {
                write!(f, ": {}", inner_4)?;
            }
        }
        Ok(())
    }
}
impl std::error::Error for ModelNotReadyException {}
/// See [`ModelNotReadyException`](crate::error::ModelNotReadyException).
pub mod model_not_ready_exception {
    
    /// A builder for [`ModelNotReadyException`](crate::error::ModelNotReadyException).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input; self
        }
        /// Consumes the builder and constructs a [`ModelNotReadyException`](crate::error::ModelNotReadyException).
        pub fn build(self) -> crate::error::ModelNotReadyException {
            crate::error::ModelNotReadyException {
                message: self.message
                ,
            }
        }
    }
    
    
}
impl ModelNotReadyException {
    /// Creates a new builder-style object to manufacture [`ModelNotReadyException`](crate::error::ModelNotReadyException).
    pub fn builder() -> crate::error::model_not_ready_exception::Builder {
        crate::error::model_not_ready_exception::Builder::default()
    }
}

/// <p> Model (owned by the customer in the container) returned 4xx or 5xx error code. </p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ModelError  {
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
    /// <p> Original status code. </p>
    #[doc(hidden)]
    pub original_status_code: std::option::Option<i32>,
    /// <p> Original message. </p>
    #[doc(hidden)]
    pub original_message: std::option::Option<std::string::String>,
    /// <p> The Amazon Resource Name (ARN) of the log stream. </p>
    #[doc(hidden)]
    pub log_stream_arn: std::option::Option<std::string::String>,
}
impl ModelError {
    /// <p> Original status code. </p>
    pub fn original_status_code(&self) -> std::option::Option<i32> {
        self.original_status_code
    }
    /// <p> Original message. </p>
    pub fn original_message(&self) -> std::option::Option<& str> {
        self.original_message.as_deref()
    }
    /// <p> The Amazon Resource Name (ARN) of the log stream. </p>
    pub fn log_stream_arn(&self) -> std::option::Option<& str> {
        self.log_stream_arn.as_deref()
    }
}
impl ModelError {
    /// Returns the error message.
                        pub fn message(&self) -> std::option::Option<& str> { self.message.as_deref() }
}
impl std::fmt::Display for ModelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ModelError")?;
        if let Some(inner_5) = &self.message {
             {
                write!(f, ": {}", inner_5)?;
            }
        }
        Ok(())
    }
}
impl std::error::Error for ModelError {}
/// See [`ModelError`](crate::error::ModelError).
pub mod model_error {
    
    /// A builder for [`ModelError`](crate::error::ModelError).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
        pub(crate) original_status_code: std::option::Option<i32>,
        pub(crate) original_message: std::option::Option<std::string::String>,
        pub(crate) log_stream_arn: std::option::Option<std::string::String>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input; self
        }
        /// <p> Original status code. </p>
        pub fn original_status_code(mut self, input: i32) -> Self {
            self.original_status_code = Some(input);
            self
        }
        /// <p> Original status code. </p>
        pub fn set_original_status_code(mut self, input: std::option::Option<i32>) -> Self {
            self.original_status_code = input; self
        }
        /// <p> Original message. </p>
        pub fn original_message(mut self, input: impl Into<std::string::String>) -> Self {
            self.original_message = Some(input.into());
            self
        }
        /// <p> Original message. </p>
        pub fn set_original_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.original_message = input; self
        }
        /// <p> The Amazon Resource Name (ARN) of the log stream. </p>
        pub fn log_stream_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.log_stream_arn = Some(input.into());
            self
        }
        /// <p> The Amazon Resource Name (ARN) of the log stream. </p>
        pub fn set_log_stream_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.log_stream_arn = input; self
        }
        /// Consumes the builder and constructs a [`ModelError`](crate::error::ModelError).
        pub fn build(self) -> crate::error::ModelError {
            crate::error::ModelError {
                message: self.message
                ,
                original_status_code: self.original_status_code
                ,
                original_message: self.original_message
                ,
                log_stream_arn: self.log_stream_arn
                ,
            }
        }
    }
    
    
}
impl ModelError {
    /// Creates a new builder-style object to manufacture [`ModelError`](crate::error::ModelError).
    pub fn builder() -> crate::error::model_error::Builder {
        crate::error::model_error::Builder::default()
    }
}

/// <p>Your request caused an exception with an internal dependency. Contact customer support. </p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct InternalDependencyException  {
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
}
impl InternalDependencyException {
    /// Returns the error message.
                        pub fn message(&self) -> std::option::Option<& str> { self.message.as_deref() }
}
impl std::fmt::Display for InternalDependencyException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InternalDependencyException")?;
        if let Some(inner_6) = &self.message {
             {
                write!(f, ": {}", inner_6)?;
            }
        }
        Ok(())
    }
}
impl std::error::Error for InternalDependencyException {}
/// See [`InternalDependencyException`](crate::error::InternalDependencyException).
pub mod internal_dependency_exception {
    
    /// A builder for [`InternalDependencyException`](crate::error::InternalDependencyException).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input; self
        }
        /// Consumes the builder and constructs a [`InternalDependencyException`](crate::error::InternalDependencyException).
        pub fn build(self) -> crate::error::InternalDependencyException {
            crate::error::InternalDependencyException {
                message: self.message
                ,
            }
        }
    }
    
    
}
impl InternalDependencyException {
    /// Creates a new builder-style object to manufacture [`InternalDependencyException`](crate::error::InternalDependencyException).
    pub fn builder() -> crate::error::internal_dependency_exception::Builder {
        crate::error::internal_dependency_exception::Builder::default()
    }
}

/// 
/// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
/// 
/// When logging an error from the SDK, it is recommended that you either wrap the error in
/// [`DisplayErrorContext`](crate::types::DisplayErrorContext), use another
/// error reporter library that visits the error's cause/source chain, or call
/// [`Error::source`](std::error::Error::source) for more details about the underlying cause.
/// 
#[derive(Debug)]
        pub struct Unhandled {
            source: Box<dyn std::error::Error + Send + Sync + 'static>,
        }
        impl Unhandled {
            #[allow(unused)]
            pub(crate) fn new(source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
                Self { source }
            }
        }
        impl std::fmt::Display for Unhandled {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                write!(f, "unhandled error")
            }
        }
        impl std::error::Error for Unhandled {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                Some(self.source.as_ref() as _)
            }
        }


// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Error type for the `GetIceServerConfig` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct GetIceServerConfigError {
    /// Kind of error that occurred.
    pub kind: GetIceServerConfigErrorKind,
    /// Additional metadata about the error, including error code, message, and request ID.
    pub(crate) meta: aws_smithy_types::Error,
}
/// Types of errors that can occur for the `GetIceServerConfig` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum GetIceServerConfigErrorKind {
    /// <p>Your request was throttled because you have exceeded the limit of allowed client calls. Try making the call later.</p>
    ClientLimitExceededException(crate::error::ClientLimitExceededException),
    /// <p>The value for this input parameter is invalid.</p>
    InvalidArgumentException(crate::error::InvalidArgumentException),
    /// <p>The specified client is invalid.</p>
    InvalidClientException(crate::error::InvalidClientException),
    /// <p>The caller is not authorized to perform this operation.</p>
    NotAuthorizedException(crate::error::NotAuthorizedException),
    /// <p>The specified resource is not found.</p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// <p>If the client session is expired. Once the client is connected, the session is valid for 45 minutes. Client should reconnect to the channel to continue sending/receiving messages.</p>
    SessionExpiredException(crate::error::SessionExpiredException),
    /// An unexpected error, e.g. invalid JSON returned by the service or an unknown error code
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for GetIceServerConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            GetIceServerConfigErrorKind::ClientLimitExceededException(_inner) => _inner.fmt(f),
            GetIceServerConfigErrorKind::InvalidArgumentException(_inner) => _inner.fmt(f),
            GetIceServerConfigErrorKind::InvalidClientException(_inner) => _inner.fmt(f),
            GetIceServerConfigErrorKind::NotAuthorizedException(_inner) => _inner.fmt(f),
            GetIceServerConfigErrorKind::ResourceNotFoundException(_inner) => _inner.fmt(f),
            GetIceServerConfigErrorKind::SessionExpiredException(_inner) => _inner.fmt(f),
            GetIceServerConfigErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for GetIceServerConfigError {
    fn code(&self) -> Option<&str> {
        GetIceServerConfigError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl GetIceServerConfigError {
    /// Creates a new `GetIceServerConfigError`.
    pub fn new(kind: GetIceServerConfigErrorKind, meta: aws_smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    /// Creates the `GetIceServerConfigError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: GetIceServerConfigErrorKind::Unhandled(err.into()),
            meta: Default::default(),
        }
    }

    /// Creates the `GetIceServerConfigError::Unhandled` variant from a `aws_smithy_types::Error`.
    pub fn generic(err: aws_smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: GetIceServerConfigErrorKind::Unhandled(err.into()),
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
    /// Returns `true` if the error kind is `GetIceServerConfigErrorKind::ClientLimitExceededException`.
    pub fn is_client_limit_exceeded_exception(&self) -> bool {
        matches!(
            &self.kind,
            GetIceServerConfigErrorKind::ClientLimitExceededException(_)
        )
    }
    /// Returns `true` if the error kind is `GetIceServerConfigErrorKind::InvalidArgumentException`.
    pub fn is_invalid_argument_exception(&self) -> bool {
        matches!(
            &self.kind,
            GetIceServerConfigErrorKind::InvalidArgumentException(_)
        )
    }
    /// Returns `true` if the error kind is `GetIceServerConfigErrorKind::InvalidClientException`.
    pub fn is_invalid_client_exception(&self) -> bool {
        matches!(
            &self.kind,
            GetIceServerConfigErrorKind::InvalidClientException(_)
        )
    }
    /// Returns `true` if the error kind is `GetIceServerConfigErrorKind::NotAuthorizedException`.
    pub fn is_not_authorized_exception(&self) -> bool {
        matches!(
            &self.kind,
            GetIceServerConfigErrorKind::NotAuthorizedException(_)
        )
    }
    /// Returns `true` if the error kind is `GetIceServerConfigErrorKind::ResourceNotFoundException`.
    pub fn is_resource_not_found_exception(&self) -> bool {
        matches!(
            &self.kind,
            GetIceServerConfigErrorKind::ResourceNotFoundException(_)
        )
    }
    /// Returns `true` if the error kind is `GetIceServerConfigErrorKind::SessionExpiredException`.
    pub fn is_session_expired_exception(&self) -> bool {
        matches!(
            &self.kind,
            GetIceServerConfigErrorKind::SessionExpiredException(_)
        )
    }
}
impl std::error::Error for GetIceServerConfigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            GetIceServerConfigErrorKind::ClientLimitExceededException(_inner) => Some(_inner),
            GetIceServerConfigErrorKind::InvalidArgumentException(_inner) => Some(_inner),
            GetIceServerConfigErrorKind::InvalidClientException(_inner) => Some(_inner),
            GetIceServerConfigErrorKind::NotAuthorizedException(_inner) => Some(_inner),
            GetIceServerConfigErrorKind::ResourceNotFoundException(_inner) => Some(_inner),
            GetIceServerConfigErrorKind::SessionExpiredException(_inner) => Some(_inner),
            GetIceServerConfigErrorKind::Unhandled(_inner) => Some(_inner.as_ref()),
        }
    }
}

/// Error type for the `SendAlexaOfferToMaster` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct SendAlexaOfferToMasterError {
    /// Kind of error that occurred.
    pub kind: SendAlexaOfferToMasterErrorKind,
    /// Additional metadata about the error, including error code, message, and request ID.
    pub(crate) meta: aws_smithy_types::Error,
}
/// Types of errors that can occur for the `SendAlexaOfferToMaster` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum SendAlexaOfferToMasterErrorKind {
    /// <p>Your request was throttled because you have exceeded the limit of allowed client calls. Try making the call later.</p>
    ClientLimitExceededException(crate::error::ClientLimitExceededException),
    /// <p>The value for this input parameter is invalid.</p>
    InvalidArgumentException(crate::error::InvalidArgumentException),
    /// <p>The caller is not authorized to perform this operation.</p>
    NotAuthorizedException(crate::error::NotAuthorizedException),
    /// <p>The specified resource is not found.</p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// An unexpected error, e.g. invalid JSON returned by the service or an unknown error code
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for SendAlexaOfferToMasterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            SendAlexaOfferToMasterErrorKind::ClientLimitExceededException(_inner) => _inner.fmt(f),
            SendAlexaOfferToMasterErrorKind::InvalidArgumentException(_inner) => _inner.fmt(f),
            SendAlexaOfferToMasterErrorKind::NotAuthorizedException(_inner) => _inner.fmt(f),
            SendAlexaOfferToMasterErrorKind::ResourceNotFoundException(_inner) => _inner.fmt(f),
            SendAlexaOfferToMasterErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for SendAlexaOfferToMasterError {
    fn code(&self) -> Option<&str> {
        SendAlexaOfferToMasterError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl SendAlexaOfferToMasterError {
    /// Creates a new `SendAlexaOfferToMasterError`.
    pub fn new(kind: SendAlexaOfferToMasterErrorKind, meta: aws_smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    /// Creates the `SendAlexaOfferToMasterError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: SendAlexaOfferToMasterErrorKind::Unhandled(err.into()),
            meta: Default::default(),
        }
    }

    /// Creates the `SendAlexaOfferToMasterError::Unhandled` variant from a `aws_smithy_types::Error`.
    pub fn generic(err: aws_smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: SendAlexaOfferToMasterErrorKind::Unhandled(err.into()),
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
    /// Returns `true` if the error kind is `SendAlexaOfferToMasterErrorKind::ClientLimitExceededException`.
    pub fn is_client_limit_exceeded_exception(&self) -> bool {
        matches!(
            &self.kind,
            SendAlexaOfferToMasterErrorKind::ClientLimitExceededException(_)
        )
    }
    /// Returns `true` if the error kind is `SendAlexaOfferToMasterErrorKind::InvalidArgumentException`.
    pub fn is_invalid_argument_exception(&self) -> bool {
        matches!(
            &self.kind,
            SendAlexaOfferToMasterErrorKind::InvalidArgumentException(_)
        )
    }
    /// Returns `true` if the error kind is `SendAlexaOfferToMasterErrorKind::NotAuthorizedException`.
    pub fn is_not_authorized_exception(&self) -> bool {
        matches!(
            &self.kind,
            SendAlexaOfferToMasterErrorKind::NotAuthorizedException(_)
        )
    }
    /// Returns `true` if the error kind is `SendAlexaOfferToMasterErrorKind::ResourceNotFoundException`.
    pub fn is_resource_not_found_exception(&self) -> bool {
        matches!(
            &self.kind,
            SendAlexaOfferToMasterErrorKind::ResourceNotFoundException(_)
        )
    }
}
impl std::error::Error for SendAlexaOfferToMasterError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            SendAlexaOfferToMasterErrorKind::ClientLimitExceededException(_inner) => Some(_inner),
            SendAlexaOfferToMasterErrorKind::InvalidArgumentException(_inner) => Some(_inner),
            SendAlexaOfferToMasterErrorKind::NotAuthorizedException(_inner) => Some(_inner),
            SendAlexaOfferToMasterErrorKind::ResourceNotFoundException(_inner) => Some(_inner),
            SendAlexaOfferToMasterErrorKind::Unhandled(_inner) => Some(_inner.as_ref()),
        }
    }
}

/// <p>The specified resource is not found.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ResourceNotFoundException {
    #[allow(missing_docs)] // documentation missing in model
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for ResourceNotFoundException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ResourceNotFoundException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl ResourceNotFoundException {
    /// Returns the error message.
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for ResourceNotFoundException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ResourceNotFoundException")?;
        if let Some(inner_1) = &self.message {
            write!(f, ": {}", inner_1)?;
        }
        Ok(())
    }
}
impl std::error::Error for ResourceNotFoundException {}
/// See [`ResourceNotFoundException`](crate::error::ResourceNotFoundException)
pub mod resource_not_found_exception {
    /// A builder for [`ResourceNotFoundException`](crate::error::ResourceNotFoundException)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
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
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`ResourceNotFoundException`](crate::error::ResourceNotFoundException)
        pub fn build(self) -> crate::error::ResourceNotFoundException {
            crate::error::ResourceNotFoundException {
                message: self.message,
            }
        }
    }
}
impl ResourceNotFoundException {
    /// Creates a new builder-style object to manufacture [`ResourceNotFoundException`](crate::error::ResourceNotFoundException)
    pub fn builder() -> crate::error::resource_not_found_exception::Builder {
        crate::error::resource_not_found_exception::Builder::default()
    }
}

/// <p>The caller is not authorized to perform this operation.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct NotAuthorizedException {
    #[allow(missing_docs)] // documentation missing in model
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for NotAuthorizedException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("NotAuthorizedException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl NotAuthorizedException {
    /// Returns the error message.
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for NotAuthorizedException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NotAuthorizedException")?;
        if let Some(inner_2) = &self.message {
            write!(f, ": {}", inner_2)?;
        }
        Ok(())
    }
}
impl std::error::Error for NotAuthorizedException {}
/// See [`NotAuthorizedException`](crate::error::NotAuthorizedException)
pub mod not_authorized_exception {
    /// A builder for [`NotAuthorizedException`](crate::error::NotAuthorizedException)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
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
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`NotAuthorizedException`](crate::error::NotAuthorizedException)
        pub fn build(self) -> crate::error::NotAuthorizedException {
            crate::error::NotAuthorizedException {
                message: self.message,
            }
        }
    }
}
impl NotAuthorizedException {
    /// Creates a new builder-style object to manufacture [`NotAuthorizedException`](crate::error::NotAuthorizedException)
    pub fn builder() -> crate::error::not_authorized_exception::Builder {
        crate::error::not_authorized_exception::Builder::default()
    }
}

/// <p>The value for this input parameter is invalid.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct InvalidArgumentException {
    #[allow(missing_docs)] // documentation missing in model
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for InvalidArgumentException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("InvalidArgumentException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl InvalidArgumentException {
    /// Returns the error message.
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for InvalidArgumentException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InvalidArgumentException")?;
        if let Some(inner_3) = &self.message {
            write!(f, ": {}", inner_3)?;
        }
        Ok(())
    }
}
impl std::error::Error for InvalidArgumentException {}
/// See [`InvalidArgumentException`](crate::error::InvalidArgumentException)
pub mod invalid_argument_exception {
    /// A builder for [`InvalidArgumentException`](crate::error::InvalidArgumentException)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
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
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`InvalidArgumentException`](crate::error::InvalidArgumentException)
        pub fn build(self) -> crate::error::InvalidArgumentException {
            crate::error::InvalidArgumentException {
                message: self.message,
            }
        }
    }
}
impl InvalidArgumentException {
    /// Creates a new builder-style object to manufacture [`InvalidArgumentException`](crate::error::InvalidArgumentException)
    pub fn builder() -> crate::error::invalid_argument_exception::Builder {
        crate::error::invalid_argument_exception::Builder::default()
    }
}

/// <p>Your request was throttled because you have exceeded the limit of allowed client calls. Try making the call later.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ClientLimitExceededException {
    #[allow(missing_docs)] // documentation missing in model
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for ClientLimitExceededException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ClientLimitExceededException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl ClientLimitExceededException {
    /// Returns the error message.
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for ClientLimitExceededException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ClientLimitExceededException")?;
        if let Some(inner_4) = &self.message {
            write!(f, ": {}", inner_4)?;
        }
        Ok(())
    }
}
impl std::error::Error for ClientLimitExceededException {}
/// See [`ClientLimitExceededException`](crate::error::ClientLimitExceededException)
pub mod client_limit_exceeded_exception {
    /// A builder for [`ClientLimitExceededException`](crate::error::ClientLimitExceededException)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
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
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`ClientLimitExceededException`](crate::error::ClientLimitExceededException)
        pub fn build(self) -> crate::error::ClientLimitExceededException {
            crate::error::ClientLimitExceededException {
                message: self.message,
            }
        }
    }
}
impl ClientLimitExceededException {
    /// Creates a new builder-style object to manufacture [`ClientLimitExceededException`](crate::error::ClientLimitExceededException)
    pub fn builder() -> crate::error::client_limit_exceeded_exception::Builder {
        crate::error::client_limit_exceeded_exception::Builder::default()
    }
}

/// <p>If the client session is expired. Once the client is connected, the session is valid for 45 minutes. Client should reconnect to the channel to continue sending/receiving messages.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct SessionExpiredException {
    #[allow(missing_docs)] // documentation missing in model
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for SessionExpiredException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("SessionExpiredException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl SessionExpiredException {
    /// Returns the error message.
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for SessionExpiredException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SessionExpiredException")?;
        if let Some(inner_5) = &self.message {
            write!(f, ": {}", inner_5)?;
        }
        Ok(())
    }
}
impl std::error::Error for SessionExpiredException {}
/// See [`SessionExpiredException`](crate::error::SessionExpiredException)
pub mod session_expired_exception {
    /// A builder for [`SessionExpiredException`](crate::error::SessionExpiredException)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
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
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`SessionExpiredException`](crate::error::SessionExpiredException)
        pub fn build(self) -> crate::error::SessionExpiredException {
            crate::error::SessionExpiredException {
                message: self.message,
            }
        }
    }
}
impl SessionExpiredException {
    /// Creates a new builder-style object to manufacture [`SessionExpiredException`](crate::error::SessionExpiredException)
    pub fn builder() -> crate::error::session_expired_exception::Builder {
        crate::error::session_expired_exception::Builder::default()
    }
}

/// <p>The specified client is invalid.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct InvalidClientException {
    #[allow(missing_docs)] // documentation missing in model
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for InvalidClientException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("InvalidClientException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl InvalidClientException {
    /// Returns the error message.
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for InvalidClientException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InvalidClientException")?;
        if let Some(inner_6) = &self.message {
            write!(f, ": {}", inner_6)?;
        }
        Ok(())
    }
}
impl std::error::Error for InvalidClientException {}
/// See [`InvalidClientException`](crate::error::InvalidClientException)
pub mod invalid_client_exception {
    /// A builder for [`InvalidClientException`](crate::error::InvalidClientException)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
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
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`InvalidClientException`](crate::error::InvalidClientException)
        pub fn build(self) -> crate::error::InvalidClientException {
            crate::error::InvalidClientException {
                message: self.message,
            }
        }
    }
}
impl InvalidClientException {
    /// Creates a new builder-style object to manufacture [`InvalidClientException`](crate::error::InvalidClientException)
    pub fn builder() -> crate::error::invalid_client_exception::Builder {
        crate::error::invalid_client_exception::Builder::default()
    }
}

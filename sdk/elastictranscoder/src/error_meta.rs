// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p>General authentication failure. The request was not signed correctly.</p>
    AccessDeniedException(crate::error::AccessDeniedException),
    #[allow(missing_docs)] // documentation missing in model
    IncompatibleVersionException(crate::error::IncompatibleVersionException),
    /// <p>Elastic Transcoder encountered an unexpected exception while trying to fulfill the request.</p>
    InternalServiceException(crate::error::InternalServiceException),
    /// <p>Too many operations for a given AWS account. For example, the number of pipelines exceeds the maximum allowed.</p>
    LimitExceededException(crate::error::LimitExceededException),
    /// <p>The resource you are attempting to change is in use. For example, you are attempting to delete a pipeline that is currently in use.</p>
    ResourceInUseException(crate::error::ResourceInUseException),
    /// <p>The requested resource does not exist or is not available. For example, the pipeline to which you're trying to add a job doesn't exist or is still being created.</p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// <p>One or more required parameter values were not provided in the request.</p>
    ValidationException(crate::error::ValidationException),
    /// An unhandled error occurred.
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AccessDeniedException(inner) => inner.fmt(f),
            Error::IncompatibleVersionException(inner) => inner.fmt(f),
            Error::InternalServiceException(inner) => inner.fmt(f),
            Error::LimitExceededException(inner) => inner.fmt(f),
            Error::ResourceInUseException(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::ValidationException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CancelJobError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::CancelJobError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::CancelJobErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::CancelJobErrorKind::IncompatibleVersionException(inner) => {
                    Error::IncompatibleVersionException(inner)
                }
                crate::error::CancelJobErrorKind::InternalServiceException(inner) => {
                    Error::InternalServiceException(inner)
                }
                crate::error::CancelJobErrorKind::ResourceInUseException(inner) => {
                    Error::ResourceInUseException(inner)
                }
                crate::error::CancelJobErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::CancelJobErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::CancelJobErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateJobError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::CreateJobError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::CreateJobErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::CreateJobErrorKind::IncompatibleVersionException(inner) => {
                    Error::IncompatibleVersionException(inner)
                }
                crate::error::CreateJobErrorKind::InternalServiceException(inner) => {
                    Error::InternalServiceException(inner)
                }
                crate::error::CreateJobErrorKind::LimitExceededException(inner) => {
                    Error::LimitExceededException(inner)
                }
                crate::error::CreateJobErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::CreateJobErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::CreateJobErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreatePipelineError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::CreatePipelineError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::CreatePipelineErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::CreatePipelineErrorKind::IncompatibleVersionException(inner) => {
                    Error::IncompatibleVersionException(inner)
                }
                crate::error::CreatePipelineErrorKind::InternalServiceException(inner) => {
                    Error::InternalServiceException(inner)
                }
                crate::error::CreatePipelineErrorKind::LimitExceededException(inner) => {
                    Error::LimitExceededException(inner)
                }
                crate::error::CreatePipelineErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::CreatePipelineErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::CreatePipelineErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreatePresetError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::CreatePresetError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::CreatePresetErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::CreatePresetErrorKind::IncompatibleVersionException(inner) => {
                    Error::IncompatibleVersionException(inner)
                }
                crate::error::CreatePresetErrorKind::InternalServiceException(inner) => {
                    Error::InternalServiceException(inner)
                }
                crate::error::CreatePresetErrorKind::LimitExceededException(inner) => {
                    Error::LimitExceededException(inner)
                }
                crate::error::CreatePresetErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::CreatePresetErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeletePipelineError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DeletePipelineError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeletePipelineErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::DeletePipelineErrorKind::IncompatibleVersionException(inner) => {
                    Error::IncompatibleVersionException(inner)
                }
                crate::error::DeletePipelineErrorKind::InternalServiceException(inner) => {
                    Error::InternalServiceException(inner)
                }
                crate::error::DeletePipelineErrorKind::ResourceInUseException(inner) => {
                    Error::ResourceInUseException(inner)
                }
                crate::error::DeletePipelineErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::DeletePipelineErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::DeletePipelineErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeletePresetError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DeletePresetError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeletePresetErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::DeletePresetErrorKind::IncompatibleVersionException(inner) => {
                    Error::IncompatibleVersionException(inner)
                }
                crate::error::DeletePresetErrorKind::InternalServiceException(inner) => {
                    Error::InternalServiceException(inner)
                }
                crate::error::DeletePresetErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::DeletePresetErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::DeletePresetErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListJobsByPipelineError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::ListJobsByPipelineError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListJobsByPipelineErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::ListJobsByPipelineErrorKind::IncompatibleVersionException(inner) => {
                    Error::IncompatibleVersionException(inner)
                }
                crate::error::ListJobsByPipelineErrorKind::InternalServiceException(inner) => {
                    Error::InternalServiceException(inner)
                }
                crate::error::ListJobsByPipelineErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::ListJobsByPipelineErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::ListJobsByPipelineErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListJobsByStatusError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::ListJobsByStatusError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListJobsByStatusErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::ListJobsByStatusErrorKind::IncompatibleVersionException(inner) => {
                    Error::IncompatibleVersionException(inner)
                }
                crate::error::ListJobsByStatusErrorKind::InternalServiceException(inner) => {
                    Error::InternalServiceException(inner)
                }
                crate::error::ListJobsByStatusErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::ListJobsByStatusErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::ListJobsByStatusErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListPipelinesError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListPipelinesError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListPipelinesErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::ListPipelinesErrorKind::IncompatibleVersionException(inner) => {
                    Error::IncompatibleVersionException(inner)
                }
                crate::error::ListPipelinesErrorKind::InternalServiceException(inner) => {
                    Error::InternalServiceException(inner)
                }
                crate::error::ListPipelinesErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::ListPipelinesErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListPresetsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListPresetsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListPresetsErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::ListPresetsErrorKind::IncompatibleVersionException(inner) => {
                    Error::IncompatibleVersionException(inner)
                }
                crate::error::ListPresetsErrorKind::InternalServiceException(inner) => {
                    Error::InternalServiceException(inner)
                }
                crate::error::ListPresetsErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::ListPresetsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ReadJobError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ReadJobError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ReadJobErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::ReadJobErrorKind::IncompatibleVersionException(inner) => {
                    Error::IncompatibleVersionException(inner)
                }
                crate::error::ReadJobErrorKind::InternalServiceException(inner) => {
                    Error::InternalServiceException(inner)
                }
                crate::error::ReadJobErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::ReadJobErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::ReadJobErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ReadPipelineError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ReadPipelineError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ReadPipelineErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::ReadPipelineErrorKind::IncompatibleVersionException(inner) => {
                    Error::IncompatibleVersionException(inner)
                }
                crate::error::ReadPipelineErrorKind::InternalServiceException(inner) => {
                    Error::InternalServiceException(inner)
                }
                crate::error::ReadPipelineErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::ReadPipelineErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::ReadPipelineErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ReadPresetError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ReadPresetError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ReadPresetErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::ReadPresetErrorKind::IncompatibleVersionException(inner) => {
                    Error::IncompatibleVersionException(inner)
                }
                crate::error::ReadPresetErrorKind::InternalServiceException(inner) => {
                    Error::InternalServiceException(inner)
                }
                crate::error::ReadPresetErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::ReadPresetErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::ReadPresetErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::TestRoleError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::TestRoleError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::TestRoleErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::TestRoleErrorKind::IncompatibleVersionException(inner) => {
                    Error::IncompatibleVersionException(inner)
                }
                crate::error::TestRoleErrorKind::InternalServiceException(inner) => {
                    Error::InternalServiceException(inner)
                }
                crate::error::TestRoleErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::TestRoleErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::TestRoleErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdatePipelineError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::UpdatePipelineError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UpdatePipelineErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::UpdatePipelineErrorKind::IncompatibleVersionException(inner) => {
                    Error::IncompatibleVersionException(inner)
                }
                crate::error::UpdatePipelineErrorKind::InternalServiceException(inner) => {
                    Error::InternalServiceException(inner)
                }
                crate::error::UpdatePipelineErrorKind::ResourceInUseException(inner) => {
                    Error::ResourceInUseException(inner)
                }
                crate::error::UpdatePipelineErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::UpdatePipelineErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::UpdatePipelineErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdatePipelineNotificationsError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::UpdatePipelineNotificationsError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::UpdatePipelineNotificationsErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::UpdatePipelineNotificationsErrorKind::IncompatibleVersionException(inner) => Error::IncompatibleVersionException(inner),
                crate::error::UpdatePipelineNotificationsErrorKind::InternalServiceException(inner) => Error::InternalServiceException(inner),
                crate::error::UpdatePipelineNotificationsErrorKind::ResourceInUseException(inner) => Error::ResourceInUseException(inner),
                crate::error::UpdatePipelineNotificationsErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::UpdatePipelineNotificationsErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::UpdatePipelineNotificationsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdatePipelineStatusError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::UpdatePipelineStatusError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UpdatePipelineStatusErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::UpdatePipelineStatusErrorKind::IncompatibleVersionException(
                    inner,
                ) => Error::IncompatibleVersionException(inner),
                crate::error::UpdatePipelineStatusErrorKind::InternalServiceException(inner) => {
                    Error::InternalServiceException(inner)
                }
                crate::error::UpdatePipelineStatusErrorKind::ResourceInUseException(inner) => {
                    Error::ResourceInUseException(inner)
                }
                crate::error::UpdatePipelineStatusErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::UpdatePipelineStatusErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::UpdatePipelineStatusErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl std::error::Error for Error {}

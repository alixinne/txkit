use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("the method doesn't support the given context")]
    ContextNotSupported,
    #[error("the method doesn't support the requested format")]
    FormatNotSupported,
    #[error("the requested method was not found")]
    MethodNotFound,
    #[error("invalid method name")]
    InvalidMethodName,
    #[cfg(feature = "cpu")]
    #[error("cpu context creation failed: {0}")]
    CpuContextCreationFailed(#[from] rayon::ThreadPoolBuildError),
    #[error("method initialization failed: {0}")]
    MethodInitializationFailed(String),
    #[error("mapping image failed: {0}")]
    MappingFailed(#[from] crate::image::ImageDataError),
    #[error("the provided parameters do not apply to the given method")]
    InvalidParameters,

    #[cfg(feature = "gpu")]
    #[error("gpu context creation failed: {0}")]
    GpuContextCreationFailed(#[from] glutin::CreationError),
    #[cfg(feature = "gpu")]
    #[error("failed to make context current: {0}")]
    GpuContextMakeCurrentFailed(#[from] glutin::ContextError),
    #[cfg(feature = "gpu")]
    #[error("opengl error: {0}")]
    OpenGlError(#[from] tinygl::Error),
    #[cfg(feature = "gpu")]
    #[error("opengl error: {0}")]
    OpenGlErrorMessage(String),
}

pub type Result<T> = std::result::Result<T, self::Error>;

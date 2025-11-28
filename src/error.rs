use thiserror::Error;

/// IiifError 定义了 IIIF 相关的错误类型
#[derive(Debug, Error, PartialEq)]
pub enum IiifError {
    #[error("Invalid IIIF image URL: {0}")]
    InvalidIiifURL(String),

    #[error("Image open failed: {0}")]
    ImageOpenFailed(String),

    /// 400 Bad Request
    ///
    /// 服务器无法满足请求，因为客户端发出的请求语法不正确。
    ///
    /// The server cannot fulfill the request, as the syntax of the request issued by the client is incorrect.
    #[error("{0}")]
    BadRequest(String),

    /// 401 Unauthorized
    ///
    /// 请求要求身份验证。未提供凭据或提供凭据但无效。
    ///
    /// The request requires user authentication. The client must authenticate itself to get the requested response.
    #[error("{0}")]
    Unauthorized(String),

    /// 403 Forbidden
    ///
    /// 用户（无论是否已认证）都不被允许执行请求的操作。
    ///
    /// The user, authenticated or not, is not permitted to perform the requested operation.
    #[error("{0}")]
    Forbidden(String),

    /// 404 Not Found
    ///
    /// 通过标识符指定的图像资源不存在，一个或多个参数的值不受此图像服务支持，或请求的尺寸大于指定的限制。
    ///
    /// The image resource specified by identifier does not exist, the value of one or more of
    /// the parameters is not supported for this image service, or the requested size is greater
    /// than the limits specified.
    #[error("{0}")]
    NotFound(String),

    /// 500 Internal Server Error
    ///
    /// 服务器遇到意外错误，导致无法满足请求。
    ///
    /// The server encountered an unexpected error that prevented it from fulfilling the request.
    #[error("{0}")]
    InternalServerError(String),

    /// 501 Not Implemented
    ///
    /// 服务器收到了一个有效的 IIIF 请求，但该请求未实现。
    ///
    /// The server received a valid IIIF request that is not implemented.
    #[error("{0}")]
    NotImplemented(String),

    /// 503 Service Unavailable
    ///
    /// 服务器因负载/维护问题繁忙/暂时不可用。
    ///
    /// The server is busy/temporarily unavailable due to load/maintenance problems.
    #[error("{0}")]
    ServiceUnavailable(String),
}

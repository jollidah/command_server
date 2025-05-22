use std::fmt::Debug;

#[derive(Debug)]
pub enum ServiceError {
    _InternalServerError,
    DatabaseConnectionError(Box<dyn Debug + Send>),
    NotFound,
    KVStoreError(Box<dyn Debug + Send>),
    ParsingError(Box<dyn Debug + Send>),
    EmailError(Box<dyn Debug + Send>),
    InvalidVerificationCode,
    VerificationCodeExpired,
    JwtTokenError(String),
    InvalidJwtToken,
    JwtTokenExpired,
    UserNotVerified,
    Unauthorized,
    RequestError(Box<dyn Debug + Send>),
    ParseError,
    PemKeyError(String),
}

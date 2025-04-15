use std::fmt::Debug;

#[derive(Debug)]
pub enum ServiceError {
    _InternalServerError,
    DatabaseConnectionError(Box<dyn Debug>),
    RowNotFound,
    KVStoreError(Box<dyn Debug>),
    ParsingError(Box<dyn Debug>),
    EmailError(Box<dyn Debug>),
    InvalidVerificationCode,
    VerificationCodeExpired,
    JwtTokenError(String),
    InvalidJwtToken,
    JwtTokenExpired,
    UserNotVerified,
    Unauthorized,
}

use axum::{extract::Json, http::StatusCode, response::IntoResponse};

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("{0}")]
    ValidationErrors(#[from] validator::ValidationErrors),
    #[error("{0}")]
    NoRefreshTokenError(String),
    #[error("{0}")]
    InvalidJwtTokenError(String),
    #[error("{0}")]
    InvalidUserRoleError(String),
    #[error("{0}")]
    InvalidCredentialError(String),
    #[error("{0}")]
    UserNotActivatedError(String),
    #[error("{0}")]
    InvalidUuidError(String),
    #[error("{0}")]
    BadRequestError(String),
    #[error("{0}")]
    UnauthorizedError(String),
    #[error("{0}")]
    UserNotFoundError(String),
    #[error("{0}")]
    DataNotFoundError(String),
    #[error("{0}")]
    UnexpectedError(#[from] anyhow::Error),
    #[error("{0}")]
    DbError(#[from] sqlx::Error),
}

#[derive(serde::Serialize)]
struct ErrorResponse {
    status_code: u16,
    error_type: String,
    details: String,
    title: String,
}

struct ErrorMetadata {
    status_code: StatusCode,
    error_type: &'static str,
    title: &'static str,
}

impl AppError {
    fn metadata(&self) -> ErrorMetadata {
        match self {
            Self::ValidationErrors(_) => ErrorMetadata {
                status_code: StatusCode::BAD_REQUEST,
                error_type: "ValidationErrors",
                title: "Validation error",
            },
            Self::NoRefreshTokenError(_) => ErrorMetadata {
                status_code: StatusCode::UNAUTHORIZED,
                error_type: "NoRefreshTokenError",
                title: "No refresh token",
            },
            Self::InvalidJwtTokenError(_) => ErrorMetadata {
                status_code: StatusCode::UNAUTHORIZED,
                error_type: "InvalidJwtTokenError",
                title: "Invalid JWT token",
            },
            Self::InvalidUserRoleError(_) => ErrorMetadata {
                status_code: StatusCode::FORBIDDEN,
                error_type: "InvalidUserRoleError",
                title: "Invalid user role",
            },
            Self::InvalidCredentialError(_) => ErrorMetadata {
                status_code: StatusCode::UNAUTHORIZED,
                error_type: "InvalidCredentialError",
                title: "Invalid credentials",
            },
            Self::UserNotActivatedError(_) => ErrorMetadata {
                status_code: StatusCode::FORBIDDEN,
                error_type: "UserNotActivatedError",
                title: "User not activated",
            },
            Self::InvalidUuidError(_) => ErrorMetadata {
                status_code: StatusCode::BAD_REQUEST,
                error_type: "InvalidUuidError",
                title: "Invalid UUID",
            },
            Self::BadRequestError(_) => ErrorMetadata {
                status_code: StatusCode::BAD_REQUEST,
                error_type: "BadRequestError",
                title: "Bad request",
            },
            Self::UnauthorizedError(_) => ErrorMetadata {
                status_code: StatusCode::UNAUTHORIZED,
                error_type: "UnauthorizedError",
                title: "Unauthorized",
            },
            Self::UserNotFoundError(_) => ErrorMetadata {
                status_code: StatusCode::UNAUTHORIZED,
                error_type: "UserNotFoundError",
                title: "User not found",
            },
            Self::DataNotFoundError(_) => ErrorMetadata {
                status_code: StatusCode::NOT_FOUND,
                error_type: "DataNotFoundError",
                title: "Data not found",
            },
            Self::UnexpectedError(_) => ErrorMetadata {
                status_code: StatusCode::BAD_REQUEST,
                error_type: "UnexpectedError",
                title: "Unexpected error",
            },
            Self::DbError(_) => ErrorMetadata {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                error_type: "DbError",
                title: "Database error",
            },
        }
    }

    fn details(&self) -> String {
        match self {
            Self::ValidationErrors(e) => {
                e.field_errors()
                    .iter()
                    .flat_map(|(field, errors)| {
                        errors.iter().map(move |error| {
                            format!("{}: {}", field, error.message.as_ref().unwrap_or(&error.code))
                        })
                    })
                    .collect::<Vec<_>>()
                    .join(", ")
            }
            Self::NoRefreshTokenError(msg)
            | Self::InvalidJwtTokenError(msg)
            | Self::InvalidUserRoleError(msg)
            | Self::InvalidCredentialError(msg)
            | Self::UserNotActivatedError(msg)
            | Self::InvalidUuidError(msg)
            | Self::BadRequestError(msg)
            | Self::UnauthorizedError(msg)
            | Self::UserNotFoundError(msg)
            | Self::DataNotFoundError(msg) => msg.clone(),
            Self::UnexpectedError(e) => e.to_string(),
            Self::DbError(e) => e.to_string(),
        }
    }

    fn into_error_response(self) -> (StatusCode, Json<ErrorResponse>) {
        let metadata = self.metadata();
        let details = self.details();
        let status_code_u16 = metadata.status_code.as_u16();

        (
            metadata.status_code,
            Json(ErrorResponse {
                status_code: status_code_u16,
                error_type: metadata.error_type.to_string(),
                details,
                title: metadata.title.to_string(),
            }),
        )
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        self.into_error_response().into_response()
    }
}

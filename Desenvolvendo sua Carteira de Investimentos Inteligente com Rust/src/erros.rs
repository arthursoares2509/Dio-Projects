use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug, thiserror::Error)]
pub enum ErroAplicacao {
    #[error("Você precisa entrar para acessar esta página.")]
    NaoAutorizado,
    #[error("Ocorreu um erro inesperado. Tente novamente.")]
    Interno,
}

impl IntoResponse for ErroAplicacao {
    fn into_response(self) -> Response {
        let codigo = match self {
            Self::NaoAutorizado => StatusCode::UNAUTHORIZED,
            Self::Interno => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (codigo, self.to_string()).into_response()
    }
}

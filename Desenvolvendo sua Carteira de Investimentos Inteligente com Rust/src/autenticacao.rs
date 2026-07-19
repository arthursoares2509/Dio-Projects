use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::erros::ErroAplicacao;

#[derive(Clone)]
pub struct ConfiguracaoJwt {
    chave: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Reivindicacoes {
    sub: Uuid,
    exp: usize,
}

impl ConfiguracaoJwt {
    pub fn nova(chave: String) -> Self {
        Self { chave }
    }

    pub fn criar_token(&self, usuario_id: Uuid) -> Result<String, ErroAplicacao> {
        let expira_em = Utc::now() + Duration::days(7);
        encode(
            &Header::default(),
            &Reivindicacoes {
                sub: usuario_id,
                exp: expira_em.timestamp() as usize,
            },
            &EncodingKey::from_secret(self.chave.as_bytes()),
        )
        .map_err(|_| ErroAplicacao::Interno)
    }

    pub fn usuario_do_token(&self, token: &str) -> Result<Uuid, ErroAplicacao> {
        decode::<Reivindicacoes>(
            token,
            &DecodingKey::from_secret(self.chave.as_bytes()),
            &Validation::default(),
        )
        .map(|dados| dados.claims.sub)
        .map_err(|_| ErroAplicacao::NaoAutorizado)
    }
}

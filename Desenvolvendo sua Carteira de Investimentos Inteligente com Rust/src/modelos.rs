use chrono::{DateTime, Utc};
use serde::Deserialize;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct Usuario {
    pub id: Uuid,
    pub nome: String,
    pub email: String,
    pub senha_hash: String,
    pub criado_em: DateTime<Utc>,
}

#[derive(Debug, FromRow)]
pub struct Ativo {
    pub id: Uuid,
    pub codigo: String,
    pub nome: String,
    pub tipo: String,
    pub quantidade: f64,
    pub preco_medio: f64,
    pub preco_atual: f64,
}

impl Ativo {
    pub fn valor_investido(&self) -> f64 {
        self.quantidade * self.preco_medio
    }
    pub fn valor_atual(&self) -> f64 {
        self.quantidade * self.preco_atual
    }
    pub fn resultado(&self) -> f64 {
        self.valor_atual() - self.valor_investido()
    }
}

#[derive(Deserialize)]
pub struct FormularioCadastro {
    pub nome: String,
    pub email: String,
    pub senha: String,
}

#[derive(Deserialize)]
pub struct FormularioEntrada {
    pub email: String,
    pub senha: String,
}

#[derive(Deserialize)]
pub struct FormularioAtivo {
    pub codigo: String,
    pub nome: String,
    pub tipo: String,
    pub quantidade: f64,
    pub preco_medio: f64,
    pub preco_atual: f64,
}

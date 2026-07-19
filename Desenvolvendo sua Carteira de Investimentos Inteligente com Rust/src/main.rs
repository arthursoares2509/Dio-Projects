mod autenticacao;
mod erros;
mod modelos;
mod rotas;

use std::{env, sync::Arc};

use axum::{
    routing::{get, post},
    Router,
};
use sqlx::postgres::PgPoolOptions;
use tower_http::{services::ServeDir, trace::TraceLayer};

use crate::{
    autenticacao::ConfiguracaoJwt,
    rotas::{adicionar_ativo, cadastrar, entrar, excluir_ativo, inicio, sair},
};

#[derive(Clone)]
pub struct EstadoAplicacao {
    pub banco: sqlx::PgPool,
    pub jwt: ConfiguracaoJwt,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let endereco_banco = env::var("DATABASE_URL").expect("DATABASE_URL não configurada");
    let chave = env::var("CHAVE_JWT").expect("CHAVE_JWT não configurada");
    let porta = env::var("PORTA").unwrap_or_else(|_| "3000".into());
    let banco = PgPoolOptions::new()
        .max_connections(5)
        .connect(&endereco_banco)
        .await
        .expect("Não foi possível conectar ao PostgreSQL");

    let estado = Arc::new(EstadoAplicacao {
        banco,
        jwt: ConfiguracaoJwt::nova(chave),
    });
    let aplicacao = Router::new()
        .route("/", get(inicio))
        .route("/cadastro", get(rotas::pagina_cadastro).post(cadastrar))
        .route("/entrar", get(rotas::pagina_entrada).post(entrar))
        .route("/sair", post(sair))
        .route("/ativos", post(adicionar_ativo))
        .route("/ativos/:id/excluir", post(excluir_ativo))
        .nest_service("/estatico", ServeDir::new("estatico"))
        .layer(TraceLayer::new_for_http())
        .with_state(estado);

    let endereco = format!("0.0.0.0:{porta}");
    let ouvinte = tokio::net::TcpListener::bind(&endereco)
        .await
        .expect("Porta indisponível");
    tracing::info!("Aplicação em http://{endereco}");
    axum::serve(ouvinte, aplicacao)
        .await
        .expect("Servidor interrompido");
}
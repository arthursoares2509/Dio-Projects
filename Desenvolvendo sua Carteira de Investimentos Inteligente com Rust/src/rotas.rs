use std::sync::Arc;

use askama::Template;
use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse, Redirect},
    Form,
};
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use bcrypt::{hash, verify, DEFAULT_COST};
use uuid::Uuid;

use crate::{
    erros::ErroAplicacao,
    modelos::{Ativo, FormularioAtivo, FormularioCadastro, FormularioEntrada, Usuario},
    EstadoAplicacao,
};

const NOME_COOKIE: &str = "sessao_carteira";

#[derive(Template)]
#[template(path = "autenticacao.html")]
struct PaginaAutenticacao<'a> {
    titulo: &'a str,
    acao: &'a str,
    cadastro: bool,
    erro: Option<&'a str>,
}

#[derive(Template)]
#[template(path = "inicio.html")]
struct PaginaInicio {
    nome: String,
    ativos: Vec<Ativo>,
    total_investido: f64,
    total_atual: f64,
    resultado: f64,
}

pub async fn pagina_cadastro() -> Html<String> {
    renderizar(PaginaAutenticacao {
        titulo: "Crie sua conta",
        acao: "/cadastro",
        cadastro: true,
        erro: None,
    })
}

pub async fn pagina_entrada() -> Html<String> {
    renderizar(PaginaAutenticacao {
        titulo: "Entre na sua carteira",
        acao: "/entrar",
        cadastro: false,
        erro: None,
    })
}

pub async fn cadastrar(
    State(estado): State<Arc<EstadoAplicacao>>,
    jar: CookieJar,
    Form(formulario): Form<FormularioCadastro>,
) -> Result<impl IntoResponse, ErroAplicacao> {
    if formulario.nome.trim().is_empty()
        || formulario.senha.len() < 8
        || !formulario.email.contains('@')
    {
        return Ok(Redirect::to("/cadastro").into_response());
    }
    let senha_hash = hash(formulario.senha, DEFAULT_COST).map_err(|_| ErroAplicacao::Interno)?;
    let usuario_id = Uuid::new_v4();
    let resultado =
        sqlx::query("INSERT INTO usuarios (id, nome, email, senha_hash) VALUES ($1, $2, $3, $4)")
            .bind(usuario_id)
            .bind(formulario.nome.trim())
            .bind(formulario.email.trim().to_lowercase())
            .bind(senha_hash)
            .execute(&estado.banco)
            .await;
    if resultado.is_err() {
        return Ok(Redirect::to("/cadastro").into_response());
    }
    Ok(resposta_com_sessao(jar, &estado, usuario_id)?.into_response())
}

pub async fn entrar(
    State(estado): State<Arc<EstadoAplicacao>>,
    jar: CookieJar,
    Form(formulario): Form<FormularioEntrada>,
) -> Result<impl IntoResponse, ErroAplicacao> {
    let usuario = sqlx::query_as::<_, Usuario>(
        "SELECT id, nome, email, senha_hash, criado_em FROM usuarios WHERE email = $1",
    )
    .bind(formulario.email.trim().to_lowercase())
    .fetch_optional(&estado.banco)
    .await
    .map_err(|_| ErroAplicacao::Interno)?;
    let valido = usuario
        .as_ref()
        .map(|usuario| verify(&formulario.senha, &usuario.senha_hash).unwrap_or(false))
        .unwrap_or(false);
    if !valido {
        return Ok(Redirect::to("/entrar").into_response());
    }
    Ok(resposta_com_sessao(jar, &estado, usuario.expect("usuário validado").id)?.into_response())
}

pub async fn sair(jar: CookieJar) -> impl IntoResponse {
    let cookie = Cookie::build((NOME_COOKIE, ""))
        .path("/")
        .max_age(time::Duration::seconds(0))
        .build();
    (jar.add(cookie), Redirect::to("/entrar"))
}

pub async fn inicio(
    State(estado): State<Arc<EstadoAplicacao>>,
    jar: CookieJar,
) -> Result<impl IntoResponse, ErroAplicacao> {
    let Some(usuario_id) = usuario_atual(&jar, &estado) else {
        return Ok(Redirect::to("/entrar").into_response());
    };
    let usuario = sqlx::query_as::<_, Usuario>(
        "SELECT id, nome, email, senha_hash, criado_em FROM usuarios WHERE id = $1",
    )
    .bind(usuario_id)
    .fetch_optional(&estado.banco)
    .await
    .map_err(|_| ErroAplicacao::Interno)?;
    let Some(usuario) = usuario else {
        return Ok(Redirect::to("/entrar").into_response());
    };
    let ativos = sqlx::query_as::<_, Ativo>("SELECT id, codigo, nome, tipo, quantidade, preco_medio, preco_atual FROM ativos WHERE usuario_id = $1 ORDER BY codigo")
        .bind(usuario_id).fetch_all(&estado.banco).await.map_err(|_| ErroAplicacao::Interno)?;
    let total_investido = ativos.iter().map(Ativo::valor_investido).sum();
    let total_atual = ativos.iter().map(Ativo::valor_atual).sum();
    Ok(renderizar(PaginaInicio {
        nome: usuario.nome,
        resultado: total_atual - total_investido,
        ativos,
        total_investido,
        total_atual,
    })
    .into_response())
}

pub async fn adicionar_ativo(
    State(estado): State<Arc<EstadoAplicacao>>,
    jar: CookieJar,
    Form(formulario): Form<FormularioAtivo>,
) -> Result<impl IntoResponse, ErroAplicacao> {
    let usuario_id = usuario_atual(&jar, &estado).ok_or(ErroAplicacao::NaoAutorizado)?;
    if formulario.codigo.trim().is_empty()
        || formulario.nome.trim().is_empty()
        || formulario.quantidade <= 0.0
        || formulario.preco_medio < 0.0
        || formulario.preco_atual < 0.0
    {
        return Ok(Redirect::to("/").into_response());
    }
    sqlx::query("INSERT INTO ativos (id, usuario_id, codigo, nome, tipo, quantidade, preco_medio, preco_atual) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) ON CONFLICT (usuario_id, codigo) DO UPDATE SET nome = EXCLUDED.nome, tipo = EXCLUDED.tipo, quantidade = EXCLUDED.quantidade, preco_medio = EXCLUDED.preco_medio, preco_atual = EXCLUDED.preco_atual")
        .bind(Uuid::new_v4()).bind(usuario_id).bind(formulario.codigo.trim().to_uppercase()).bind(formulario.nome.trim()).bind(formulario.tipo.trim()).bind(formulario.quantidade).bind(formulario.preco_medio).bind(formulario.preco_atual)
        .execute(&estado.banco).await.map_err(|_| ErroAplicacao::Interno)?;
    Ok(Redirect::to("/").into_response())
}

pub async fn excluir_ativo(
    State(estado): State<Arc<EstadoAplicacao>>,
    jar: CookieJar,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, ErroAplicacao> {
    let usuario_id = usuario_atual(&jar, &estado).ok_or(ErroAplicacao::NaoAutorizado)?;
    sqlx::query("DELETE FROM ativos WHERE id = $1 AND usuario_id = $2")
        .bind(id)
        .bind(usuario_id)
        .execute(&estado.banco)
        .await
        .map_err(|_| ErroAplicacao::Interno)?;
    Ok(Redirect::to("/").into_response())
}

fn usuario_atual(jar: &CookieJar, estado: &EstadoAplicacao) -> Option<Uuid> {
    jar.get(NOME_COOKIE)
        .and_then(|cookie| estado.jwt.usuario_do_token(cookie.value()).ok())
}

fn resposta_com_sessao(
    jar: CookieJar,
    estado: &EstadoAplicacao,
    usuario_id: Uuid,
) -> Result<(CookieJar, Redirect), ErroAplicacao> {
    let token = estado.jwt.criar_token(usuario_id)?;
    let cookie = Cookie::build((NOME_COOKIE, token))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .build();
    Ok((jar.add(cookie), Redirect::to("/")))
}

fn renderizar<T: Template>(pagina: T) -> Html<String> {
    Html(
        pagina
            .render()
            .unwrap_or_else(|_| "<h1>Erro ao renderizar página</h1>".into()),
    )
}

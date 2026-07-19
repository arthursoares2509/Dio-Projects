# Carteira de Investimentos

Aplicação Fullstack em Rust para registrar e acompanhar ativos de investimento.

## Tecnologias

- Axum para rotas HTTP
- PostgreSQL com SQLx
- Askama para páginas HTML renderizadas no servidor
- JWT em cookie HTTP-only e senhas protegidas com bcrypt

## Execução

1. Copie `.env.exemplo` para `.env` e ajuste as variáveis.
2. Inicie o banco com `docker compose up -d`.
3. Execute a migração em `migrations/0001_estrutura.sql` no banco.
4. Rode `cargo run` e abra `http://localhost:3000`.

## Funcionalidades

- Cadastro e entrada de usuários
- Sessão segura por JWT
- Inclusão, atualização e exclusão de ativos
- Total investido, valor atual e resultado por ativo e da carteira

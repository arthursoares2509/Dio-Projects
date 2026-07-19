CREATE TABLE usuarios (
    id UUID PRIMARY KEY,
    nome VARCHAR(120) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    senha_hash VARCHAR(255) NOT NULL,
    criado_em TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE ativos (
    id UUID PRIMARY KEY,
    usuario_id UUID NOT NULL REFERENCES usuarios(id) ON DELETE CASCADE,
    codigo VARCHAR(16) NOT NULL,
    nome VARCHAR(120) NOT NULL,
    tipo VARCHAR(30) NOT NULL,
    quantidade DOUBLE PRECISION NOT NULL CHECK (quantidade > 0),
    preco_medio DOUBLE PRECISION NOT NULL CHECK (preco_medio >= 0),
    preco_atual DOUBLE PRECISION NOT NULL CHECK (preco_atual >= 0),
    criado_em TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(usuario_id, codigo)
);

CREATE INDEX idx_ativos_usuario_id ON ativos(usuario_id);

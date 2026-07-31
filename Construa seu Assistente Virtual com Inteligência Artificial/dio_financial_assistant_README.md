# Assistente Financeiro Inteligente (Desafio DIO)

Aplicação web desenvolvida em Python com Streamlit para o desafio de IA Generativa e Relacionamento Financeiro da DIO. O projeto integra compreensão de linguagem natural via LLM, persistência de contexto de conversação, motor de cálculos matemáticos financeiros e visualização gráfica de dados com foco em UX.

## Funcionalidades
- **FAQ Inteligente & Assistente de Produtos:** Chatbot integrado com IA do Google Gemini com prompt especializado em consultoria financeira de alta clareza e segurança.
- **Simulador Financeiro Matemático:** Motor em Python puro que calcula juros compostos considerando aportes mensais e prazos variados.
- **Visualização de Dados:** Gráficos interativos gerados com Plotly para acompanhamento da evolução patrimonial.
- **Gestão de Contexto:** Histórico de conversas preservado via sessão nativa do framework.

## Tecnologias Utilizadas
- **Python**
- **Streamlit**
- **Google GenAI SDK (Gemini)**
- **Pandas & Plotly**

## Como Executar
1. Clone o repositório.
2. Instale as dependências:
   ```bash
   pip install -r requirements.txt
   ```
3. Configure sua chave de API do Gemini no arquivo `.env` ou diretamente na barra lateral da aplicação:
   ```env
   GEMINI_API_KEY=sua_chave_aqui
   ```
4. Inicie a aplicação:
   ```bash
   streamlit run app.py
   ```

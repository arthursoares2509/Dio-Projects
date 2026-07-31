import streamlit as st
import os
from dotenv import load_dotenv
from services.ai_service import get_gemini_response
from utils.financial_math import calculate_compound_interest
import plotly.express as px

load_dotenv()

st.set_page_config(
    page_title="Consultor Financeiro Inteligente - DIO",
    page_icon="💰",
    layout="wide"
)

st.title("💰 Consultor Financeiro com IA Generativa")
st.markdown("Experiência digital de relacionamento financeiro guiada por IA, combinando linguagem natural e cálculos precisos.")

# Sidebar para configurações e simulação rápida
with st.sidebar:
    st.header("Configurações & Ferramentas")
    api_key_input = st.text_input("Gemini API Key", value=os.getenv("GEMINI_API_KEY", ""), type="password")
    
    st.divider()
    st.subheader("Simulador de Juros Compostos")
    principal = st.number_input("Valor Inicial (R$)", value=1000.0, step=500.0)
    monthly = st.number_input("Aporte Mensal (R$)", value=300.0, step=100.0)
    rate = st.number_input("Taxa Anual (% a.a.)", value=10.0, step=0.5)
    years = st.slider("Prazo (Anos)", 1, 30, 5)
    
    if st.button("Executar Simulação Matemática"):
        st.session_state["sim_df"] = calculate_compound_interest(principal, monthly, rate, years)
        st.success("Simulação executada com sucesso via Python!")

# Inicialização do histórico de chat
if "messages" not in st.session_state:
    st.session_state["messages"] = [
        {"role": "assistant", "content": "Olá! Sou seu assistente financeiro inteligente. Como posso ajudar com suas dúvidas sobre investimentos, planejamento ou produtos financeiros hoje?"}
    ]

# Layout em abas para separar Chat (FAQ/IA) de Simulações Gráficas
tab1, tab2 = st.tabs(["💬 Assistente IA (FAQ & Produtos)", "📊 Resultados e Gráficos de Simulação"])

with tab1:
    st.markdown("### Chat com IA Generativa")
    
    for message in st.session_state["messages"]:
        with st.chat_message(message["role"]):
            st.markdown(message["content"])
            
    if user_input := st.chat_input("Digite sua dúvida financeira..."):
        st.session_state["messages"].append({"role": "user", "content": user_input})
        with st.chat_message("user"):
            st.markdown(user_input)
            
        with st.chat_message("assistant"):
            with st.spinner("Processando resposta com IA..."):
                response = get_gemini_response(st.session_state["messages"][:-1], user_input, api_key_input)
                st.markdown(response)
                st.session_state["messages"].append({"role": "assistant", "content": response})

with tab2:
    st.markdown("### Análise Gráfica de Investimentos")
    if "sim_df" in st.session_state:
        df = st.session_state["sim_df"]
        
        col1, col2, col3 = st.columns(3)
        col1.metric("Total Investido", f"R$ {df['Total Investido'].iloc[-1]:,.2f}")
        col2.metric("Montante Final", f"R$ {df['Montante Final'].iloc[-1]:,.2f}")
        col3.metric("Juros Acumulados", f"R$ {df['Juros Acumulados'].iloc[-1]:,.2f}")
        
        fig = px.area(df, x="Mes", y=["Total Investido", "Montante Final"],
                      labels={"value": "Valor (R$)", "Mes": "Meses", "variable": "Legenda"},
                      title="Evolução do Patrimônio ao Longo do Tempo")
        st.plotly_chart(fig, use_container_width=True)
        
        st.dataframe(df, use_container_width=True)
    else:
        st.info("Utilize a barra lateral à esquerda para configurar e rodar a simulação matemática.")
